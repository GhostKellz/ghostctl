use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use reqwest::blocking::get;
use std::fs;
use std::path::Path;
use std::process::Command;

const SCRIPT_CATEGORIES: &[(&str, &str)] = &[
    ("System", "system"),
    ("Network", "network"),
    ("Docker", "docker"),
    ("Security", "security"),
    ("Development", "dev"),
    ("Homelab", "homelab"),
];

const BASE_SCRIPT_URL: &str = "https://raw.githubusercontent.com/ghostkellz/ghostctl/main/scripts";

pub fn scripts_menu() {
    println!("📋 Scripts & Tools Management");
    println!("=============================");

    let options = [
        "🌐 Remote Script Runner",
        "📝 Script Templates",
        "🔑 GhostCert Management",
        "🏠 Proxmox Helpers",
        "📄 Local Scripts",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Scripts & Tools")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => remote_script_runner(),
        1 => script_templates(),
        2 => ghostcert_menu(),
        3 => crate::proxmox::proxmox_menu(),
        4 => local_scripts_menu(),
        _ => return,
    }
}

fn ghostcert_menu() {
    println!("🔑 GhostCert SSL Certificate Manager");
    println!("====================================");

    let options = [
        "🚀 Run GhostCert Script",
        "📋 Certificate Status Check",
        "🔍 View Certificate Details",
        "🔑 Generate New Certificate",
        "🔄 Renew Certificates",
        "📊 Certificate Inventory",
        "🌐 Check Web Server SSL",
        "⚙️  Configuration",
        "❓ Help & Documentation",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Certificate Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => run_ghostcert_script(),
        1 => certificate_status_check(),
        2 => view_certificate_details(),
        3 => generate_new_certificate(),
        4 => renew_certificates(),
        5 => certificate_inventory(),
        6 => check_web_server_ssl(),
        7 => ghostcert_configuration(),
        8 => view_ghostcert_help(),
        _ => return,
    }
}

// Implementation functions with no duplicates
fn remote_script_runner() {
    println!("🌐 Remote Script Runner");
    println!("=======================");

    let options = [
        "📋 Browse Script Categories",
        "🔗 Run Custom URL",
        "🔄 Refresh Cache",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Remote Scripts")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => list_script_categories(),
        1 => custom_script_url(),
        2 => refresh_script_cache(),
        _ => return,
    }
}

fn script_templates() {
    println!("📝 Script Templates");
    println!("===================");

    let templates = [
        "🐚 Basic Bash Script",
        "🔧 System Maintenance",
        "📦 Package Installation",
        "🔄 Service Management",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Script Templates")
        .items(&templates)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => create_bash_template(),
        1 => create_maintenance_template(),
        2 => create_package_template(),
        3 => create_service_template(),
        _ => return,
    }
}

fn create_bash_template() {
    let template = r#"#!/bin/bash
# Basic Bash Script Template
# Created by GhostCTL

set -e  # Exit on error
set -u  # Exit on undefined variable

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Main script logic
main() {
    log_info "Script started"
    
    # Add your code here
    
    log_info "Script completed successfully"
}

# Run main function
main "$@"
"#;

    save_template("bash-template.sh", template);
}

fn create_maintenance_template() {
    let template = r#"#!/bin/bash
# System Maintenance Script Template

set -e

echo "🔧 System Maintenance Script"
echo "============================"

# Update system packages
echo "📦 Updating system packages..."
if command -v pacman &> /dev/null; then
    sudo pacman -Syu --noconfirm
elif command -v apt &> /dev/null; then
    sudo apt update && sudo apt upgrade -y
elif command -v dnf &> /dev/null; then
    sudo dnf update -y
fi

# Clean package cache
echo "🧹 Cleaning package cache..."
if command -v pacman &> /dev/null; then
    sudo pacman -Sc --noconfirm
elif command -v apt &> /dev/null; then
    sudo apt autoremove -y && sudo apt autoclean
fi

# Clean logs
echo "📝 Cleaning old logs..."
sudo journalctl --vacuum-time=7d

echo "✅ Maintenance completed"
"#;

    save_template("maintenance-template.sh", template);
}

