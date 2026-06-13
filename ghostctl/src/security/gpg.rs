use anyhow::{Context, Result};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::fs;
use std::process::Command;
use thiserror::Error;

/// Validate GPG key ID input to prevent shell injection
/// Valid formats: 8 or 16 hex chars, or email address
fn validate_key_id(input: &str) -> Result<(), &'static str> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err("Key ID cannot be empty");
    }

    // Check for shell metacharacters
    let dangerous_chars = [
        ';', '|', '&', '$', '`', '(', ')', '{', '}', '[', ']', '<', '>', '!', '\\', '"', '\'',
        '\n', '\r', '\t', '*', '?', '#', '~', '%', '^',
    ];
    if trimmed.chars().any(|c| dangerous_chars.contains(&c)) {
        return Err("Key ID contains invalid characters");
    }

    // Allow hex key IDs (8 or 16 chars for short/long format)
    let is_hex = trimmed.chars().all(|c| c.is_ascii_hexdigit() || c == ' ');
    if is_hex && (trimmed.len() == 8 || trimmed.len() == 16 || trimmed.len() == 40) {
        return Ok(());
    }

    // Allow email addresses
    if trimmed.contains('@') && trimmed.len() < 256 {
        // Basic email validation
        let parts: Vec<&str> = trimmed.split('@').collect();
        if parts.len() == 2
            && !parts[0].is_empty()
            && !parts[1].is_empty()
            && parts[1].contains('.')
        {
            return Ok(());
        }
    }

    // Allow name patterns (alphanumeric with spaces)
    let is_name = trimmed
        .chars()
        .all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' || c == '.');
    if is_name && trimmed.len() < 256 {
        return Ok(());
    }

    Err("Invalid key ID format. Use hex ID, email, or name")
}

/// Validate GPG name/comment input
fn validate_gpg_text(input: &str) -> Result<(), &'static str> {
    let trimmed = input.trim();

    // Check for characters that could cause issues in batch file
    let dangerous_chars = ['`', '$', '\\', '\n', '\r'];
    if trimmed.chars().any(|c| dangerous_chars.contains(&c)) {
        return Err("Input contains invalid characters");
    }

    if trimmed.len() > 256 {
        return Err("Input too long (max 256 characters)");
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum GpgError {
    #[error("GPG command failed: {0}")]
    CommandError(String),
    #[error("File operation failed: {0}")]
    FileError(String),
    #[error("Invalid input: {0}")]
    ValidationError(String),
    #[error("Key operation failed: {0}")]
    KeyError(String),
}

pub fn gpg_key_management() {
    println!("🔑 GPG Key Management");
    println!("====================");
    let options = [
        "📋 List GPG keys",
        "🔑 Generate new GPG key",
        "📤 Export public key",
        "📥 Import public key",
        "🔐 Change key passphrase",
        "🗑️  Delete GPG key",
        "⚙️  GPG configuration",
        "🔄 Refresh keys from keyserver",
        "📊 Key trust management",
        "🛠️  Custom GPG generation",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("GPG Key Management")
        .items(&options)
        .default(0)
        .interact()
    {
        Ok(choice) => choice,
        Err(e) => {
            eprintln!("❌ Menu selection failed: {}", e);
            return;
        }
    };

    let result = match choice {
        0 => list_gpg_keys(),
        1 => generate_gpg_key(),
        2 => export_public_key(),
        3 => import_public_key(),
        4 => change_passphrase(),
        5 => delete_gpg_key(),
        6 => show_gpg_config(),
        7 => refresh_keys(),
        8 => manage_trust(),
        9 => custom_gpg_generation(),
        _ => Ok(()),
    };

    if let Err(e) = result {
        eprintln!("❌ GPG operation failed: {}", e);
    }
}

pub fn list_gpg_keys() -> Result<()> {
    println!("📋 GPG Keys");
    println!("===========");
    println!("\n🔑 Public Keys:");
    if let Err(e) = Command::new("gpg").args(&["--list-keys"]).status() {
        log::warn!("Failed to list public keys: {}", e);
        println!("❌ Failed to list public keys");
    }
    println!("\n🔐 Private Keys:");
    if let Err(e) = Command::new("gpg").args(&["--list-secret-keys"]).status() {
        log::warn!("Failed to list private keys: {}", e);
        println!("❌ Failed to list private keys");
    }
    Ok(())
}

pub fn generate_gpg_key() -> Result<()> {
    println!("Generate New GPG Key");
    println!("====================");

    let name: String = Input::new()
        .with_prompt("Real name")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                return Err("Name cannot be empty");
            }
            validate_gpg_text(input)
        })
        .interact_text()
        .context("Failed to get name")?;

    let email: String = Input::new()
        .with_prompt("Email address")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                return Err("Email cannot be empty");
            }
            if !input.contains('@') {
                return Err("Please enter a valid email address");
            }
            validate_gpg_text(input)
        })
        .interact_text()
        .context("Failed to get email")?;

    let key_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Key type")
        .items(&["RSA 4096 (recommended)", "RSA 2048"])
        .default(0)
        .interact()
        .context("Failed to get key type selection")?;

    let algo = if key_type == 1 { "rsa2048" } else { "rsa4096" };
    let uid = format!("{} <{}>", name.trim(), email.trim());

    println!("Generating {} key for {}...", algo, uid);
    let status = Command::new("gpg")
        .args(["--batch", "--quick-gen-key", &uid, algo, "sign,cert", "1y"])
        .status()
        .context("Failed to execute gpg")?;

    if status.success() {
        println!("GPG key generated for {}", uid);
    } else {
        return Err(GpgError::CommandError("GPG key generation failed".to_string()).into());
    }

    Ok(())
}

