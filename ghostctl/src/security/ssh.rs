use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::fs;
use std::process::Command;

pub fn ssh_management() {
    println!("🔐 SSH Key Management");
    println!("====================");

    let options = [
        "🔑 Generate SSH key pair",
        "📋 List SSH keys",
        "📤 Copy public key to clipboard",
        "🌐 Add key to SSH agent",
        "⚙️  SSH configuration",
        "🔒 Secure SSH daemon",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("SSH Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => generate_ssh_key(),
        1 => list_ssh_keys(),
        2 => copy_public_key(),
        3 => add_to_agent(),
        4 => ssh_configuration(),
        5 => secure_ssh_daemon(),
        _ => return,
    }
}

fn generate_ssh_key() {
    println!("🔑 Generate SSH Key Pair");
    println!("========================");

    let key_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Key type")
        .items(&["ed25519 (recommended)", "rsa 4096", "ecdsa"])
        .default(0)
        .interact()
        .unwrap();

    let email: String = Input::new()
        .with_prompt("Email for key comment")
        .interact_text()
        .unwrap();

    let filename: String = Input::new()
        .with_prompt("Key filename")
        .default("id_ed25519".into())
        .interact_text()
        .unwrap();

    let (key_type_str, key_size) = match key_type {
        0 => ("ed25519", ""),
        1 => ("rsa", "-b 4096"),
        2 => ("ecdsa", "-b 521"),
        _ => ("ed25519", ""),
    };

    let ssh_dir = dirs::home_dir().unwrap().join(".ssh");
    fs::create_dir_all(&ssh_dir).unwrap();

    let key_path = ssh_dir.join(&filename);

    println!("🔧 Generating {} key...", key_type_str);

    let mut cmd = Command::new("ssh-keygen");
    cmd.args(&["-t", key_type_str]);

    if !key_size.is_empty() {
        cmd.args(key_size.split_whitespace());
    }

    cmd.args(&["-C", &email])
        .args(&["-f", key_path.to_str().unwrap()]);

    let status = cmd.status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ SSH key pair generated!");
            println!("📁 Private key: {}", key_path.display());
            println!("📁 Public key: {}.pub", key_path.display());

            let add_to_agent = Confirm::new()
                .with_prompt("Add key to SSH agent?")
                .default(true)
                .interact()
                .unwrap();

            if add_to_agent {
                let _ = Command::new("ssh-add")
                    .arg(key_path.to_str().unwrap())
                    .status();
                println!("✅ Key added to SSH agent");
            }
        }
        _ => println!("❌ Failed to generate SSH key"),
    }
}

fn list_ssh_keys() {
    println!("📋 SSH Keys");
    println!("===========");

    let ssh_dir = dirs::home_dir().unwrap().join(".ssh");

    if !ssh_dir.exists() {
        println!("❌ SSH directory does not exist: {}", ssh_dir.display());
        return;
    }

    println!("\n🔑 Private Keys:");
    if let Ok(entries) = fs::read_dir(&ssh_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && !path.extension().map_or(false, |ext| ext == "pub") {
                if let Some(filename) = path.file_name() {
                    let filename_str = filename.to_string_lossy();
                    if filename_str.starts_with("id_") || filename_str.contains("key") {
                        println!("  📄 {}", filename_str);
                    }
                }
            }
        }
    }

    println!("\n🔓 Public Keys:");
    if let Ok(entries) = fs::read_dir(&ssh_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "pub") {
                if let Some(filename) = path.file_name() {
                    println!("  📄 {}", filename.to_string_lossy());

                    // Show key fingerprint
                    if let Ok(output) = Command::new("ssh-keygen")
                        .args(&["-lf", path.to_str().unwrap()])
                        .output()
                    {
                        let fingerprint = String::from_utf8_lossy(&output.stdout);
                        println!("    🔍 {}", fingerprint.trim());
                    }
                }
            }
        }
    }

    println!("\n🔐 SSH Agent Keys:");
    let _ = Command::new("ssh-add").arg("-l").status();
}