fn create_package_template() {
    let template = r#"#!/bin/bash
# Package Installation Template

set -e

PACKAGES=("git" "curl" "wget" "vim")

echo "📦 Package Installation Script"
echo "============================="

install_packages() {
    if command -v pacman &> /dev/null; then
        sudo pacman -S --noconfirm "${PACKAGES[@]}"
    elif command -v apt &> /dev/null; then
        sudo apt update && sudo apt install -y "${PACKAGES[@]}"
    elif command -v dnf &> /dev/null; then
        sudo dnf install -y "${PACKAGES[@]}"
    fi
}

echo "📦 Installing packages: ${PACKAGES[*]}"
install_packages

echo "✅ Package installation completed"
"#;

    save_template("package-template.sh", template);
}

fn create_service_template() {
    let template = r#"#!/bin/bash
# Service Management Template

set -e

SERVICE_NAME="example-service"

echo "🔧 Service Management Script"
echo "============================"

manage_service() {
    case $1 in
        start)
            echo "🚀 Starting $SERVICE_NAME..."
            sudo systemctl start $SERVICE_NAME
            ;;
        stop)
            echo "🛑 Stopping $SERVICE_NAME..."
            sudo systemctl stop $SERVICE_NAME
            ;;
        restart)
            echo "🔄 Restarting $SERVICE_NAME..."
            sudo systemctl restart $SERVICE_NAME
            ;;
        status)
            echo "📊 Status of $SERVICE_NAME:"
            systemctl status $SERVICE_NAME
            ;;
        *)
            echo "Usage: $0 {start|stop|restart|status}"
            ;;
    esac
}

if [ $# -eq 0 ]; then
    echo "Select action:"
    select action in start stop restart status; do
        manage_service "$action"
        break
    done
else
    manage_service "$1"
fi
"#;

    save_template("service-template.sh", template);
}

fn save_template(filename: &str, content: &str) {
    let scripts_dir = dirs::config_dir().unwrap().join("ghostctl/scripts");
    fs::create_dir_all(&scripts_dir).unwrap();

    let file_path = scripts_dir.join(filename);

    match fs::write(&file_path, content) {
        Ok(_) => {
            // Make executable
            let _ = Command::new("chmod")
                .args(&["+x", file_path.to_str().unwrap()])
                .status();

            println!("✅ Template saved: {:?}", file_path);
        }
        Err(e) => println!("❌ Failed to save template: {}", e),
    }
}

pub fn run_script_by_url(url: &str) {
    println!("🌐 Running script from: {}", url);

    match get(url) {
        Ok(response) => {
            if response.status().is_success() {
                match response.text() {
                    Ok(content) => {
                        // Save to temp file and execute
                        let temp_file = "/tmp/ghostctl_script.sh";

                        match fs::write(temp_file, content) {
                            Ok(_) => {
                                let _ = Command::new("chmod").args(&["+x", temp_file]).status();

                                let confirm = Confirm::new()
                                    .with_prompt("Execute downloaded script?")
                                    .default(false)
                                    .interact()
                                    .unwrap();

                                if confirm {
                                    let _ = Command::new("bash").arg(temp_file).status();
                                }

                                let _ = fs::remove_file(temp_file);
                            }
                            Err(e) => println!("❌ Failed to save script: {}", e),
                        }
                    }
                    Err(e) => println!("❌ Failed to read response: {}", e),
                }
            } else {
                println!("❌ HTTP Error: {}", response.status());
            }
        }
        Err(e) => println!("❌ Network error: {}", e),
    }
}

pub fn list_script_categories() {
    println!("📋 Script Categories");
    println!("====================");

    for (i, (name, _)) in SCRIPT_CATEGORIES.iter().enumerate() {
        println!("{}. 📁 {}", i + 1, name);
    }

    let category_index: usize = Input::new()
        .with_prompt("Select category (number)")
        .interact()
        .unwrap_or(1);

    if category_index > 0 && category_index <= SCRIPT_CATEGORIES.len() {
        let (category_name, category_path) = SCRIPT_CATEGORIES[category_index - 1];
        show_category_scripts(category_name, category_path);
    }
}

fn show_category_scripts(category_name: &str, category_path: &str) {
    println!("📁 {} Scripts", category_name);
    println!("============================");

    // This would fetch from the repository
    println!("💡 Script category: {}", category_path);
    println!("🔗 URL: {}/{}", BASE_SCRIPT_URL, category_path);

    // For now, show placeholder
    println!("🚧 Script browsing will be implemented in future updates");
}

