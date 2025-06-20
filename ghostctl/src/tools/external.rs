use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn external_tools_menu() {
    println!("🛠️  External Tools & Utilities");
    println!("==============================");

    let options = [
        "🔐 acme.sh (SSL Certificate Manager)",
        "📦 Additional Package Managers",
        "🔧 System Utilities",
        "🌐 Network Tools",
        "📊 Monitoring Tools",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("External Tools")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => acme_sh_management(),
        1 => additional_package_managers(),
        2 => system_utilities(),
        3 => network_tools(),
        4 => monitoring_tools(),
        _ => return,
    }
}

pub fn acme_sh_management() {
    println!("🔐 acme.sh SSL Certificate Manager");
    println!("==================================");

    let options = [
        "📦 Install acme.sh",
        "🔄 Update acme.sh",
        "📋 Check Status",
        "🆕 Issue Certificate",
        "🔄 Renew Certificates",
        "📂 List Certificates",
        "⚙️  Configure DNS API",
        "🗑️  Uninstall acme.sh",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("acme.sh Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_acme_sh(),
        1 => update_acme_sh(),
        2 => check_acme_status(),
        3 => issue_certificate(),
        4 => renew_certificates(),
        5 => list_certificates(),
        6 => configure_dns_api(),
        7 => uninstall_acme_sh(),
        _ => return,
    }
}

pub fn install_acme_sh() {
    println!("📦 Installing acme.sh");
    println!("=====================");

    if is_acme_sh_installed() {
        println!("✅ acme.sh is already installed");
        check_acme_status();
        return;
    }

    let install_methods = [
        "🌐 Official curl installer (Recommended)",
        "📥 wget installer",
        "📦 Manual installation",
    ];

    let method = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Installation method")
        .items(&install_methods)
        .default(0)
        .interact()
        .unwrap();

    let confirm = Confirm::new()
        .with_prompt("Install acme.sh SSL certificate manager?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        match method {
            0 => install_acme_curl(),
            1 => install_acme_wget(),
            2 => install_acme_manual(),
            _ => return,
        }
    }
}

fn install_acme_curl() {
    println!("📥 Installing acme.sh with curl...");

    let status = Command::new("bash")
        .arg("-c")
        .arg("curl https://get.acme.sh | sh")
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ acme.sh installed successfully");
            post_install_setup();
        }
        _ => {
            println!("❌ Failed to install acme.sh");
            println!("💡 Try manual installation or check your internet connection");
        }
    }
}

fn install_acme_wget() {
    println!("📥 Installing acme.sh with wget...");

    let status = Command::new("bash")
        .arg("-c")
        .arg("wget -O - https://get.acme.sh | sh")
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ acme.sh installed successfully");
            post_install_setup();
        }
        _ => {
            println!("❌ Failed to install acme.sh");
            println!("💡 Try curl method or manual installation");
        }
    }
}

fn install_acme_manual() {
    println!("📦 Manual acme.sh Installation");
    println!("==============================");

    println!("💡 Manual installation steps:");
    println!("1. git clone https://github.com/acmesh-official/acme.sh.git");
    println!("2. cd acme.sh");
    println!("3. ./acme.sh --install");

    let proceed = Confirm::new()
        .with_prompt("Run manual installation commands?")
        .default(true)
        .interact()
        .unwrap();

    if proceed {
        println!("📥 Cloning acme.sh repository...");
        let clone_status = Command::new("git")
            .args(&[
                "clone",
                "https://github.com/acmesh-official/acme.sh.git",
                "/tmp/acme.sh",
            ])
            .status();

        if clone_status.is_ok() && clone_status.unwrap().success() {
            println!("🔧 Installing acme.sh...");
            let install_status = Command::new("./acme.sh")
                .args(&["--install"])
                .current_dir("/tmp/acme.sh")
                .status();

            if install_status.is_ok() && install_status.unwrap().success() {
                println!("✅ acme.sh installed successfully");
                post_install_setup();
            }

            // Cleanup
            let _ = std::fs::remove_dir_all("/tmp/acme.sh");
        }
    }
}

