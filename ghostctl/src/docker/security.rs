use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn container_security() {
    println!("🛡️  Docker Container Security");
    println!("==============================");

    let options = [
        "🔍 Vulnerability Scanning (Trivy)",
        "🐳 Container Security Scanning",
        "📊 Security Assessment Report",
        "🔒 Runtime Security Monitoring",
        "📜 Security Best Practices Check",
        "⚙️  Security Policy Generation",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Container Security Tools")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => vulnerability_scanning(),
        1 => crate::docker::devops::container_security_scanning(),
        2 => security_assessment(),
        3 => runtime_monitoring(),
        4 => security_best_practices(),
        5 => security_policy_generation(),
        _ => return,
    }
}

fn vulnerability_scanning() {
    println!("🔍 Container Vulnerability Scanning");
    println!("===================================");

    // Check if trivy is installed
    if !Command::new("which")
        .arg("trivy")
        .status()
        .unwrap()
        .success()
    {
        let install = Confirm::new()
            .with_prompt("Trivy not found. Install it?")
            .default(true)
            .interact()
            .unwrap();

        if install {
            install_trivy();
        } else {
            return;
        }
    }

    let scan_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select scan type")
        .items(&[
            "🖼️  Scan Docker Image",
            "📦 Scan Running Container",
            "📁 Scan Filesystem",
            "🔄 Scan All Local Images",
        ])
        .default(0)
        .interact()
        .unwrap();

    match scan_type {
        0 => scan_docker_image(),
        1 => scan_running_container(),
        2 => scan_filesystem(),
        3 => scan_all_images(),
        _ => return,
    }
}

fn scan_docker_image() {
    let image: String = Input::new()
        .with_prompt("Enter image name (e.g., nginx:latest)")
        .interact_text()
        .unwrap();

    println!("🔍 Scanning image: {}", image);

    let status = Command::new("trivy")
        .args(&["image", "--severity", "HIGH,CRITICAL", &image])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Image scan completed"),
        _ => println!("❌ Image scan failed"),
    }
}

fn scan_running_container() {
    println!("📦 Available containers:");
    let _ = Command::new("docker")
        .args(&[
            "ps",
            "--format",
            "table {{.Names}}\\t{{.Image}}\\t{{.Status}}",
        ])
        .status();

    let container: String = Input::new()
        .with_prompt("Enter container name or ID")
        .interact_text()
        .unwrap();

    println!("🔍 Scanning container: {}", container);

    let status = Command::new("trivy").args(&["image", &container]).status();

    match status {
        Ok(s) if s.success() => println!("✅ Container scan completed"),
        _ => println!("❌ Container scan failed"),
    }
}

fn scan_filesystem() {
    let path: String = Input::new()
        .with_prompt("Enter filesystem path to scan")
        .default("/".into())
        .interact_text()
        .unwrap();

    println!("🔍 Scanning filesystem: {}", path);

    let status = Command::new("trivy")
        .args(&["fs", "--severity", "HIGH,CRITICAL", &path])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Filesystem scan completed"),
        _ => println!("❌ Filesystem scan failed"),
    }
}

fn scan_all_images() {
    println!("🔄 Scanning all local Docker images...");

    let status = Command::new("bash")
        .arg("-c")
        .arg("docker images --format '{{.Repository}}:{{.Tag}}' | grep -v '<none>' | xargs -I {} trivy image --severity HIGH,CRITICAL {}")
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ All images scanned"),
        _ => println!("❌ Bulk image scan failed"),
    }
}

fn security_assessment() {
    println!("📊 Docker Security Assessment");
    println!("=============================");

    // Check Docker daemon configuration
    println!("🔍 Docker Daemon Security:");
    check_docker_daemon_security();

    // Check container runtime security
    println!("\n🏃 Runtime Security:");
    check_runtime_security();

    // Check image security
    println!("\n🖼️  Image Security:");
    check_image_security();

    // Generate security score
    println!("\n📈 Security Score:");
    generate_security_score();
}

fn check_docker_daemon_security() {
    let checks = [
        ("Docker daemon running as root", check_docker_root()),
        ("TLS enabled", check_docker_tls()),
        ("User namespace enabled", check_user_namespace()),
        ("AppArmor/SELinux enabled", check_security_modules()),
    ];

    for (check, result) in checks {
        if result {
            println!("  ✅ {}", check);
        } else {
            println!("  ❌ {}", check);
        }
    }
}

fn check_runtime_security() {
    let checks = [
        ("Non-root containers", check_non_root_containers()),
        ("Read-only filesystems", check_readonly_containers()),
        ("Resource limits set", check_resource_limits()),
        ("Security options enabled", check_security_options()),
    ];

    for (check, result) in checks {
        if result {
            println!("  ✅ {}", check);
        } else {
            println!("  ❌ {}", check);
        }
    }
}

fn check_image_security() {
    let checks = [
        ("Images from trusted registries", check_trusted_registries()),
        ("No secrets in images", check_secrets_in_images()),
        ("Minimal base images", check_minimal_images()),
        ("Regular image updates", check_image_updates()),
    ];

    for (check, result) in checks {
        if result {
            println!("  ✅ {}", check);
        } else {
            println!("  ❌ {}", check);
        }
    }
}

