use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::process::Command;

pub fn reverse_proxy_setup() {
    println!("🔄 Nginx Reverse Proxy Setup");
    println!("============================");

    let proxy_options = [
        "🌐 Basic reverse proxy",
        "⚖️  Load balancer setup",
        "🔒 SSL termination proxy",
        "🎯 API gateway configuration",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Proxy type")
        .items(&proxy_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => setup_basic_proxy(),
        1 => setup_load_balancer(),
        2 => setup_ssl_termination(),
        3 => setup_api_gateway(),
        _ => return,
    }
}

fn setup_basic_proxy() {
    println!("🌐 Basic Reverse Proxy Setup");
    println!("============================");

    let frontend_domain: String = Input::new()
        .with_prompt("Frontend domain")
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
        
        # Timeouts
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }}
    
    # Health check endpoint
    location /nginx-health {{
        access_log off;
        return 200 "healthy\n";
        add_header Content-Type text/plain;
    }}
}}
"#,
        frontend_domain, backend_url
    );

    save_proxy_config(&format!("{}_proxy", frontend_domain), &config);
}

fn setup_load_balancer() {
    println!("⚖️  Load Balancer Setup");
    println!("======================");

    let upstream_name: String = Input::new()
        .with_prompt("Upstream name")
        .interact_text()
        .unwrap();

    let backend_count: String = Input::new()
        .with_prompt("Number of backend servers")
        .default("3".into())
        .interact_text()
        .unwrap();

    let count: usize = backend_count.parse().unwrap_or(3);
    let mut backends = Vec::new();

    for i in 1..=count {
        let backend: String = Input::new()
            .with_prompt(&format!("Backend {} (e.g., 192.168.1.{}:8080)", i, i))
            .interact_text()
            .unwrap();
        backends.push(backend);
    }

    let frontend_domain: String = Input::new()
        .with_prompt("Frontend domain")
        .interact_text()
        .unwrap();

    // Load balancing method
    let methods = ["round_robin", "least_conn", "ip_hash", "hash"];
    let method_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Load balancing method")
        .items(&methods)
        .default(0)
        .interact()
        .unwrap();

    let method_directive = match method_choice {
        1 => "least_conn;",
        2 => "ip_hash;",
        3 => "hash $request_uri consistent;",
        _ => "", // round_robin is default
    };

    let mut config = format!("upstream {} {{\n", upstream_name);
    if !method_directive.is_empty() {
        config.push_str(&format!("    {}\n", method_directive));
    }

    for backend in backends {
        config.push_str(&format!(
            "    server {} max_fails=3 fail_timeout=30s;\n",
            backend
        ));
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
        
        # Health checks
        proxy_next_upstream error timeout invalid_header http_500 http_502 http_503 http_504;
        proxy_next_upstream_tries 3;
        proxy_next_upstream_timeout 10s;
    }}
    
    # Load balancer status
    location /lb-status {{
        stub_status on;
        access_log off;
        allow 127.0.0.1;
        allow 10.0.0.0/8;
        allow 192.168.0.0/16;
        allow 172.16.0.0/12;
        deny all;
    }}
}}
"#,
        frontend_domain, upstream_name
    ));

    save_proxy_config(&format!("{}_lb", frontend_domain), &config);
}

