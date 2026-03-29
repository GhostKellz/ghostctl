use chrono::Utc;
use log::{debug, error, info};
use std::fs::OpenOptions;
use std::io::Write;

pub struct GhostLogger;

impl GhostLogger {
    pub fn init() {
        env_logger::init();

        if let Some(data_dir) = dirs::data_dir() {
            let log_dir = data_dir.join("ghostctl");
            std::fs::create_dir_all(&log_dir).unwrap_or_else(|e| {
                eprintln!("Warning: Could not create log directory: {}", e);
            });
        }

        info!(
            "GhostCTL started at {}",
            Utc::now().format("%Y-%m-%d %H:%M:%S")
        );
    }

    pub fn log_action(action: &str, success: bool, details: Option<&str>) {
        let Some(data_dir) = dirs::data_dir() else {
            return;
        };
        let log_dir = data_dir.join("ghostctl");
        let log_file = log_dir.join("history.log");

        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
        let status = if success { "SUCCESS" } else { "FAILED" };
        let details_str = details.unwrap_or("");

        let log_entry = format!("[{}] {} - {} {}\n", timestamp, status, action, details_str);

        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_file) {
            let _ = file.write_all(log_entry.as_bytes());
        }

        if success {
            info!("{} completed successfully", action);
        } else {
            error!("{} failed: {}", action, details_str);
        }
    }

    pub fn show_recent_logs() {
        let Some(data_dir) = dirs::data_dir() else {
            println!("Could not determine data directory");
            return;
        };
        let log_file = data_dir.join("ghostctl/history.log");

        if !log_file.exists() {
            println!("📝 No log file found yet");
            return;
        }

        match std::fs::read_to_string(&log_file) {
            Ok(content) => {
                let lines: Vec<&str> = content.lines().collect();
                let recent_lines = lines.iter().rev().take(20).rev();

                println!("📋 Recent GhostCTL Activity (last 20 entries):");
                println!("═══════════════════════════════════════════════");

                for line in recent_lines {
                    if line.contains("SUCCESS") {
                        println!("✅ {}", line);
                    } else if line.contains("FAILED") {
                        println!("❌ {}", line);
                    } else {
                        println!("ℹ️  {}", line);
                    }
                }
            }
            Err(e) => println!("❌ Could not read log file: {}", e),
        }
    }
}

// Wrapper for consistent error handling
pub fn execute_with_logging<F>(action_name: &str, operation: F) -> Result<(), String>
where
    F: FnOnce() -> Result<(), String>,
{
    debug!("Starting: {}", action_name);

    match operation() {
        Ok(_) => {
            GhostLogger::log_action(action_name, true, None);
            Ok(())
        }
        Err(e) => {
            GhostLogger::log_action(action_name, false, Some(&e));
            Err(e)
        }
    }
}

pub fn safe_command(cmd: &str, args: &[&str], action_name: &str) -> Result<(), String> {
    execute_with_logging(action_name, || {
        let output = std::process::Command::new(cmd)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute {}: {}", cmd, e))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Command failed: {}", stderr))
        }
    })
}