fn post_install_setup() {
    println!("⚙️  Setting up acme.sh environment...");

    // Reload shell configuration
    let _shell_files = [
        format!("{}/.bashrc", dirs::home_dir().unwrap().display()),
        format!("{}/.zshrc", dirs::home_dir().unwrap().display()),
    ];

    println!("💡 To use acme.sh immediately, run:");
    println!("source ~/.bashrc  # or source ~/.zshrc");

    // Check if acme.sh directory exists
    let acme_dir = format!("{}/.acme.sh", dirs::home_dir().unwrap().display());
    if std::path::Path::new(&acme_dir).exists() {
        println!("📁 acme.sh installed in: {}", acme_dir);

        // Show quick start info
        println!("\n🚀 Quick Start:");
        println!("  📋 Check status: ~/.acme.sh/acme.sh --version");
        println!(
            "  🆕 Issue cert: ~/.acme.sh/acme.sh --issue -d example.com --webroot /var/www/html"
        );
        println!("  📂 Install cert: ~/.acme.sh/acme.sh --install-cert -d example.com");
    }
}

fn update_acme_sh() {
    println!("🔄 Updating acme.sh");
    println!("===================");

    if !is_acme_sh_installed() {
        println!("❌ acme.sh is not installed");
        return;
    }

    let acme_path = get_acme_sh_path();
    let status = Command::new(&acme_path).args(&["--upgrade"]).status();

    match status {
        Ok(s) if s.success() => println!("✅ acme.sh updated successfully"),
        _ => println!("❌ Failed to update acme.sh"),
    }
}

fn check_acme_status() {
    println!("📋 acme.sh Status Check");
    println!("=======================");

    if !is_acme_sh_installed() {
        println!("❌ acme.sh is not installed");
        return;
    }

    let acme_path = get_acme_sh_path();

    // Check version
    println!("📦 Version:");
    let _ = Command::new(&acme_path).args(&["--version"]).status();

    // Check account info
    println!("\n👤 Account Info:");
    let _ = Command::new(&acme_path).args(&["--info"]).status();

    // Check cron job
    println!("\n⏰ Cron Job Status:");
    let _ = Command::new(&acme_path).args(&["--info"]).status();

    // List current certificates
    println!("\n📂 Current Certificates:");
    let _ = Command::new(&acme_path).args(&["--list"]).status();
}

pub fn issue_certificate() {
    println!("🆕 Issue SSL Certificate");
    println!("========================");

    if !is_acme_sh_installed() {
        println!("❌ acme.sh is not installed");
        return;
    }

    let domain: String = Input::new()
        .with_prompt("Domain name")
        .interact_text()
        .unwrap();

    let validation_methods = [
        "🌐 Webroot (HTTP-01)",
        "🔌 Standalone (HTTP-01)",
        "📡 DNS API (DNS-01)",
        "🏷️  DNS Manual (DNS-01)",
    ];

    let method = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Validation method")
        .items(&validation_methods)
        .default(0)
        .interact()
        .unwrap();

    let acme_path = get_acme_sh_path();

    match method {
        0 => issue_webroot(&acme_path, &domain),
        1 => issue_standalone(&acme_path, &domain),
        2 => issue_dns_api(&acme_path, &domain),
        3 => issue_dns_manual(&acme_path, &domain),
        _ => return,
    }
}

fn issue_webroot(acme_path: &str, domain: &str) {
    let webroot: String = Input::new()
        .with_prompt("Webroot path (e.g., /var/www/html)")
        .interact_text()
        .unwrap();

    println!("🌐 Issuing certificate using webroot method...");

    let status = Command::new(acme_path)
        .args(&["--issue", "-d", domain, "--webroot", &webroot])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Certificate issued successfully");
            install_certificate(acme_path, domain);
        }
        _ => println!("❌ Failed to issue certificate"),
    }
}

fn issue_standalone(acme_path: &str, domain: &str) {
    println!("🔌 Issuing certificate using standalone method...");
    println!("⚠️  Make sure port 80 is available");

    let status = Command::new(acme_path)
        .args(&["--issue", "-d", domain, "--standalone"])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Certificate issued successfully");
            install_certificate(acme_path, domain);
        }
        _ => println!("❌ Failed to issue certificate"),
    }
}