fn setup_ssl_termination() {
    println!("🔒 SSL Termination Proxy");
    println!("========================");

    let frontend_domain: String = Input::new()
        .with_prompt("Frontend domain")
        .interact_text()
        .unwrap();

    let backend_url: String = Input::new()
        .with_prompt("Backend URL (http://...)")
        .interact_text()
        .unwrap();

    let cert_path: String = Input::new()
        .with_prompt("SSL certificate path")
        .default(format!(
            "/etc/letsencrypt/live/{}/fullchain.pem",
            frontend_domain
        ))
        .interact_text()
        .unwrap();

    let key_path: String = Input::new()
        .with_prompt("SSL private key path")
        .default(format!(
            "/etc/letsencrypt/live/{}/privkey.pem",
            frontend_domain
        ))
        .interact_text()
        .unwrap();

    let config = format!(
        r#"# HTTP to HTTPS redirect
server {{
    listen 80;
    server_name {};
    return 301 https://$server_name$request_uri;
}}

# HTTPS SSL termination
server {{
    listen 443 ssl http2;
    server_name {};
    
    # SSL configuration
    ssl_certificate {};
    ssl_certificate_key {};
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;
    
    # Security headers
    add_header Strict-Transport-Security "max-age=63072000" always;
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    
    location / {{
        proxy_pass {};
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto https;
        proxy_cache_bypass $http_upgrade;
        
        # Buffer settings
        proxy_buffering on;
        proxy_buffer_size 128k;
        proxy_buffers 4 256k;
        proxy_busy_buffers_size 256k;
    }}
}}
"#,
        frontend_domain, frontend_domain, cert_path, key_path, backend_url
    );

    save_proxy_config(&format!("{}_ssl", frontend_domain), &config);
}

fn setup_api_gateway() {
    println!("🎯 API Gateway Configuration");
    println!("============================");

    let gateway_domain: String = Input::new()
        .with_prompt("Gateway domain")
        .interact_text()
        .unwrap();

    let config = format!(
        r#"# Rate limiting
limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;

server {{
    listen 80;
    server_name {};
    
    # Global rate limiting
    limit_req zone=api burst=20 nodelay;
    
    # API v1 endpoints
    location /api/v1/users {{
        proxy_pass http://user-service:8001;
        include /etc/nginx/proxy_params;
    }}
    
    location /api/v1/orders {{
        proxy_pass http://order-service:8002;
        include /etc/nginx/proxy_params;
    }}
    
    location /api/v1/payments {{
        proxy_pass http://payment-service:8003;
        include /etc/nginx/proxy_params;
        
        # Stricter rate limiting for payments
        limit_req zone=api burst=5 nodelay;
    }}
    
    # Health check
    location /health {{
        access_log off;
        return 200 "OK";
        add_header Content-Type text/plain;
    }}
    
    # API documentation
    location /docs {{
        proxy_pass http://docs-service:8080;
        include /etc/nginx/proxy_params;
    }}
    
    # Default fallback
    location / {{
        return 404 "API endpoint not found";
        add_header Content-Type text/plain;
    }}
}}
"#,
        gateway_domain
    );

    // Create proxy_params file if it doesn't exist
    let proxy_params = r#"proxy_set_header Host $http_host;
proxy_set_header X-Real-IP $remote_addr;
proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
proxy_set_header X-Forwarded-Proto $scheme;
proxy_connect_timeout 30s;
proxy_send_timeout 30s;
proxy_read_timeout 30s;
"#;

    let _ = Command::new("sudo")
        .args(&["tee", "/etc/nginx/proxy_params"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(stdin) = child.stdin.as_mut() {
                let _ = stdin.write_all(proxy_params.as_bytes());
            }
            child.wait()
        });

    save_proxy_config(&format!("{}_gateway", gateway_domain), &config);
}

fn save_proxy_config(name: &str, config: &str) {
    let config_dir = "/etc/nginx/sites-available";
    let enabled_dir = "/etc/nginx/sites-enabled";
    let config_file = format!("{}/{}", config_dir, name);
    let enabled_file = format!("{}/{}", enabled_dir, name);

    println!("💾 Saving proxy configuration: {}", config_file);

    // Create directories
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

    // Enable site
    let _ = Command::new("sudo")
        .args(&["ln", "-sf", &config_file, &enabled_file])
        .status();

    // Test and reload
    println!("🧪 Testing nginx configuration...");
    let test_result = Command::new("sudo").args(&["nginx", "-t"]).status();

    match test_result {
        Ok(status) if status.success() => {
            println!("✅ Configuration test passed");
            let _ = Command::new("sudo")
                .args(&["systemctl", "reload", "nginx"])
                .status();
            println!("🔄 Nginx reloaded");
        }
        _ => {
            println!("❌ Configuration test failed");
        }
    }
}
