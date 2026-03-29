pub mod cleanup;
pub mod compose;
pub mod container;
pub mod devops;
pub mod registry;
pub mod security;

use crate::tui;
use crate::utils::is_headless;
use std::process::Command;

/// Validate a Docker container name or ID
/// Container names must match: [a-zA-Z0-9][a-zA-Z0-9_.-]*
/// Container IDs are 12 or 64 hex characters
pub fn validate_container_name(name: &str) -> Result<(), String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("Container name cannot be empty".to_string());
    }

    // Check for shell injection characters
    if name.contains(|c: char| {
        matches!(
            c,
            '`' | '$' | '(' | ')' | '{' | '}' | ';' | '&' | '|' | '<' | '>' | '\n' | '\r'
        )
    }) {
        return Err("Container name contains invalid characters".to_string());
    }

    // Check if it looks like a container ID (12 or 64 hex chars)
    if (name.len() == 12 || name.len() == 64) && name.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok(());
    }

    // Check container name format
    let chars: Vec<char> = name.chars().collect();
    if !chars[0].is_ascii_alphanumeric() {
        return Err("Container name must start with an alphanumeric character".to_string());
    }

    for c in &chars {
        if !c.is_ascii_alphanumeric() && *c != '_' && *c != '.' && *c != '-' {
            return Err(format!(
                "Container name contains invalid character: '{}'",
                c
            ));
        }
    }

    Ok(())
}

/// Validate a Docker image name
/// Format: [registry/][repository/]name[:tag]
pub fn validate_image_name(image: &str) -> Result<(), String> {
    let image = image.trim();
    if image.is_empty() {
        return Err("Image name cannot be empty".to_string());
    }

    // Check for shell injection characters
    if image.contains(|c: char| {
        matches!(
            c,
            '`' | '$' | '(' | ')' | '{' | '}' | ';' | '&' | '|' | '<' | '>' | '\n' | '\r' | ' '
        )
    }) {
        return Err("Image name contains invalid characters".to_string());
    }

    // Basic validation: must contain only valid characters
    for c in image.chars() {
        if !c.is_ascii_alphanumeric() && !matches!(c, '/' | ':' | '.' | '-' | '_' | '@') {
            return Err(format!("Image name contains invalid character: '{}'", c));
        }
    }

    Ok(())
}

/// Validate a filter duration string (e.g., "24h", "7d")
pub fn validate_duration_filter(duration: &str) -> Result<(), String> {
    let duration = duration.trim();
    if duration.is_empty() {
        return Err("Duration cannot be empty".to_string());
    }

    // Must be digits followed by a valid unit
    let chars: Vec<char> = duration.chars().collect();
    let unit_start = chars.iter().position(|c| !c.is_ascii_digit());

    if unit_start.is_none() || unit_start == Some(0) {
        return Err("Duration must be a number followed by a unit (h, d, w, m)".to_string());
    }

    let unit_idx = unit_start.unwrap();
    let unit: String = chars[unit_idx..].iter().collect();

    if !matches!(unit.as_str(), "h" | "d" | "w" | "m" | "s") {
        return Err(
            "Duration unit must be one of: s (seconds), h (hours), d (days), w (weeks), m (months)"
                .to_string(),
        );
    }

    Ok(())
}