fn issue_dns_api(acme_path: &str, domain: &str) {
    println!("📡 Available DNS APIs:");
    println!("  • Cloudflare: --dns dns_cf");
    println!("  • DigitalOcean: --dns dns_dgon");
    println!("  • AWS Route53: --dns dns_aws");
    println!("  • And many more...");

    let dns_provider: String = Input::new()
        .with_prompt("DNS provider (e.g., dns_cf)")
        .interact_text()
        .unwrap();

    println!("💡 Configure DNS API credentials first:");
    println!("  For Cloudflare: export CF_Token=\"your-token\"");

    let status = Command::new(acme_path)
        .args(&["--issue", "-d", domain, "--dns", &dns_provider])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Certificate issued successfully");
            install_certificate(acme_path, domain);
        }
        _ => println!("❌ Failed to issue certificate"),
    }
}

fn issue_dns_manual(acme_path: &str, domain: &str) {
    println!("🏷️  Manual DNS validation");
    println!("========================");

    let status = Command::new(acme_path)
        .args(&[
            "--issue",
            "-d",
            domain,
            "--dns",
            "--yes-I-know-dns-manual-mode-enough-go-ahead-please",
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Certificate issued successfully");
            install_certificate(acme_path, domain);
        }
        _ => println!("❌ Failed to issue certificate"),
    }
}

fn install_certificate(acme_path: &str, domain: &str) {
    println!("📂 Installing Certificate");
    println!("=========================");

    let install_cert = Confirm::new()
        .with_prompt("Install certificate to custom location?")
        .default(true)
        .interact()
        .unwrap();

    if install_cert {
        // Use your custom certificate structure
        let cert_dir = format!("/etc/nginx/certs/{}", domain);

        // Create directory
        let _ = Command::new("sudo")
            .args(&["mkdir", "-p", &cert_dir])
            .status();

        // Install certificate
        let status = Command::new("sudo")
            .arg(acme_path)
            .args(&[
                "--install-cert",
                "-d",
                domain,
                "--cert-file",
                &format!("{}/cert.pem", cert_dir),
                "--key-file",
                &format!("{}/private.key", cert_dir),
                "--fullchain-file",
                &format!("{}/fullchain.pem", cert_dir),
                "--reloadcmd",
                "systemctl reload nginx",
            ])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Certificate installed to: {}", cert_dir);
                println!("🔄 Nginx reloaded");
            }
            _ => println!("❌ Failed to install certificate"),
        }
    }
}