fn local_scripts_menu() {
    println!("📄 Local Scripts");
    println!("================");

    let scripts_dir = dirs::config_dir().unwrap().join("ghostctl/scripts");

    if !scripts_dir.exists() {
        fs::create_dir_all(&scripts_dir).unwrap();
        println!("📁 Created scripts directory: {:?}", scripts_dir);
        return;
    }

    if let Ok(entries) = fs::read_dir(&scripts_dir) {
        let mut scripts = Vec::new();

        for entry in entries.flatten() {
            if entry.path().is_file() {
                scripts.push(entry.file_name().to_string_lossy().to_string());
            }
        }

        if scripts.is_empty() {
            println!("📭 No local scripts found");
            println!("💡 Add scripts to: {:?}", scripts_dir);
        } else {
            println!("📋 Available scripts:");
            for script in &scripts {
                println!("  📄 {}", script);
            }
        }
    }
}

fn custom_script_url() {
    let url: String = Input::new()
        .with_prompt("Script URL")
        .interact_text()
        .unwrap();

    if !url.trim().is_empty() {
        run_script_by_url(&url);
    }
}

fn refresh_script_cache() {
    println!("🔄 Refreshing script cache...");
    println!("✅ Cache refreshed (placeholder)");
}

// Single implementations of certificate functions
fn run_ghostcert_script() {
    println!("🔑 Running GhostCert Certificate Management");

    let script_path = find_ghostcert_script();

    if let Some(path) = script_path {
        println!("📍 Found GhostCert script: {}", path);
        let _ = Command::new("bash").arg(&path).status();
    } else {
        println!("📥 GhostCert script not found. Downloading...");
        download_ghostcert_script();
    }
}

fn download_ghostcert_script() {
    println!("📥 Downloading GhostCert Script");

    let config_dir = dirs::config_dir().unwrap().join("ghostctl/scripts");
    fs::create_dir_all(&config_dir).unwrap();

    let script_url = "https://raw.githubusercontent.com/ghostkellz/ghostcert/main/ghostcert.sh";
    let script_path = config_dir.join("ghostcert.sh");

    match get(script_url) {
        Ok(response) => {
            if response.status().is_success() {
                match response.text() {
                    Ok(content) => match fs::write(&script_path, content) {
                        Ok(_) => {
                            let _ = Command::new("chmod")
                                .args(&["+x", script_path.to_str().unwrap()])
                                .status();
                            println!("✅ Downloaded: {:?}", script_path);
                            let _ = Command::new("bash").arg(&script_path).status();
                        }
                        Err(e) => println!("❌ Failed to save: {}", e),
                    },
                    Err(e) => println!("❌ Failed to read: {}", e),
                }
            }
        }
        Err(e) => println!("❌ Network error: {}", e),
    }
}

fn certificate_status_check() {
    println!("📋 Certificate Status Check");
    check_letsencrypt_certificates();
}

fn check_certificate_file(cert_path: &Path) {
    println!("🔍 Checking: {}", cert_path.display());
    check_certificate_expiry(cert_path);
}

fn check_letsencrypt_certificates() {
    println!("🔒 Let's Encrypt Certificates");
    let le_dir = Path::new("/etc/letsencrypt/live/");

    if le_dir.exists() {
        if let Ok(entries) = fs::read_dir(le_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let cert_file = entry.path().join("cert.pem");
                    if cert_file.exists() {
                        println!("🌐 Domain: {:?}", entry.file_name());
                        check_certificate_expiry(&cert_file);
                    }
                }
            }
        }
    } else {
        println!("📭 No Let's Encrypt certificates found");
    }
}

fn check_certificate_expiry(cert_path: &Path) {
    let output = Command::new("openssl")
        .args(&[
            "x509",
            "-in",
            cert_path.to_str().unwrap(),
            "-checkend",
            "2592000",
        ])
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                println!("  ✅ Valid for >30 days");
            } else {
                println!("  ⚠️  Expires within 30 days!");
            }
        }
        Err(_) => println!("  ❓ Could not check expiry"),
    }
}

