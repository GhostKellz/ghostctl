use anyhow::{Context, Result};
use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SshError {
    #[error("SSH directory not accessible: {0}")]
    DirectoryError(String),
    #[error("SSH key operation failed: {0}")]
    KeyError(String),
    #[error("SSH command failed: {0}")]
    CommandError(String),
    #[error("File operation failed: {0}")]
    FileError(String),
    #[error("Invalid input: {0}")]
    ValidationError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("SSH Management")
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
        0 => generate_ssh_key(),
        1 => list_ssh_keys(),
        2 => copy_public_key(),
        3 => {
            add_to_agent();
            Ok(())
        }
        4 => {
            ssh_configuration();
            Ok(())
        }
        5 => {
            secure_ssh_daemon();
            Ok(())
        }
        _ => return,
    };

    if let Err(e) = result {
        eprintln!("❌ Operation failed: {}", e);
    }
}

fn generate_ssh_key() -> Result<()> {
    println!("🔑 Generate SSH Key Pair");
    println!("========================");

    let key_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Key type")
        .items(&["ed25519 (recommended)", "rsa 4096", "ecdsa"])
        .default(0)
        .interact()
        .context("Failed to get key type selection")?;

    let email: String = Input::new()
        .with_prompt("Email for key comment")
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
        .context("Failed to get email input")?;

    let filename: String = Input::new()
        .with_prompt("Key filename")
        .default("id_ed25519".into())
        .validate_with(|input: &String| -> Result<(), &str> { validate_key_filename(input) })
        .interact_text()
        .context("Failed to get filename input")?;

    let (key_type_str, key_size) = match key_type {
        0 => ("ed25519", ""),
        1 => ("rsa", "-b 4096"),
        2 => ("ecdsa", "-b 521"),
        _ => ("ed25519", ""),
    };

    let ssh_dir = get_ssh_dir()?;
    fs::create_dir_all(&ssh_dir)
        .with_context(|| format!("Failed to create SSH directory: {}", ssh_dir.display()))?;

    let key_path = ssh_dir.join(&filename);

    println!("🔧 Generating {} key...", key_type_str);

    let mut cmd = Command::new("ssh-keygen");
    cmd.args(&["-t", key_type_str]);

    if !key_size.is_empty() {
        cmd.args(key_size.split_whitespace());
    }

    cmd.args(&["-C", &email]).args(&[
        "-f",
        key_path
            .to_str()
            .ok_or_else(|| SshError::FileError("Invalid key path".to_string()))?,
    ]);

    let status = cmd
        .status()
        .context("Failed to execute ssh-keygen command")?;

    if !status.success() {
        return Err(SshError::CommandError("ssh-keygen failed".to_string()).into());
    }

    println!("✅ SSH key pair generated!");
    println!("📁 Private key: {}", key_path.display());
    println!("📁 Public key: {}.pub", key_path.display());

    let add_to_agent = Confirm::new()
        .with_prompt("Add key to SSH agent?")
        .default(true)
        .interact()
        .context("Failed to get agent confirmation")?;

    if add_to_agent {
        if let Err(e) = Command::new("ssh-add")
            .arg(
                key_path
                    .to_str()
                    .ok_or_else(|| SshError::FileError("Invalid key path".to_string()))?,
            )
            .status()
        {
            log::warn!("Failed to add key to SSH agent: {}", e);
            println!("⚠️  Warning: Could not add key to SSH agent");
        } else {
            println!("✅ Key added to SSH agent");
        }
    }

    Ok(())
}