/// Check if Docker is installed
pub fn is_docker_installed() -> bool {
    Command::new("which")
        .arg("docker")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Check if Docker daemon is running
pub fn is_docker_running() -> bool {
    Command::new("docker")
        .arg("info")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Get detailed Docker status
pub fn get_docker_status() -> DockerStatus {
    let installed = is_docker_installed();
    let running = if installed {
        is_docker_running()
    } else {
        false
    };

    DockerStatus { installed, running }
}

pub struct DockerStatus {
    pub installed: bool,
    pub running: bool,
}

impl DockerStatus {
    pub fn display(&self) {
        if !self.installed {
            println!("❌ Docker is not installed");
            println!();
            println!("💡 Install Docker:");
            println!("   Arch Linux: sudo pacman -S docker docker-compose");
            println!("   Ubuntu/Debian: sudo apt install docker.io docker-compose");
            println!(
                "   Or use the official install script: curl -fsSL https://get.docker.com | sh"
            );
        } else if !self.running {
            println!("⚠️  Docker is installed but the daemon is not running");
            println!();
            println!("💡 Start Docker:");
            println!("   sudo systemctl start docker");
            println!("   sudo systemctl enable docker  # Auto-start on boot");
            println!();
            println!("   If you're not in the docker group:");
            println!("   sudo usermod -aG docker $USER");
            println!("   (Log out and back in for this to take effect)");
        }
    }
}

/// Offer to install Docker if not present
fn offer_docker_install() -> bool {
    if is_headless() {
        tui::warn("Docker installation requires interactive mode.");
        tui::info("Install manually: curl -fsSL https://get.docker.com | sh");
        return false;
    }

    let options = [
        "📦 Install Docker (Official Script)",
        "📦 Install via Package Manager (Arch)",
        "📦 Install via Package Manager (Debian/Ubuntu)",
        "📋 Show manual instructions",
    ];

    let choice = match tui::select_with_back("Docker Installation", &options, 0) {
        Some(c) => c,
        None => return false,
    };

    match choice {
        0 => {
            println!("📥 Installing Docker via official script...");
            println!(
                "  Note: For production use, verify the install script manually before running."
            );

            // Download and execute in two steps (slightly safer than direct pipe)
            let download = Command::new("curl")
                .args([
                    "-fsSL",
                    "-o",
                    "/tmp/get-docker.sh",
                    "https://get.docker.com",
                ])
                .status();

            if !download.map(|s| s.success()).unwrap_or(false) {
                println!("❌ Failed to download Docker installation script");
                return false;
            }

            let status = Command::new("sh").arg("/tmp/get-docker.sh").status();

            // Clean up
            let _ = std::fs::remove_file("/tmp/get-docker.sh");

            match status {
                Ok(s) if s.success() => {
                    println!("✅ Docker installed successfully");

                    // Add user to docker group
                    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
                    match Command::new("sudo")
                        .args(["usermod", "-aG", "docker", &user])
                        .status()
                    {
                        Ok(s) if s.success() => {
                            println!("👤 Added {} to docker group", user);
                        }
                        _ => {
                            tui::warn(&format!(
                                "Could not add {} to docker group. You may need to do this manually.",
                                user
                            ));
                        }
                    }
                    println!("⚠️  Please log out and back in for group changes to take effect");

                    // Start and enable docker service
                    if let Err(e) = Command::new("sudo")
                        .args(["systemctl", "start", "docker"])
                        .status()
                    {
                        tui::warn(&format!("Could not start docker service: {}", e));
                    }
                    if let Err(e) = Command::new("sudo")
                        .args(["systemctl", "enable", "docker"])
                        .status()
                    {
                        tui::warn(&format!("Could not enable docker service: {}", e));
                    }

                    println!("🚀 Docker service started and enabled");
                    true
                }
                _ => {
                    println!("❌ Docker installation failed");
                    false
                }
            }
        }
        1 => {
            println!("📥 Installing Docker via pacman...");
            let status = Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "docker", "docker-compose"])
                .status();

            match status {
                Ok(s) if s.success() => {
                    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
                    if let Err(e) = Command::new("sudo")
                        .args(["usermod", "-aG", "docker", &user])
                        .status()
                    {
                        tui::warn(&format!("Could not add user to docker group: {}", e));
                    }
                    if let Err(e) = Command::new("sudo")
                        .args(["systemctl", "start", "docker"])
                        .status()
                    {
                        tui::warn(&format!("Could not start docker: {}", e));
                    }
                    if let Err(e) = Command::new("sudo")
                        .args(["systemctl", "enable", "docker"])
                        .status()
                    {
                        tui::warn(&format!("Could not enable docker: {}", e));
                    }

                    println!("✅ Docker installed and started");
                    println!("⚠️  Please log out and back in for group changes to take effect");
                    true
                }
                _ => {
                    println!("❌ Installation failed");
                    false
                }
            }
        }
        2 => {
            println!("📥 Installing Docker via apt...");
            let status = Command::new("sudo")
                .args(["apt", "install", "-y", "docker.io", "docker-compose"])
                .status();

            match status {
                Ok(s) if s.success() => {
                    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
                    if let Err(e) = Command::new("sudo")
                        .args(["usermod", "-aG", "docker", &user])
                        .status()
                    {
                        tui::warn(&format!("Could not add user to docker group: {}", e));
                    }
                    if let Err(e) = Command::new("sudo")
                        .args(["systemctl", "start", "docker"])
                        .status()
                    {
                        tui::warn(&format!("Could not start docker: {}", e));
                    }
                    if let Err(e) = Command::new("sudo")
                        .args(["systemctl", "enable", "docker"])
                        .status()
                    {
                        tui::warn(&format!("Could not enable docker: {}", e));
                    }

                    println!("✅ Docker installed and started");
                    println!("⚠️  Please log out and back in for group changes to take effect");
                    true
                }
                _ => {
                    println!("❌ Installation failed");
                    false
                }
            }
        }
        3 => {
            println!("\n📋 Manual Docker Installation:");
            println!("================================");
            println!();
            println!("Arch Linux:");
            println!("  sudo pacman -S docker docker-compose");
            println!("  sudo systemctl enable --now docker");
            println!("  sudo usermod -aG docker $USER");
            println!();
            println!("Debian/Ubuntu:");
            println!("  sudo apt update");
            println!("  sudo apt install docker.io docker-compose");
            println!("  sudo systemctl enable --now docker");
            println!("  sudo usermod -aG docker $USER");
            println!();
            println!("Official Script (any distro):");
            println!("  curl -fsSL https://get.docker.com | sh");
            println!();
            println!("After installation, log out and back in for group changes.");
            false
        }
        _ => false,
    }
}

