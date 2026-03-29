use anyhow::{Context, Result};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
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
    #[error("Migration failed: {0}")]
    MigrationError(String),
    #[error("Key derivation failed: {0}")]
    KeyDerivationError(String),
}

/// Legacy credential store format (for migration only)
#[derive(Debug, Serialize, Deserialize)]
struct LegacyCredentialStore {
    credentials: HashMap<String, LegacyEncryptedCredential>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LegacyEncryptedCredential {
    encrypted_value: String,
    salt: String,
    created_at: String,
}

/// Secure credential manager using age backend
///
/// This replaces the legacy XOR-based storage with proper age encryption.
/// Legacy credentials.json files are automatically migrated on first access.
pub struct SecureCredentialManager {
    backend: Box<dyn CredentialBackend>,
    legacy_store_path: PathBuf,
    derived_key: Option<[u8; 32]>,
}

impl SecureCredentialManager {
    pub fn new() -> Result<Self> {
        let legacy_store_path = dirs::config_dir()
            .ok_or_else(|| {
                CredentialError::FileError("Could not determine config directory".to_string())
            })?
            .join("ghostctl")
            .join("credentials.json");

        // Use the best available backend (age or pass)
        let backend = credential_backends::detect_backend(None);

        Ok(Self {
            backend,
            legacy_store_path,
            derived_key: None,
        })
    }

    /// Unlock the credential manager with a master password
    /// Uses Argon2 for secure key derivation
    pub fn unlock(&mut self, master_password: &str) -> Result<()> {
        // Generate salt from system entropy
        let mut salt_bytes = [0u8; 16];
        getrandom::getrandom(&mut salt_bytes)
            .map_err(|e| CredentialError::KeyDerivationError(format!("RNG failed: {}", e)))?;
        let salt = SaltString::encode_b64(&salt_bytes).map_err(|e| {
            CredentialError::KeyDerivationError(format!("Salt encoding failed: {}", e))
        })?;

        // Derive a key using Argon2
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(master_password.as_bytes(), &salt)
            .map_err(|e| CredentialError::KeyDerivationError(e.to_string()))?;

        // Extract the hash output as our derived key
        let hash_output = password_hash
            .hash
            .ok_or_else(|| CredentialError::KeyDerivationError("No hash output".to_string()))?;

        let mut key = [0u8; 32];
        let hash_bytes = hash_output.as_bytes();
        let copy_len = hash_bytes.len().min(32);
        key[..copy_len].copy_from_slice(&hash_bytes[..copy_len]);

        self.derived_key = Some(key);

        // Check for and migrate legacy credentials
        if self.legacy_store_path.exists() {
            self.migrate_legacy_credentials(master_password)?;
        }

        Ok(())
    }

    /// Migrate credentials from legacy XOR-encrypted format to age backend
    fn migrate_legacy_credentials(&self, legacy_master_key: &str) -> Result<()> {
        if !self.backend.is_available() {
            log::warn!("Cannot migrate legacy credentials: no secure backend available");
            return Ok(());
        }

        log::info!("Migrating legacy credentials to secure backend...");

        let content = fs::read_to_string(&self.legacy_store_path).with_context(|| {
            format!(
                "Failed to read legacy credential store: {}",
                self.legacy_store_path.display()
            )
        })?;

        let store: LegacyCredentialStore =
            serde_json::from_str(&content).context("Failed to parse legacy credential store")?;

        let mut migrated = 0;
        let mut failed = 0;

        for (key, credential) in store.credentials {
            // Decrypt with legacy XOR method
            match legacy_xor_decrypt(&credential.encrypted_value, legacy_master_key) {
                Ok(plaintext) => {
                    // Store in new backend
                    if let Err(e) = self.backend.store(&key, &plaintext) {
                        log::warn!("Failed to migrate credential '{}': {}", key, e);
                        failed += 1;
                    } else {
                        migrated += 1;
                    }
                }
                Err(e) => {
                    log::warn!("Failed to decrypt legacy credential '{}': {}", key, e);
                    failed += 1;
                }
            }
        }

        if failed == 0 {
            // All migrated successfully, remove legacy file
            let backup_path = self.legacy_store_path.with_extension("json.migrated");
            fs::rename(&self.legacy_store_path, &backup_path).with_context(|| {
                format!("Failed to backup legacy store to {}", backup_path.display())
            })?;
            log::info!(
                "Migrated {} credentials. Legacy store backed up to {}",
                migrated,
                backup_path.display()
            );
        } else {
            log::warn!(
                "Migrated {} credentials, {} failed. Legacy store preserved.",
                migrated,
                failed
            );
        }

        Ok(())
    }

    pub fn store_credential(&self, key: &str, value: &str) -> Result<()> {
        if !self.backend.is_available() {
            return Err(CredentialError::NoBackend.into());
        }

        self.backend.store(key, value)?;
        log::info!("Stored credential: {}", key);
        Ok(())
    }

    pub fn get_credential(&self, key: &str) -> Result<String> {
        if !self.backend.is_available() {
            return Err(CredentialError::NoBackend.into());
        }

        self.backend.get(key)
    }

    pub fn list_credentials(&self) -> Result<Vec<String>> {
        if !self.backend.is_available() {
            return Err(CredentialError::NoBackend.into());
        }

        self.backend.list()
    }

    pub fn delete_credential(&self, key: &str) -> Result<()> {
        if !self.backend.is_available() {
            return Err(CredentialError::NoBackend.into());
        }

        self.backend.delete(key)?;
        log::info!("Deleted credential: {}", key);
        Ok(())
    }

    /// Get the name of the active backend
    pub fn backend_name(&self) -> &'static str {
        self.backend.name()
    }

    /// Check if a secure backend is available
    pub fn is_available(&self) -> bool {
        self.backend.is_available()
    }