fn list_ssh_keys() -> Result<()> {
    println!("📋 SSH Keys");
    println!("===========");

    let ssh_dir = get_ssh_dir()?;

    if !ssh_dir.exists() {
        return Err(SshError::DirectoryError(format!(
            "SSH directory does not exist: {}",
            ssh_dir.display()
        ))
        .into());
    }

    println!("\n🔑 Private Keys:");
    if let Ok(entries) = fs::read_dir(&ssh_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && path.extension().is_none_or(|ext| ext != "pub")
                && let Some(filename) = path.file_name()
            {
                let filename_str = filename.to_string_lossy();
                if filename_str.starts_with("id_") || filename_str.contains("key") {
                    println!("  📄 {}", filename_str);
                }
            }
        }
    }

    println!("\n🔓 Public Keys:");
    if let Ok(entries) = fs::read_dir(&ssh_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "pub")
                && let Some(filename) = path.file_name()
            {
                println!("  📄 {}", filename.to_string_lossy());

                // Show key fingerprint
                if let Some(path_str) = path.to_str()
                    && let Ok(output) = Command::new("ssh-keygen").args(&["-lf", path_str]).output()
                {
                    let fingerprint = String::from_utf8_lossy(&output.stdout);
                    println!("    🔍 {}", fingerprint.trim());
                }
            }
        }
    }

    println!("\n🔐 SSH Agent Keys:");
    if let Err(e) = Command::new("ssh-add").arg("-l").status() {
        log::warn!("Could not list SSH agent keys: {}", e);
        println!("  ⚠️  SSH agent not available or no keys loaded");
    }

    Ok(())
}

fn copy_public_key() -> Result<()> {
    println!("📤 Copy Public Key to Clipboard");
    println!("===============================");

    let ssh_dir = get_ssh_dir()?;
    let mut pub_keys = Vec::new();

    if let Ok(entries) = fs::read_dir(&ssh_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "pub")
                && let Some(filename) = path.file_name()
            {
                pub_keys.push((filename.to_string_lossy().to_string(), path));
            }
        }
    }

    if pub_keys.is_empty() {
        println!("❌ No public keys found");
        return Ok(());
    }

    let key_names: Vec<String> = pub_keys.iter().map(|(name, _)| name.clone()).collect();

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select public key to copy")
        .items(&key_names)
        .default(0)
        .interact_opt()
    {
        Ok(Some(choice)) => choice,
        Ok(None) => return Ok(()), // User cancelled
        Err(e) => {
            println!("Selection failed: {}", e);
            return Ok(());
        }
    };

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

                if let Ok(mut child) = cmd.stdin(std::process::Stdio::piped()).spawn()
                    && let Some(mut stdin) = child.stdin.take()
                {
                    use std::io::Write;
                    if stdin.write_all(content.trim().as_bytes()).is_ok() {
                        drop(stdin); // Close stdin to signal EOF
                        if child.wait().is_ok_and(|s| s.success()) {
                            println!("Public key copied to clipboard using {}", tool);
                            copied = true;
                            break;
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
    Ok(())
}

fn add_to_agent() {
    println!("🌐 Add Key to SSH Agent");
    println!("=======================");

    // Start SSH agent if not running
    if Command::new("ssh-add").arg("-l").status().is_err() {
        println!("Starting SSH agent...");
        match Command::new("ssh-agent").arg("-s").output() {
            Ok(output) if output.status.success() => {
                // Parse and display the agent environment variables
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("SSH agent started. Run the following to use it:");
                println!("{}", stdout);
            }
            Ok(_) => {
                println!("Warning: SSH agent started but may not be configured correctly");
            }
            Err(e) => {
                log::warn!("Failed to start SSH agent: {}", e);
                println!("Warning: Could not start SSH agent: {}", e);
            }
        }
    }

    let ssh_dir = match dirs::home_dir() {
        Some(home) => home.join(".ssh"),
        None => {
            println!("Could not determine home directory");
            return;
        }
    };
    let mut private_keys = Vec::new();

    if let Ok(entries) = fs::read_dir(&ssh_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && path.extension().is_none_or(|ext| ext != "pub")
                && let Some(filename) = path.file_name()
            {
                let filename_str = filename.to_string_lossy();
                if filename_str.starts_with("id_") || filename_str.contains("key") {
                    private_keys.push((filename_str.to_string(), path));
                }
            }
        }
    }

    if private_keys.is_empty() {
        println!("❌ No private keys found");
        return;
    }

    let key_names: Vec<String> = private_keys.iter().map(|(name, _)| name.clone()).collect();

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select private key to add")
        .items(&key_names)
        .default(0)
        .interact_opt()
    {
        Ok(Some(choice)) => choice,
        Ok(None) => return, // User cancelled
        Err(e) => {
            println!("Selection failed: {}", e);
            return;
        }
    };

    let (_, key_path) = &private_keys[choice];

    let key_path_str = match key_path.to_str() {
        Some(s) => s,
        None => {
            println!("Invalid key path");
            return;
        }
    };

    let status = Command::new("ssh-add").arg(key_path_str).status();

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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("SSH Configuration")
        .items(&config_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(choice)) => choice,
        Ok(None) => return, // User cancelled
        Err(e) => {
            println!("Selection failed: {}", e);
            return;
        }
    };

    match choice {
        0 => edit_ssh_config(),
        1 => show_ssh_config(),
        2 => create_ssh_config_template(),
        3 => configure_key_auth(),
        _ => return,
    }
}