pub fn import_public_key() -> Result<()> {
    println!("Import Public Key");
    println!("=================");

    let path: String = Input::new()
        .with_prompt("Path to key file")
        .validate_with(|input: &String| -> Result<(), &str> {
            let p = std::path::Path::new(input.trim());
            if !p.exists() {
                return Err("File not found");
            }
            validate_gpg_text(input)
        })
        .interact_text()
        .context("Failed to get file path")?;

    let status = Command::new("gpg")
        .args(["--import", path.trim()])
        .status()
        .context("Failed to execute gpg --import")?;

    if status.success() {
        println!("Key imported from {}", path.trim());
    } else {
        return Err(GpgError::CommandError("Import failed".to_string()).into());
    }

    Ok(())
}

pub fn change_passphrase() -> Result<()> {
    println!("Change Key Passphrase");
    println!("=====================");

    println!("Secret keys:");
    let _ = Command::new("gpg")
        .args(["--list-secret-keys", "--keyid-format", "SHORT"])
        .status();

    let key_id: String = Input::new()
        .with_prompt("Key ID to change passphrase")
        .validate_with(|input: &String| -> Result<(), &str> { validate_key_id(input) })
        .interact_text()
        .context("Failed to get key ID")?;

    let status = Command::new("gpg")
        .args(["--passwd", key_id.trim()])
        .status()
        .context("Failed to execute gpg --passwd")?;

    if status.success() {
        println!("Passphrase changed for key {}", key_id.trim());
    } else {
        return Err(GpgError::CommandError("Passphrase change failed".to_string()).into());
    }

    Ok(())
}

pub fn delete_gpg_key() -> Result<()> {
    println!("Delete GPG Key");
    println!("==============");

    println!("Available keys:");
    let _ = Command::new("gpg")
        .args(["--list-keys", "--keyid-format", "SHORT"])
        .status();

    let key_id: String = Input::new()
        .with_prompt("Key ID to delete")
        .validate_with(|input: &String| -> Result<(), &str> { validate_key_id(input) })
        .interact_text()
        .context("Failed to get key ID")?;

    let confirm = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Delete key {} (secret + public)?", key_id.trim()))
        .items(&["No, cancel", "Yes, delete"])
        .default(0)
        .interact()
        .context("Failed to get confirmation")?;

    if confirm != 1 {
        println!("Cancelled");
        return Ok(());
    }

    // Delete secret key first, then public
    let status = Command::new("gpg")
        .args([
            "--batch",
            "--yes",
            "--delete-secret-and-public-key",
            key_id.trim(),
        ])
        .status()
        .context("Failed to execute gpg --delete-secret-and-public-key")?;

    if status.success() {
        println!("Key {} deleted", key_id.trim());
    } else {
        return Err(GpgError::CommandError("Key deletion failed".to_string()).into());
    }

    Ok(())
}

