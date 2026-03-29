use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn acme_management() {
    println!("🔒 ACME.sh SSL Certificate Management");
    println!("=====================================");

    let options = [
        "📥 Install acme.sh",
        "🌐 Issue Let's Encrypt certificate",
        "🔄 Renew all certificates",
        "📋 List certificates",
        "🗑️  Remove certificate",
        "🔧 Configure DNS API",
        "📊 Certificate status",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("ACME.sh Management")
        .items(&options)
        .default(0)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    match choice {
        0 => install_acme_sh(),
        1 => issue_certificate(),
        2 => renew_all_certificates(),
        3 => list_certificates(),
        4 => remove_certificate(),
        5 => configure_dns_api(),
        6 => certificate_status(),
        _ => return,
    }
}

pub fn install_acme_sh() {
    println!("📥 Installing acme.sh...");

    // Check if already installed
    if Command::new("which").arg("acme.sh").output().is_ok() {
        println!("✅ acme.sh is already installed");
        return;
    }

    // Download and install acme.sh
    let status = Command::new("sh")
        .arg("-c")
        .arg("curl https://get.acme.sh | sh -s email=admin@ghost.local")
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ acme.sh installed successfully!");
            println!("📁 Installation directory: ~/.acme.sh/");

            // Set up alias
            let _ = Command::new("sh")
                .arg("-c")
                .arg("echo 'alias acme.sh=~/.acme.sh/acme.sh' >> ~/.bashrc")
                .status();
        }
        _ => println!("❌ Failed to install acme.sh"),
    }
}

pub fn issue_certificate() {
    let domain: String = match Input::new().with_prompt("Domain name").interact_text() {
        Ok(d) => d,
        Err(_) => return,
    };

    let webroot: String = match Input::new()
        .with_prompt("Webroot path")
        .default("/var/www/html".to_string())
        .interact_text()
    {
        Ok(w) => w,
        Err(_) => return,
    };

    println!("🚀 Issuing certificate for: {}", domain);

    // Create nginx certs directory structure
    let cert_dir = format!("/etc/nginx/certs/{}", domain);
    let _ = Command::new("sudo")
        .args(&["mkdir", "-p", &cert_dir])
        .status();

    // Issue certificate with acme.sh
    let status = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "~/.acme.sh/acme.sh --issue -d {} -w {} --server letsencrypt",
            domain, webroot
        ))
        .status();

    if let Ok(s) = status {
        if !s.success() {
            println!("❌ Failed to issue certificate");
            return;
        }
    } else {
        println!("❌ Failed to issue certificate");
        return;
    }

    {
        // Install certificate to nginx directory
        let install_status = Command::new("sh")
            .arg("-c")
            .arg(&format!(
                "~/.acme.sh/acme.sh --install-cert -d {} \
                --key-file {}/privkey.pem \
                --fullchain-file {}/fullchain.pem \
                --reloadcmd 'sudo systemctl reload nginx'",
                domain, cert_dir, cert_dir
            ))
            .status();

        match install_status {
            Ok(s) if s.success() => {
                println!("✅ Certificate issued and installed!");
                println!("📁 Certificate location: {}", cert_dir);
            }
            _ => println!("❌ Failed to install certificate"),
        }
    }
}

pub fn renew_all_certificates() {
    println!("🔄 Renewing all certificates...");

    let status = Command::new("sh")
        .arg("-c")
        .arg("~/.acme.sh/acme.sh --renew-all")
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ All certificates renewed successfully!"),
        _ => println!("❌ Some certificates failed to renew"),
    }
}

pub fn list_certificates() {
    println!("📋 Installed Certificates:");
    println!("=========================");

    let _ = Command::new("sh")
        .arg("-c")
        .arg("~/.acme.sh/acme.sh --list")
        .status();
}

fn remove_certificate() {
    let domain: String = match Input::new().with_prompt("Domain to remove").interact_text() {
        Ok(d) => d,
        Err(_) => return,
    };

    println!("🗑️  Removing certificate for: {}", domain);

    let status = Command::new("sh")
        .arg("-c")
        .arg(&format!("~/.acme.sh/acme.sh --remove -d {}", domain))
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Certificate removed from acme.sh");

            // Also remove from nginx directory
            let cert_dir = format!("/etc/nginx/certs/{}", domain);
            let _ = Command::new("sudo")
                .args(&["rm", "-rf", &cert_dir])
                .status();
        }
        _ => println!("❌ Failed to remove certificate"),
    }
}

fn configure_dns_api() {
    println!("🔧 Configure DNS API for DNS-01 Challenge");
    println!("========================================");

    let providers = [
        "Cloudflare",
        "Route53 (AWS)",
        "Azure DNS",
        "DigitalOcean",
        "PowerDNS",
        "GoDaddy",
        "Namecheap",
        "Manual Configuration",
        "Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select DNS Provider")
        .items(&providers)
        .default(0)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    match choice {
        0 => configure_cloudflare(),
        1 => configure_route53(),
        2 => configure_azure_dns(),
        3 => configure_digitalocean(),
        4 => configure_powerdns(),
        5 => configure_godaddy(),
        6 => configure_namecheap(),
        7 => manual_dns_config(),
        _ => return,
    }
}

