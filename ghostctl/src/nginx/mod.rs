pub mod acme;
pub mod config;
pub mod proxy;
pub mod ssl;

use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::process::Command;

pub fn nginx_menu() {
    println!("🌐 Nginx Configuration Manager");
    println!("==============================");

    let options = [
        "⚙️  Configuration Builder",
        "🔒 SSL/TLS Management",
        "🌐 ACME.sh Certificate Management",
        "🔄 Reverse Proxy Setup",
        "📊 Status & Monitoring",
        "🧪 Configuration Testing",
        "🔄 Reload/Restart Services",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Nginx Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => config::configuration_builder(),
        1 => ssl::ssl_management(),
        2 => acme::acme_management(),
        3 => proxy::reverse_proxy_setup(),
        4 => nginx_status(),
        5 => test_configuration(),
        6 => service_management(),
        _ => return,
    }
}

fn nginx_status() {
    println!("📊 Nginx Status & Monitoring");
    println!("============================");

    // Check if nginx is running
    let nginx_status = Command::new("systemctl")
        .args(&["is-active", "nginx"])
        .output();

    match nginx_status {
        Ok(output) if output.status.success() => {
            let status = String::from_utf8_lossy(&output.stdout);
            let status = status.trim();
            if status == "active" {
                println!("✅ Nginx is running");
            } else {
                println!("❌ Nginx is not active: {}", status);
            }
        }
        _ => println!("❌ Failed to check nginx status"),
    }

    // Show nginx processes
    println!("\n🔍 Nginx Processes:");
    let _ = Command::new("ps").args(&["aux"]).status();

    // Show nginx configuration test
    println!("\n🧪 Configuration Test:");
    let _ = Command::new("nginx").args(&["-t"]).status();

    // Show access logs (last 10 lines)
    println!("\n📜 Recent Access Logs:");
    let _ = Command::new("tail")
        .args(&["-n", "10", "/var/log/nginx/access.log"])
        .status();

    // Show error logs (last 10 lines)
    println!("\n❌ Recent Error Logs:");
    let _ = Command::new("tail")
        .args(&["-n", "10", "/var/log/nginx/error.log"])
        .status();
}

fn test_configuration() {
    println!("🧪 Testing Nginx Configuration");
    println!("==============================");

    let status = Command::new("nginx").args(&["-t"]).status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Configuration test passed!");

            let reload = Confirm::new()
                .with_prompt("Reload nginx with new configuration?")
                .default(true)
                .interact()
                .unwrap();

            if reload {
                reload_nginx();
            }
        }
        _ => println!("❌ Configuration test failed!"),
    }
}

fn service_management() {
    println!("🔄 Nginx Service Management");
    println!("===========================");

    let actions = [
        "🔄 Reload configuration",
        "🔄 Restart service",
        "🛑 Stop service",
        "🚀 Start service",
        "📊 Service status",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Service action")
        .items(&actions)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => reload_nginx(),
        1 => restart_nginx(),
        2 => stop_nginx(),
        3 => start_nginx(),
        4 => nginx_service_status(),
        _ => return,
    }
}

fn reload_nginx() {
    println!("🔄 Reloading nginx configuration...");

    let status = Command::new("sudo")
        .args(&["systemctl", "reload", "nginx"])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Nginx reloaded successfully"),
        _ => println!("❌ Failed to reload nginx"),
    }
}

fn restart_nginx() {
    println!("🔄 Restarting nginx service...");

    let status = Command::new("sudo")
        .args(&["systemctl", "restart", "nginx"])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Nginx restarted successfully"),
        _ => println!("❌ Failed to restart nginx"),
    }
}

fn stop_nginx() {
    println!("🛑 Stopping nginx service...");

    let confirm = Confirm::new()
        .with_prompt("Are you sure you want to stop nginx?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let status = Command::new("sudo")
            .args(&["systemctl", "stop", "nginx"])
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Nginx stopped"),
            _ => println!("❌ Failed to stop nginx"),
        }
    }
}

fn start_nginx() {
    println!("🚀 Starting nginx service...");

    let status = Command::new("sudo")
        .args(&["systemctl", "start", "nginx"])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Nginx started successfully"),
        _ => println!("❌ Failed to start nginx"),
    }
}

fn nginx_service_status() {
    println!("📊 Nginx Service Status");
    println!("=======================");

    let _ = Command::new("sudo")
        .args(&["systemctl", "status", "nginx"])
        .status();
}

// Functions called from main.rs
pub fn generate_config() {
    config::configuration_builder();
}
pub fn ssl_management() {
    ssl::ssl_management();
}
pub fn proxy_config() {
    proxy::reverse_proxy_setup();
}
pub fn test_config() {
    test_configuration();
}
pub fn reload_service() {
    reload_nginx();
}
pub fn setup_ssl_for_domain(domain: &str) {
    println!("🔒 Setting up SSL for domain: {}", domain);
    ssl::ssl_management();
}