/// Offer to start Docker if not running
fn offer_docker_start() -> bool {
    if !tui::confirm("Start Docker daemon now?", true) {
        return false;
    }

    tui::status("🚀", "Starting Docker daemon...");
    let status = Command::new("sudo")
        .args(["systemctl", "start", "docker"])
        .status();

    match status {
        Ok(s) if s.success() => {
            // Wait a moment for Docker to be ready
            std::thread::sleep(std::time::Duration::from_secs(2));

            if is_docker_running() {
                tui::success("Docker daemon started");
                true
            } else {
                tui::warn("Docker started but may need a moment to be ready");
                true
            }
        }
        _ => {
            tui::error("Failed to start Docker");
            tui::info("Try manually: sudo systemctl start docker");
            false
        }
    }
}

/// Check Docker prerequisites before allowing access to Docker features
pub fn require_docker() -> bool {
    let status = get_docker_status();

    if !status.installed {
        status.display();
        println!();
        return offer_docker_install();
    }

    if !status.running {
        status.display();
        println!();
        return offer_docker_start();
    }

    true
}

pub fn docker_menu() {
    if is_headless() {
        tui::warn("Docker menu cannot be displayed in headless mode. Use CLI subcommands instead.");
        tui::info("Example: ghostctl docker ps");
        return;
    }

    // Check Docker prerequisites first
    if !require_docker() {
        tui::warn("Docker is required for this feature.");
        return;
    }

    let options = [
        "Docker Management & DevOps",
        "Container Operations",
        "Compose Stack Manager",
        "Registry Management",
        "Docker Cleanup Tools",
        "Security & Scanning",
    ];

    while let Some(selection) = tui::select_with_back("Docker Management", &options, 0) {
        match selection {
            0 => devops::docker_management(),
            1 => container::container_management(),
            2 => compose::compose_stack_manager(),
            3 => registry::registry_management(),
            4 => cleanup::cleanup_menu(),
            5 => security::container_security(),
            _ => {}
        }
    }
}

pub fn install_docker() {
    println!("🐳 Installing Docker");
    println!("===================");
    devops::docker_management();
}

pub fn homelab_stacks_menu() {
    println!("🏠 Homelab Docker Stacks");
    println!("========================");
    compose::compose_stack_manager();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docker_status_display_not_installed() {
        let status = DockerStatus {
            installed: false,
            running: false,
        };
        // Just verify no panic when displaying
        status.display();
    }

    #[test]
    fn test_docker_status_display_not_running() {
        let status = DockerStatus {
            installed: true,
            running: false,
        };
        // Just verify no panic when displaying
        status.display();
    }

    #[test]
    fn test_docker_status_display_running() {
        let status = DockerStatus {
            installed: true,
            running: true,
        };
        // Display should do nothing when both are true
        status.display();
    }

    #[test]
    fn test_docker_status_fields() {
        let status = DockerStatus {
            installed: true,
            running: true,
        };
        assert!(status.installed);
        assert!(status.running);

        let status2 = DockerStatus {
            installed: false,
            running: false,
        };
        assert!(!status2.installed);
        assert!(!status2.running);
    }
}
