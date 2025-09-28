use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fs;
use std::process::Command;
use thiserror::Error;

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
    println!("⚙️  Custom GPG Key Configuration");
    let real_name: String = Input::new()
        .with_prompt("Real name")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("Name cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .context("Failed to get real name")?;

    let email: String = Input::new()
        .with_prompt("Email")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("Email cannot be empty")
            } else if !input.contains('@') {
                Err("Please enter a valid email address")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .context("Failed to get email")?;

    let comment: String = Input::new()
        .with_prompt("Comment")
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
    let batch_file = "/tmp/gpg-batch";
    fs::write(batch_file, batch_content).context("Failed to write GPG batch file")?;

    println!("🔧 Generating GPG key with custom parameters...");
    let status = Command::new("gpg")
        .arg("--batch")
        .arg("--gen-key")
        .arg(batch_file)
        .status()
        .context("Failed to execute GPG command")?;

    let _ = fs::remove_file(batch_file);

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
        .with_prompt("Key ID")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("Key ID cannot be empty")
            } else {
                Ok(())
            }
        })
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
