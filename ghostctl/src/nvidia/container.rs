use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::fs;
use std::process::Command;

pub fn container_menu() {
    println!("🐳 NVIDIA Container & Virtualization");
    println!("====================================");

    let options = [
        "📊 Check container runtime status",
        "🐳 Setup Docker GPU support",
        "🦀 Setup Podman GPU support",
        "🏗️  Install NVIDIA Container Runtime",
        "🧪 Test GPU access in containers",
        "🔧 Fix container GPU issues",
        "📋 List available GPU devices",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Container & GPU Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => check_container_status(),
        1 => setup_docker_gpu(),
        2 => setup_podman_gpu(),
        3 => install_nvidia_container_runtime(),
        4 => test_gpu_containers(),
        5 => fix_container_issues(),
        6 => list_gpu_devices(),
        _ => return,
    }
}

pub fn check_container_status() {
    println!("📊 Checking NVIDIA container runtime status...\n");

    // Check if NVIDIA drivers are working
    println!("=== NVIDIA DRIVER STATUS ===");
    let nvidia_smi_status = Command::new("nvidia-smi").status();
    match nvidia_smi_status {
        Ok(status) if status.success() => println!("✅ NVIDIA drivers working"),
        _ => {
            println!("❌ NVIDIA drivers not working");
            return;
        }
    }

    // Check NVIDIA Container Runtime
    println!("\n=== NVIDIA CONTAINER RUNTIME ===");
    let output = Command::new("which")
        .arg("nvidia-container-runtime")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!("✅ nvidia-container-runtime found");

            // Check version
            let version_output = Command::new("nvidia-container-runtime")
                .arg("--version")
                .output();

            if let Ok(version_output) = version_output {
                println!(
                    "  Version: {}",
                    String::from_utf8_lossy(&version_output.stdout).trim()
                );
            }
        }
        _ => println!("❌ nvidia-container-runtime not found"),
    }

    // Check NVIDIA Container CLI
    println!("\n=== NVIDIA CONTAINER CLI ===");
    let output = Command::new("which").arg("nvidia-container-cli").output();

    match output {
        Ok(output) if output.status.success() => {
            println!("✅ nvidia-container-cli found");

            // List available devices
            let devices_output = Command::new("nvidia-container-cli")
                .args(&["list"])
                .output();

            if let Ok(devices_output) = devices_output {
                println!("  Available devices:");
                for line in String::from_utf8_lossy(&devices_output.stdout).lines() {
                    if !line.trim().is_empty() {
                        println!("    {}", line);
                    }
                }
            }
        }
        _ => println!("❌ nvidia-container-cli not found"),
    }

    // Check Docker configuration
    println!("\n=== DOCKER CONFIGURATION ===");
    check_docker_nvidia_config();

    // Check Podman configuration
    println!("\n=== PODMAN CONFIGURATION ===");
    check_podman_nvidia_config();
}

fn check_docker_nvidia_config() {
    // Check if Docker is installed
    let docker_status = Command::new("which").arg("docker").status();
    if !docker_status.map(|s| s.success()).unwrap_or(false) {
        println!("⚠️  Docker not installed");
        return;
    }

    // Check daemon.json
    let daemon_json_path = "/etc/docker/daemon.json";
    if let Ok(content) = fs::read_to_string(daemon_json_path) {
        if content.contains("nvidia") {
            println!("✅ Docker daemon.json configured for NVIDIA");
        } else {
            println!("⚠️  Docker daemon.json not configured for NVIDIA");
        }
    } else {
        println!("⚠️  Docker daemon.json not found");
    }

    // Check Docker service status
    let status = Command::new("systemctl")
        .args(&["is-active", "docker"])
        .output();

    if let Ok(status) = status {
        let status_output = String::from_utf8_lossy(&status.stdout);
        let status_str = status_output.trim();
        if status_str == "active" {
            println!("✅ Docker service is running");
        } else {
            println!("⚠️  Docker service is not running");
        }
    }
}

fn check_podman_nvidia_config() {
    let podman_status = Command::new("which").arg("podman").status();
    if !podman_status.map(|s| s.success()).unwrap_or(false) {
        println!("⚠️  Podman not installed");
        return;
    }

    // Check OCI runtime configuration
    let output = Command::new("podman")
        .args(&["info", "--format", "json"])
        .output();

    if let Ok(output) = output {
        let info = String::from_utf8_lossy(&output.stdout);
        if info.contains("nvidia") {
            println!("✅ Podman configured for NVIDIA");
        } else {
            println!("⚠️  Podman not configured for NVIDIA");
        }
    }
}

