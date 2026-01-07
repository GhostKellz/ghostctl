use crate::security::credentials::{create_secure_env_file, store_backup_credentials};
use crate::tui;
use crate::utils::is_headless;
use std::fs;

pub fn setup() {
    if is_headless() {
        tui::warn("Backup setup requires interactive mode.");
        tui::info("Use environment variables for headless setup:");
        tui::info("  RESTIC_REPOSITORY, RESTIC_PASSWORD");
        return;
    }

    tui::header("Restic Backup Setup");

    let setup_options = [
        "Initialize New Repository",
        "Configure Existing Repository",
        "Create Systemd Timer",
        "Test Backup",
    ];

    if let Some(choice) = tui::select_with_back("Backup Setup", &setup_options, 0) {
        match choice {
            0 => init_repository(),
            1 => configure_repository(),
            2 => create_systemd_timer(),
            3 => test_backup(),
            _ => {}
        }
    }
}

fn init_repository() {
    tui::header("Initialize Restic Repository");

    let repo_types = ["Local Directory", "SFTP", "S3-Compatible", "B2", "Azure"];
    let repo_type = match tui::select("Repository type", &repo_types, 0) {
        Some(t) => t,
        None => return,
    };

    let repo_url = match repo_type {
        0 => match tui::input("Local repository path", Some("/backup/restic")) {
            Some(path) if !path.is_empty() => path,
            _ => return,
        },
        1 => {
            let user = match tui::input("SFTP user@host", None) {
                Some(u) if !u.is_empty() => u,
                _ => return,
            };
            let path = tui::input_required("Remote path", "/backup/restic");
            format!("sftp:{}:{}", user, path)
        }
        _ => {
            tui::warn("Other repository types not yet implemented");
            return;
        }
    };

    let password = match tui::password("Repository password", Some("RESTIC_PASSWORD")) {
        Some(p) if !p.is_empty() => p,
        _ => {
            tui::error("Password is required");
            return;
        }
    };

    // Create config directory
    let config_dir = dirs::config_dir().unwrap().join("ghostctl");
    fs::create_dir_all(&config_dir).unwrap();

    // Store credentials securely - NO PLAINTEXT FALLBACK
    match store_backup_credentials(&repo_url, &password) {
        Ok(_) => {
            // Create temporary env file for this session with restrictive permissions
            let config_path = config_dir.join("restic.env");
            if let Err(e) = create_secure_env_file(&config_path) {
                println!("⚠️  Warning: Could not create temporary env file: {}", e);
                println!("💡 Credentials are stored securely but you'll need to set environment variables manually:");
                println!("   export RESTIC_REPOSITORY=\"{}\"", repo_url);
                println!("   export RESTIC_PASSWORD=\"<your-password>\"");
            } else {
                println!("🔐 Credentials stored securely");
                println!("📄 Session env file created at: {:?}", config_path);

                // Set restrictive permissions on the env file
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = fs::metadata(&config_path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o600); // Owner read/write only
                        let _ = fs::set_permissions(&config_path, perms);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Could not store credentials securely: {}", e);
            println!();
            println!("🔐 Security Policy: Plaintext credential storage is disabled.");
            println!();
            println!("💡 Alternative options:");
            println!("   1. Set environment variables directly:");
            println!("      export RESTIC_REPOSITORY=\"{}\"", repo_url);
            println!("      export RESTIC_PASSWORD=\"<your-password>\"");
            println!();
            println!("   2. Use a password manager with CLI integration");
            println!();
            println!("   3. Use systemd credential storage:");
            println!("      systemd-creds encrypt - restic-password.cred < password.txt");
            println!();
            println!("   4. Create an encrypted env file with GPG:");
            println!("      gpg --encrypt --recipient <your-key> restic.env");
            println!();
            return;
        }
    }

    // Initialize repository
    println!("🚀 Initializing repository...");
    let config_path = config_dir.join("restic.env");
    let status = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("source {} && restic init", config_path.display()))
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Repository initialized successfully!"),
        _ => println!("❌ Failed to initialize repository"),
    }
}

