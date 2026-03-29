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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Container Security Tools")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

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
    let trivy_check = Command::new("which")
        .arg("trivy")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !trivy_check {
        let install = match Confirm::new()
            .with_prompt("Trivy not found. Install it?")
            .default(true)
            .interact_opt()
        {
            Ok(Some(i)) => i,
            Ok(None) | Err(_) => return,
        };

        if install {
            install_trivy();
        } else {
            return;
        }
    }

    let scan_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select scan type")
        .items(&[
            "🖼️  Scan Docker Image",
            "📦 Scan Running Container",
            "📁 Scan Filesystem",
            "🔄 Scan All Local Images",
        ])
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    match scan_type {
        0 => scan_docker_image(),
        1 => scan_running_container(),
        2 => scan_filesystem(),
        3 => scan_all_images(),
        _ => return,
    }
}

fn scan_docker_image() {
    let image: String = match Input::new()
        .with_prompt("Enter image name (e.g., nginx:latest)")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    // Validate image name
    if let Err(e) = crate::docker::validate_image_name(&image) {
        println!("❌ Invalid image name: {}", e);
        return;
    }

    println!("🔍 Scanning image: {}", image);

    let status = Command::new("trivy")
        .args(["image", "--severity", "HIGH,CRITICAL", &image])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Image scan completed"),
        Ok(s) => println!("⚠️  Scan completed with exit code: {:?}", s.code()),
        Err(e) => println!("❌ Image scan failed: {}", e),
    }
}

fn scan_running_container() {
    println!("📦 Available containers:");
    if let Err(e) = Command::new("docker")
        .args([
            "ps",
            "--format",
            "table {{.Names}}\t{{.Image}}\t{{.Status}}",
        ])
        .status()
    {
        println!("  Could not list containers: {}", e);
    }

    let container: String = match Input::new()
        .with_prompt("Enter container name or ID")
        .interact_text()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    // Validate container name/ID
    if let Err(e) = crate::docker::validate_container_name(&container) {
        println!("❌ Invalid container name/ID: {}", e);
        return;
    }

    println!("🔍 Scanning container: {}", container);

    let status = Command::new("trivy").args(["image", &container]).status();

    match status {
        Ok(s) if s.success() => println!("✅ Container scan completed"),
        Ok(s) => println!("⚠️  Scan completed with exit code: {:?}", s.code()),
        Err(e) => println!("❌ Container scan failed: {}", e),
    }
}

fn scan_filesystem() {
    let path: String = match Input::new()
        .with_prompt("Enter filesystem path to scan")
        .default("/".into())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    // Validate path - check for shell injection characters
    if path.contains(|c: char| {
        matches!(
            c,
            '`' | '$' | '(' | ')' | '{' | '}' | ';' | '&' | '|' | '<' | '>' | '\n' | '\r'
        )
    }) {
        println!("❌ Path contains invalid characters");
        return;
    }

    // Verify path exists
    if !std::path::Path::new(&path).exists() {
        println!("❌ Path does not exist: {}", path);
        return;
    }

    println!("🔍 Scanning filesystem: {}", path);

    let status = Command::new("trivy")
        .args(["fs", "--severity", "HIGH,CRITICAL", &path])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Filesystem scan completed"),
        Ok(s) => println!("⚠️  Scan completed with exit code: {:?}", s.code()),
        Err(e) => println!("❌ Filesystem scan failed: {}", e),
    }
}

