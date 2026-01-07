use indicatif::{ProgressBar, ProgressStyle};
use std::process::{Command, Output};
use std::time::Duration;

/// Execute a command with a spinner progress indicator
pub fn execute_with_spinner(cmd: &mut Command, message: &str) -> std::io::Result<Output> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(80));

    let result = cmd.output();

    spinner.finish_and_clear();
    result
}

/// Execute a command with a spinner and display success/failure
pub fn execute_with_status(cmd: &mut Command, message: &str) -> std::io::Result<Output> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(80));

    let result = cmd.output();

    match &result {
        Ok(output) if output.status.success() => {
            spinner.finish_with_message(format!("✅ {}", message));
        }
        Ok(_) => {
            spinner.finish_with_message(format!("⚠️  {}", message));
        }
        Err(_) => {
            spinner.finish_with_message(format!("❌ {}", message));
        }
    }

    result
}

/// Filter expected permission errors from pacman output
pub fn filter_permission_errors(stderr: &str) -> String {
    let permission_patterns = ["Permission denied", "failed to calculate SHA256 checksum"];

    let protected_paths = [
        "/etc/audit/",
        "/etc/sudoers",
        "/etc/shadow",
        "/usr/share/factory/etc/audit/",
        "/var/log/audit/",
    ];

    stderr
        .lines()
        .filter(|line| {
            // Keep the line if it doesn't match permission error patterns
            let has_permission_error = permission_patterns
                .iter()
                .any(|pattern| line.contains(pattern));

            if !has_permission_error {
                return true;
            }

            // If it's a permission error, only filter if it's on a protected path
            let is_protected = protected_paths.iter().any(|path| line.contains(path));

            !is_protected
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Parse broken packages from pacman -Qk output
pub fn parse_broken_packages(output: &str) -> Vec<String> {
    let mut broken_packages = Vec::new();

    for line in output.lines() {
        // pacman -Qk format: "package: x total files, y altered files"
        // We want packages with altered files > 0 or missing files
        if line.contains("altered files") {
            if let Some(pkg_end) = line.find(':') {
                let package = line[..pkg_end].trim();

                // Check if has altered files
                if line.contains(", 0 altered files") {
                    continue;
                }

                // Also check for "missing" or "warning" in the line
                if line.contains("altered") && !line.contains(", 0 altered") {
                    broken_packages.push(package.to_string());
                }
            }
        } else if line.starts_with("warning:") {
            // Extract package name from warning lines
            // Format: "warning: package: file (reason)"
            if let Some(rest) = line.strip_prefix("warning: ")
                && let Some(pkg_end) = rest.find(':')
            {
                let package = rest[..pkg_end].trim();
                if !broken_packages.contains(&package.to_string()) {
                    // Only add if not from permission errors
                    let is_permission_error = line.contains("Permission denied")
                        || line.contains("failed to calculate SHA256");
                    if !is_permission_error {
                        broken_packages.push(package.to_string());
                    }
                }
            }
        }
    }

    broken_packages.sort();
    broken_packages.dedup();
    broken_packages
}

/// Create a progress bar for iterating over items
pub fn create_progress_bar(len: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message(message.to_string());
    pb
}
