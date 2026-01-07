use std::process::Command;

/// System diagnostics and issue detection
#[derive(Debug, Clone)]
pub struct SystemDiagnostics {
    pub has_network_issues: bool,
    pub has_mirror_issues: bool,
    pub has_pacman_lock: bool,
    pub has_keyring_issues: bool,
    pub has_getcwd_error: bool,
    pub has_permission_errors: bool,
    pub has_orphaned_packages: bool,
    pub has_database_issues: bool,
    pub network_error_details: Option<String>,
}

impl SystemDiagnostics {
    pub fn new() -> Self {
        Self {
            has_network_issues: false,
            has_mirror_issues: false,
            has_pacman_lock: false,
            has_keyring_issues: false,
            has_getcwd_error: false,
            has_permission_errors: false,
            has_orphaned_packages: false,
            has_database_issues: false,
            network_error_details: None,
        }
    }

    /// Run comprehensive system diagnostics
    pub fn scan() -> Self {
        let mut diag = Self::new();

        println!("🔍 Running system diagnostics...");

        // Check for getcwd errors (current directory issues)
        diag.has_getcwd_error = Self::check_getcwd_error();
        if diag.has_getcwd_error {
            println!("  ⚠️  getcwd error detected");
        }

        // Check for pacman lock
        diag.has_pacman_lock = Self::check_pacman_lock();
        if diag.has_pacman_lock {
            println!("  🔒 Pacman lock file found");
        }

        // Check network connectivity
        let (network_issue, details) = Self::check_network();
        diag.has_network_issues = network_issue;
        diag.network_error_details = details.clone();
        if network_issue {
            if let Some(detail) = &details {
                println!("  📡 Network issue: {}", detail);
            } else {
                println!("  📡 Network connectivity issues detected");
            }
        }

        // Check mirrors
        diag.has_mirror_issues = Self::check_mirrors();
        if diag.has_mirror_issues {
            println!("  🌐 Mirror issues detected");
        }

        // Check keyring
        diag.has_keyring_issues = Self::check_keyring();
        if diag.has_keyring_issues {
            println!("  🔑 Keyring issues detected");
        }

        // Check for orphaned packages
        diag.has_orphaned_packages = Self::check_orphans();
        if diag.has_orphaned_packages {
            println!("  📦 Orphaned packages found");
        }

        // Check database integrity
        diag.has_database_issues = Self::check_database();
        if diag.has_database_issues {
            println!("  💾 Database issues detected");
        }

        println!("✅ Diagnostics complete\n");

        diag
    }

    fn check_getcwd_error() -> bool {
        // Try to get current directory
        std::env::current_dir().is_err()
    }

    fn check_pacman_lock() -> bool {
        std::path::Path::new("/var/lib/pacman/db.lck").exists()
    }

    fn check_network() -> (bool, Option<String>) {
        // Try to ping common DNS servers
        let ping_result = Command::new("ping")
            .args(&["-c", "1", "-W", "2", "1.1.1.1"])
            .output();

        match ping_result {
            Ok(output) => {
                if !output.status.success() {
                    return (true, Some("Network unreachable".to_string()));
                }
            }
            Err(e) => {
                return (true, Some(format!("Network check failed: {}", e)));
            }
        }

        // Check if we can reach archlinux.org
        let curl_result = Command::new("curl")
            .args(&["-s", "-I", "-m", "5", "https://archlinux.org"])
            .output();

        match curl_result {
            Ok(output) => {
                if !output.status.success() {
                    return (true, Some("Cannot reach archlinux.org".to_string()));
                }
            }
            Err(_) => {
                // curl might not be installed, that's ok
            }
        }

        (false, None)
    }

