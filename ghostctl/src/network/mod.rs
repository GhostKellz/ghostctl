pub mod dns;
pub mod mesh;
pub mod netcat;
pub mod scan;

use dialoguer::{Select, theme::ColorfulTheme};

pub fn network_menu() {
    println!("🌐 Network Management");
    println!("====================");

    let options = [
        "🔍 DNS Lookup",
        "📡 Network Scanning",
        "🌐 Netcat Utilities",
        "🔗 Mesh Networking",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Network Management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => dns_menu(),
        1 => scan_menu(),
        2 => netcat_menu(),
        3 => mesh::status(),
        _ => return,
    }
}

fn dns_menu() {
    println!("🔍 DNS Tools");
    println!("============");

    let options = ["🔍 DNS Lookup", "🔒 DNSSEC Check", "⬅️  Back"];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("DNS Tools")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    use dialoguer::Input;
    let Ok(domain) = Input::<String>::new()
        .with_prompt("Domain name")
        .interact_text()
    else {
        return;
    };

    match choice {
        0 => dns::lookup(&domain),
        1 => dns::check_dnssec(&domain),
        _ => return,
    }
}

fn scan_menu() {
    println!("📡 Network Scanning");
    println!("===================");

    let options = ["🎯 Target Scan", "🔍 Interactive Scan", "⬅️  Back"];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Scanning Options")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => {
            use dialoguer::Input;
            let Ok(target) = Input::<String>::new()
                .with_prompt("Target IP, CIDR, or range")
                .interact_text()
            else {
                return;
            };
            // Launch new scanner with default settings
            println!("🔍 Launching scanner for {}...", target);
            let _ = scan::scan_cli(vec![target], None, None);
        }
        1 => {
            // Interactive mode - launch scanner menu
            scan::network_security_scanning();
        }
        _ => return,
    }
}

fn netcat_menu() {
    println!("🌐 Netcat Utilities");
    println!("==================");

    let options = [
        "📤 Send a file",
        "📥 Receive a file",
        "💬 Chat session",
        "🔍 Check port connectivity",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Netcat Utilities")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => {
            use dialoguer::Input;
            let Ok(file) = Input::<String>::new()
                .with_prompt("File to send")
                .interact_text()
            else {
                return;
            };
            let Ok(host) = Input::<String>::new()
                .with_prompt("Target host")
                .interact_text()
            else {
                return;
            };
            let Ok(port) = Input::<u16>::new().with_prompt("Target port").interact() else {
                return;
            };
            netcat::send_file(&file, &host, port);
        }
        1 => {
            use dialoguer::Input;
            let Ok(file) = Input::<String>::new()
                .with_prompt("File to save as")
                .interact_text()
            else {
                return;
            };
            let Ok(port) = Input::<u16>::new()
                .with_prompt("Port to listen on")
                .interact()
            else {
                return;
            };
            netcat::receive_file(&file, port);
        }
        2 => {
            use dialoguer::Input;
            let Ok(port) = Input::<u16>::new().with_prompt("Port to use").interact() else {
                return;
            };
            let Ok(host) = Input::<String>::new()
                .with_prompt("Host to connect to (leave empty to start server)")
                .allow_empty(true)
                .interact_text()
            else {
                return;
            };
            let host_opt = if host.is_empty() {
                None
            } else {
                Some(host.as_str())
            };
            netcat::chat(host_opt, port);
        }
        3 => {
            use dialoguer::Input;
            let Ok(host) = Input::<String>::new()
                .with_prompt("Host to check")
                .interact_text()
            else {
                return;
            };
            let Ok(port) = Input::<u16>::new().with_prompt("Port to check").interact() else {
                return;
            };
            netcat::check_port(&host, port);
        }
        _ => return,
    }
}

pub fn security_audit() {
    println!("🔍 Network Security Audit - TODO: Implement");
}