pub fn show_gpg_config() -> Result<()> {
    println!("GPG Configuration");
    println!("=================");

    let gpg_home = dirs::home_dir()
        .map(|h| h.join(".gnupg"))
        .unwrap_or_else(|| std::path::PathBuf::from("~/.gnupg"));

    let conf_path = gpg_home.join("gpg.conf");
    println!("Config file: {}", conf_path.display());
    println!();

    if conf_path.exists() {
        let content = fs::read_to_string(&conf_path).context("Failed to read gpg.conf")?;
        if content.trim().is_empty() {
            println!("(empty)");
        } else {
            println!("{}", content);
        }
    } else {
        println!("No gpg.conf found");
    }

    println!();
    println!("GnuPG home: {}", gpg_home.display());
    let _ = Command::new("gpg").args(["--version"]).status();

    Ok(())
}

pub fn refresh_keys() -> Result<()> {
    println!("Refresh Keys from Keyserver");
    println!("===========================");

    let keyserver: String = Input::new()
        .with_prompt("Keyserver URL (press Enter for default)")
        .default("hkps://keys.openpgp.org".to_string())
        .validate_with(|input: &String| -> Result<(), &str> { validate_gpg_text(input) })
        .interact_text()
        .context("Failed to get keyserver")?;

    println!("Refreshing keys from {}...", keyserver.trim());
    let status = Command::new("gpg")
        .args(["--keyserver", keyserver.trim(), "--refresh-keys"])
        .status()
        .context("Failed to execute gpg --refresh-keys")?;

    if status.success() {
        println!("Keys refreshed");
    } else {
        return Err(GpgError::CommandError(
            "Key refresh failed (check network and keyserver)".to_string(),
        )
        .into());
    }

    Ok(())
}

pub fn manage_trust() -> Result<()> {
    println!("Key Trust Management");
    println!("====================");

    println!("Available keys:");
    let _ = Command::new("gpg")
        .args(["--list-keys", "--keyid-format", "SHORT"])
        .status();

    let key_id: String = Input::new()
        .with_prompt("Key ID to manage trust")
        .validate_with(|input: &String| -> Result<(), &str> { validate_key_id(input) })
        .interact_text()
        .context("Failed to get key ID")?;

    println!("Launching GPG trust editor for {}...", key_id.trim());
    println!("(Type 'trust' at the gpg> prompt, select level, then 'quit')");
    let status = Command::new("gpg")
        .args(["--edit-key", key_id.trim(), "trust"])
        .status()
        .context("Failed to execute gpg --edit-key")?;

    if !status.success() {
        return Err(GpgError::CommandError("Trust management failed".to_string()).into());
    }

    Ok(())
}

/// Key info for non-interactive display
pub fn key_info(key_id: &str) -> Result<()> {
    validate_key_id(key_id).map_err(|e| GpgError::ValidationError(e.to_string()))?;

    let output = Command::new("gpg")
        .args(["--with-colons", "--fixed-list-mode", "--list-keys", key_id])
        .output()
        .context("Failed to execute gpg")?;

    if !output.status.success() {
        return Err(GpgError::KeyError(format!("Key '{}' not found", key_id)).into());
    }

    // Also show human-readable output
    let _ = Command::new("gpg")
        .args(["--list-keys", "--keyid-format", "long", key_id])
        .status();

    Ok(())
}

/// Extend key expiration non-interactively
pub fn renew_key(key_id: &str, duration: &str) -> Result<()> {
    validate_key_id(key_id).map_err(|e| GpgError::ValidationError(e.to_string()))?;

    // Validate duration format
    let valid_durations = ["1y", "2y", "3y", "5y", "10y"];
    if !valid_durations.contains(&duration) {
        anyhow::bail!(
            "Invalid duration '{}'. Use one of: {}",
            duration,
            valid_durations.join(", ")
        );
    }

    println!("Extending expiration of key {} by {}...", key_id, duration);
    let status = Command::new("gpg")
        .args(["--batch", "--quick-set-expire", key_id, duration])
        .status()
        .context("Failed to execute gpg --quick-set-expire")?;

    if status.success() {
        println!("Key {} expiration extended by {}", key_id, duration);
    } else {
        return Err(GpgError::CommandError("Failed to renew key expiration".to_string()).into());
    }

    Ok(())
}