    /// Test if basic network connectivity is working
    fn test_connectivity() -> bool {
        // Quick ping test to 1.1.1.1 (Cloudflare DNS)
        Command::new("ping")
            .args(&["-c", "1", "-W", "2", "1.1.1.1"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn check_mirrors() -> bool {
        // Check if mirrorlist exists and is not empty
        if let Ok(content) = std::fs::read_to_string("/etc/pacman.d/mirrorlist") {
            // Check if there are any uncommented Server lines
            let has_active_mirrors = content
                .lines()
                .any(|line| line.trim_start().starts_with("Server = "));
            return !has_active_mirrors;
        }
        true
    }

    fn check_keyring() -> bool {
        // Check if keyring directory exists and has keys
        let keyring_path = std::path::Path::new("/etc/pacman.d/gnupg");
        if !keyring_path.exists() {
            return true;
        }

        // Check if pubring.gpg exists and is not empty
        let pubring = keyring_path.join("pubring.gpg");
        if !pubring.exists() {
            return true;
        }

        // Check file size
        if let Ok(metadata) = std::fs::metadata(&pubring)
            && metadata.len() < 100 {
                return true;
            }

        false
    }

    fn check_orphans() -> bool {
        // Run pacman -Qtdq to check for orphans
        let output = Command::new("pacman").args(&["-Qtdq"]).output();

        match output {
            Ok(result) => !result.stdout.is_empty(),
            Err(_) => false,
        }
    }

    fn check_database() -> bool {
        // Try to query the database
        let output = Command::new("pacman").args(&["-Q"]).output();

        match output {
            Ok(result) => !result.status.success(),
            Err(_) => true,
        }
    }

    /// Get a prioritized list of issues to fix
    pub fn get_fix_sequence(&self) -> Vec<FixAction> {
        let mut actions = Vec::new();

        // Fix in priority order
        if self.has_getcwd_error {
            actions.push(FixAction::FixGetcwd);
        }

        if self.has_pacman_lock {
            actions.push(FixAction::RemovePacmanLock);
        }

        if self.has_network_issues {
            actions.push(FixAction::FixNetwork);
        }

        if self.has_mirror_issues {
            actions.push(FixAction::UpdateMirrors);
        }

        if self.has_keyring_issues {
            actions.push(FixAction::RefreshKeyring);
        }

        if self.has_database_issues {
            actions.push(FixAction::SyncDatabase);
        }

        if self.has_orphaned_packages {
            actions.push(FixAction::RemoveOrphans);
        }

        actions
    }

    /// Check if any issues were detected
    pub fn has_issues(&self) -> bool {
        self.has_network_issues
            || self.has_mirror_issues
            || self.has_pacman_lock
            || self.has_keyring_issues
            || self.has_getcwd_error
            || self.has_orphaned_packages
            || self.has_database_issues
    }

    /// Print a summary of detected issues
    pub fn print_summary(&self) {
        if !self.has_issues() {
            println!("✅ No issues detected!");
            return;
        }

        println!("⚠️  Issues detected:");
        if self.has_getcwd_error {
            println!("  • Current directory error (getcwd)");
        }
        if self.has_pacman_lock {
            println!("  • Pacman database locked");
        }
        if self.has_network_issues {
            if let Some(details) = &self.network_error_details {
                println!("  • Network: {}", details);
            } else {
                println!("  • Network connectivity issues");
            }
        }
        if self.has_mirror_issues {
            println!("  • Mirror configuration problems");
        }
        if self.has_keyring_issues {
            println!("  • Keyring needs refresh");
        }
        if self.has_database_issues {
            println!("  • Database synchronization needed");
        }
        if self.has_orphaned_packages {
            println!("  • Orphaned packages present");
        }
        println!();
    }
}

impl Default for SystemDiagnostics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FixAction {
    FixGetcwd,
    RemovePacmanLock,
    FixNetwork,
    UpdateMirrors,
    RefreshKeyring,
    SyncDatabase,
    RemoveOrphans,
}

impl FixAction {
    pub fn description(&self) -> &str {
        match self {
            FixAction::FixGetcwd => "Fix working directory",
            FixAction::RemovePacmanLock => "Remove pacman lock",
            FixAction::FixNetwork => "Fix network connectivity",
            FixAction::UpdateMirrors => "Update mirror list",
            FixAction::RefreshKeyring => "Refresh keyring",
            FixAction::SyncDatabase => "Sync package database",
            FixAction::RemoveOrphans => "Remove orphaned packages",
        }
    }

    pub fn execute(&self) -> bool {
        match self {
            FixAction::FixGetcwd => {
                println!("🔧 Fixing working directory...");
                // Try to change to home directory
                if let Ok(home) = std::env::var("HOME")
                    && std::env::set_current_dir(&home).is_ok() {
                        println!("  ✅ Changed to home directory");
                        return true;
                    }
                // Fallback to /tmp
                if std::env::set_current_dir("/tmp").is_ok() {
                    println!("  ✅ Changed to /tmp");
                    return true;
                }
                println!("  ❌ Failed to fix working directory");
                false
            }
            FixAction::RemovePacmanLock => {
                println!("🔧 Removing pacman lock...");
                let result = Command::new("sudo")
                    .args(&["rm", "-f", "/var/lib/pacman/db.lck"])
                    .status();
                match result {
                    Ok(status) if status.success() => {
                        println!("  ✅ Pacman lock removed");
                        true
                    }
                    _ => {
                        println!("  ❌ Failed to remove pacman lock");
                        false
                    }
                }
            }
            FixAction::FixNetwork => {
                println!("🔧 Attempting to fix network connectivity...");

                // Detect network manager in use
                let has_networkmanager = Command::new("systemctl")
                    .args(&["is-active", "NetworkManager"])
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "active")
                    .unwrap_or(false);

                let has_systemd_networkd = Command::new("systemctl")
                    .args(&["is-active", "systemd-networkd"])
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "active")
                    .unwrap_or(false);

                let has_dhcpcd = Command::new("systemctl")
                    .args(&["is-active", "dhcpcd"])
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "active")
                    .unwrap_or(false);

                let mut fixed = false;

                // Try NetworkManager first
                if has_networkmanager {
                    println!("  📡 Detected NetworkManager, attempting restart...");
                    let result = Command::new("sudo")
                        .args(&["systemctl", "restart", "NetworkManager"])
                        .status();

                    if let Ok(status) = result
                        && status.success() {
                            println!("  ✅ NetworkManager restarted");
                            // Wait for connection
                            std::thread::sleep(std::time::Duration::from_secs(3));

                            // Check if connection is now working
                            if SystemDiagnostics::test_connectivity() {
                                println!("  ✅ Network connectivity restored");
                                fixed = true;
                            }
                        }
                }

                // Try systemd-networkd
                if !fixed && has_systemd_networkd {
                    println!("  📡 Detected systemd-networkd, attempting restart...");
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "restart", "systemd-networkd"])
                        .status();
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "restart", "systemd-resolved"])
                        .status();