    #[deprecated(note = "Use unlock() instead which derives key securely")]
    pub fn set_master_key(&mut self, _key: String) {
        log::warn!("set_master_key is deprecated, use unlock() instead");
    }
}

/// Legacy XOR decryption for migration purposes only
fn legacy_xor_decrypt(ciphertext: &str, key: &str) -> Result<String, String> {
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

/// Get the best available credential backend
pub fn get_secure_backend() -> Option<Box<dyn credential_backends::CredentialBackend>> {
    // Try age backend first (modern, simpler)
    let age = credential_backends::AgeBackend::new(None);
    if age.is_available() {
        return Some(Box::new(age));
    }

    // Try pass backend (GPG-based)
    let pass = credential_backends::PassBackend::new(None);
    if pass.is_available() {
        return Some(Box::new(pass));
    }

    None
}

pub fn credential_management() {
    use dialoguer::{Select, theme::ColorfulTheme};

    println!("Secure Credential Management");
    println!("============================");

    let options = [
        "Store credential",
        "Retrieve credential",
        "List credentials",
        "Delete credential",
        "Setup/unlock backend",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Credential Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(choice)) => choice,
        Ok(None) => return, // User cancelled
        Err(e) => {
            println!("Menu selection failed: {}", e);
            return;
        }
    };

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
    if !manager.is_available() {
        println!("No secure credential backend available.");
        println!("Install 'age' or 'pass' to enable secure credential storage.");
        return;
    }

    let key: String = match dialoguer::Input::new()
        .with_prompt("Credential name/key")
        .interact_text()
    {
        Ok(key) => key,
        Err(e) => {
            println!("Input failed: {}", e);
            return;
        }
    };

    let value: String = match dialoguer::Password::new()
        .with_prompt("Credential value (hidden)")
        .interact()
    {
        Ok(value) => value,
        Err(e) => {
            println!("Input failed: {}", e);
            return;
        }
    };

    match manager.store_credential(&key, &value) {
        Ok(_) => println!(
            "Credential '{}' stored securely via {}",
            key,
            manager.backend_name()
        ),
        Err(e) => println!("Failed to store credential: {}", e),
    }
}

/// Try to copy text to clipboard using available clipboard tools
fn copy_to_clipboard(text: &str) -> bool {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let clipboard_tools = ["xclip", "wl-copy", "pbcopy"];

    for tool in &clipboard_tools {
        // Check if tool exists
        if Command::new("which")
            .arg(tool)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|s| s.success())
        {
            let mut cmd = Command::new(tool);

            if *tool == "xclip" {
                cmd.args(["-selection", "clipboard"]);
            }

            if let Ok(mut child) = cmd.stdin(Stdio::piped()).stdout(Stdio::null()).spawn()
                && let Some(mut stdin) = child.stdin.take()
                && stdin.write_all(text.as_bytes()).is_ok()
            {
                drop(stdin);
                if child.wait().is_ok_and(|s| s.success()) {
                    return true;
                }
            }
        }
    }
    false
}

fn retrieve_credential_interactive(manager: &mut SecureCredentialManager) {
    if !manager.is_available() {
        println!("No secure credential backend available.");
        return;
    }

    let key: String = match dialoguer::Input::new()
        .with_prompt("Credential name/key")
        .interact_text()
    {
        Ok(key) => key,
        Err(e) => {
            println!("Input failed: {}", e);
            return;
        }
    };

    match manager.get_credential(&key) {
        Ok(value) => {
            // Security: Ask user before displaying credential in plain text
            let show_value = dialoguer::Confirm::new()
                .with_prompt("Display credential value in plain text? (security risk)")
                .default(false)
                .interact()
                .unwrap_or(false);

            if show_value {
                println!("Credential '{}': {}", key, value);
            } else {
                // Copy to clipboard if available, otherwise show masked
                if copy_to_clipboard(&value) {
                    println!("Credential '{}' copied to clipboard", key);
                } else {
                    println!(
                        "Credential '{}': {} (length: {} chars)",
                        key,
                        "*".repeat(value.len().min(20)),
                        value.len()
                    );
                }
            }
        }
        Err(e) => println!("Failed to retrieve credential: {}", e),
    }
}

fn list_credentials_interactive(manager: &SecureCredentialManager) {
    if !manager.is_available() {
        println!("No secure credential backend available.");
        return;
    }

    match manager.list_credentials() {
        Ok(keys) => {
            if keys.is_empty() {
                println!("No credentials stored");
            } else {
                println!("Stored credentials ({}):", manager.backend_name());
                for key in keys {
                    println!("  - {}", key);
                }
            }
        }
        Err(e) => println!("Failed to list credentials: {}", e),
    }
}

fn delete_credential_interactive(manager: &mut SecureCredentialManager) {
    if !manager.is_available() {
        println!("No secure credential backend available.");
        return;
    }

    let key: String = match dialoguer::Input::new()
        .with_prompt("Credential name/key to delete")
        .interact_text()
    {
        Ok(key) => key,
        Err(e) => {
            println!("Input failed: {}", e);
            return;
        }
    };

    match manager.delete_credential(&key) {
        Ok(_) => println!("Credential '{}' deleted", key),
        Err(e) => println!("Failed to delete credential: {}", e),
    }
}

fn setup_master_key_interactive(manager: &mut SecureCredentialManager) {
    let master_key: String = match dialoguer::Password::new()
        .with_prompt("Enter master password")
        .interact()
    {
        Ok(key) => key,
        Err(e) => {
            println!("Input failed: {}", e);
            return;
        }
    };

    if let Err(e) = manager.unlock(&master_key) {
        println!("Failed to unlock: {}", e);
    } else {
        println!("Unlocked using {} backend", manager.backend_name());
    }
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
