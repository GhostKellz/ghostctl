use dialoguer::{Select, theme::ColorfulTheme};

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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("External Tools")
        .items(&options)
        .default(0)
        .interact()
    {
        Ok(choice) => choice,
        Err(e) => {
            eprintln!("Failed to display menu: {}", e);
            return;
        }
    };

    match choice {
        0 => crate::nginx::acme::acme_management(),
        1 => additional_package_managers(),
        2 => system_utilities(),
        3 => network_tools(),
        4 => monitoring_tools(),
        _ => return,
    }
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
