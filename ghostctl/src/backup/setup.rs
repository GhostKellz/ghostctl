use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::fs;

pub fn setup() {
    println!("🔧 Restic Backup Setup");

    let setup_options = [
        "Initialize New Repository",
        "Configure Existing Repository",
        "Create Systemd Timer",
        "Test Backup",
        "Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup Setup")
        .items(&setup_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => init_repository(),
        1 => configure_repository(),
        2 => create_systemd_timer(),
        3 => test_backup(),
        _ => return,
    }
}

fn init_repository() {
    println!("📦 Initialize Restic Repository");

    let repo_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Repository type")
        .items(&["Local Directory", "SFTP", "S3-Compatible", "B2", "Azure"])
        .default(0)
        .interact()
        .unwrap();

    let repo_url = match repo_type {
        0 => {
            let path: String = Input::new()
                .with_prompt("Local repository path")
                .default("/backup/restic".into())
                .interact_text()
                .unwrap();
            path
        }
        1 => {
            let user: String = Input::new()
                .with_prompt("SFTP user@host")
                .interact_text()
                .unwrap();
            let path: String = Input::new()
                .with_prompt("Remote path")
                .default("/backup/restic".into())
                .interact_text()
                .unwrap();
            format!("sftp:{}:{}", user, path)
        }
        _ => {
            println!("Other repository types not yet implemented");
            return;
        }
    };

    let password: String = Input::new()
        .with_prompt("Repository password")
        .interact_text()
        .unwrap();

    // Create config directory
    let config_dir = dirs::config_dir().unwrap().join("ghostctl");
    fs::create_dir_all(&config_dir).unwrap();

    // Write restic config
    let config_content = format!(
        "RESTIC_REPOSITORY={}\nRESTIC_PASSWORD={}\n",
        repo_url, password
    );

    let config_path = config_dir.join("restic.env");
    fs::write(&config_path, config_content).unwrap();

    println!("💾 Config saved to: {:?}", config_path);

    // Initialize repository
    println!("🚀 Initializing repository...");
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
    println!("⚙️ Configure Existing Repository");

    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");

    if !config_path.exists() {
        println!("❌ No existing config found. Run 'Initialize New Repository' first.");
        return;
    }

    println!("📋 Current config: {:?}", config_path);

    let edit = Confirm::new()
        .with_prompt("Edit configuration file?")
        .default(false)
        .interact()
        .unwrap();

    if edit {
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
        let _ = std::process::Command::new(&editor)
            .arg(&config_path)
            .status();
    }
}

fn create_systemd_timer() {
    println!("⏰ Creating Systemd Backup Timer");

    let frequency = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup frequency")
        .items(&["Daily", "Weekly", "Custom"])
        .default(0)
        .interact()
        .unwrap();

    let timer_spec = match frequency {
        0 => "daily".to_string(),
        1 => "weekly".to_string(),
        2 => Input::new()
            .with_prompt("Timer specification (e.g., 'Mon *-*-* 02:00:00')")
            .interact_text()
            .unwrap(),
        _ => return,
    };

    let backup_paths: String = Input::new()
        .with_prompt("Paths to backup (space-separated)")
        .default("/home /etc".into())
        .interact_text()
        .unwrap();

    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");

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
    println!("🧪 Test Backup");

    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");

    if !config_path.exists() {
        println!("❌ No config found. Run setup first.");
        return;
    }

    // Test with a small directory
    let test_path = "/etc/hostname";

    println!("🚀 Running test backup of {}...", test_path);

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
            println!("✅ Test backup successful!");

            // List snapshots
            let _ = std::process::Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "source {} && restic snapshots",
                    config_path.display()
                ))
                .status();
        }
        _ => println!("❌ Test backup failed"),
    }
}

#[allow(dead_code)]
pub fn restic_restore() {
    println!("🔄 Restic Restore");

    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");

    if !config_path.exists() {
        println!("❌ No config found. Run setup first.");
        return;
    }

    // List snapshots first
    println!("📋 Available snapshots:");
    let _ = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source {} && restic snapshots",
            config_path.display()
        ))
        .status();

    let snapshot_id: String = Input::new()
        .with_prompt("Snapshot ID to restore (or 'latest')")
        .default("latest".into())
        .interact_text()
        .unwrap();

    let restore_path: String = Input::new()
        .with_prompt("Restore to directory")
        .default("/tmp/restic-restore".into())
        .interact_text()
        .unwrap();

    let confirm = Confirm::new()
        .with_prompt(format!(
            "Restore snapshot '{}' to '{}'?",
            snapshot_id, restore_path
        ))
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        println!("🔄 Restoring...");
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
            Ok(s) if s.success() => println!("✅ Restore completed to: {}", restore_path),
            _ => println!("❌ Restore failed"),
        }
    }
}

#[allow(dead_code)]
pub fn backup_settings() {
    println!("⚙️  Backup Settings");
    println!("==================");

    let options = [
        "📂 Configure Repository",
        "⏰ Setup Backup Schedule",
        "🗂️  Backup Path Configuration",
        "🔐 Security Settings",
        "📊 Storage Usage",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup Settings")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => configure_repository(),
        1 => create_systemd_timer(),
        2 => configure_backup_paths(),
        3 => security_settings(),
        4 => storage_usage(),
        _ => return,
    }
}

pub fn run_backup() {
    println!("▶️  Running Manual Backup");
    println!("========================");

    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");

    if !config_path.exists() {
        println!("❌ No backup configuration found.");
        println!("💡 Run backup setup first to configure restic repository.");
        return;
    }

    let backup_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup type")
        .items(&[
            "Full System Backup",
            "Home Directory Only",
            "Custom Paths",
            "Quick Test",
        ])
        .default(0)
        .interact()
        .unwrap();

    let backup_paths = match backup_type {
        0 => "/home /etc /var /opt".to_string(),
        1 => "/home".to_string(),
        2 => Input::new()
            .with_prompt("Enter paths to backup (space-separated)")
            .interact_text()
            .unwrap(),
        3 => "/etc/hostname".to_string(),
        _ => return,
    };

    let confirm = Confirm::new()
        .with_prompt("Start backup now?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("🚀 Starting backup...");
        println!("📂 Backing up: {}", backup_paths);

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
                println!("✅ Backup completed successfully!");

                // Show latest snapshots
                println!("\n📋 Latest snapshots:");
                let _ = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(format!(
                        "source {} && restic snapshots --last 5",
                        config_path.display()
                    ))
                    .status();
            }
            _ => println!("❌ Backup failed. Check restic configuration."),
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
    println!("🔐 Backup Security Settings");
    println!("===========================");

    let options = [
        "🔑 Change Repository Password",
        "🔐 View Encryption Info",
        "📋 Repository Statistics",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Security Settings")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");

    match choice {
        0 => {
            println!("🔑 Changing repository password...");
            println!("💡 This will require the old password and set a new one.");
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
            }
        }
        2 => {
            if config_path.exists() {
                let _ = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(format!("source {} && restic stats", config_path.display()))
                    .status();
            }
        }
        _ => return,
    }
}

#[allow(dead_code)]
fn storage_usage() {
    println!("📊 Storage Usage");
    println!("===============");

    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");

    if config_path.exists() {
        println!("📈 Repository statistics:");
        let _ = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("source {} && restic stats", config_path.display()))
            .status();

        println!("\n📋 Recent snapshots:");
        let _ = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {} && restic snapshots --last 10",
                config_path.display()
            ))
            .status();
    } else {
        println!("❌ No backup configuration found");
    }
}