pub fn setup_docker_gpu() {
    println!("🐳 Setting up Docker GPU support...");

    // Check if Docker is installed
    if !check_docker_installed() {
        let install = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Docker not found. Install Docker?")
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(false);

        if install {
            install_docker();
        } else {
            return;
        }
    }

    // Install NVIDIA Container Runtime if not present
    install_nvidia_container_runtime();

    // Configure Docker daemon
    configure_docker_daemon();

    // Restart Docker service
    restart_docker_service();

    println!("✅ Docker GPU support configured");
    println!("💡 Test with: docker run --rm --gpus all nvidia/cuda:11.0-base nvidia-smi");
}

pub fn setup_podman_gpu() {
    println!("🦀 Setting up Podman GPU support...");

    // Check if Podman is installed
    if !check_podman_installed() {
        let install = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Podman not found. Install Podman?")
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(false);

        if install {
            install_podman();
        } else {
            return;
        }
    }

    // Install required packages
    install_nvidia_container_runtime();

    // Configure Podman for NVIDIA
    configure_podman_nvidia();

    println!("✅ Podman GPU support configured");
    println!(
        "💡 Test with: podman run --rm --device nvidia.com/gpu=all nvidia/cuda:11.0-base nvidia-smi"
    );
}

pub fn install_nvidia_container_runtime() {
    println!("🏗️  Installing NVIDIA Container Runtime...");

    // Check if already installed
    let output = Command::new("which")
        .arg("nvidia-container-runtime")
        .output();

    if let Ok(output) = output
        && output.status.success()
    {
        println!("⚠️  NVIDIA Container Runtime already installed");
        let reinstall = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Reinstall anyway?")
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(false);

        if !reinstall {
            return;
        }
    }

    // Install from repositories
    let packages = [
        "nvidia-container-toolkit",
        "libnvidia-container",
        "nvidia-container-runtime",
    ];

    println!("📦 Installing packages: {}", packages.join(", "));

    let mut cmd = Command::new("sudo");
    cmd.args(&["pacman", "-S", "--noconfirm"]);
    cmd.args(&packages);

    let status = cmd.status();
    match status {
        Ok(status) if status.success() => {
            println!("✅ NVIDIA Container Runtime installed successfully");
        }
        _ => {
            println!("⚠️  Failed to install from repositories, trying AUR...");
            install_from_aur();
        }
    }
}

fn install_from_aur() {
    // Try to install from AUR
    let aur_helper = detect_aur_helper();
    if let Some(helper) = aur_helper {
        let packages = ["nvidia-container-toolkit", "libnvidia-container"];

        let mut cmd = Command::new(helper);
        cmd.args(&["-S", "--noconfirm"]);
        cmd.args(&packages);

        let status = cmd.status();
        match status {
            Ok(status) if status.success() => {
                println!("✅ NVIDIA Container Runtime installed from AUR");
            }
            _ => println!("❌ Failed to install NVIDIA Container Runtime"),
        }
    } else {
        println!("❌ No AUR helper found and repository installation failed");
    }
}

fn detect_aur_helper() -> Option<String> {
    let helpers = ["yay", "paru", "trizen", "pikaur"];
    for helper in &helpers {
        if Command::new("which").arg(helper).status().is_ok() {
            return Some(helper.to_string());
        }
    }
    None
}