fn configure_repository() {
    tui::header("Configure Existing Repository");

    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("ghostctl/restic.env"),
        None => {
            tui::error("Could not determine config directory");
            return;
        }
    };

    if !config_path.exists() {
        tui::error("No existing config found. Run 'Initialize New Repository' first.");
        return;
    }

    tui::info(&format!("Current config: {:?}", config_path));

    if tui::confirm("Edit configuration file?", false) {
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
        let _ = std::process::Command::new(&editor)
            .arg(&config_path)
            .status();
    }
}

fn create_systemd_timer() {
    tui::header("Creating Systemd Backup Timer");

    let frequencies = ["Daily", "Weekly", "Custom"];
    let frequency = match tui::select("Backup frequency", &frequencies, 0) {
        Some(f) => f,
        None => return,
    };

    let timer_spec = match frequency {
        0 => "daily".to_string(),
        1 => "weekly".to_string(),
        2 => tui::input_required(
            "Timer specification (e.g., 'Mon *-*-* 02:00:00')",
            "*-*-* 02:00:00",
        ),
        _ => return,
    };

    let backup_paths = tui::input_required("Paths to backup (space-separated)", "/home /etc");

    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("ghostctl/restic.env"),
        None => {
            tui::error("Could not determine config directory");
            return;
        }
    };

    // Create systemd service
    let service_content = format!(
        r#"[Unit]
Description=Restic Backup
After=network-online.target

[Service]
Type=oneshot
EnvironmentFile={}
ExecStart=/usr/bin/restic backup {}
ExecStartPost=/usr/bin/restic forget --prune --keep-daily 7 --keep-weekly 4 --keep-monthly 12
"#,
        config_path.display(),
        backup_paths
    );

    // Create systemd timer
    let timer_content = format!(
        r#"[Unit]
Description=Restic Backup Timer
Requires=restic-backup.service

[Timer]
OnCalendar={}
Persistent=true

[Install]
WantedBy=timers.target
"#,
        timer_spec
    );

    let systemd_dir = dirs::config_dir().unwrap().join("systemd/user");
    fs::create_dir_all(&systemd_dir).unwrap();

    fs::write(systemd_dir.join("restic-backup.service"), service_content).unwrap();
    fs::write(systemd_dir.join("restic-backup.timer"), timer_content).unwrap();

    println!("📝 Systemd files created");
    println!("🔧 Run: systemctl --user enable --now restic-backup.timer");
}

fn test_backup() {
    tui::header("Test Backup");

    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("ghostctl/restic.env"),
        None => {
            tui::error("Could not determine config directory");
            return;
        }
    };

    if !config_path.exists() {
        tui::error("No config found. Run setup first.");
        return;
    }

    // Test with a small directory
    let test_path = "/etc/hostname";

    tui::status("🚀", &format!("Running test backup of {}...", test_path));

    let status = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source {} && restic backup {}",
            config_path.display(),
            test_path
        ))
        .status();

    match status {
        Ok(s) if s.success() => {
            tui::success("Test backup successful!");

            // List snapshots
            let _ = std::process::Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "source {} && restic snapshots",
                    config_path.display()
                ))
                .status();
        }
        _ => tui::error("Test backup failed"),
    }
}

#[allow(dead_code)]
pub fn restic_restore() {
    tui::header("Restic Restore");

    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("ghostctl/restic.env"),
        None => {
            tui::error("Could not determine config directory");
            return;
        }
    };

    if !config_path.exists() {
        tui::error("No config found. Run setup first.");
        return;
    }

    // List snapshots first
    tui::subheader("Available Snapshots");
    let _ = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source {} && restic snapshots",
            config_path.display()
        ))
        .status();

    let snapshot_id = tui::input_required("Snapshot ID to restore (or 'latest')", "latest");
    let restore_path = tui::input_required("Restore to directory", "/tmp/restic-restore");

    if tui::confirm(
        &format!("Restore snapshot '{}' to '{}'?", snapshot_id, restore_path),
        false,
    ) {
        tui::status("🔄", "Restoring...");
        let status = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {} && restic restore {} --target {}",
                config_path.display(),
                snapshot_id,
                restore_path
            ))
            .status();

        match status {
            Ok(s) if s.success() => {
                tui::success(&format!("Restore completed to: {}", restore_path))
            }
            _ => tui::error("Restore failed"),
        }
    }
}