pub fn renew_certificates() {
    println!("🔄 Renewing Certificates");
    println!("========================");

    if !is_acme_sh_installed() {
        println!("❌ acme.sh is not installed");
        return;
    }

    let options = [
        "🔄 Renew all certificates",
        "🎯 Renew specific domain",
        "🧪 Test renewal (dry-run)",
        "⏰ Check cron job",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Renewal options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    let acme_path = get_acme_sh_path();

    match choice {
        0 => {
            println!("🔄 Renewing all certificates...");
            let _ = Command::new(&acme_path).args(&["--renew-all"]).status();
        }
        1 => {
            let domain: String = Input::new()
                .with_prompt("Domain to renew")
                .interact_text()
                .unwrap();

            println!("🔄 Renewing certificate for: {}", domain);
            let _ = Command::new(&acme_path)
                .args(&["--renew", "-d", &domain])
                .status();
        }
        2 => {
            println!("🧪 Testing renewal process...");
            let _ = Command::new(&acme_path)
                .args(&["--renew-all", "--dry-run"])
                .status();
        }
        3 => {
            println!("⏰ Checking cron job...");
            let _ = Command::new(&acme_path).args(&["--info"]).status();
        }
        _ => return,
    }
}

pub fn list_certificates() {
    println!("📂 Certificate List");
    println!("===================");

    if !is_acme_sh_installed() {
        println!("❌ acme.sh is not installed");
        return;
    }

    let acme_path = get_acme_sh_path();
    let _ = Command::new(&acme_path).args(&["--list"]).status();
}

fn configure_dns_api() {
    println!("⚙️  Configure DNS API");
    println!("=====================");

    println!("🌐 Popular DNS providers:");
    println!("  • Cloudflare: CF_Token or CF_Key + CF_Email");
    println!("  • DigitalOcean: DO_API_KEY");
    println!("  • AWS Route53: AWS_ACCESS_KEY_ID + AWS_SECRET_ACCESS_KEY");
    println!("  • Namecheap: NAMECHEAP_USERNAME + NAMECHEAP_API_KEY");

    println!("\n💡 Set environment variables in your shell config:");
    println!("  echo 'export CF_Token=\"your-token\"' >> ~/.bashrc");

    println!("\n📚 Full list: https://github.com/acmesh-official/acme.sh/wiki/dnsapi");
}

fn uninstall_acme_sh() {
    println!("🗑️  Uninstall acme.sh");
    println!("=====================");

    if !is_acme_sh_installed() {
        println!("❌ acme.sh is not installed");
        return;
    }

    let warning = "⚠️  This will remove acme.sh and disable automatic certificate renewal";
    println!("{}", warning);

    let confirm = Confirm::new()
        .with_prompt("Are you sure you want to uninstall acme.sh?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let acme_path = get_acme_sh_path();

        // Run uninstall
        let status = Command::new(&acme_path).args(&["--uninstall"]).status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ acme.sh uninstalled");

                // Remove directory
                let acme_dir = format!("{}/.acme.sh", dirs::home_dir().unwrap().display());
                let _ = std::fs::remove_dir_all(&acme_dir);

                println!("📁 Removed directory: {}", acme_dir);
            }
            _ => println!("❌ Failed to uninstall acme.sh"),
        }
    }
}

fn is_acme_sh_installed() -> bool {
    let acme_dir = format!("{}/.acme.sh", dirs::home_dir().unwrap().display());
    let acme_script = format!("{}/acme.sh", acme_dir);

    std::path::Path::new(&acme_script).exists()
        || Command::new("which").arg("acme.sh").status().is_ok()
}

fn get_acme_sh_path() -> String {
    // Try common locations
    let locations = [
        format!("{}/.acme.sh/acme.sh", dirs::home_dir().unwrap().display()),
        "/usr/local/bin/acme.sh".to_string(),
        "acme.sh".to_string(), // If in PATH
    ];

    for location in &locations {
        if std::path::Path::new(location).exists() {
            return location.clone();
        }
    }

    // Default to home directory installation
    format!("{}/.acme.sh/acme.sh", dirs::home_dir().unwrap().display())
}

// Placeholder functions for other external tools
fn additional_package_managers() {
    println!("📦 Additional Package Managers");
    println!("==============================");

    println!("💡 Consider these package managers:");
    println!("  • Snap: Universal Linux packages");
    println!("  • Flatpak: Sandboxed applications");
    println!("  • AppImage: Portable applications");
    println!("  • Nix: Functional package manager");
}

fn system_utilities() {
    println!("🔧 System Utilities");
    println!("===================");

    println!("💡 Useful system utilities:");
    println!("  • htop: Interactive process viewer");
    println!("  • tmux: Terminal multiplexer");
    println!("  • fd: Fast find alternative");
    println!("  • ripgrep: Fast grep alternative");
    println!("  • bat: Cat with syntax highlighting");
}

fn network_tools() {
    println!("🌐 Network Tools");
    println!("================");

    println!("💡 Network diagnostic tools:");
    println!("  • nmap: Network scanner");
    println!("  • curl: HTTP client");
    println!("  • dig: DNS lookup");
    println!("  • netstat: Network connections");
    println!("  • iperf3: Network bandwidth testing");
}

fn monitoring_tools() {
    println!("📊 Monitoring Tools");
    println!("===================");

    println!("💡 System monitoring tools:");
    println!("  • Prometheus: Metrics collection");
    println!("  • Grafana: Data visualization");
    println!("  • Netdata: Real-time monitoring");
    println!("  • Glances: System monitoring");
}
