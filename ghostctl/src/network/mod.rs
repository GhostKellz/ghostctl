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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Network Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let options = [
        "🔍 DNS Lookup",
        "🔒 DNSSEC Check",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("DNS Tools")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    use dialoguer::Input;
    let domain: String = Input::new()
        .with_prompt("Domain name")
        .interact_text()
        .unwrap();

    match choice {
        0 => dns::lookup(&domain),
        1 => dns::check_dnssec(&domain),
        _ => return,
    }
}

fn scan_menu() {
    println!("📡 Network Scanning");
    println!("===================");

    let options = [
        "🎯 Target Scan",
        "🔍 Interactive Scan",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Scanning Options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            use dialoguer::Input;
            let target: String = Input::new()
                .with_prompt("Target IP, CIDR, or range")
                .interact_text()
                .unwrap();
            scan::gscan_port_scan(&target, None, None, false);
        }
        1 => scan::gscan_interactive(),
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Netcat Utilities")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            use dialoguer::Input;
            let file: String = Input::new().with_prompt("File to send").interact_text().unwrap();
            let host: String = Input::new().with_prompt("Target host").interact_text().unwrap();
            let port: u16 = Input::new().with_prompt("Target port").interact().unwrap();
            netcat::send_file(&file, &host, port);
        }
        1 => {
            use dialoguer::Input;
            let file: String = Input::new().with_prompt("File to save as").interact_text().unwrap();
            let port: u16 = Input::new().with_prompt("Port to listen on").interact().unwrap();
            netcat::receive_file(&file, port);
        }
        2 => {
            use dialoguer::Input;
            let port: u16 = Input::new().with_prompt("Port to use").interact().unwrap();
            let host = Input::<String>::new().with_prompt("Host to connect to (leave empty to start server)").allow_empty(true).interact_text().unwrap();
            let host_opt = if host.is_empty() { None } else { Some(host.as_str()) };
            netcat::chat(host_opt, port);
        }
        3 => {
            use dialoguer::Input;
            let host: String = Input::new().with_prompt("Host to check").interact_text().unwrap();
            let port: u16 = Input::new().with_prompt("Port to check").interact().unwrap();
            netcat::check_port(&host, port);
        }
        _ => return,
    }
}

pub fn security_audit() {
    println!("🔍 Network Security Audit - TODO: Implement");
}
