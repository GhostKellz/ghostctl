use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn ssl_management() {
    println!("🔒 Nginx SSL/TLS Management");
    println!("===========================");

    let ssl_options = [
        "🌐 Setup Let's Encrypt SSL",
        "🔐 Generate self-signed certificate",
        "📜 Install custom certificate",
        "🔄 Renew certificates",
        "📋 List SSL certificates",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("SSL Management")
        .items(&ssl_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => setup_letsencrypt(),
        1 => generate_self_signed(),
        2 => install_custom_cert(),
        3 => renew_certificates(),
        4 => list_certificates(),
        _ => return,
    }
}

fn setup_letsencrypt() {
    println!("🌐 Let's Encrypt SSL Setup");
    println!("==========================");

    // Check if certbot is installed
    if Command::new("which").arg("certbot").status().is_err() {
        println!("❌ Certbot not found. Installing...");
        let _ = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", "certbot", "certbot-nginx"])
            .status();
    }

    let domain: String = Input::new()
        .with_prompt("Domain name")
        .interact_text()
        .unwrap();

    let email: String = Input::new()
        .with_prompt("Email address")
        .interact_text()
        .unwrap();

    // Create custom nginx cert directory structure
    let cert_dir = format!("/etc/nginx/certs/{}", domain);
    println!("� Creating certificate directory: {}", cert_dir);
    let _ = Command::new("sudo")
        .args(&["mkdir", "-p", &cert_dir])
        .status();

    println!("�🚀 Setting up Let's Encrypt for: {}", domain);

    let status = Command::new("sudo")
        .args(&[
            "certbot",
            "certonly",
            "--standalone",
            "-d",
            &domain,
            "--email",
            &email,
            "--agree-tos",
            "--non-interactive",
        ])
        .status();

    if status.is_ok() && status.unwrap().success() {
        // Copy certificates to custom structure
        println!("📋 Copying certificates to custom nginx structure...");
        let letsencrypt_dir = format!("/etc/letsencrypt/live/{}", domain);

        let _ = Command::new("sudo")
            .args(&[
                "cp",
                &format!("{}/fullchain.pem", letsencrypt_dir),
                &format!("{}/cert.pem", cert_dir),
            ])
            .status();

        let _ = Command::new("sudo")
            .args(&[
                "cp",
                &format!("{}/privkey.pem", letsencrypt_dir),
                &format!("{}/privkey.pem", cert_dir),
            ])
            .status();

        println!("✅ Certificates installed to: {}", cert_dir);
    }
}

fn generate_self_signed() {
    println!("🔐 Generate Self-Signed Certificate");
    println!("===================================");

    let domain: String = Input::new()
        .with_prompt("Domain/Common Name")
        .interact_text()
        .unwrap();

    println!("🔑 Generating self-signed certificate for: {}", domain);

    // Create SSL directory
    let _ = Command::new("sudo")
        .args(&["mkdir", "-p", "/etc/nginx/ssl"])
        .status();

    // Generate private key and certificate
    let _ = Command::new("sudo")
        .args(&[
            "openssl",
            "req",
            "-x509",
            "-newkey",
            "rsa:4096",
            "-keyout",
            "/etc/nginx/ssl/nginx-selfsigned.key",
            "-out",
            "/etc/nginx/ssl/nginx-selfsigned.crt",
            "-days",
            "365",
            "-nodes",
            "-subj",
            &format!("/CN={}", domain),
        ])
        .status();

    println!("✅ Self-signed certificate generated:");
    println!("  🔑 Key: /etc/nginx/ssl/nginx-selfsigned.key");
    println!("  📜 Certificate: /etc/nginx/ssl/nginx-selfsigned.crt");
}

fn install_custom_cert() {
    println!("📜 Install Custom Certificate");
    println!("=============================");

    let cert_path: String = Input::new()
        .with_prompt("Certificate file path")
        .interact_text()
        .unwrap();

    let key_path: String = Input::new()
        .with_prompt("Private key file path")
        .interact_text()
        .unwrap();

    let dest_name: String = Input::new()
        .with_prompt("Certificate name")
        .interact_text()
        .unwrap();

    // Copy certificate files to nginx SSL directory
    let _ = Command::new("sudo")
        .args(&["mkdir", "-p", "/etc/nginx/ssl"])
        .status();

    let _ = Command::new("sudo")
        .args(&[
            "cp",
            &cert_path,
            &format!("/etc/nginx/ssl/{}.crt", dest_name),
        ])
        .status();

    let _ = Command::new("sudo")
        .args(&[
            "cp",
            &key_path,
            &format!("/etc/nginx/ssl/{}.key", dest_name),
        ])
        .status();

    println!("✅ Custom certificate installed");
}

fn renew_certificates() {
    println!("🔄 Renewing SSL Certificates");
    println!("============================");

    // Try Let's Encrypt renewal
    if Command::new("which").arg("certbot").status().is_ok() {
        println!("🔄 Renewing Let's Encrypt certificates...");
        let _ = Command::new("sudo")
            .args(&["certbot", "renew", "--nginx"])
            .status();
    } else {
        println!("❌ Certbot not found for automatic renewal");
    }
}

fn list_certificates() {
    println!("📋 SSL Certificates");
    println!("===================");

    // List Let's Encrypt certificates
    if Command::new("which").arg("certbot").status().is_ok() {
        println!("🌐 Let's Encrypt certificates:");
        let _ = Command::new("sudo")
            .args(&["certbot", "certificates"])
            .status();
    }

    // List custom certificates
    println!("\n📁 Custom certificates in /etc/nginx/ssl/:");
    let _ = Command::new("sudo")
        .args(&["ls", "-la", "/etc/nginx/ssl/"])
        .status();
}

// Custom certificate path management for your /etc/nginx/certs/domain/ structure
fn get_certificate_paths(domain: &str) -> (String, String) {
    let custom_cert_dir = format!("/etc/nginx/certs/{}", domain);

    // Check custom structure first
    let custom_cert = format!("{}/cert.pem", custom_cert_dir);
    let custom_key = format!("{}/private.key", custom_cert_dir);

    if std::path::Path::new(&custom_cert).exists() && std::path::Path::new(&custom_key).exists() {
        return (custom_cert, custom_key);
    }

    // Check for alternative names in custom structure
    let alt_cert_names = ["fullchain.pem", "certificate.crt", "ssl.crt"];
    let alt_key_names = ["privkey.pem", "private.key", "ssl.key"];

    for cert_name in &alt_cert_names {
        for key_name in &alt_key_names {
            let cert_path = format!("{}/{}", custom_cert_dir, cert_name);
            let key_path = format!("{}/{}", custom_cert_dir, key_name);

            if std::path::Path::new(&cert_path).exists() && std::path::Path::new(&key_path).exists()
            {
                return (cert_path, key_path);
            }
        }
    }

    // Fallback to Let's Encrypt standard paths
    let letsencrypt_cert = format!("/etc/letsencrypt/live/{}/fullchain.pem", domain);
    let letsencrypt_key = format!("/etc/letsencrypt/live/{}/privkey.pem", domain);

    if std::path::Path::new(&letsencrypt_cert).exists() {
        return (letsencrypt_cert, letsencrypt_key);
    }

    // Return custom paths (will be created)
    (custom_cert, custom_key)
}

fn ensure_custom_cert_structure(domain: &str) {
    let cert_dir = format!("/etc/nginx/certs/{}", domain);

    // Create directory if it doesn't exist
    let _ = Command::new("sudo")
        .args(&["mkdir", "-p", &cert_dir])
        .status();

    println!("📁 Certificate directory: {}", cert_dir);
}
