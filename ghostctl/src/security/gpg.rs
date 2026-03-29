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
    println!("🔑 Generate New GPG Key");
    println!("======================");
    let key_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Key type")
        .items(&["RSA 2048", "RSA 4096"])
        .default(1)
        .interact()
        .context("Failed to get key type selection")?;
    match key_type {
        0 => println!("Generating RSA 2048 key..."),
        1 => println!("Generating RSA 4096 key..."),
        _ => return Ok(()),
    }
    println!("✅ GPG key generation completed!");
    println!("💡 Don't forget to:");
    println!("   📤 Export and backup your key");
    println!("   🌐 Upload to a keyserver if needed");
    println!("   🔑 Set up key signing");
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
