//! Credential storage backends for secure secret management
//!
//! Supports multiple backends:
//! - `pass` (password-store): GPG-based, git-syncable
//! - `age`: Modern encryption, simpler key management
//! - `builtin`: XOR-based fallback (not recommended for sensitive data)

use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// Trait for credential storage backends
pub trait CredentialBackend: Send + Sync {
    /// Get the name of this backend
    fn name(&self) -> &'static str;

    /// Check if this backend is available on the system
    fn is_available(&self) -> bool;

    /// Store a credential
    fn store(&self, key: &str, value: &str) -> Result<()>;

    /// Retrieve a credential
    fn get(&self, key: &str) -> Result<String>;

    /// Delete a credential
    fn delete(&self, key: &str) -> Result<()>;

    /// List all stored credential keys
    fn list(&self) -> Result<Vec<String>>;
}

/// Password-store (pass) backend
/// Uses GPG encryption, supports git sync
pub struct PassBackend {
    store_dir: PathBuf,
    prefix: String,
}

impl PassBackend {
    pub fn new(store_dir: Option<PathBuf>) -> Self {
        let store_dir = store_dir.unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join(".password-store")
        });

        Self {
            store_dir,
            prefix: "ghostctl".to_string(),
        }
    }

    /// Check if pass is installed and initialized
    fn is_pass_installed() -> bool {
        Command::new("which")
            .arg("pass")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Get the full pass path for a key
    fn pass_path(&self, key: &str) -> String {
        format!("{}/{}", self.prefix, key)
    }
}

impl CredentialBackend for PassBackend {
    fn name(&self) -> &'static str {
        "pass"
    }

    fn is_available(&self) -> bool {
        Self::is_pass_installed() && self.store_dir.exists()
    }

    fn store(&self, key: &str, value: &str) -> Result<()> {
        let pass_path = self.pass_path(key);

        // Use pass insert with echo to avoid interactive prompt
        let output = Command::new("bash")
            .arg("-c")
            .arg(format!("echo '{}' | pass insert -f '{}'", value, pass_path))
            .output()
            .context("Failed to execute pass insert")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("pass insert failed: {}", stderr);
        }

        log::info!("Stored credential '{}' via pass", key);
        Ok(())
    }

    fn get(&self, key: &str) -> Result<String> {
        let pass_path = self.pass_path(key);

        let output = Command::new("pass")
            .arg("show")
            .arg(&pass_path)
            .output()
            .context("Failed to execute pass show")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("pass show failed: {}", stderr);
        }

        let value = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in pass output")?
            .trim()
            .to_string();

        Ok(value)
    }

    fn delete(&self, key: &str) -> Result<()> {
        let pass_path = self.pass_path(key);

        let output = Command::new("pass")
            .arg("rm")
            .arg("-f")
            .arg(&pass_path)
            .output()
            .context("Failed to execute pass rm")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("pass rm failed: {}", stderr);
        }

        log::info!("Deleted credential '{}' via pass", key);
        Ok(())
    }

    fn list(&self) -> Result<Vec<String>> {
        let ghostctl_dir = self.store_dir.join(&self.prefix);

        if !ghostctl_dir.exists() {
            return Ok(Vec::new());
        }

        let mut keys = Vec::new();
        for entry in std::fs::read_dir(&ghostctl_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "gpg").unwrap_or(false)
                && let Some(stem) = path.file_stem()
            {
                keys.push(stem.to_string_lossy().to_string());
            }
        }

        Ok(keys)
    }
}

/// Age encryption backend
/// Uses age for modern, simple encryption
pub struct AgeBackend {
    identity_path: PathBuf,
    store_dir: PathBuf,
}

impl AgeBackend {
    pub fn new(identity_path: Option<PathBuf>) -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("ghostctl");

        let identity_path = identity_path.unwrap_or_else(|| config_dir.join("age-key.txt"));
        let store_dir = config_dir.join("credentials");