fn view_certificate_details() {
    println!("📋 Certificate Details");

    let domain_name: String = Input::new()
        .with_prompt("Domain name")
        .interact_text()
        .unwrap();

    // Create filenames that will live long enough
    let cert_filename = format!("{}.crt", domain_name);
    let key_filename = format!("{}.key", domain_name);

    let cert_files = [
        ("Certificate", cert_filename.as_str()),
        ("Private Key", key_filename.as_str()),
    ];

    for (file_type, filename) in &cert_files {
        let cert_path = Path::new(filename);
        if cert_path.exists() {
            println!("✅ Found {}: {}", file_type, filename);
        } else {
            println!("❌ Missing {}: {}", file_type, filename);
        }
    }
}

fn generate_new_certificate() {
    println!("🔑 Certificate Generation");

    let cert_types = [
        "🌐 Let's Encrypt (Automated)",
        "🔒 Self-Signed (Development)",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Certificate type")
        .items(&cert_types)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => println!("💡 Use: sudo certbot --nginx -d yourdomain.com"),
        1 => println!(
            "💡 Use: openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes"
        ),
        _ => return,
    }
}

fn renew_certificates() {
    println!("🔄 Certificate Renewal");

    if Command::new("which").arg("certbot").status().is_ok() {
        let _ = Command::new("sudo").args(&["certbot", "renew"]).status();
    } else {
        println!("❌ certbot not found");
    }
}

fn certificate_inventory() {
    println!("📊 Certificate Inventory");
    certificate_status_check();
}

fn check_web_server_ssl() {
    println!("🌐 Web Server SSL Check");

    let domain: String = Input::new()
        .with_prompt("Domain to check")
        .interact_text()
        .unwrap();

    println!("🔍 Checking SSL for: {}", domain);
    let _ = Command::new("curl")
        .args(&["-I", &format!("https://{}", domain)])
        .status();
}

fn ghostcert_configuration() {
    println!("⚙️  GhostCert Configuration");
    println!("💡 Configuration managed by GhostCert script");
}

fn view_ghostcert_help() {
    println!("❓ GhostCert Help");
    println!("🔑 SSL certificate management tool");
    println!("🔗 Repository: https://github.com/ghostkellz/ghostcert");
}

fn find_ghostcert_script() -> Option<String> {
    let locations = [
        &dirs::home_dir()
            .unwrap()
            .join(".config/ghostctl/scripts/ghostcert.sh")
            .to_string_lossy()
            .to_string(),
        "/usr/local/bin/ghostcert.sh",
        "/usr/bin/ghostcert.sh",
        "./ghostcert.sh",
    ];

    for location in &locations {
        if Path::new(location).exists() {
            return Some(location.to_string());
        }
    }

    None
}

// Stub implementations for script generation functions to avoid missing function errors
fn save_script(_filename: &str, _content: &str) {
    println!("💡 Script generation feature will be added in future updates");
}

pub fn create_proxmox_docker_script() {
    save_script("proxmox-docker-deploy.sh", "# Docker deployment script");
}

pub fn create_proxmox_vm_script() {
    save_script("proxmox-vm-template.sh", "# VM template script");
}

pub fn create_proxmox_lxc_script() {
    save_script("proxmox-lxc-create.sh", "# LXC creation script");
}

pub fn create_proxmox_ssl_script() {
    save_script("proxmox-ssl-manager.sh", "# SSL management script");
}

pub fn create_proxmox_backup_script() {
    save_script("proxmox-backup-automation.sh", "# Backup automation script");
}

pub fn create_proxmox_monitoring_script() {
    save_script("proxmox-monitoring.sh", "# Monitoring setup script");
}

pub fn local_script_management() {
    println!("📝 Local Script Management");
    println!("===========================");
    local_scripts_menu();
}

pub fn run_specific_script(script_name: &str) {
    println!("🚀 Running script: {}", script_name);
    if let Some(script_url) = find_script_by_name(script_name) {
        run_script_by_url(&script_url);
    } else {
        println!("❌ Script not found: {}", script_name);
    }
}

pub fn list_category_scripts(category: &str) {
    println!("📋 Scripts in category: {}", category);
    for (cat_name, _path) in SCRIPT_CATEGORIES {
        if *cat_name == category {
            println!("  • Category: {}", cat_name);
            // TODO: Add actual script listing logic when implementing script discovery
            return;
        }
    }
    println!("❌ Category not found: {}", category);
}

fn find_script_by_name(_name: &str) -> Option<String> {
    for (_category, _path) in SCRIPT_CATEGORIES {
        // TODO: Implement actual script discovery and matching
        // This is a placeholder until script discovery is implemented
    }
    None
}