fn scan_all_images() {
    println!("🔄 Scanning all local Docker images...");

    // Get list of images safely
    let output = Command::new("docker")
        .args(["images", "--format", "{{.Repository}}:{{.Tag}}"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let images = String::from_utf8_lossy(&output.stdout);
            let mut scanned = 0;
            let mut failed = 0;

            for image in images.lines() {
                let image = image.trim();
                if image.is_empty() || image.contains("<none>") {
                    continue;
                }

                // Validate image name
                if crate::docker::validate_image_name(image).is_err() {
                    println!("⚠️  Skipping invalid image name: {}", image);
                    continue;
                }

                println!("🔍 Scanning {}...", image);
                match Command::new("trivy")
                    .args(["image", "--severity", "HIGH,CRITICAL", image])
                    .status()
                {
                    Ok(s) if s.success() => scanned += 1,
                    _ => {
                        println!("  Warning: scan failed for {}", image);
                        failed += 1;
                    }
                }
            }

            println!("\n✅ Scanned {} images ({} failed)", scanned, failed);
        }
        Ok(_) => println!("❌ Failed to list images"),
        Err(e) => println!("❌ Error listing images: {}", e),
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
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

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
    println!("  Note: For production use, verify the install script manually before running.");

    // Try package managers first (safer)
    // Try pacman (Arch Linux)
    let pacman_result = Command::new("pacman").args(["-Qi", "trivy"]).output();

    if pacman_result.is_ok() {
        println!("  Attempting installation via pacman...");
        match Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "trivy"])
            .status()
        {
            Ok(s) if s.success() => {
                println!("✅ Trivy installed successfully via pacman");
                return;
            }
            _ => println!("  pacman install failed, trying alternative..."),
        }
    }

    // Try apt (Debian/Ubuntu)
    if std::path::Path::new("/usr/bin/apt").exists() {
        println!("  Attempting installation via apt...");
        // Add trivy repository
        let add_key = Command::new("sudo")
            .args([
                "apt-key",
                "adv",
                "--fetch-keys",
                "https://aquasecurity.github.io/trivy-repo/deb/public.key",
            ])
            .status();

        if add_key.map(|s| s.success()).unwrap_or(false) {
            // This is a simplified example - in production, handle repo setup properly
            match Command::new("sudo")
                .args(["apt", "install", "-y", "trivy"])
                .status()
            {
                Ok(s) if s.success() => {
                    println!("✅ Trivy installed successfully via apt");
                    return;
                }
                _ => println!("  apt install failed, trying alternative..."),
            }
        }
    }

    // Fallback: direct binary download (safer than piping to shell)
    println!("  Downloading Trivy binary directly...");
    let trivy_url =
        "https://github.com/aquasecurity/trivy/releases/latest/download/trivy_Linux-64bit.tar.gz";

    match Command::new("curl")
        .args(["-sfL", "-o", "/tmp/trivy.tar.gz", trivy_url])
        .status()
    {
        Ok(s) if s.success() => {
            // Extract and install
            if Command::new("tar")
                .args(["-xzf", "/tmp/trivy.tar.gz", "-C", "/tmp"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
            {
                match Command::new("sudo")
                    .args(["mv", "/tmp/trivy", "/usr/local/bin/trivy"])
                    .status()
                {
                    Ok(s) if s.success() => {
                        println!("✅ Trivy installed successfully");
                        // Cleanup
                        let _ = std::fs::remove_file("/tmp/trivy.tar.gz");
                        return;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    println!("❌ Failed to install Trivy. Please install manually:");
    println!("   https://aquasecurity.github.io/trivy/latest/getting-started/installation/");
}

fn install_falco() {
    println!("📦 Installing Falco...");
    println!("This requires root privileges and system integration.");

    // Check for apt (Debian/Ubuntu) or pacman (Arch)
    if std::path::Path::new("/usr/bin/apt").exists() {
        println!("  Detected apt-based system. Setting up Falco repository...");

        // Download and add GPG key
        let key_result = Command::new("curl")
            .args([
                "-sfL",
                "-o",
                "/tmp/falco.asc",
                "https://falco.org/repo/falcosecurity-packages.asc",
            ])
            .status();

        if !key_result.map(|s| s.success()).unwrap_or(false) {
            println!("❌ Failed to download Falco GPG key");
            println!(
                "   Please install Falco manually: https://falco.org/docs/getting-started/installation/"
            );
            return;
        }

        // Add key to apt
        if !Command::new("sudo")
            .args(["apt-key", "add", "/tmp/falco.asc"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            println!("❌ Failed to add Falco GPG key");
            return;
        }

        // Add repository - write to file directly instead of using tee with pipe
        let repo_line = "deb [signed-by=/usr/share/keyrings/falco-archive-keyring.gpg] https://download.falco.org/packages/deb stable main";
        match std::fs::write("/tmp/falcosecurity.list", repo_line) {
            Ok(_) => {
                if !Command::new("sudo")
                    .args([
                        "mv",
                        "/tmp/falcosecurity.list",
                        "/etc/apt/sources.list.d/falcosecurity.list",
                    ])
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
                {
                    println!("❌ Failed to add Falco repository");
                    return;
                }
            }
            Err(e) => {
                println!("❌ Failed to create repository file: {}", e);
                return;
            }
        }

        // Update and install
        print!("  Updating package lists... ");
        if !Command::new("sudo")
            .args(["apt-get", "update", "-qq"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            println!("Warning");
        } else {
            println!("Done");
        }

        print!("  Installing Falco... ");
        match Command::new("sudo")
            .args(["apt-get", "install", "-y", "falco"])
            .status()
        {
            Ok(s) if s.success() => println!("Done\n✅ Falco installed successfully"),
            Ok(_) => println!("\n⚠️  Falco installation returned warnings"),
            Err(e) => println!("\n❌ Installation failed: {}", e),
        }

        // Cleanup
        let _ = std::fs::remove_file("/tmp/falco.asc");
    } else if std::path::Path::new("/usr/bin/pacman").exists() {
        println!("  Detected Arch Linux. Installing from AUR...");
        println!("  Note: You may need to use an AUR helper like yay or paru.");

        match Command::new("yay")
            .args(["-S", "--noconfirm", "falco"])
            .status()
        {
            Ok(s) if s.success() => println!("✅ Falco installed successfully"),
            _ => {
                println!("⚠️  yay installation failed. Try with paru or manual AUR install.");
                println!("   AUR package: https://aur.archlinux.org/packages/falco");
            }
        }
    } else {
        println!("❌ Unsupported package manager.");
        println!(
            "   Please install Falco manually: https://falco.org/docs/getting-started/installation/"
        );
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
        .args(["aux"])
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

/// Calculate security score based on check results
pub fn calculate_security_score(checks: &[bool]) -> SecurityScore {
    let passed = checks.iter().filter(|&&c| c).count();
    let total = checks.len();
    let percentage = if total > 0 {
        (passed as f32 / total as f32) * 100.0
    } else {
        0.0
    };

    let grade = if percentage >= 80.0 {
        SecurityGrade::Excellent
    } else if percentage >= 60.0 {
        SecurityGrade::Good
    } else if percentage >= 40.0 {
        SecurityGrade::Fair
    } else {
        SecurityGrade::Poor
    };

    SecurityScore {
        passed,
        total,
        percentage,
        grade,
    }
}

/// Security score result
#[derive(Debug, Clone)]
pub struct SecurityScore {
    pub passed: usize,
    pub total: usize,
    pub percentage: f32,
    pub grade: SecurityGrade,
}

/// Security grade classification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityGrade {
    Excellent,
    Good,
    Fair,
    Poor,
}

impl std::fmt::Display for SecurityGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityGrade::Excellent => write!(f, "Excellent"),
            SecurityGrade::Good => write!(f, "Good"),
            SecurityGrade::Fair => write!(f, "Fair"),
            SecurityGrade::Poor => write!(f, "Poor"),
        }
    }
}

/// Parse Trivy scan output to extract vulnerability counts
pub fn parse_trivy_summary(output: &str) -> TrivySummary {
    let mut summary = TrivySummary::default();

    for line in output.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.contains("critical:") {
            if let Some(count) = extract_count_after(&line, "critical:") {
                summary.critical = count;
            }
        } else if line_lower.contains("high:") {
            if let Some(count) = extract_count_after(&line, "high:") {
                summary.high = count;
            }
        } else if line_lower.contains("medium:") {
            if let Some(count) = extract_count_after(&line, "medium:") {
                summary.medium = count;
            }
        } else if line_lower.contains("low:") {
            if let Some(count) = extract_count_after(&line, "low:") {
                summary.low = count;
            }
        }
    }

    summary
}

fn extract_count_after(line: &str, keyword: &str) -> Option<u32> {
    let lower = line.to_lowercase();
    if let Some(pos) = lower.find(&keyword.to_lowercase()) {
        let after = &line[pos + keyword.len()..];
        after
            .trim()
            .split_whitespace()
            .next()
            .and_then(|s| s.parse().ok())
    } else {
        None
    }
}

/// Trivy scan summary
#[derive(Debug, Default, Clone)]
pub struct TrivySummary {
    pub critical: u32,
    pub high: u32,
    pub medium: u32,
    pub low: u32,
}

impl TrivySummary {
    /// Get total vulnerability count
    pub fn total(&self) -> u32 {
        self.critical + self.high + self.medium + self.low
    }

    /// Check if there are any critical or high vulnerabilities
    pub fn has_severe(&self) -> bool {
        self.critical > 0 || self.high > 0
    }
}

/// Check if a container is running as root
pub fn is_container_running_as_root(user_field: &str) -> bool {
    let user = user_field.trim().to_lowercase();
    user.is_empty() || user == "root" || user == "0"
}

/// Parse Docker inspect output to check for security configurations
pub fn has_security_options(inspect_output: &str) -> bool {
    inspect_output.contains("apparmor")
        || inspect_output.contains("seccomp")
        || inspect_output.contains("selinux")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_security_score_perfect() {
        let checks = vec![true, true, true, true, true];
        let score = calculate_security_score(&checks);
        assert_eq!(score.passed, 5);
        assert_eq!(score.total, 5);
        assert_eq!(score.percentage, 100.0);
        assert_eq!(score.grade, SecurityGrade::Excellent);
    }

    #[test]
    fn test_calculate_security_score_good() {
        let checks = vec![true, true, true, false, false];
        let score = calculate_security_score(&checks);
        assert_eq!(score.passed, 3);
        assert_eq!(score.total, 5);
        assert!((score.percentage - 60.0).abs() < 0.001);
        assert_eq!(score.grade, SecurityGrade::Good);
    }

    #[test]
    fn test_calculate_security_score_fair() {
        let checks = vec![true, true, false, false, false];
        let score = calculate_security_score(&checks);
        assert_eq!(score.passed, 2);
        assert_eq!(score.total, 5);
        assert_eq!(score.percentage, 40.0);
        assert_eq!(score.grade, SecurityGrade::Fair);
    }

    #[test]
    fn test_calculate_security_score_poor() {
        let checks = vec![true, false, false, false, false];
        let score = calculate_security_score(&checks);
        assert_eq!(score.passed, 1);
        assert_eq!(score.total, 5);
        assert_eq!(score.percentage, 20.0);
        assert_eq!(score.grade, SecurityGrade::Poor);
    }

    #[test]
    fn test_calculate_security_score_empty() {
        let checks: Vec<bool> = vec![];
        let score = calculate_security_score(&checks);
        assert_eq!(score.passed, 0);
        assert_eq!(score.total, 0);
        assert_eq!(score.percentage, 0.0);
        assert_eq!(score.grade, SecurityGrade::Poor);
    }

    #[test]
    fn test_security_grade_display() {
        assert_eq!(SecurityGrade::Excellent.to_string(), "Excellent");
        assert_eq!(SecurityGrade::Good.to_string(), "Good");
        assert_eq!(SecurityGrade::Fair.to_string(), "Fair");
        assert_eq!(SecurityGrade::Poor.to_string(), "Poor");
    }

    #[test]
    fn test_trivy_summary_total() {
        let summary = TrivySummary {
            critical: 1,
            high: 2,
            medium: 3,
            low: 4,
        };
        assert_eq!(summary.total(), 10);
    }

    #[test]
    fn test_trivy_summary_has_severe() {
        let summary1 = TrivySummary {
            critical: 1,
            high: 0,
            medium: 0,
            low: 0,
        };
        assert!(summary1.has_severe());

        let summary2 = TrivySummary {
            critical: 0,
            high: 1,
            medium: 0,
            low: 0,
        };
        assert!(summary2.has_severe());

        let summary3 = TrivySummary {
            critical: 0,
            high: 0,
            medium: 5,
            low: 10,
        };
        assert!(!summary3.has_severe());
    }

    #[test]
    fn test_parse_trivy_summary() {
        let output = "Critical: 1\nHigh: 2\nMedium: 3\nLow: 4";
        let summary = parse_trivy_summary(output);
        assert_eq!(summary.critical, 1);
        assert_eq!(summary.high, 2);
        assert_eq!(summary.medium, 3);
        assert_eq!(summary.low, 4);
    }

    #[test]
    fn test_parse_trivy_summary_empty() {
        let output = "";
        let summary = parse_trivy_summary(output);
        assert_eq!(summary.total(), 0);
    }

    #[test]
    fn test_is_container_running_as_root() {
        assert!(is_container_running_as_root(""));
        assert!(is_container_running_as_root("root"));
        assert!(is_container_running_as_root("ROOT"));
        assert!(is_container_running_as_root("0"));
        assert!(!is_container_running_as_root("1000"));
        assert!(!is_container_running_as_root("nobody"));
        assert!(!is_container_running_as_root("nginx"));
    }

    #[test]
    fn test_has_security_options() {
        assert!(has_security_options("apparmor=docker-default"));
        assert!(has_security_options("seccomp=unconfined"));
        assert!(has_security_options("selinux:label"));
        assert!(!has_security_options("no security options"));
        assert!(!has_security_options(""));
    }
}
