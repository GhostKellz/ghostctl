pub mod cleanup;
pub mod compose;
pub mod container;
pub mod devops;
pub mod registry;
pub mod security;

use crate::tui;
use crate::utils::is_headless;
use std::process::Command;

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
            let status = Command::new("bash")
                .arg("-c")
                .arg("curl -fsSL https://get.docker.com | sh")
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("✅ Docker installed successfully");

                    // Add user to docker group
                    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
                    let _ = Command::new("sudo")
                        .args(&["usermod", "-aG", "docker", &user])
                        .status();

                    println!("👤 Added {} to docker group", user);
                    println!("⚠️  Please log out and back in for group changes to take effect");

                    // Start and enable docker service
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "start", "docker"])
                        .status();
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "enable", "docker"])
                        .status();

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
                .args(&["pacman", "-S", "--noconfirm", "docker", "docker-compose"])
                .status();

            match status {
                Ok(s) if s.success() => {
                    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
                    let _ = Command::new("sudo")
                        .args(&["usermod", "-aG", "docker", &user])
                        .status();
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "start", "docker"])
                        .status();
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "enable", "docker"])
                        .status();

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
                .args(&["apt", "install", "-y", "docker.io", "docker-compose"])
                .status();

            match status {
                Ok(s) if s.success() => {
                    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
                    let _ = Command::new("sudo")
                        .args(&["usermod", "-aG", "docker", &user])
                        .status();
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "start", "docker"])
                        .status();
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "enable", "docker"])
                        .status();

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
        .args(&["systemctl", "start", "docker"])
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