        Self {
            identity_path,
            store_dir,
        }
    }

    /// Check if age is installed
    fn is_age_installed() -> bool {
        Command::new("which")
            .arg("age")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Get the recipient (public key) from the identity file
    fn get_recipient(&self) -> Result<String> {
        // age-keygen outputs: "# public key: age1..."
        // The identity file contains the private key, we need to derive public key
        let output = Command::new("age-keygen")
            .arg("-y")
            .arg(&self.identity_path)
            .output()
            .context("Failed to get age public key")?;

        if !output.status.success() {
            anyhow::bail!("Failed to derive age public key");
        }

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    /// Initialize age identity if not exists
    pub fn init_identity(&self) -> Result<()> {
        if self.identity_path.exists() {
            return Ok(());
        }

        // Create parent directories
        if let Some(parent) = self.identity_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Generate new identity
        let output = Command::new("age-keygen")
            .arg("-o")
            .arg(&self.identity_path)
            .output()
            .context("Failed to generate age identity")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("age-keygen failed: {}", stderr);
        }

        // Set restrictive permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&self.identity_path)?.permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(&self.identity_path, perms)?;
        }

        log::info!("Generated new age identity at {:?}", self.identity_path);
        Ok(())
    }

    /// Get credential file path
    fn credential_path(&self, key: &str) -> PathBuf {
        self.store_dir.join(format!("{}.age", key))
    }
}

impl CredentialBackend for AgeBackend {
    fn name(&self) -> &'static str {
        "age"
    }

    fn is_available(&self) -> bool {
        Self::is_age_installed() && self.identity_path.exists()
    }

    fn store(&self, key: &str, value: &str) -> Result<()> {
        // Ensure store directory exists
        std::fs::create_dir_all(&self.store_dir)?;

        let recipient = self.get_recipient()?;
        let cred_path = self.credential_path(key);

        // Encrypt using age
        let output = Command::new("bash")
            .arg("-c")
            .arg(format!(
                "echo '{}' | age -r '{}' -o '{}'",
                value,
                recipient,
                cred_path.display()
            ))
            .output()
            .context("Failed to execute age encrypt")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("age encrypt failed: {}", stderr);
        }

        // Set restrictive permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&cred_path)?.permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(&cred_path, perms)?;
        }

        log::info!("Stored credential '{}' via age", key);
        Ok(())
    }

    fn get(&self, key: &str) -> Result<String> {
        let cred_path = self.credential_path(key);

        if !cred_path.exists() {
            anyhow::bail!("Credential '{}' not found", key);
        }

        let output = Command::new("age")
            .arg("-d")
            .arg("-i")
            .arg(&self.identity_path)
            .arg(&cred_path)
            .output()
            .context("Failed to execute age decrypt")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("age decrypt failed: {}", stderr);
        }

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    fn delete(&self, key: &str) -> Result<()> {
        let cred_path = self.credential_path(key);

        if cred_path.exists() {
            std::fs::remove_file(&cred_path)?;
            log::info!("Deleted credential '{}' via age", key);
        }

        Ok(())
    }

    fn list(&self) -> Result<Vec<String>> {
        if !self.store_dir.exists() {
            return Ok(Vec::new());
        }

        let mut keys = Vec::new();
        for entry in std::fs::read_dir(&self.store_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "age").unwrap_or(false)
                && let Some(stem) = path.file_stem()
            {
                keys.push(stem.to_string_lossy().to_string());
            }
        }

        Ok(keys)
    }
}

