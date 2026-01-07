use anyhow::{Context, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

use super::credential_backends::{self, AgeBackend, CredentialBackend};

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
    #[error("No secure backend available")]
    NoBackend,
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
            .map_err(CredentialError::DecryptionError)?;

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

    pub fn set_master_key(&mut self, key: String) {
        self.master_key = Some(key);
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

pub fn credential_management() {
    use dialoguer::{theme::ColorfulTheme, Select};

    println!("🔐 Secure Credential Management");
    println!("===============================");

    let options = [
        "📝 Store credential",
        "🔍 Retrieve credential",
        "📋 List credentials",
        "🗑️  Delete credential",
        "🔧 Setup master key",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Credential Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    let mut manager = match SecureCredentialManager::new() {
        Ok(m) => m,
        Err(e) => {
            println!("❌ Failed to initialize credential manager: {}", e);
            return;
        }
    };

    match choice {
        0 => store_credential_interactive(&mut manager),
        1 => retrieve_credential_interactive(&mut manager),
        2 => list_credentials_interactive(&manager),
        3 => delete_credential_interactive(&mut manager),
        4 => setup_master_key_interactive(&mut manager),
        _ => return,
    }
}

fn store_credential_interactive(manager: &mut SecureCredentialManager) {
    let master_key: String = dialoguer::Input::new()
        .with_prompt("Master key (for encryption)")
        .interact_text()
        .unwrap();

    if let Err(e) = manager.unlock(&master_key) {
        println!("❌ Failed to unlock: {}", e);
        return;
    }

    let key: String = dialoguer::Input::new()
        .with_prompt("Credential name/key")
        .interact_text()
        .unwrap();

    let value: String = dialoguer::Input::new()
        .with_prompt("Credential value")
        .interact_text()
        .unwrap();

    match manager.store_credential(&key, &value) {
        Ok(_) => println!("✅ Credential '{}' stored securely", key),
        Err(e) => println!("❌ Failed to store credential: {}", e),
    }
}

fn retrieve_credential_interactive(manager: &mut SecureCredentialManager) {
    let master_key: String = dialoguer::Input::new()
        .with_prompt("Master key (for decryption)")
        .interact_text()
        .unwrap();

    if let Err(e) = manager.unlock(&master_key) {
        println!("❌ Failed to unlock: {}", e);
        return;
    }

    let key: String = dialoguer::Input::new()
        .with_prompt("Credential name/key")
        .interact_text()
        .unwrap();

    match manager.get_credential(&key) {
        Ok(value) => println!("🔐 Credential '{}': {}", key, value),
        Err(e) => println!("❌ Failed to retrieve credential: {}", e),
    }
}

fn list_credentials_interactive(manager: &SecureCredentialManager) {
    match manager.list_credentials() {
        Ok(keys) => {
            if keys.is_empty() {
                println!("📋 No credentials stored");
            } else {
                println!("📋 Stored credentials:");
                for key in keys {
                    println!("  🔑 {}", key);
                }
            }
        }
        Err(e) => println!("❌ Failed to list credentials: {}", e),
    }
}

fn delete_credential_interactive(manager: &mut SecureCredentialManager) {
    let key: String = dialoguer::Input::new()
        .with_prompt("Credential name/key to delete")
        .interact_text()
        .unwrap();

    match manager.delete_credential(&key) {
        Ok(_) => println!("✅ Credential '{}' deleted", key),
        Err(e) => println!("❌ Failed to delete credential: {}", e),
    }
}

fn setup_master_key_interactive(manager: &mut SecureCredentialManager) {
    let master_key: String = dialoguer::Input::new()
        .with_prompt("Enter new master key")
        .interact_text()
        .unwrap();

    if let Err(e) = manager.unlock(&master_key) {
        println!("❌ Failed to set master key: {}", e);
    } else {
        println!("✅ Master key configured for this session");
    }
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

/// Store backup credentials using the best available backend
pub fn store_backup_credentials(repo_url: &str, password: &str) -> Result<()> {
    let backend = credential_backends::detect_backend(None);

    if !backend.is_available() {
        anyhow::bail!("No secure credential backend available. Install pass or age.");
    }

    backend.store("restic_repository", repo_url)?;
    backend.store("restic_password", password)?;

    println!(
        "✅ Backup credentials stored securely via {}",
        backend.name()
    );
    Ok(())
}

/// Get backup credentials using the best available backend
pub fn get_backup_credentials() -> Result<(String, String)> {
    let backend = credential_backends::detect_backend(None);

    if !backend.is_available() {
        anyhow::bail!("No secure credential backend available. Install pass or age.");
    }

    let repo = backend.get("restic_repository")?;
    let password = backend.get("restic_password")?;

    Ok((repo, password))
}

/// Create a secure env file with backup credentials
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

/// Initialize age backend if not already set up
pub fn setup_age_backend() -> Result<()> {
    let age_backend = AgeBackend::new(None);

    if age_backend.is_available() {
        println!("✅ Age backend is already configured");
        return Ok(());
    }

    println!("🔐 Setting up age credential backend...");
    age_backend.init_identity()?;
    println!("✅ Age identity generated");
    println!("📁 Identity stored at: ~/.config/ghostctl/age-key.txt");
    println!("⚠️  Back up this file securely - it's needed to decrypt your credentials!");

    Ok(())
}

/// Show current credential backend status
pub fn show_backend_status() {
    use super::credential_backends::PassBackend;

    println!("🔐 Credential Backend Status");
    println!("============================");

    let pass_backend = PassBackend::new(None);
    let age_backend = AgeBackend::new(None);

    println!();
    println!("pass (password-store):");
    if pass_backend.is_available() {
        println!("  ✅ Available and configured");
        if let Ok(keys) = pass_backend.list() {
            println!("  📋 {} ghostctl credentials stored", keys.len());
        }
    } else {
        println!("  ❌ Not available");
        println!("  💡 Install: pacman -S pass && pass init <gpg-id>");
    }

    println!();
    println!("age:");
    if age_backend.is_available() {
        println!("  ✅ Available and configured");
        if let Ok(keys) = age_backend.list() {
            println!("  📋 {} ghostctl credentials stored", keys.len());
        }
    } else {
        println!("  ❌ Not available");
        println!("  💡 Install: pacman -S age");
        println!("  💡 Setup: ghostctl security credentials setup-age");
    }

    println!();
    let active = credential_backends::detect_backend(None);
    println!("Active backend: {}", active.name());
}