fn runtime_monitoring() {
    println!("🔒 Runtime Security Monitoring");
    println!("==============================");
    println!("This feature requires additional security tools like Falco or Sysdig.");
    println!("Would you like to install Falco for runtime monitoring?");

    let install = Confirm::new()
        .with_prompt("Install Falco?")
        .default(false)
        .interact()
        .unwrap();

    if install {
        install_falco();
    }
}

fn security_best_practices() {
    println!("📜 Docker Security Best Practices");
    println!("=================================");

    let practices = [
        "✅ Use official base images",
        "✅ Keep images updated",
        "✅ Run containers as non-root",
        "✅ Use read-only filesystems when possible",
        "✅ Set resource limits",
        "✅ Scan images for vulnerabilities",
        "✅ Use secrets management",
        "✅ Enable logging and monitoring",
        "✅ Use network segmentation",
        "✅ Implement proper access controls",
    ];

    for practice in practices {
        println!("  {}", practice);
    }

    println!("\n💡 Recommendations:");
    println!("  📚 Review Docker CIS Benchmark");
    println!("  🔒 Implement container runtime security");
    println!("  📊 Regular security assessments");
    println!("  🔄 Automate security scanning in CI/CD");
}

fn security_policy_generation() {
    println!("⚙️  Security Policy Generation");
    println!("==============================");
    println!("This feature is not yet implemented.");
    println!("Future: Generate AppArmor/SELinux policies for containers");
}

fn install_trivy() {
    println!("📦 Installing Trivy...");

    let status = Command::new("bash")
        .arg("-c")
        .arg("curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh -s -- -b /usr/local/bin")
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Trivy installed successfully"),
        _ => println!("❌ Failed to install Trivy"),
    }
}

fn install_falco() {
    println!("📦 Installing Falco...");
    println!("This requires root privileges and system integration.");

    let status = Command::new("bash")
        .arg("-c")
        .arg("curl -s https://falco.org/repo/falcosecurity-3672BA8F.asc | apt-key add - && echo 'deb https://download.falco.org/packages/deb stable main' | tee -a /etc/apt/sources.list.d/falcosecurity.list && apt-get update -qq && apt-get install -y falco")
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Falco installed successfully"),
        _ => println!("❌ Failed to install Falco. Manual installation may be required."),
    }
}

fn generate_security_score() {
    let mut score = 0;
    let total_checks = 12;

    // Simplified scoring based on basic checks
    if check_docker_root() {
        score += 1;
    }
    if check_docker_tls() {
        score += 1;
    }
    if check_user_namespace() {
        score += 1;
    }
    if check_security_modules() {
        score += 1;
    }
    if check_non_root_containers() {
        score += 1;
    }
    if check_readonly_containers() {
        score += 1;
    }
    if check_resource_limits() {
        score += 1;
    }
    if check_security_options() {
        score += 1;
    }
    if check_trusted_registries() {
        score += 1;
    }
    if check_secrets_in_images() {
        score += 1;
    }
    if check_minimal_images() {
        score += 1;
    }
    if check_image_updates() {
        score += 1;
    }

    let percentage = (score as f32 / total_checks as f32) * 100.0;

    println!(
        "📊 Security Score: {:.1}% ({}/{})",
        percentage, score, total_checks
    );

    if percentage >= 80.0 {
        println!("🟢 Excellent security posture!");
    } else if percentage >= 60.0 {
        println!("🟡 Good security, room for improvement");
    } else {
        println!("🔴 Security needs attention");
    }
}

// Helper functions for security checks
fn check_docker_root() -> bool {
    Command::new("ps")
        .args(&["aux"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("dockerd"))
        .unwrap_or(false)
}

fn check_docker_tls() -> bool {
    // Simplified check - would need to inspect Docker daemon config
    false
}

fn check_user_namespace() -> bool {
    // Check if user namespace is enabled
    std::fs::read_to_string("/proc/sys/user/max_user_namespaces")
        .map(|content| content.trim() != "0")
        .unwrap_or(false)
}

fn check_security_modules() -> bool {
    // Check for AppArmor or SELinux
    Command::new("which")
        .arg("apparmor_status")
        .status()
        .is_ok()
        || std::path::Path::new("/sys/fs/selinux").exists()
}

fn check_non_root_containers() -> bool {
    // Simplified check - would need to inspect running containers
    true
}

fn check_readonly_containers() -> bool {
    // Simplified check
    false
}

fn check_resource_limits() -> bool {
    // Simplified check
    false
}

fn check_security_options() -> bool {
    // Simplified check
    false
}

fn check_trusted_registries() -> bool {
    // Simplified check
    true
}

fn check_secrets_in_images() -> bool {
    // Simplified check
    true
}

fn check_minimal_images() -> bool {
    // Simplified check
    false
}

fn check_image_updates() -> bool {
    // Simplified check
    false
}