/// Detect and return the best available credential backend
pub fn detect_backend(preferred: Option<&str>) -> Box<dyn CredentialBackend> {
    // Check environment variable override
    let env_backend = std::env::var("GHOSTCTL_CREDENTIAL_BACKEND").ok();
    let backend_name = preferred.or(env_backend.as_deref()).unwrap_or("auto");

    match backend_name {
        "pass" => {
            let backend = PassBackend::new(None);
            if backend.is_available() {
                return Box::new(backend);
            }
            log::warn!("pass backend requested but not available, falling back to auto");
        }
        "age" => {
            let backend = AgeBackend::new(None);
            if backend.is_available() {
                return Box::new(backend);
            }
            log::warn!("age backend requested but not available, falling back to auto");
        }
        #[cfg(feature = "keyring-backend")]
        "keyring" => {
            let backend = KeyringBackend::new();
            if backend.is_available() {
                return Box::new(backend);
            }
            log::warn!("keyring backend requested but not available, falling back to auto");
        }
        _ => {}
    }

    // Auto-detect: try pass first, then age, then keyring (if available)
    let pass_backend = PassBackend::new(None);
    if pass_backend.is_available() {
        log::info!("Using pass credential backend");
        return Box::new(pass_backend);
    }

    let age_backend = AgeBackend::new(None);
    if age_backend.is_available() {
        log::info!("Using age credential backend");
        return Box::new(age_backend);
    }

    // Try keyring if feature is enabled
    #[cfg(feature = "keyring-backend")]
    {
        let keyring_backend = KeyringBackend::new();
        if keyring_backend.is_available() {
            log::info!("Using keyring credential backend");
            return Box::new(keyring_backend);
        }
    }

    // No secure backend available
    log::warn!(
        "No secure credential backend available (pass or age). Credential storage will fail."
    );
    log::warn!("Install pass: pacman -S pass && pass init <gpg-id>");
    log::warn!("Or install age: pacman -S age && ghostctl security credentials setup-age");
    #[cfg(feature = "keyring-backend")]
    log::warn!("Or ensure GNOME Keyring/KDE Wallet is running for keyring backend");

    // Return a dummy backend that always fails
    Box::new(NoopBackend)
}

/// Keyring backend using system keyring (GNOME Keyring, KDE Wallet, etc.)
/// Only available when compiled with the `keyring-backend` feature
#[cfg(feature = "keyring-backend")]
pub struct KeyringBackend {
    service: String,
}

#[cfg(feature = "keyring-backend")]
impl KeyringBackend {
    pub fn new() -> Self {
        Self {
            service: "ghostctl".to_string(),
        }
    }
}

#[cfg(feature = "keyring-backend")]
impl CredentialBackend for KeyringBackend {
    fn name(&self) -> &'static str {
        "keyring"
    }

    fn is_available(&self) -> bool {
        // Try to access the keyring to check availability
        keyring::Entry::new(&self.service, "test")
            .map(|_| true)
            .unwrap_or(false)
    }

    fn store(&self, key: &str, value: &str) -> Result<()> {
        let entry =
            keyring::Entry::new(&self.service, key).context("Failed to create keyring entry")?;
        entry
            .set_password(value)
            .context("Failed to store in keyring")?;
        log::info!("Stored credential '{}' via keyring", key);
        Ok(())
    }

    fn get(&self, key: &str) -> Result<String> {
        let entry =
            keyring::Entry::new(&self.service, key).context("Failed to access keyring entry")?;
        entry
            .get_password()
            .context("Failed to retrieve from keyring")
    }

    fn delete(&self, key: &str) -> Result<()> {
        let entry =
            keyring::Entry::new(&self.service, key).context("Failed to access keyring entry")?;
        entry
            .delete_password()
            .context("Failed to delete from keyring")?;
        log::info!("Deleted credential '{}' via keyring", key);
        Ok(())
    }

    fn list(&self) -> Result<Vec<String>> {
        // Keyring doesn't support listing - return empty
        // Users can use `secret-tool search --all service ghostctl` manually
        log::warn!("Keyring backend doesn't support listing credentials");
        Ok(Vec::new())
    }
}

/// Dummy backend that refuses to store anything
struct NoopBackend;

impl CredentialBackend for NoopBackend {
    fn name(&self) -> &'static str {
        "none"
    }

    fn is_available(&self) -> bool {
        false
    }

    fn store(&self, _key: &str, _value: &str) -> Result<()> {
        anyhow::bail!("No secure credential backend available. Install pass or age.")
    }

    fn get(&self, _key: &str) -> Result<String> {
        anyhow::bail!("No secure credential backend available. Install pass or age.")
    }

    fn delete(&self, _key: &str) -> Result<()> {
        anyhow::bail!("No secure credential backend available. Install pass or age.")
    }

    fn list(&self) -> Result<Vec<String>> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_backend_path() {
        let backend = PassBackend::new(None);
        assert_eq!(
            backend.pass_path("restic_password"),
            "ghostctl/restic_password"
        );
    }

    #[test]
    fn test_age_backend_credential_path() {
        let backend = AgeBackend::new(None);
        let path = backend.credential_path("test_key");
        assert!(path.to_string_lossy().ends_with("test_key.age"));
    }
}
