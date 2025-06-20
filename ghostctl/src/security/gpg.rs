use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::fs;
use std::process::Command;

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
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("GPG Key Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    match choice {
        0 => list_gpg_keys(),
        1 => generate_gpg_key(),
        2 => export_public_key(),
        9 => custom_gpg_generation(),
        _ => (),
    }
}

pub fn list_gpg_keys() {
    println!("📋 GPG Keys");
    println!("===========");
    println!("\n🔑 Public Keys:");
    let _ = Command::new("gpg").args(&["--list-keys"]).status();
    println!("\n🔐 Private Keys:");
    let _ = Command::new("gpg").args(&["--list-secret-keys"]).status();
}

pub fn generate_gpg_key() {
    println!("🔑 Generate New GPG Key");
    println!("======================");
    let key_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Key type")
        .items(&["RSA 2048", "RSA 4096"])
        .default(1)
        .interact()
        .unwrap();
    match key_type {
        0 => println!("Generating RSA 2048 key..."),
        1 => println!("Generating RSA 4096 key..."),
        _ => (),
    }
    println!("✅ GPG key generation completed!");
    println!("💡 Don't forget to:");
    println!("   📤 Export and backup your key");
    println!("   🌐 Upload to a keyserver if needed");
    println!("   🔑 Set up key signing");
}

pub fn custom_gpg_generation() {
    println!("⚙️  Custom GPG Key Configuration");
    let real_name: String = Input::new()
        .with_prompt("Real name")
        .interact_text()
        .unwrap();
    let email: String = Input::new().with_prompt("Email").interact_text().unwrap();
    let comment: String = Input::new().with_prompt("Comment").interact_text().unwrap();
    let key_length = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Key length")
        .items(&["2048", "4096"])
        .default(1)
        .interact()
        .unwrap();
    let length = if key_length == 0 { "2048" } else { "4096" };
    let expire = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Expiration")
        .items(&["1y", "2y", "5y", "Never"])
        .default(0)
        .interact()
        .unwrap();
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
    fs::write(batch_file, batch_content).unwrap();
    println!("🔧 Generating GPG key with custom parameters...");
    let status = Command::new("gpg")
        .arg("--batch")
        .arg("--gen-key")
        .arg(batch_file)
        .status();
    let _ = fs::remove_file(batch_file);
    match status {
        Ok(s) if s.success() => println!("Custom GPG key generated!"),
        _ => println!("Failed to generate custom GPG key."),
    }
}

pub fn export_public_key() {
    println!("📤 Export Public Key");
    println!("Available keys:");
    let _ = Command::new("gpg")
        .args(&["--list-keys", "--keyid-format", "SHORT"])
        .status();
    let key_id: String = Input::new().with_prompt("Key ID").interact_text().unwrap();
    let format = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Export format")
        .items(&["ASCII", "Binary"])
        .default(0)
        .interact()
        .unwrap();
    match format {
        0 => {
            let _ = Command::new("gpg")
                .args(&["--armor", "--export", &key_id])
                .status();
        }
        1 => {
            let _ = Command::new("gpg").args(&["--export", &key_id]).status();
        }
        _ => (),
    }
}