                    std::thread::sleep(std::time::Duration::from_secs(3));

                    if SystemDiagnostics::test_connectivity() {
                        println!("  ✅ Network connectivity restored");
                        fixed = true;
                    }
                }

                // Try dhcpcd
                if !fixed && has_dhcpcd {
                    println!("  📡 Detected dhcpcd, attempting restart...");
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "restart", "dhcpcd"])
                        .status();

                    std::thread::sleep(std::time::Duration::from_secs(3));

                    if SystemDiagnostics::test_connectivity() {
                        println!("  ✅ Network connectivity restored");
                        fixed = true;
                    }
                }

                // Try flushing DNS cache
                if !fixed {
                    println!("  🔄 Flushing DNS cache...");
                    let _ = Command::new("sudo")
                        .args(&["systemd-resolve", "--flush-caches"])
                        .status();

                    // Try resolvectl as fallback
                    let _ = Command::new("sudo")
                        .args(&["resolvectl", "flush-caches"])
                        .status();

                    if SystemDiagnostics::test_connectivity() {
                        println!("  ✅ Network connectivity restored after DNS flush");
                        fixed = true;
                    }
                }

                // If nothing worked, try bringing interface up
                if !fixed {
                    println!("  🔌 Attempting to bring network interface up...");
                    // Get first ethernet/wireless interface
                    if let Ok(output) = Command::new("ip").args(&["link", "show"]).output() {
                        let output_str = String::from_utf8_lossy(&output.stdout);
                        for line in output_str.lines() {
                            if line.contains("state DOWN")
                                && let Some(iface) = line.split(':').nth(1) {
                                    let iface = iface.trim();
                                    if iface.starts_with("en")
                                        || iface.starts_with("eth")
                                        || iface.starts_with("wl")
                                    {
                                        println!("  📡 Bringing up interface: {}", iface);
                                        let _ = Command::new("sudo")
                                            .args(&["ip", "link", "set", iface, "up"])
                                            .status();
                                    }
                                }
                        }
                    }

                    std::thread::sleep(std::time::Duration::from_secs(2));

                    if SystemDiagnostics::test_connectivity() {
                        println!("  ✅ Network connectivity restored");
                        fixed = true;
                    }
                }

                if !fixed {
                    println!("  ❌ Automatic network fix failed");
                    println!();
                    println!("  💡 Manual troubleshooting steps:");
                    println!("     1. Check physical connection (cable/wifi)");
                    println!("     2. Verify interface is up: ip link show");
                    println!("     3. Check IP address: ip addr show");
                    println!("     4. Test gateway: ping $(ip route | grep default | awk '{{print $3}}')");
                    println!("     5. Test DNS: ping 1.1.1.1 && ping archlinux.org");
                    println!();
                    println!("  🔧 Common fixes:");
                    println!("     - sudo systemctl restart NetworkManager");
                    println!("     - sudo dhcpcd <interface>");
                    println!("     - nmcli device wifi connect <SSID> --ask");
                }

                fixed
            }
            FixAction::UpdateMirrors => {
                println!("🔧 Updating mirror list...");

                // Check if reflector is installed
                let has_reflector = Command::new("command")
                    .args(&["-v", "reflector"])
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false);

                if has_reflector {
                    let result = Command::new("sudo")
                        .args(&[
                            "reflector",
                            "--country",
                            "US,Canada",
                            "--latest",
                            "20",
                            "--protocol",
                            "https",
                            "--sort",
                            "rate",
                            "--save",
                            "/etc/pacman.d/mirrorlist",
                        ])
                        .status();
                    match result {
                        Ok(status) if status.success() => {
                            println!("  ✅ Mirrors updated");
                            return true;
                        }
                        _ => {
                            println!("  ⚠️  reflector failed, trying alternative...");
                        }
                    }
                } else {
                    println!("  ⚠️  reflector not installed, installing...");
                    let _ = Command::new("sudo")
                        .args(&["pacman", "-S", "--noconfirm", "reflector"])
                        .status();
                }

                println!("  ❌ Failed to update mirrors");
                false
            }
            FixAction::RefreshKeyring => {
                println!("🔧 Refreshing keyring...");

                // Step 1: Try refreshing keys from keyserver first (least destructive)
                println!("  📡 Attempting key refresh from keyserver...");
                let refresh = Command::new("sudo")
                    .args(&["pacman-key", "--refresh-keys"])
                    .status();

                if let Ok(status) = refresh
                    && status.success() {
                        println!("  ✅ Keys refreshed from keyserver");
                        return true;
                    }

                // Step 2: Try reinstalling archlinux-keyring
                println!("  📦 Trying keyring package reinstall...");
                let keyring_install = Command::new("sudo")
                    .args(&["pacman", "-S", "--noconfirm", "archlinux-keyring"])
                    .status();

                if let Ok(status) = keyring_install
                    && status.success() {
                        let _ = Command::new("sudo")
                            .args(&["pacman-key", "--populate", "archlinux"])
                            .status();
                        println!("  ✅ Keyring reinstalled and populated");
                        return true;
                    }

                // Step 3: Full reset (backup first)
                println!("  ⚠️  Full keyring reset required...");
                println!("  💾 Creating backup...");
                let _ = Command::new("sudo")
                    .args(&[
                        "cp",
                        "-a",
                        "/etc/pacman.d/gnupg",
                        "/etc/pacman.d/gnupg.backup",
                    ])
                    .status();

                // Remove old keyring
                let _ = Command::new("sudo")
                    .args(&["rm", "-rf", "/etc/pacman.d/gnupg"])
                    .status();

                // Initialize new keyring
                let init = Command::new("sudo")
                    .args(&["pacman-key", "--init"])
                    .status();

                if init.is_err() || !init.unwrap().success() {
                    println!("  ❌ Failed to initialize keyring");
                    println!("  💡 Restore backup: sudo cp -a /etc/pacman.d/gnupg.backup /etc/pacman.d/gnupg");
                    return false;
                }

                // Populate keyring
                let populate = Command::new("sudo")
                    .args(&["pacman-key", "--populate", "archlinux"])
                    .status();

                match populate {
                    Ok(status) if status.success() => {
                        println!("  ✅ Keyring refreshed (backup at /etc/pacman.d/gnupg.backup)");
                        true
                    }
                    _ => {
                        println!("  ❌ Failed to populate keyring");
                        println!("  💡 Restore backup: sudo cp -a /etc/pacman.d/gnupg.backup /etc/pacman.d/gnupg");
                        false
                    }
                }
            }
            FixAction::SyncDatabase => {
                println!("🔧 Syncing package database...");
                let result = Command::new("sudo").args(&["pacman", "-Sy"]).status();
                match result {
                    Ok(status) if status.success() => {
                        println!("  ✅ Database synced");
                        true
                    }
                    _ => {
                        println!("  ❌ Failed to sync database");
                        false
                    }
                }
            }
            FixAction::RemoveOrphans => {
                println!("🔧 Removing orphaned packages...");

                // Get list of orphans
                let orphans = Command::new("pacman").args(&["-Qtdq"]).output();

                match orphans {
                    Ok(output) if !output.stdout.is_empty() => {
                        let result = Command::new("sudo")
                            .args(&["pacman", "-Rns", "--noconfirm"])
                            .stdin(std::process::Stdio::piped())
                            .spawn()
                            .and_then(|mut child| {
                                use std::io::Write;
                                if let Some(stdin) = child.stdin.as_mut() {
                                    stdin.write_all(&output.stdout)?;
                                }
                                child.wait()
                            });

                        match result {
                            Ok(status) if status.success() => {
                                println!("  ✅ Orphaned packages removed");
                                return true;
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        println!("  ℹ️  No orphaned packages found");
                        return true;
                    }
                }

                println!("  ❌ Failed to remove orphans");
                false
            }
        }
    }
}