fn edit_ssh_config() {
    let ssh_dir = match dirs::home_dir() {
        Some(home) => home.join(".ssh"),
        None => {
            println!("Could not determine home directory");
            return;
        }
    };
    let config_path = ssh_dir.join("config");

    // Create .ssh directory if it doesn't exist with secure permissions
    if let Err(e) = fs::create_dir_all(&ssh_dir) {
        println!("Failed to create SSH directory: {}", e);
        return;
    }

    // Set secure permissions on .ssh directory (0700)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(&ssh_dir) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o700);
            if let Err(e) = fs::set_permissions(&ssh_dir, perms) {
                log::warn!("Failed to set .ssh directory permissions: {}", e);
            }
        }
    }

    // Create empty config if it doesn't exist
    if !config_path.exists() {
        if let Err(e) = fs::write(&config_path, "# SSH Client Configuration\n") {
            println!("Failed to create SSH config: {}", e);
            return;
        }

        // Set secure permissions on config file (0600)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(&config_path) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o600);
                if let Err(e) = fs::set_permissions(&config_path, perms) {
                    log::warn!("Failed to set SSH config permissions: {}", e);
                }
            }
        }

        println!("Created new SSH config file with secure permissions");
    }

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

    let config_path_str = match config_path.to_str() {
        Some(s) => s,
        None => {
            println!("Invalid config path");
            return;
        }
    };

    let status = Command::new(&editor).arg(config_path_str).status();

    match status {
        Ok(s) if s.success() => println!("SSH config edited"),
        _ => println!("Failed to edit SSH config"),
    }
}

fn show_ssh_config() {
    let config_path = match dirs::home_dir() {
        Some(home) => home.join(".ssh/config"),
        None => {
            println!("Could not determine home directory");
            return;
        }
    };

    if let Ok(content) = fs::read_to_string(&config_path) {
        println!("SSH Client Configuration:");
        println!("============================");
        println!("{}", content);
    } else {
        println!("No SSH config found at ~/.ssh/config");
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

    let config_path = match dirs::home_dir() {
        Some(home) => home.join(".ssh/config.template"),
        None => {
            println!("Could not determine home directory");
            return;
        }
    };

    if let Err(e) = fs::write(&config_path, template) {
        println!("Failed to create template: {}", e);
        return;
    }

    // Set secure permissions (0600)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(&config_path) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o600);
            if let Err(e) = fs::set_permissions(&config_path, perms) {
                log::warn!("Failed to set template permissions: {}", e);
            }
        }
    }

    println!("SSH config template created at ~/.ssh/config.template");
    println!("Copy sections to ~/.ssh/config as needed");
}