/// Export a public key by ID (non-interactive, for CLI subcommand)
pub fn export_public_key_by_id(key_id: &str) -> Result<()> {
    validate_key_id(key_id).map_err(|e| GpgError::ValidationError(e.to_string()))?;

    let status = Command::new("gpg")
        .args(["--armor", "--export", key_id])
        .status()
        .context("Failed to execute gpg --export")?;

    if !status.success() {
        return Err(GpgError::KeyError(format!("Failed to export key '{}'", key_id)).into());
    }

    Ok(())
}

pub fn custom_gpg_generation() -> Result<()> {
    println!("Custom GPG Key Configuration");
    let real_name: String = Input::new()
        .with_prompt("Real name")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                return Err("Name cannot be empty");
            }
            validate_gpg_text(input)
        })
        .interact_text()
        .context("Failed to get real name")?;

    let email: String = Input::new()
        .with_prompt("Email")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                return Err("Email cannot be empty");
            }
            if !input.contains('@') {
                return Err("Please enter a valid email address");
            }
            validate_gpg_text(input)
        })
        .interact_text()
        .context("Failed to get email")?;

    let comment: String = Input::new()
        .with_prompt("Comment (optional)")
        .allow_empty(true)
        .validate_with(|input: &String| -> Result<(), &str> { validate_gpg_text(input) })
        .interact_text()
        .context("Failed to get comment")?;
    let key_length = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Key length")
        .items(&["2048", "4096"])
        .default(1)
        .interact()
        .context("Failed to get key length selection")?;
    let length = if key_length == 0 { "2048" } else { "4096" };
    let expire = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Expiration")
        .items(&["1y", "2y", "5y", "Never"])
        .default(0)
        .interact()
        .context("Failed to get expiration selection")?;
    let expire_time = match expire {
        0 => "1y",
        1 => "2y",
        2 => "5y",
        _ => "0",
    };
    let batch_content = format!(
        r#"Key-Type: RSA
Key-Length: {}
Subkey-Type: RSA
Subkey-Length: {}
Name-Real: {}
Name-Email: {}
Name-Comment: {}
Expire-Date: {}
Passphrase:
%commit
"#,
        length, length, real_name, email, comment, expire_time
    );

    // Use a unique temp file with secure permissions
    let batch_file = format!("/tmp/gpg-batch-{}", std::process::id());
    let batch_path = std::path::Path::new(&batch_file);

    fs::write(batch_path, &batch_content).context("Failed to write GPG batch file")?;

    // Set restrictive permissions (0600) on batch file
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(batch_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(batch_path, perms)?;
    }

    println!("Generating GPG key with custom parameters...");
    let status = Command::new("gpg")
        .arg("--batch")
        .arg("--gen-key")
        .arg(&batch_file)
        .status()
        .context("Failed to execute GPG command")?;

    // Always clean up the batch file, log any errors
    if let Err(e) = fs::remove_file(batch_path) {
        log::warn!("Failed to remove GPG batch file {}: {}", batch_file, e);
    }

    if status.success() {
        println!("Custom GPG key generated!");
        Ok(())
    } else {
        Err(GpgError::CommandError("Failed to generate custom GPG key".to_string()).into())
    }
}

pub fn export_public_key() -> Result<()> {
    println!("📤 Export Public Key");
    println!("Available keys:");
    if let Err(e) = Command::new("gpg")
        .args(&["--list-keys", "--keyid-format", "SHORT"])
        .status()
    {
        log::warn!("Failed to list keys: {}", e);
        println!("❌ Failed to list available keys");
    }

    let key_id: String = Input::new()
        .with_prompt("Key ID (hex ID, email, or name)")
        .validate_with(|input: &String| -> Result<(), &str> { validate_key_id(input) })
        .interact_text()
        .context("Failed to get key ID")?;
    let format = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Export format")
        .items(&["ASCII", "Binary"])
        .default(0)
        .interact()
        .context("Failed to get format selection")?;
    match format {
        0 => {
            if let Err(e) = Command::new("gpg")
                .args(&["--armor", "--export", &key_id])
                .status()
            {
                return Err(
                    GpgError::CommandError(format!("Failed to export ASCII key: {}", e)).into(),
                );
            }
        }
        1 => {
            if let Err(e) = Command::new("gpg").args(&["--export", &key_id]).status() {
                return Err(
                    GpgError::CommandError(format!("Failed to export binary key: {}", e)).into(),
                );
            }
        }
        _ => {}
    }
    Ok(())
}