fn configure_cloudflare() {
    let email: String = match Input::new().with_prompt("Cloudflare Email").interact_text() {
        Ok(e) => e,
        Err(_) => return,
    };

    let api_key: String = match Input::new()
        .with_prompt("Cloudflare API Key")
        .interact_text()
    {
        Ok(k) => k,
        Err(_) => return,
    };

    // Export environment variables
    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export CF_Email=\"{}\"' >> ~/.bashrc",
            email
        ))
        .status();

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export CF_Key=\"{}\"' >> ~/.bashrc",
            api_key
        ))
        .status();

    println!("✅ Cloudflare DNS API configured!");
}

fn configure_route53() {
    println!("📋 Route53 requires AWS CLI to be configured");
    println!("Run: aws configure");
}

fn configure_azure_dns() {
    println!("☁️  Azure DNS Configuration");
    println!("==========================");

    let subscription_id: String = match Input::new()
        .with_prompt("Azure Subscription ID")
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    let tenant_id: String = match Input::new().with_prompt("Azure Tenant ID").interact_text() {
        Ok(t) => t,
        Err(_) => return,
    };

    let client_id: String = match Input::new()
        .with_prompt("Azure Client ID (App ID)")
        .interact_text()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    let client_secret: String = match Input::new()
        .with_prompt("Azure Client Secret")
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    // Export environment variables for acme.sh Azure DNS hook
    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export AZUREDNS_SUBSCRIPTIONID=\"{}\"' >> ~/.bashrc",
            subscription_id
        ))
        .status();

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export AZUREDNS_TENANTID=\"{}\"' >> ~/.bashrc",
            tenant_id
        ))
        .status();

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export AZUREDNS_APPID=\"{}\"' >> ~/.bashrc",
            client_id
        ))
        .status();

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export AZUREDNS_CLIENTSECRET=\"{}\"' >> ~/.bashrc",
            client_secret
        ))
        .status();

    println!("✅ Azure DNS API configured!");
    println!("📋 You can now use DNS-01 challenge with Azure DNS");
    println!("Example: ~/.acme.sh/acme.sh --issue -d example.com --dns dns_azure");
}

fn configure_digitalocean() {
    let token: String = match Input::new()
        .with_prompt("DigitalOcean API Token")
        .interact_text()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export DO_API_TOKEN=\"{}\"' >> ~/.bashrc",
            token
        ))
        .status();

    println!("✅ DigitalOcean DNS API configured!");
}

fn configure_powerdns() {
    println!("🔧 PowerDNS API Configuration");
    println!("=============================");

    let api_url: String = match Input::new()
        .with_prompt("PowerDNS API URL (e.g., http://localhost:8081)")
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    let api_key: String = match Input::new().with_prompt("PowerDNS API Key").interact_text() {
        Ok(k) => k,
        Err(_) => return,
    };

    let server_id: String = match Input::new()
        .with_prompt("PowerDNS Server ID")
        .default("localhost".to_string())
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    // Export environment variables for acme.sh PowerDNS hook
    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export PDNS_Url=\"{}\"' >> ~/.bashrc",
            api_url
        ))
        .status();

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export PDNS_ApiKey=\"{}\"' >> ~/.bashrc",
            api_key
        ))
        .status();

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export PDNS_ServerId=\"{}\"' >> ~/.bashrc",
            server_id
        ))
        .status();

    println!("✅ PowerDNS API configured!");
    println!("📋 You can now use DNS-01 challenge with PowerDNS");
    println!("Example: ~/.acme.sh/acme.sh --issue -d example.com --dns dns_pdns");
}

fn configure_godaddy() {
    let key: String = match Input::new().with_prompt("GoDaddy API Key").interact_text() {
        Ok(k) => k,
        Err(_) => return,
    };

    let secret: String = match Input::new()
        .with_prompt("GoDaddy API Secret")
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!("echo 'export GD_Key=\"{}\"' >> ~/.bashrc", key))
        .status();

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export GD_Secret=\"{}\"' >> ~/.bashrc",
            secret
        ))
        .status();

    println!("✅ GoDaddy DNS API configured!");
}

fn configure_namecheap() {
    let user: String = match Input::new()
        .with_prompt("Namecheap Username")
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    let api_key: String = match Input::new()
        .with_prompt("Namecheap API Key")
        .interact_text()
    {
        Ok(k) => k,
        Err(_) => return,
    };

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export NAMECHEAP_USERNAME=\"{}\"' >> ~/.bashrc",
            user
        ))
        .status();

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "echo 'export NAMECHEAP_API_KEY=\"{}\"' >> ~/.bashrc",
            api_key
        ))
        .status();

    println!("✅ Namecheap DNS API configured!");
}

fn manual_dns_config() {
    println!("📋 Manual DNS Configuration");
    println!("For other DNS providers, set the appropriate environment variables");
    println!("Refer to: https://github.com/acmesh-official/acme.sh/wiki/dnsapi");
}

fn certificate_status() {
    println!("📊 Certificate Status");
    println!("====================");

    // Show cron job
    println!("\n⏰ Auto-renewal cron job:");
    let _ = Command::new("crontab").arg("-l").status();

    // Show certificate details
    println!("\n📋 Certificate details:");
    let _ = Command::new("sh")
        .arg("-c")
        .arg("~/.acme.sh/acme.sh --list")
        .status();
}
