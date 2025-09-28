use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::fs;
use std::process::Command;

pub fn configuration_builder() {
    println!("⚙️  Nginx Configuration Builder");
    println!("===============================");

    let config_options = [
        "🌐 Basic web server",
        "🔄 Reverse proxy setup",
        "🔒 SSL/TLS configuration",
        "📁 Static file server",
        "🎯 Load balancer",
        "📝 Custom configuration",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Configuration type")
        .items(&config_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => create_basic_server(),
        1 => create_reverse_proxy(),
        2 => configure_ssl(),
        3 => create_static_server(),
        4 => create_load_balancer(),
        5 => custom_configuration(),
        _ => return,
    }
}

fn create_basic_server() {
    println!("🌐 Basic Web Server Configuration");
    println!("=================================");

    let server_name: String = Input::new()
        .with_prompt("Server name (domain)")
        .interact_text()
        .unwrap();

    let root_path: String = Input::new()
        .with_prompt("Document root path")
        .default("/var/www/html".into())
        .interact_text()
        .unwrap();

    let port: String = Input::new()
        .with_prompt("Port")
        .default("80".into())
        .interact_text()
        .unwrap();

    let config = format!(
        r#"server {{
    listen {};
    server_name {};
    
    root {};
    index index.html index.htm;
    
    location / {{
        try_files $uri $uri/ =404;
    }}
    
    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header X-Content-Type-Options "nosniff" always;
    
    # Logging
    access_log /var/log/nginx/{}_access.log;
    error_log /var/log/nginx/{}_error.log;
}}
"#,
        port, server_name, root_path, server_name, server_name
    );

    save_nginx_config(&server_name, &config);
}

fn create_reverse_proxy() {
    println!("🔄 Reverse Proxy Configuration");
    println!("==============================");

    let server_name: String = Input::new()
        .with_prompt("Server name (domain)")
        .interact_text()
        .unwrap();

    let backend_url: String = Input::new()
        .with_prompt("Backend URL (e.g., http://localhost:3000)")
        .interact_text()
        .unwrap();

    let config = format!(
        r#"server {{
    listen 80;
    server_name {};
    
    location / {{
        proxy_pass {};
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }}
    
    # Logging
    access_log /var/log/nginx/{}_access.log;
    error_log /var/log/nginx/{}_error.log;
}}
"#,
        server_name, backend_url, server_name, server_name
    );

    save_nginx_config(&server_name, &config);
}

fn configure_ssl() {
    println!("🔒 SSL/TLS Configuration");
    println!("========================");

    let server_name: String = Input::new()
        .with_prompt("Server name (domain)")
        .interact_text()
        .unwrap();

    let ssl_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("SSL certificate type")
        .items(&[
            "🌐 Custom nginx structure",
            "🌐 Let's Encrypt",
            "🔐 Self-signed",
            "📜 Custom certificates",
        ])
        .default(0)
        .interact()
        .unwrap();

    let (cert_path, key_path) = match ssl_type {
        0 => (
            format!("/etc/nginx/certs/{}/cert.pem", server_name),
            format!("/etc/nginx/certs/{}/privkey.pem", server_name),
        ),
        1 => (
            format!("/etc/letsencrypt/live/{}/fullchain.pem", server_name),
            format!("/etc/letsencrypt/live/{}/privkey.pem", server_name),
        ),
        2 => (
            "/etc/nginx/ssl/selfsigned.crt".to_string(),
            "/etc/nginx/ssl/selfsigned.key".to_string(),
        ),
        3 => {
            let cert: String = Input::new()
                .with_prompt("Certificate path")
                .interact_text()
                .unwrap();
            let key: String = Input::new()
                .with_prompt("Private key path")
                .interact_text()
                .unwrap();
            (cert, key)
        }
        _ => return,
    };

    // Create custom nginx cert directory if using custom structure
    if ssl_type == 0 {
        let cert_dir = format!("/etc/nginx/certs/{}", server_name);
        let _ = Command::new("sudo")
            .args(&["mkdir", "-p", &cert_dir])
            .status();
        println!("📁 Created certificate directory: {}", cert_dir);
    }

    let config = format!(
        r#"server {{
    listen 80;
    server_name {};
    return 301 https://$server_name$request_uri;
}}

server {{
    listen 443 ssl http2;
    server_name {};
    
    ssl_certificate {};
    ssl_certificate_key {};
    
    # SSL configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;
    
    # Security headers
    add_header Strict-Transport-Security "max-age=63072000" always;
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    
    root /var/www/{};
    index index.html index.htm;
    
    location / {{
        try_files $uri $uri/ =404;
    }}
    
    # Logging
    access_log /var/log/nginx/{}_ssl_access.log;
    error_log /var/log/nginx/{}_ssl_error.log;
}}
"#,
        server_name, server_name, cert_path, key_path, server_name, server_name, server_name
    );

    save_nginx_config(&format!("{}_ssl", server_name), &config);
}