fn check_docker_installed() -> bool {
    Command::new("which")
        .arg("docker")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn check_podman_installed() -> bool {
    Command::new("which")
        .arg("podman")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn install_docker() {
    println!("📦 Installing Docker...");

    let _ = Command::new("sudo")
        .args(&["pacman", "-S", "--noconfirm", "docker"])
        .status();

    // Enable and start Docker service
    let _ = Command::new("sudo")
        .args(&["systemctl", "enable", "--now", "docker"])
        .status();

    // Add user to docker group
    if let Ok(username) = std::env::var("USER") {
        let _ = Command::new("sudo")
            .args(&["usermod", "-aG", "docker", &username])
            .status();

        println!(
            "💡 Added {} to docker group. Log out and back in for changes to take effect",
            username
        );
    }
}

fn install_podman() {
    println!("📦 Installing Podman...");

    let _ = Command::new("sudo")
        .args(&["pacman", "-S", "--noconfirm", "podman", "crun"])
        .status();
}

fn configure_docker_daemon() {
    println!("⚙️  Configuring Docker daemon...");

    let daemon_config = r#"{
    "runtimes": {
        "nvidia": {
            "path": "nvidia-container-runtime",
            "runtimeArgs": []
        }
    },
    "default-runtime": "nvidia"
}
"#;

    // Create /etc/docker directory if it doesn't exist
    let _ = Command::new("sudo")
        .args(&["mkdir", "-p", "/etc/docker"])
        .status();

    // Write daemon.json
    let _ = fs::write("/tmp/daemon.json", daemon_config);
    let _ = Command::new("sudo")
        .args(&["mv", "/tmp/daemon.json", "/etc/docker/daemon.json"])
        .status();

    println!("✅ Docker daemon configured");
}

fn configure_podman_nvidia() {
    println!("⚙️  Configuring Podman for NVIDIA...");

    // Configure containers.conf
    let containers_conf = r#"[containers]
default_capabilities = [
  "CHOWN",
  "DAC_OVERRIDE", 
  "FOWNER",
  "FSETID",
  "KILL",
  "NET_BIND_SERVICE",
  "SETFCAP",
  "SETGID",
  "SETPCAP",
  "SETUID",
  "SYS_CHROOT"
]

[engine]
runtime = "nvidia"

[engine.runtimes]
nvidia = [
    "/usr/bin/nvidia-container-runtime"
]
"#;

    // Create config directory
    let config_dir = format!(
        "{}/.config/containers",
        std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
    );
    let _ = fs::create_dir_all(&config_dir);

    // Write config
    let config_path = format!("{}/containers.conf", config_dir);
    let _ = fs::write(&config_path, containers_conf);

    println!("✅ Podman configured for NVIDIA");
}

fn restart_docker_service() {
    println!("🔄 Restarting Docker service...");

    let _ = Command::new("sudo")
        .args(&["systemctl", "restart", "docker"])
        .status();

    // Wait a moment for service to start
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Check if service started successfully
    let status = Command::new("systemctl")
        .args(&["is-active", "docker"])
        .output();

    if let Ok(status) = status {
        let status_output = String::from_utf8_lossy(&status.stdout);
        let status_str = status_output.trim();
        if status_str == "active" {
            println!("✅ Docker service restarted successfully");
        } else {
            println!("⚠️  Docker service failed to start");
        }
    }
}

pub fn test_gpu_containers() {
    println!("🧪 Testing GPU access in containers...");

    let container_engines = if check_docker_installed() && check_podman_installed() {
        vec!["Docker", "Podman"]
    } else if check_docker_installed() {
        vec!["Docker"]
    } else if check_podman_installed() {
        vec!["Podman"]
    } else {
        println!("❌ No container engines found");
        return;
    };

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select container engine to test")
        .items(&container_engines)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match container_engines[choice] {
        "Docker" => test_docker_gpu(),
        "Podman" => test_podman_gpu(),
        _ => {}
    }
}

fn test_docker_gpu() {
    println!("🐳 Testing Docker GPU access...");

    // Pull CUDA test image
    println!("📥 Pulling CUDA test image...");
    let _ = Command::new("docker")
        .args(&["pull", "nvidia/cuda:11.0-base"])
        .status();

    // Test GPU access
    println!("🧪 Running GPU test...");
    let status = Command::new("docker")
        .args(&[
            "run",
            "--rm",
            "--gpus",
            "all",
            "nvidia/cuda:11.0-base",
            "nvidia-smi",
        ])
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("✅ Docker GPU access working!");
        }
        _ => {
            println!("❌ Docker GPU access failed");
            println!("💡 Try: docker run --rm --runtime=nvidia nvidia/cuda:11.0-base nvidia-smi");
        }
    }
}

fn test_podman_gpu() {
    println!("🦀 Testing Podman GPU access...");

    // Pull CUDA test image
    println!("📥 Pulling CUDA test image...");
    let _ = Command::new("podman")
        .args(&["pull", "nvidia/cuda:11.0-base"])
        .status();

    // Test GPU access
    println!("🧪 Running GPU test...");
    let status = Command::new("podman")
        .args(&[
            "run",
            "--rm",
            "--device",
            "nvidia.com/gpu=all",
            "nvidia/cuda:11.0-base",
            "nvidia-smi",
        ])
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("✅ Podman GPU access working!");
        }
        _ => {
            println!("❌ Podman GPU access failed");
            println!(
                "💡 Try: podman run --rm --security-opt=label=disable --device=nvidia.com/gpu=all nvidia/cuda:11.0-base nvidia-smi"
            );
        }
    }
}

pub fn fix_container_issues() {
    println!("🔧 Fixing container GPU issues...");

    let options = [
        "Fix Docker permissions",
        "Restart container services",
        "Reset container configurations",
        "Check SELinux/AppArmor conflicts",
        "Update container runtime",
        "Cancel",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select fix option")
        .items(&options)
        .default(5)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        _ => return,
    };

    match choice {
        0 => fix_docker_permissions(),
        1 => restart_container_services(),
        2 => reset_container_configs(),
        3 => check_security_conflicts(),
        4 => update_container_runtime(),
        _ => return,
    }
}