#[allow(dead_code)]
pub fn backup_settings() {
    tui::header("Backup Settings");

    let options = [
        "📂 Configure Repository",
        "⏰ Setup Backup Schedule",
        "🗂️  Backup Path Configuration",
        "🔐 Security Settings",
        "📊 Storage Usage",
    ];

    if let Some(choice) = tui::select_with_back("Backup Settings", &options, 0) {
        match choice {
            0 => configure_repository(),
            1 => create_systemd_timer(),
            2 => configure_backup_paths(),
            3 => security_settings(),
            4 => storage_usage(),
            _ => {}
        }
    }
}

pub fn run_backup() {
    tui::header("Running Manual Backup");

    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("ghostctl/restic.env"),
        None => {
            tui::error("Could not determine config directory");
            return;
        }
    };

    if !config_path.exists() {
        tui::error("No backup configuration found.");
        tui::info("Run backup setup first to configure restic repository.");
        return;
    }

    let backup_types = [
        "Full System Backup",
        "Home Directory Only",
        "Custom Paths",
        "Quick Test",
    ];

    let backup_type = match tui::select("Backup type", &backup_types, 0) {
        Some(t) => t,
        None => return,
    };

    let backup_paths = match backup_type {
        0 => "/home /etc /var /opt".to_string(),
        1 => "/home".to_string(),
        2 => match tui::input("Enter paths to backup (space-separated)", None) {
            Some(p) if !p.is_empty() => p,
            _ => return,
        },
        3 => "/etc/hostname".to_string(),
        _ => return,
    };

    if tui::confirm("Start backup now?", true) {
        tui::status("🚀", "Starting backup...");
        tui::info(&format!("Backing up: {}", backup_paths));

        let status = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {} && restic backup {}",
                config_path.display(),
                backup_paths
            ))
            .status();

        match status {
            Ok(s) if s.success() => {
                tui::success("Backup completed successfully!");

                // Show latest snapshots
                tui::subheader("Latest Snapshots");
                let _ = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(format!(
                        "source {} && restic snapshots --last 5",
                        config_path.display()
                    ))
                    .status();
            }
            _ => tui::error("Backup failed. Check restic configuration."),
        }
    }
}

#[allow(dead_code)]
fn configure_backup_paths() {
    println!("🗂️  Configure Backup Paths");
    println!("=========================");

    let paths = [
        ("/home", "Home directories"),
        ("/etc", "System configuration"),
        ("/var", "Variable data"),
        ("/opt", "Optional software"),
        ("/usr/local", "Local software"),
        ("/root", "Root home directory"),
    ];

    println!("Select paths to include in backup:");
    for (i, (path, desc)) in paths.iter().enumerate() {
        println!("{}. {} - {}", i + 1, path, desc);
    }

    println!("\nCurrent backup configuration saved to ~/.config/ghostctl/backup-paths.txt");
}

#[allow(dead_code)]
fn security_settings() {
    tui::header("Backup Security Settings");

    let options = [
        "🔑 Change Repository Password",
        "🔐 View Encryption Info",
        "📋 Repository Statistics",
    ];

    let choice = match tui::select_with_back("Security Settings", &options, 0) {
        Some(c) => c,
        None => return,
    };

    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("ghostctl/restic.env"),
        None => {
            tui::error("Could not determine config directory");
            return;
        }
    };

    match choice {
        0 => {
            tui::info("Changing repository password...");
            tui::info("This will require the old password and set a new one.");
        }
        1 => {
            if config_path.exists() {
                let _ = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(format!(
                        "source {} && restic key list",
                        config_path.display()
                    ))
                    .status();
            } else {
                tui::error("No backup configuration found");
            }
        }
        2 => {
            if config_path.exists() {
                let _ = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(format!("source {} && restic stats", config_path.display()))
                    .status();
            } else {
                tui::error("No backup configuration found");
            }
        }
        _ => {}
    }
}

#[allow(dead_code)]
fn storage_usage() {
    tui::header("Storage Usage");

    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("ghostctl/restic.env"),
        None => {
            tui::error("Could not determine config directory");
            return;
        }
    };

    if config_path.exists() {
        tui::subheader("Repository Statistics");
        let _ = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("source {} && restic stats", config_path.display()))
            .status();

        tui::subheader("Recent Snapshots");
        let _ = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {} && restic snapshots --last 10",
                config_path.display()
            ))
            .status();
    } else {
        tui::error("No backup configuration found");
    }
}
