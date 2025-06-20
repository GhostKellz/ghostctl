use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::fs;

pub fn setup_schedule() {
    println!("📅 Setup Backup Schedule");
    println!("========================");

    let options = [
        "⏰ Create Systemd Timer",
        "📋 View Current Schedule",
        "🔄 Enable/Disable Timer",
        "🗑️  Remove Schedule",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Schedule Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => create_systemd_timer(),
        1 => view_current_schedule(),
        2 => toggle_timer(),
        3 => remove_schedule(),
        _ => (),
    }
}

fn create_systemd_timer() {
    println!("⏰ Create Backup Schedule");
    println!("========================");

    let frequency = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Backup frequency")
        .items(&[
            "🌅 Daily at 2 AM",
            "📅 Weekly on Sunday 3 AM",
            "🗓️  Monthly on 1st at 4 AM",
            "⚙️  Custom schedule",
        ])
        .default(0)
        .interact()
        .unwrap();

    let timer_spec = match frequency {
        0 => "*-*-* 02:00:00".to_string(),
        1 => "Sun *-*-* 03:00:00".to_string(),
        2 => "*-*-01 04:00:00".to_string(),
        3 => {
            println!("📝 Enter systemd timer specification:");
            println!("Examples:");
            println!("  Daily 2 AM: *-*-* 02:00:00");
            println!("  Every 6 hours: *-*-* 00/6:00:00");
            println!("  Weekdays 9 AM: Mon..Fri *-*-* 09:00:00");

            Input::new()
                .with_prompt("Timer specification")
                .interact_text()
                .unwrap()
        }
        _ => return,
    };

    let backup_paths: String = Input::new()
        .with_prompt("Paths to backup (space-separated)")
        .default("/home /etc /var/log".into())
        .interact_text()
        .unwrap();

    let config_path = dirs::config_dir().unwrap().join("ghostctl/restic.env");
    if !config_path.exists() {
        println!("❌ No restic configuration found. Run backup setup first.");
        return;
    }

    // Create systemd service
    let service_content = format!(
        r#"[Unit]
Description=Ghostctl Restic Backup
After=network-online.target
Wants=network-online.target

[Service]
Type=oneshot
EnvironmentFile={}
ExecStart=/usr/bin/restic backup {}
ExecStartPost=/usr/bin/restic forget --prune --keep-daily 7 --keep-weekly 4 --keep-monthly 12
User=%i
"#,
        config_path.display(),
        backup_paths
    );

    // Create systemd timer
    let timer_content = format!(
        r#"[Unit]
Description=Ghostctl Backup Timer
Requires=ghostctl-backup.service

[Timer]
OnCalendar={}
Persistent=true
RandomizedDelaySec=30min

[Install]
WantedBy=timers.target
"#,
        timer_spec
    );

    let systemd_dir = dirs::config_dir().unwrap().join("systemd/user");
    fs::create_dir_all(&systemd_dir).unwrap();

    fs::write(systemd_dir.join("ghostctl-backup.service"), service_content).unwrap();
    fs::write(systemd_dir.join("ghostctl-backup.timer"), timer_content).unwrap();

    println!("✅ Systemd timer created!");
    println!("🔧 To enable: systemctl --user enable --now ghostctl-backup.timer");
    println!("📋 To check: systemctl --user status ghostctl-backup.timer");
}

fn view_current_schedule() {
    println!("📋 Current Backup Schedule");
    println!("=========================");

    // Check if systemd timer exists
    let timer_status = std::process::Command::new("systemctl")
        .args(["--user", "is-active", "ghostctl-backup.timer"])
        .output();

    match timer_status {
        Ok(output) if output.status.success() => {
            println!("✅ Backup timer is active");

            // Show timer details
            let _ = std::process::Command::new("systemctl")
                .args(["--user", "status", "ghostctl-backup.timer"])
                .status();

            println!("\n📅 Next scheduled runs:");
            let _ = std::process::Command::new("systemctl")
                .args(["--user", "list-timers", "ghostctl-backup.timer"])
                .status();
        }
        _ => {
            println!("❌ No active backup timer found");
            println!("💡 Create a schedule using 'Create Systemd Timer'");
        }
    }
}

fn toggle_timer() {
    println!("🔄 Enable/Disable Backup Timer");
    println!("==============================");

    let timer_status = std::process::Command::new("systemctl")
        .args(["--user", "is-enabled", "ghostctl-backup.timer"])
        .output();

    let is_enabled = timer_status.is_ok() && timer_status.unwrap().status.success();

    if is_enabled {
        let disable = Confirm::new()
            .with_prompt("Timer is currently enabled. Disable it?")
            .default(false)
            .interact()
            .unwrap();

        if disable {
            let _ = std::process::Command::new("systemctl")
                .args(["--user", "disable", "--now", "ghostctl-backup.timer"])
                .status();
            println!("⏹️  Backup timer disabled");
        }
    } else {
        let enable = Confirm::new()
            .with_prompt("Timer is currently disabled. Enable it?")
            .default(true)
            .interact()
            .unwrap();

        if enable {
            let _ = std::process::Command::new("systemctl")
                .args(["--user", "enable", "--now", "ghostctl-backup.timer"])
                .status();
            println!("▶️  Backup timer enabled");
        }
    }
}

fn remove_schedule() {
    println!("🗑️  Remove Backup Schedule");
    println!("=========================");

    let confirm = Confirm::new()
        .with_prompt("Are you sure you want to remove the backup schedule?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        // Stop and disable timer
        let _ = std::process::Command::new("systemctl")
            .args(["--user", "disable", "--now", "ghostctl-backup.timer"])
            .status();

        // Remove systemd files
        let systemd_dir = dirs::config_dir().unwrap().join("systemd/user");
        let _ = fs::remove_file(systemd_dir.join("ghostctl-backup.service"));
        let _ = fs::remove_file(systemd_dir.join("ghostctl-backup.timer"));

        println!("✅ Backup schedule removed");
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Running restic backup...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("restic backup /etc /home")
        .status();
    match status {
        Ok(s) if s.success() => println!("Backup completed successfully."),
        _ => println!("Backup failed."),
    }
}

#[allow(dead_code)]
pub fn schedule() {
    println!("Scheduling restic backup (stub, implement systemd timer or cron)");
}