fn create_static_server() {
    println!("📁 Static File Server Configuration");
    println!("===================================");

    let server_name: String = Input::new()
        .with_prompt("Server name")
        .interact_text()
        .unwrap();

    let root_path: String = Input::new()
        .with_prompt("Static files directory")
        .interact_text()
        .unwrap();

    let config = format!(
        r#"server {{
    listen 80;
    server_name {};
    
    root {};
    
    # Enable directory browsing
    autoindex on;
    autoindex_exact_size off;
    autoindex_localtime on;
    
    # Cache static files
    location ~* \.(jpg|jpeg|png|gif|ico|css|js)$ {{
        expires 1y;
        add_header Cache-Control "public, immutable";
    }}
    
    # Security
    location ~ /\. {{
        deny all;
    }}
    
    # Logging
    access_log /var/log/nginx/{}_access.log;
    error_log /var/log/nginx/{}_error.log;
}}
"#,
        server_name, root_path, server_name, server_name
    );

    save_nginx_config(&format!("{}_static", server_name), &config);
}

fn create_load_balancer() {
    println!("🎯 Load Balancer Configuration");
    println!("==============================");

    let upstream_name: String = Input::new()
        .with_prompt("Upstream name")
        .interact_text()
        .unwrap();

    let backend_count: String = Input::new()
        .with_prompt("Number of backend servers")
        .default("2".into())
        .interact_text()
        .unwrap();

    let count: usize = backend_count.parse().unwrap_or(2);
    let mut backends = Vec::new();

    for i in 1..=count {
        let backend: String = Input::new()
            .with_prompt(&format!("Backend {} (e.g., 192.168.1.{}:80)", i, i))
            .interact_text()
            .unwrap();
        backends.push(backend);
    }

    let server_name: String = Input::new()
        .with_prompt("Server name")
        .interact_text()
        .unwrap();

    let mut config = format!("upstream {} {{\n", upstream_name);
    for backend in backends {
        config.push_str(&format!("    server {};\n", backend));
    }
    config.push_str("}\n\n");

    config.push_str(&format!(
        r#"server {{
    listen 80;
    server_name {};
    
    location / {{
        proxy_pass http://{};
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }}
    
    # Health check endpoint
    location /nginx_status {{
        stub_status on;
        access_log off;
        allow 127.0.0.1;
        deny all;
    }}
    
    # Logging
    access_log /var/log/nginx/{}_access.log;
    error_log /var/log/nginx/{}_error.log;
}}
"#,
        server_name, upstream_name, server_name, server_name
    ));

    save_nginx_config(&format!("{}_lb", server_name), &config);
}

fn custom_configuration() {
    println!("📝 Custom Configuration Editor");
    println!("=============================");

    let config_name: String = Input::new()
        .with_prompt("Configuration name")
        .interact_text()
        .unwrap();

    println!("Opening editor for custom configuration...");
    println!("Please write your nginx server block configuration.");

    let temp_file = format!("/tmp/nginx_{}.conf", config_name);
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

    let _ = Command::new(&editor).arg(&temp_file).status();

    if std::path::Path::new(&temp_file).exists() {
        if let Ok(config) = fs::read_to_string(&temp_file) {
            save_nginx_config(&config_name, &config);
        }
        let _ = fs::remove_file(&temp_file);
    }
}

fn save_nginx_config(name: &str, config: &str) {
    let config_dir = "/etc/nginx/sites-available";
    let enabled_dir = "/etc/nginx/sites-enabled";
    let config_file = format!("{}/{}", config_dir, name);
    let enabled_file = format!("{}/{}", enabled_dir, name);

    println!("💾 Saving configuration: {}", config_file);

    // Create directories if they don't exist
    let _ = Command::new("sudo")
        .args(&["mkdir", "-p", config_dir, enabled_dir])
        .status();

    // Write configuration
    let _ = Command::new("sudo")
        .args(&["tee", &config_file])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(stdin) = child.stdin.as_mut() {
                let _ = stdin.write_all(config.as_bytes());
            }
            child.wait()
        });

    // Ask to enable the site
    let enable = Confirm::new()
        .with_prompt("Enable this site?")
        .default(true)
        .interact()
        .unwrap();

    if enable {
        let _ = Command::new("sudo")
            .args(&["ln", "-sf", &config_file, &enabled_file])
            .status();

        println!("✅ Site enabled");

        // Test configuration
        println!("🧪 Testing nginx configuration...");
        let test_result = Command::new("sudo").args(&["nginx", "-t"]).status();

        match test_result {
            Ok(status) if status.success() => {
                println!("✅ Configuration test passed");

                let reload = Confirm::new()
                    .with_prompt("Reload nginx?")
                    .default(true)
                    .interact()
                    .unwrap();

                if reload {
                    let _ = Command::new("sudo")
                        .args(&["systemctl", "reload", "nginx"])
                        .status();
                    println!("🔄 Nginx reloaded");
                }
            }
            _ => {
                println!("❌ Configuration test failed");
                println!("Check the configuration and run 'sudo nginx -t' for details");
            }
        }
    }
}