fn copy_public_key() {
    println!("📤 Copy Public Key to Clipboard");
    println!("===============================");

    let ssh_dir = dirs::home_dir().unwrap().join(".ssh");
    let mut pub_keys = Vec::new();

    if let Ok(entries) = fs::read_dir(&ssh_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "pub") {
                if let Some(filename) = path.file_name() {
                    pub_keys.push((filename.to_string_lossy().to_string(), path));
                }
            }
        }
    }

    if pub_keys.is_empty() {
        println!("❌ No public keys found");
        return;
    }

    let key_names: Vec<String> = pub_keys.iter().map(|(name, _)| name.clone()).collect();

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select public key to copy")
        .items(&key_names)
        .default(0)
        .interact()
        .unwrap();

    let (_, key_path) = &pub_keys[choice];

    if let Ok(content) = fs::read_to_string(key_path) {
        // Try different clipboard tools
        let clipboard_tools = ["xclip", "pbcopy", "wl-copy"];
        let mut copied = false;

        for tool in &clipboard_tools {
            if Command::new("which").arg(tool).status().is_ok() {
                let mut cmd = Command::new(tool);

                match *tool {
                    "xclip" => {
                        cmd.args(&["-selection", "clipboard"]);
                    }
                    "wl-copy" => {}
                    "pbcopy" => {}
                    _ => {}
                }

                if let Ok(mut child) = cmd.stdin(std::process::Stdio::piped()).spawn() {
                    if let Some(stdin) = child.stdin.take() {
                        use std::io::Write;
                        if writeln!(&stdin, "{}", content.trim()).is_ok() {
                            if child.wait().is_ok() {
                                println!("✅ Public key copied to clipboard using {}", tool);
                                copied = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        if !copied {
            println!("❌ Could not copy to clipboard. Here's your public key:");
            println!("{}", content);
        }
    } else {
        println!("❌ Could not read public key file");
    }
}

fn add_to_agent() {
    println!("🌐 Add Key to SSH Agent");
    println!("=======================");

    // Start SSH agent if not running
    if Command::new("ssh-add").arg("-l").status().is_err() {
        println!("🚀 Starting SSH agent...");
        let _ = Command::new("ssh-agent").arg("bash").status();
    }

    let ssh_dir = dirs::home_dir().unwrap().join(".ssh");
    let mut private_keys = Vec::new();

    if let Ok(entries) = fs::read_dir(&ssh_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && !path.extension().map_or(false, |ext| ext == "pub") {
                if let Some(filename) = path.file_name() {
                    let filename_str = filename.to_string_lossy();
                    if filename_str.starts_with("id_") || filename_str.contains("key") {
                        private_keys.push((filename_str.to_string(), path));
                    }
                }
            }
        }
    }

    if private_keys.is_empty() {
        println!("❌ No private keys found");
        return;
    }

    let key_names: Vec<String> = private_keys.iter().map(|(name, _)| name.clone()).collect();

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select private key to add")
        .items(&key_names)
        .default(0)
        .interact()
        .unwrap();

    let (_, key_path) = &private_keys[choice];

    let status = Command::new("ssh-add")
        .arg(key_path.to_str().unwrap())
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Key added to SSH agent"),
        _ => println!("❌ Failed to add key to SSH agent"),
    }
}

fn ssh_configuration() {
    println!("⚙️  SSH Configuration");
    println!("====================");

    let config_options = [
        "📝 Edit SSH client config (~/.ssh/config)",
        "🔍 Show current SSH config",
        "🆕 Create SSH config template",
        "🔑 Configure key-based authentication",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("SSH Configuration")
        .items(&config_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => edit_ssh_config(),
        1 => show_ssh_config(),
        2 => create_ssh_config_template(),
        3 => configure_key_auth(),
        _ => return,
    }
}

fn edit_ssh_config() {
    let ssh_dir = dirs::home_dir().unwrap().join(".ssh");
    let config_path = ssh_dir.join("config");

    // Create .ssh directory if it doesn't exist
    fs::create_dir_all(&ssh_dir).unwrap();

    // Create empty config if it doesn't exist
    if !config_path.exists() {
        fs::write(&config_path, "# SSH Client Configuration\n").unwrap();
        println!("✅ Created new SSH config file");
    }

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

    let status = Command::new(&editor)
        .arg(config_path.to_str().unwrap())
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ SSH config edited"),
        _ => println!("❌ Failed to edit SSH config"),
    }
}

fn show_ssh_config() {
    let config_path = dirs::home_dir().unwrap().join(".ssh/config");

    if let Ok(content) = fs::read_to_string(&config_path) {
        println!("📋 SSH Client Configuration:");
        println!("============================");
        println!("{}", content);
    } else {
        println!("❌ No SSH config found at ~/.ssh/config");
    }
}

fn create_ssh_config_template() {
    let template = r#"# SSH Client Configuration Template
# Copy and modify the sections you need

# Example: Personal server
Host personal
    HostName your-server.com
    User your-username
    Port 22
    IdentityFile ~/.ssh/id_ed25519
    ForwardAgent yes

# Example: Work server with jump host
Host work-server
    HostName 10.0.1.100
    User work-user
    ProxyJump bastion.company.com
    IdentityFile ~/.ssh/work_key

# Example: GitHub
Host github.com
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_ed25519
    AddKeysToAgent yes

# Global settings
Host *
    AddKeysToAgent yes
    UseKeychain yes
    ServerAliveInterval 60
    ServerAliveCountMax 3
"#;

    let config_path = dirs::home_dir().unwrap().join(".ssh/config.template");

    if let Err(e) = fs::write(&config_path, template) {
        println!("❌ Failed to create template: {}", e);
        return;
    }

    println!("✅ SSH config template created at ~/.ssh/config.template");
    println!("💡 Copy sections to ~/.ssh/config as needed");
}

fn configure_key_auth() {
    println!("🔑 Configure Key-Based Authentication");
    println!("====================================");

    let server: String = Input::new()
        .with_prompt("Server hostname or IP")
        .interact_text()
        .unwrap();

    let username: String = Input::new()
        .with_prompt("Username")
        .interact_text()
        .unwrap();

    // List available public keys
    let ssh_dir = dirs::home_dir().unwrap().join(".ssh");
    let mut pub_keys = Vec::new();

    if let Ok(entries) = fs::read_dir(&ssh_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "pub") {
                if let Some(filename) = path.file_name() {
                    pub_keys.push((filename.to_string_lossy().to_string(), path));
                }
            }
        }
    }

    if pub_keys.is_empty() {
        println!("❌ No public keys found. Generate one first.");
        return;
    }

    let key_names: Vec<String> = pub_keys.iter().map(|(name, _)| name.clone()).collect();

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select public key to install")
        .items(&key_names)
        .default(0)
        .interact()
        .unwrap();

    let (_, key_path) = &pub_keys[choice];

    println!("🚀 Installing public key on {}@{}", username, server);

    let status = Command::new("ssh-copy-id")
        .args(&[
            "-i",
            key_path.to_str().unwrap(),
            &format!("{}@{}", username, server),
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Public key installed successfully!");
            println!("🔒 You can now login without a password");
        }
        _ => {
            println!("❌ Failed to install public key");
            println!("💡 Make sure ssh-copy-id is available and the server is accessible");
        }
    }
}

fn secure_ssh_daemon() {
    println!("🔒 Secure SSH Daemon Configuration");
    println!("===================================");

    println!("⚠️  This will modify /etc/ssh/sshd_config");
    println!("💡 Recommended security improvements:");
    println!("   • Disable root login");
    println!("   • Disable password authentication");
    println!("   • Change default port");
    println!("   • Enable key-based auth only");

    let proceed = Confirm::new()
        .with_prompt("Apply security hardening to SSH daemon?")
        .default(false)
        .interact()
        .unwrap();

    if !proceed {
        return;
    }

    // Backup original config
    let _ = Command::new("sudo")
        .args(&["cp", "/etc/ssh/sshd_config", "/etc/ssh/sshd_config.backup"])
        .status();

    let hardening_options = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select hardening options")
        .items(&[
            "🚫 Disable root login",
            "🔑 Disable password authentication",
            "🔢 Change default port (22 -> 2222)",
            "⏱️  Set login timeout",
            "📊 Enable detailed logging",
        ])
        .interact()
        .unwrap();

    println!("⚠️  SSH daemon hardening not fully implemented yet");
    println!("💡 Manual steps:");

    if hardening_options.contains(&0) {
        println!("   • Set 'PermitRootLogin no' in /etc/ssh/sshd_config");
    }

    if hardening_options.contains(&1) {
        println!("   • Set 'PasswordAuthentication no' in /etc/ssh/sshd_config");
        println!("   • Set 'PubkeyAuthentication yes' in /etc/ssh/sshd_config");
    }

    if hardening_options.contains(&2) {
        println!("   • Set 'Port 2222' in /etc/ssh/sshd_config");
    }

    if hardening_options.contains(&3) {
        println!("   • Set 'LoginGraceTime 30' in /etc/ssh/sshd_config");
    }

    if hardening_options.contains(&4) {
        println!("   • Set 'LogLevel VERBOSE' in /etc/ssh/sshd_config");
    }

    println!("\n🔄 After making changes, restart SSH daemon:");
    println!("   sudo systemctl restart sshd");
}