fn configure_key_auth() {
    println!("🔑 Configure Key-Based Authentication");
    println!("====================================");

    let server: String = match Input::new()
        .with_prompt("Server hostname or IP")
        .validate_with(|input: &String| -> Result<(), &str> { validate_hostname(input) })
        .interact_text()
    {
        Ok(s) => s,
        Err(e) => {
            println!("Input failed: {}", e);
            return;
        }
    };

    let username: String = match Input::new()
        .with_prompt("Username")
        .validate_with(|input: &String| -> Result<(), &str> { validate_username(input) })
        .interact_text()
    {
        Ok(s) => s,
        Err(e) => {
            println!("Input failed: {}", e);
            return;
        }
    };

    // List available public keys
    let ssh_dir = match dirs::home_dir() {
        Some(home) => home.join(".ssh"),
        None => {
            println!("Could not determine home directory");
            return;
        }
    };
    let mut pub_keys = Vec::new();

    if let Ok(entries) = fs::read_dir(&ssh_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "pub")
                && let Some(filename) = path.file_name()
            {
                pub_keys.push((filename.to_string_lossy().to_string(), path));
            }
        }
    }

    if pub_keys.is_empty() {
        println!("❌ No public keys found. Generate one first.");
        return;
    }

    let key_names: Vec<String> = pub_keys.iter().map(|(name, _)| name.clone()).collect();

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select public key to install")
        .items(&key_names)
        .default(0)
        .interact_opt()
    {
        Ok(Some(choice)) => choice,
        Ok(None) => return, // User cancelled
        Err(e) => {
            println!("Selection failed: {}", e);
            return;
        }
    };

    let (_, key_path) = &pub_keys[choice];

    let key_path_str = match key_path.to_str() {
        Some(s) => s,
        None => {
            println!("Invalid key path");
            return;
        }
    };

    println!("Installing public key on {}@{}", username, server);

    let status = Command::new("ssh-copy-id")
        .args(&["-i", key_path_str, &format!("{}@{}", username, server)])
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

    let proceed = match Confirm::new()
        .with_prompt("Apply security hardening to SSH daemon?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(true)) => true,
        Ok(Some(false)) | Ok(None) => return, // User said no or cancelled
        Err(e) => {
            println!("Input failed: {}", e);
            return;
        }
    };

    if !proceed {
        return;
    }

    // Backup original config
    match Command::new("sudo")
        .args(["cp", "/etc/ssh/sshd_config", "/etc/ssh/sshd_config.backup"])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Backed up sshd_config to /etc/ssh/sshd_config.backup");
        }
        Ok(_) => {
            println!("Warning: Could not backup sshd_config (permission denied?)");
        }
        Err(e) => {
            log::warn!("Failed to backup sshd_config: {}", e);
            println!("Warning: Could not backup sshd_config: {}", e);
        }
    }

    let hardening_options = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select hardening options")
        .items(&[
            "Disable root login",
            "Disable password authentication",
            "Change default port (22 -> 2222)",
            "Set login timeout",
            "Enable detailed logging",
        ])
        .interact_opt()
    {
        Ok(Some(options)) => options,
        Ok(None) => return, // User cancelled
        Err(e) => {
            println!("Selection failed: {}", e);
            return;
        }
    };

    println!("SSH daemon hardening not fully implemented yet");
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

// Helper functions for error handling and validation
fn get_ssh_dir() -> Result<PathBuf> {
    dirs::home_dir()
        .ok_or_else(|| SshError::DirectoryError("Could not determine home directory".to_string()))
        .map(|home| home.join(".ssh"))
        .map_err(Into::into)
}

fn validate_key_filename(filename: &str) -> Result<(), &'static str> {
    if filename.trim().is_empty() {
        return Err("Filename cannot be empty");
    }
    if filename.contains('/') || filename.contains('\\') {
        return Err("Filename cannot contain path separators");
    }
    if filename.contains("..") {
        return Err("Filename cannot contain '..'");
    }
    if filename.starts_with('.') && filename.len() > 1 {
        return Err("Filename should not start with '.'");
    }
    Ok(())
}

fn validate_hostname(hostname: &str) -> Result<(), &'static str> {
    if hostname.trim().is_empty() {
        return Err("Hostname cannot be empty");
    }
    if hostname.contains(' ') {
        return Err("Hostname cannot contain spaces");
    }
    if hostname.len() > 253 {
        return Err("Hostname too long");
    }
    Ok(())
}

fn validate_username(username: &str) -> Result<(), &'static str> {
    if username.trim().is_empty() {
        return Err("Username cannot be empty");
    }
    if username.contains(' ') {
        return Err("Username cannot contain spaces");
    }
    if username.contains('@') {
        return Err("Username should not contain '@' symbol");
    }
    Ok(())
}
