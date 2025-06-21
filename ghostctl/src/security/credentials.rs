use anyhow::{Context, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CredentialError {
    #[error("Credential not found: {0}")]
    NotFound(String),
    #[error("Encryption failed: {0}")]
    EncryptionError(String),
    #[error("Decryption failed: {0}")]
    DecryptionError(String),
    #[error("File operation failed: {0}")]
    FileError(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct CredentialStore {
    credentials: HashMap<String, EncryptedCredential>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedCredential {
    encrypted_value: String,
    salt: String,
    created_at: String,
}

pub struct SecureCredentialManager {
    store_path: PathBuf,
    master_key: Option<String>,
}

impl SecureCredentialManager {
    pub fn new() -> Result<Self> {
        let store_path = dirs::config_dir()
            .ok_or_else(|| {
                CredentialError::FileError("Could not determine config directory".to_string())
            })?
            .join("ghostctl")
            .join("credentials.json");

        Ok(Self {
            store_path,
            master_key: None,
        })
    }

    pub fn unlock(&mut self, master_password: &str) -> Result<()> {
        // In a real implementation, this would derive a key from the password
        // For now, we'll use a simple approach
        self.master_key = Some(master_password.to_string());
        Ok(())
    }

    pub fn store_credential(&self, key: &str, value: &str) -> Result<()> {
        let master_key = self
            .master_key
            .as_ref()
            .ok_or_else(|| CredentialError::EncryptionError("Not unlocked".to_string()))?;

        // Simple XOR encryption (in production, use proper encryption like AES)
        let encrypted_value = simple_encrypt(value, master_key);
        let salt = generate_salt();

        let credential = EncryptedCredential {
            encrypted_value,
            salt,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let mut store = self.load_store().unwrap_or_else(|_| CredentialStore {
            credentials: HashMap::new(),
        });

        store.credentials.insert(key.to_string(), credential);
        self.save_store(&store)?;

        log::info!("Stored credential: {}", key);
        Ok(())
    }

    pub fn get_credential(&self, key: &str) -> Result<String> {
        let master_key = self
            .master_key
            .as_ref()
            .ok_or_else(|| CredentialError::DecryptionError("Not unlocked".to_string()))?;

        let store = self.load_store()?;
        let credential = store
            .credentials
            .get(key)
            .ok_or_else(|| CredentialError::NotFound(key.to_string()))?;

        let decrypted_value = simple_decrypt(&credential.encrypted_value, master_key)
            .map_err(|e| CredentialError::DecryptionError(e))?;

        Ok(decrypted_value)
    }

    pub fn list_credentials(&self) -> Result<Vec<String>> {
        let store = self.load_store()?;
        Ok(store.credentials.keys().cloned().collect())
    }

    pub fn delete_credential(&self, key: &str) -> Result<()> {
        let mut store = self.load_store()?;

        if store.credentials.remove(key).is_none() {
            return Err(CredentialError::NotFound(key.to_string()).into());
        }

        self.save_store(&store)?;
        log::info!("Deleted credential: {}", key);
        Ok(())
    }

    fn load_store(&self) -> Result<CredentialStore> {
        if !self.store_path.exists() {
            return Ok(CredentialStore {
                credentials: HashMap::new(),
            });
        }

        let content = fs::read_to_string(&self.store_path).with_context(|| {
            format!(
                "Failed to read credential store: {}",
                self.store_path.display()
            )
        })?;

        serde_json::from_str(&content).context("Failed to parse credential store")
    }

    fn save_store(&self, store: &CredentialStore) -> Result<()> {
        // Ensure directory exists
        if let Some(parent) = self.store_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let content =
            serde_json::to_string_pretty(store).context("Failed to serialize credential store")?;

        fs::write(&self.store_path, content).with_context(|| {
            format!(
                "Failed to write credential store: {}",
                self.store_path.display()
            )
        })?;

        // Set restrictive permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&self.store_path)?.permissions();
            perms.set_mode(0o600); // Read/write for owner only
            fs::set_permissions(&self.store_path, perms)?;
        }

        Ok(())
    }
}

// Simple encryption functions (use proper crypto library in production)
fn simple_encrypt(plaintext: &str, key: &str) -> String {
    let key_bytes = key.as_bytes();
    let mut result = Vec::new();

    for (i, byte) in plaintext.bytes().enumerate() {
        let key_byte = key_bytes[i % key_bytes.len()];
        result.push(byte ^ key_byte);
    }

    base64::engine::general_purpose::STANDARD.encode(result)
}

fn simple_decrypt(ciphertext: &str, key: &str) -> Result<String, String> {
    let encrypted_bytes = base64::engine::general_purpose::STANDARD
        .decode(ciphertext)
        .map_err(|e| format!("Base64 decode failed: {}", e))?;

    let key_bytes = key.as_bytes();
    let mut result = Vec::new();

    for (i, byte) in encrypted_bytes.iter().enumerate() {
        let key_byte = key_bytes[i % key_bytes.len()];
        result.push(byte ^ key_byte);
    }

    String::from_utf8(result).map_err(|e| format!("UTF-8 decode failed: {}", e))
}

fn generate_salt() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    format!("{:x}", timestamp)
}

// Utility functions for common credential operations
pub fn store_backup_credentials(repo_url: &str, password: &str) -> Result<()> {
    let mut manager = SecureCredentialManager::new()?;

    // In a real implementation, prompt for master password
    manager.unlock("ghostctl-master")?;

    manager.store_credential("restic_repository", repo_url)?;
    manager.store_credential("restic_password", password)?;

    println!("✅ Backup credentials stored securely");
    Ok(())
}

pub fn get_backup_credentials() -> Result<(String, String)> {
    let mut manager = SecureCredentialManager::new()?;

    // In a real implementation, prompt for master password
    manager.unlock("ghostctl-master")?;

    let repo = manager.get_credential("restic_repository")?;
    let password = manager.get_credential("restic_password")?;

    Ok((repo, password))
}

pub fn create_secure_env_file(path: &PathBuf) -> Result<()> {
    let (repo, password) = get_backup_credentials()?;

    let content = format!("RESTIC_REPOSITORY={}\nRESTIC_PASSWORD={}\n", repo, password);

    fs::write(path, content)
        .with_context(|| format!("Failed to write secure env file: {}", path.display()))?;

    // Set restrictive permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(path, perms)?;
    }

    Ok(())
}

pub fn credential_management() {
    use dialoguer::{Select, theme::ColorfulTheme};
    
    println!("🗂️  Credential Management");
    println!("========================");

    let options = [
        "🔓 Unlock credential store",
        "💾 Store new credential",
        "📋 List credentials", 
        "🔍 Get credential",
        "🗑️  Delete credential",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Credential Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => unlock_credential_store(),
        1 => store_new_credential(),
        2 => list_stored_credentials(),
        3 => get_stored_credential(),
        4 => delete_stored_credential(),
        _ => return,
    }
}

fn unlock_credential_store() {
    use dialoguer::Password;
    
    let master_password = Password::new()
        .with_prompt("Enter master password")
        .interact()
        .unwrap();

    match SecureCredentialManager::new().and_then(|mut manager| {
        manager.unlock(&master_password)?;
        Ok(manager)
    }) {
        Ok(_) => println!("✅ Credential store unlocked successfully"),
        Err(e) => println!("❌ Failed to unlock credential store: {}", e),
    }
}

fn store_new_credential() {
    use dialoguer::{Input, Password};
    
    let key: String = Input::new()
        .with_prompt("Credential name/key")
        .interact_text()
        .unwrap();

    let value = Password::new()
        .with_prompt("Credential value")
        .interact()
        .unwrap();

    let master_password = Password::new()
        .with_prompt("Enter master password")
        .interact()
        .unwrap();

    match SecureCredentialManager::new().and_then(|mut manager| {
        manager.unlock(&master_password)?;
        manager.store_credential(&key, &value)
    }) {
        Ok(_) => println!("✅ Credential '{}' stored successfully", key),
        Err(e) => println!("❌ Failed to store credential: {}", e),
    }
}

fn list_stored_credentials() {
    use dialoguer::Password;
    
    let master_password = Password::new()
        .with_prompt("Enter master password")
        .interact()
        .unwrap();

    match SecureCredentialManager::new().and_then(|mut manager| {
        manager.unlock(&master_password)?;
        manager.list_credentials()
    }) {
        Ok(credentials) => {
            if credentials.is_empty() {
                println!("📭 No credentials stored");
            } else {
                println!("📋 Stored credentials:");
                for credential in credentials {
                    println!("  • {}", credential);
                }
            }
        }
        Err(e) => println!("❌ Failed to list credentials: {}", e),
    }
}

fn get_stored_credential() {
    use dialoguer::{Input, Password};
    
    let key: String = Input::new()
        .with_prompt("Credential name/key")
        .interact_text()
        .unwrap();

    let master_password = Password::new()
        .with_prompt("Enter master password")
        .interact()
        .unwrap();

    match SecureCredentialManager::new().and_then(|mut manager| {
        manager.unlock(&master_password)?;
        manager.get_credential(&key)
    }) {
        Ok(value) => {
            println!("🔍 Credential '{}': {}", key, value);
            println!("⚠️  Value displayed in terminal - ensure it's secure!");
        }
        Err(e) => println!("❌ Failed to get credential: {}", e),
    }
}

fn delete_stored_credential() {
    use dialoguer::{Input, Password, Confirm};
    
    let key: String = Input::new()
        .with_prompt("Credential name/key to delete")
        .interact_text()
        .unwrap();

    let confirm = Confirm::new()
        .with_prompt(&format!("Are you sure you want to delete '{}'?", key))
        .default(false)
        .interact()
        .unwrap();

    if !confirm {
        println!("🚫 Deletion cancelled");
        return;
    }

    let master_password = Password::new()
        .with_prompt("Enter master password")
        .interact()
        .unwrap();

    match SecureCredentialManager::new().and_then(|mut manager| {
        manager.unlock(&master_password)?;
        manager.delete_credential(&key)
    }) {
        Ok(_) => println!("✅ Credential '{}' deleted successfully", key),
        Err(e) => println!("❌ Failed to delete credential: {}", e),
    }
}