fn fix_docker_permissions() {
    println!("🔧 Fixing Docker permissions...");

    if let Ok(username) = std::env::var("USER") {
        let status = Command::new("sudo")
            .args(["usermod", "-aG", "docker", &username])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Added {} to docker group", username);
                println!("💡 Log out and back in for changes to take effect");
                println!("   (or run: newgrp docker)");
            }
            Ok(_) => {
                println!("❌ Failed to add {} to docker group", username);
            }
            Err(e) => {
                println!("❌ Failed to run usermod: {}", e);
            }
        }
    } else {
        println!("⚠️  Could not determine current user");
    }

    // Ensure docker socket has correct group ownership (docker group, not world-writable)
    let status = Command::new("sudo")
        .args(["chgrp", "docker", "/var/run/docker.sock"])
        .status();

    if let Ok(s) = status
        && s.success()
    {
        println!("✅ Docker socket group ownership set to 'docker'");
    }

    println!("✅ Docker permissions configured (group-based access)");
}

fn restart_container_services() {
    println!("🔄 Restarting container services...");

    if check_docker_installed() {
        match Command::new("sudo")
            .args(["systemctl", "restart", "docker"])
            .status()
        {
            Ok(status) if status.success() => {
                println!("  ✅ Docker service restarted");
            }
            Ok(_) => {
                println!("  ⚠️  Docker restart returned non-zero status");
            }
            Err(e) => {
                println!("  ❌ Failed to restart Docker: {}", e);
            }
        }
    }

    // Restart containerd if present (non-critical, may not be installed)
    match Command::new("sudo")
        .args(["systemctl", "restart", "containerd"])
        .status()
    {
        Ok(status) if status.success() => {
            println!("  ✅ Containerd service restarted");
        }
        Ok(_) => {
            // Containerd may not be installed, don't warn
        }
        Err(_) => {
            // Containerd may not be installed
        }
    }

    println!("✅ Container services restart complete");
}

fn reset_container_configs() {
    println!("♻️  Resetting container configurations...");

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will reset Docker and Podman configurations. Continue?")
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if !confirm {
        return;
    }

    // Backup and reset Docker config
    if std::path::Path::new("/etc/docker/daemon.json").exists() {
        let _ = Command::new("sudo")
            .args(&[
                "cp",
                "/etc/docker/daemon.json",
                "/etc/docker/daemon.json.backup",
            ])
            .status();
        let _ = Command::new("sudo")
            .args(&["rm", "/etc/docker/daemon.json"])
            .status();
    }

    // Reconfigure
    configure_docker_daemon();
    configure_podman_nvidia();

    println!("✅ Container configurations reset");
}

fn check_security_conflicts() {
    println!("🔒 Checking for security conflicts...");

    // Check SELinux
    let selinux_status = Command::new("getenforce").output();
    if let Ok(output) = selinux_status {
        let status_output = String::from_utf8_lossy(&output.stdout);
        let status = status_output.trim();
        if status == "Enforcing" {
            println!("⚠️  SELinux is enforcing - may block GPU access");
            println!("💡 Consider: sudo setsebool -P container_use_devices on");
        }
    }

    // Check AppArmor
    let apparmor_status = Command::new("systemctl")
        .args(&["is-active", "apparmor"])
        .output();

    if let Ok(output) = apparmor_status {
        let status_output = String::from_utf8_lossy(&output.stdout);
        let status = status_output.trim();
        if status == "active" {
            println!("⚠️  AppArmor is active - may block GPU access");
        }
    }

    println!("✅ Security check complete");
}

fn update_container_runtime() {
    println!("📦 Updating container runtime packages...");

    let _ = Command::new("sudo")
        .args(&["pacman", "-Syu", "--noconfirm", "nvidia-container-toolkit"])
        .status();

    println!("✅ Container runtime updated");
}

pub fn list_gpu_devices() {
    println!("📋 Listing available GPU devices...");

    println!("\n=== NVIDIA-SMI OUTPUT ===");
    let _ = Command::new("nvidia-smi").args(&["-L"]).status();

    println!("\n=== NVIDIA CONTAINER CLI ===");
    let output = Command::new("nvidia-container-cli")
        .args(&["list"])
        .output();

    if let Ok(output) = output {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("❌ nvidia-container-cli not available");
    }

    println!("\n=== PCI DEVICES ===");
    let _ = Command::new("lspci")
        .args(&["-nn", "|", "grep", "-i", "nvidia"])
        .status();
}
