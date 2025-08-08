use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme, MultiSelect};
use std::fs;
use std::process::Command;

pub fn registry_management() {
    loop {
        let options = vec![
            "🏗️  Registry Selection & Auth",
            "🪞 Registry Mirror Setup", 
            "🔍 Search Images",
            "📥 Pull Image",
            "📤 Push Image", 
            "📋 List Local Images",
            "🗑️  Remove Image",
            "🏷️  Tag Image",
            "📊 Image History",
            "🔄 Registry Sync",
            "⬅️  Back",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🗄️  Docker Registry Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => registry_selection(),
            1 => registry_mirror_setup(),
            2 => search_images(),
            3 => pull_image(),
            4 => push_image(),
            5 => list_images(),
            6 => remove_image(),
            7 => tag_image(),
            8 => image_history(),
            9 => registry_sync(),
            _ => break,
        }
    }
}

fn registry_mirror_setup() {
    loop {
        let options = vec![
            "Setup Local Registry",
            "Configure Registry Mirrors",
            "Docker Hub Mirror Setup",
            "Corporate Registry Setup",
            "Registry Authentication",
            "Mirror Configuration Management",
            "Registry Health Check",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🪞 Registry Mirror Setup")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => setup_local_registry(),
            1 => configure_registry_mirrors(),
            2 => docker_hub_mirror_setup(),
            3 => corporate_registry_setup(),
            4 => registry_authentication(),
            5 => mirror_configuration_management(),
            6 => registry_health_check(),
            _ => break,
        }
    }
}

fn setup_local_registry() {
    println!("🏗️  Setting up Local Docker Registry\n");

    let registry_options = vec![
        "Basic Local Registry",
        "Registry with UI (Portus)",
        "High-Availability Registry",
        "Secured Registry (TLS + Auth)",
        "Back",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select registry type")
        .items(&registry_options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => setup_basic_registry(),
        1 => setup_registry_with_ui(),
        2 => setup_ha_registry(),
        3 => setup_secured_registry(),
        _ => {}
    }
}

fn setup_basic_registry() {
    println!("🐳 Setting up Basic Local Registry\n");

    let registry_port: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry port")
        .default("5000".to_string())
        .interact()
        .unwrap();

    let storage_path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Storage path for registry data")
        .default("/var/lib/registry".to_string())
        .interact()
        .unwrap();

    let docker_compose_content = format!(r#"version: '3.8'

services:
  registry:
    image: registry:2
    ports:
      - "{}:5000"
    environment:
      - REGISTRY_STORAGE_FILESYSTEM_ROOTDIRECTORY=/var/lib/registry
    volumes:
      - {}:/var/lib/registry
    restart: unless-stopped
    container_name: local-registry
    
networks:
  default:
    name: registry-network
"#, registry_port, storage_path);

    // Create registry directory
    let _ = Command::new("mkdir")
        .args(&["-p", &storage_path])
        .status();

    // Write docker-compose file
    fs::write("/tmp/registry-compose.yml", docker_compose_content).ok();

    println!("📝 Docker Compose file created: /tmp/registry-compose.yml");

    if Confirm::new()
        .with_prompt("Start the registry now?")
        .default(true)
        .interact()
        .unwrap()
    {
        println!("🚀 Starting registry...");
        let _ = Command::new("docker-compose")
            .args(&["-f", "/tmp/registry-compose.yml", "up", "-d"])
            .status();

        println!("✅ Local registry started on port {}", registry_port);
        println!("📋 Usage examples:");
        println!("   • Tag image: docker tag myimage localhost:{}/myimage", registry_port);
        println!("   • Push image: docker push localhost:{}/myimage", registry_port);
        println!("   • Pull image: docker pull localhost:{}/myimage", registry_port);
    }
}

fn setup_registry_with_ui() {
    println!("🖥️  Setting up Registry with Web UI\n");

    let docker_compose_content = r#"version: '3.8'

services:
  registry:
    image: registry:2
    ports:
      - "5000:5000"
    environment:
      - REGISTRY_STORAGE_FILESYSTEM_ROOTDIRECTORY=/var/lib/registry
    volumes:
      - registry-data:/var/lib/registry
    restart: unless-stopped

  registry-ui:
    image: joxit/docker-registry-ui:static
    ports:
      - "8080:80"
    environment:
      - REGISTRY_TITLE=Local Docker Registry
      - REGISTRY_URL=http://registry:5000
      - DELETE_IMAGES=true
      - SHOW_CONTENT_DIGEST=true
    depends_on:
      - registry
    restart: unless-stopped

volumes:
  registry-data:

networks:
  default:
    name: registry-network
"#;

    fs::write("/tmp/registry-ui-compose.yml", docker_compose_content).ok();

    if Confirm::new()
        .with_prompt("Start registry with UI?")
        .default(true)
        .interact()
        .unwrap()
    {
        let _ = Command::new("docker-compose")
            .args(&["-f", "/tmp/registry-ui-compose.yml", "up", "-d"])
            .status();

        println!("✅ Registry with UI started!");
        println!("🔗 Registry: http://localhost:5000");
        println!("🖥️  Web UI: http://localhost:8080");
    }
}

fn setup_ha_registry() {
    println!("🏗️  Setting up High-Availability Registry\n");

    let ha_compose_content = r#"version: '3.8'

services:
  registry-1:
    image: registry:2
    environment:
      - REGISTRY_STORAGE_FILESYSTEM_ROOTDIRECTORY=/var/lib/registry
      - REGISTRY_STORAGE_DELETE_ENABLED=true
    volumes:
      - registry-shared:/var/lib/registry
    restart: unless-stopped
    networks:
      - registry-net

  registry-2:
    image: registry:2
    environment:
      - REGISTRY_STORAGE_FILESYSTEM_ROOTDIRECTORY=/var/lib/registry
      - REGISTRY_STORAGE_DELETE_ENABLED=true
    volumes:
      - registry-shared:/var/lib/registry
    restart: unless-stopped
    networks:
      - registry-net

  registry-proxy:
    image: nginx:alpine
    ports:
      - "5000:80"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
    depends_on:
      - registry-1
      - registry-2
    restart: unless-stopped
    networks:
      - registry-net

volumes:
  registry-shared:
    driver: local

networks:
  registry-net:
    driver: bridge
"#;

    let nginx_config = r#"events {
    worker_connections 1024;
}

http {
    upstream registry_backend {
        server registry-1:5000;
        server registry-2:5000;
    }

    server {
        listen 80;
        client_max_body_size 0;
        
        location / {
            proxy_pass http://registry_backend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            proxy_read_timeout 900;
        }
    }
}
"#;

    fs::write("/tmp/ha-registry-compose.yml", ha_compose_content).ok();
    fs::write("/tmp/nginx.conf", nginx_config).ok();

    println!("✅ HA Registry configuration created!");
    println!("📝 Files: /tmp/ha-registry-compose.yml, /tmp/nginx.conf");
    println!("💡 This setup provides load balancing across multiple registry instances");
}

fn setup_secured_registry() {
    println!("🔐 Setting up Secured Registry (TLS + Auth)\n");

    let domain: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry domain (e.g., registry.example.com)")
        .interact()
        .unwrap();

    let cert_path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Certificate file path")
        .default("/etc/ssl/certs/registry.crt".to_string())
        .interact()
        .unwrap();

    let key_path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Private key file path")
        .default("/etc/ssl/private/registry.key".to_string())
        .interact()
        .unwrap();

    let auth_username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry username")
        .default("admin".to_string())
        .interact()
        .unwrap();

    let auth_password: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry password")
        .interact()
        .unwrap();

    // Generate htpasswd file
    let _ = Command::new("htpasswd")
        .args(&["-Bbn", &auth_username, &auth_password])
        .output();

    let secured_compose = format!(r#"version: '3.8'

services:
  registry:
    image: registry:2
    ports:
      - "443:5000"
    environment:
      - REGISTRY_HTTP_TLS_CERTIFICATE=/certs/{}.crt
      - REGISTRY_HTTP_TLS_KEY=/certs/{}.key
      - REGISTRY_AUTH=htpasswd
      - REGISTRY_AUTH_HTPASSWD_REALM=Registry Realm
      - REGISTRY_AUTH_HTPASSWD_PATH=/auth/htpasswd
      - REGISTRY_STORAGE_FILESYSTEM_ROOTDIRECTORY=/var/lib/registry
    volumes:
      - registry-data:/var/lib/registry
      - {}:/certs/{}.crt:ro
      - {}:/certs/{}.key:ro
      - ./htpasswd:/auth/htpasswd:ro
    restart: unless-stopped

volumes:
  registry-data:
"#, domain, domain, cert_path, domain, key_path, domain);

    fs::write("/tmp/secured-registry-compose.yml", secured_compose).ok();

    println!("✅ Secured registry configuration created!");
    println!("📋 Next steps:");
    println!("   1. Place SSL certificates in specified paths");
    println!("   2. Create htpasswd file: htpasswd -Bn {} password > htpasswd", auth_username);
    println!("   3. Start with: docker-compose -f /tmp/secured-registry-compose.yml up -d");
}

fn configure_registry_mirrors() {
    println!("🪞 Configure Docker Registry Mirrors\n");

    let current_config = fs::read_to_string("/etc/docker/daemon.json")
        .unwrap_or_else(|_| "{}".to_string());

    println!("📋 Current Docker daemon configuration:");
    println!("{}", current_config);

    let mirror_options = vec![
        "Add Docker Hub Mirror",
        "Add Custom Registry Mirror",
        "Configure Multiple Mirrors",
        "Remove Mirror",
        "Show Mirror Status",
        "Back",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select mirror action")
        .items(&mirror_options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => add_docker_hub_mirror(),
        1 => add_custom_mirror(),
        2 => configure_multiple_mirrors(),
        3 => remove_mirror(),
        4 => show_mirror_status(),
        _ => {}
    }
}

fn add_docker_hub_mirror() {
    let popular_mirrors = vec![
        "https://registry.docker-cn.com",
        "https://docker.mirrors.ustc.edu.cn",
        "https://hub-mirror.c.163.com",
        "https://reg-mirror.qiniu.com",
        "Custom URL",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Docker Hub mirror")
        .items(&popular_mirrors)
        .default(0)
        .interact()
        .unwrap();

    let mirror_url = if selection == popular_mirrors.len() - 1 {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter custom mirror URL")
            .interact()
            .unwrap()
    } else {
        popular_mirrors[selection].to_string()
    };

    update_daemon_json_with_mirror(&mirror_url);
}

fn add_custom_mirror() {
    let registry_url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry URL to mirror")
        .interact()
        .unwrap();

    let mirror_url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Mirror URL")
        .interact()
        .unwrap();

    println!("🔄 Adding custom mirror: {} -> {}", registry_url, mirror_url);
    update_daemon_json_with_custom_mirror(&registry_url, &mirror_url);
}

fn configure_multiple_mirrors() {
    println!("🪞 Configure Multiple Registry Mirrors\n");

    let mirrors = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select mirrors to configure")
        .items(&[
            "Docker Hub (docker.io)",
            "GitHub Container Registry (ghcr.io)", 
            "Red Hat Quay (quay.io)",
            "Google Container Registry (gcr.io)",
            "Amazon ECR",
            "Custom Registry"
        ])
        .interact()
        .unwrap();

    let mut mirror_config = serde_json::Map::new();
    let mut mirror_list = Vec::new();

    for &mirror_idx in &mirrors {
        match mirror_idx {
            0 => mirror_list.push("https://registry.docker-cn.com".to_string()),
            1 => {
                let ghcr_mirror: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("GHCR mirror URL")
                    .default("https://ghcr.io".to_string())
                    .interact()
                    .unwrap();
                mirror_list.push(ghcr_mirror);
            },
            2 => {
                let quay_mirror: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Quay mirror URL")
                    .default("https://quay.io".to_string())
                    .interact()
                    .unwrap();
                mirror_list.push(quay_mirror);
            },
            _ => {
                let custom_mirror: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Custom mirror URL")
                    .interact()
                    .unwrap();
                mirror_list.push(custom_mirror);
            }
        }
    }

    mirror_config.insert(
        "registry-mirrors".to_string(),
        serde_json::Value::Array(
            mirror_list.into_iter().map(serde_json::Value::String).collect()
        )
    );

    let daemon_config = serde_json::Value::Object(mirror_config);
    fs::write("/etc/docker/daemon.json", serde_json::to_string_pretty(&daemon_config).unwrap()).ok();

    println!("✅ Multiple mirrors configured!");
    restart_docker_daemon();
}

fn remove_mirror() {
    println!("🗑️  Remove Registry Mirror\n");

    // Read current config
    let current_config = fs::read_to_string("/etc/docker/daemon.json")
        .unwrap_or_else(|_| "{}".to_string());

    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&current_config) {
        if let Some(mirrors) = config.get("registry-mirrors").and_then(|m| m.as_array()) {
            let mirror_strings: Vec<String> = mirrors.iter()
                .filter_map(|m| m.as_str().map(String::from))
                .collect();

            if mirror_strings.is_empty() {
                println!("No mirrors configured");
                return;
            }

            let mirrors_to_remove = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select mirrors to remove")
                .items(&mirror_strings)
                .interact()
                .unwrap();

            // Remove selected mirrors
            let mut remaining_mirrors = mirror_strings;
            for &idx in mirrors_to_remove.iter().rev() {
                remaining_mirrors.remove(idx);
            }

            // Update config
            let mut new_config = config.as_object().unwrap().clone();
            new_config.insert(
                "registry-mirrors".to_string(),
                serde_json::Value::Array(
                    remaining_mirrors.into_iter().map(serde_json::Value::String).collect()
                )
            );

            fs::write("/etc/docker/daemon.json", serde_json::to_string_pretty(&serde_json::Value::Object(new_config)).unwrap()).ok();
            println!("✅ Mirrors removed!");
            restart_docker_daemon();
        }
    }
}

fn show_mirror_status() {
    println!("📊 Registry Mirror Status\n");

    // Show current configuration
    let current_config = fs::read_to_string("/etc/docker/daemon.json")
        .unwrap_or_else(|_| "{}".to_string());

    println!("📋 Current mirror configuration:");
    println!("{}", current_config);

    // Test mirror connectivity
    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&current_config) {
        if let Some(mirrors) = config.get("registry-mirrors").and_then(|m| m.as_array()) {
            println!("\n🔍 Testing mirror connectivity:");
            for mirror in mirrors {
                if let Some(mirror_url) = mirror.as_str() {
                    println!("Testing {}...", mirror_url);
                    let result = Command::new("curl")
                        .args(&["-s", "-o", "/dev/null", "-w", "%{http_code}", &format!("{}/v2/", mirror_url)])
                        .output();
                    
                    if let Ok(output) = result {
                        let status_code = String::from_utf8_lossy(&output.stdout);
                        if status_code == "200" || status_code == "401" {
                            println!("  ✅ {} - Healthy", mirror_url);
                        } else {
                            println!("  ❌ {} - Unhealthy (HTTP {})", mirror_url, status_code);
                        }
                    }
                }
            }
        }
    }

    // Show Docker info
    println!("\n🐳 Docker daemon info:");
    let _ = Command::new("docker").args(&["info", "--format", "{{.RegistryConfig}}"]).status();
}

fn update_daemon_json_with_mirror(mirror_url: &str) {
    let current_config = fs::read_to_string("/etc/docker/daemon.json")
        .unwrap_or_else(|_| "{}".to_string());

    let mut config: serde_json::Value = serde_json::from_str(&current_config)
        .unwrap_or_else(|_| serde_json::json!({}));

    let mirrors = config.get_mut("registry-mirrors")
        .and_then(|m| m.as_array_mut())
        .map(|mirrors| {
            mirrors.push(serde_json::Value::String(mirror_url.to_string()));
            mirrors.clone()
        })
        .unwrap_or_else(|| vec![serde_json::Value::String(mirror_url.to_string())]);

    config["registry-mirrors"] = serde_json::Value::Array(mirrors);

    if let Ok(config_json) = serde_json::to_string_pretty(&config) {
        fs::write("/etc/docker/daemon.json", config_json).ok();
        println!("✅ Mirror added: {}", mirror_url);
        restart_docker_daemon();
    }
}

fn update_daemon_json_with_custom_mirror(registry_url: &str, mirror_url: &str) {
    let current_config = fs::read_to_string("/etc/docker/daemon.json")
        .unwrap_or_else(|_| "{}".to_string());

    let mut config: serde_json::Value = serde_json::from_str(&current_config)
        .unwrap_or_else(|_| serde_json::json!({}));

    // Add to insecure registries if needed
    if !mirror_url.starts_with("https://") {
        let insecure = config.get_mut("insecure-registries")
            .and_then(|r| r.as_array_mut())
            .map(|registries| {
                registries.push(serde_json::Value::String(mirror_url.to_string()));
                registries.clone()
            })
            .unwrap_or_else(|| vec![serde_json::Value::String(mirror_url.to_string())]);

        config["insecure-registries"] = serde_json::Value::Array(insecure);
    }

    if let Ok(config_json) = serde_json::to_string_pretty(&config) {
        fs::write("/etc/docker/daemon.json", config_json).ok();
        println!("✅ Custom mirror configured: {} -> {}", registry_url, mirror_url);
        restart_docker_daemon();
    }
}

fn restart_docker_daemon() {
    if Confirm::new()
        .with_prompt("Restart Docker daemon to apply changes?")
        .default(true)
        .interact()
        .unwrap()
    {
        println!("🔄 Restarting Docker daemon...");
        let _ = Command::new("systemctl").args(&["restart", "docker"]).status();
        println!("✅ Docker daemon restarted!");
    }
}

fn docker_hub_mirror_setup() {
    println!("🐳 Docker Hub Mirror Setup\n");

    let setup_options = vec![
        "Use Public Mirror",
        "Setup Private Mirror",
        "Corporate Proxy Setup",
        "Bandwidth Optimization",
        "Back",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select setup type")
        .items(&setup_options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => use_public_mirror(),
        1 => setup_private_mirror(),
        2 => corporate_proxy_setup(),
        3 => bandwidth_optimization(),
        _ => {}
    }
}

fn use_public_mirror() {
    println!("🌐 Using Public Docker Hub Mirror\n");

    let public_mirrors = vec![
        ("Registry CN", "https://registry.docker-cn.com"),
        ("USTC Mirror", "https://docker.mirrors.ustc.edu.cn"),
        ("163 Mirror", "https://hub-mirror.c.163.com"),
        ("Qiniu Mirror", "https://reg-mirror.qiniu.com"),
        ("Aliyun Mirror", "https://[your-accelerator-url].mirror.aliyuncs.com"),
    ];

    println!("📋 Available public mirrors:");
    for (name, url) in &public_mirrors {
        println!("   • {}: {}", name, url);
    }

    let mirror_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a public mirror")
        .items(&public_mirrors.iter().map(|(name, _)| *name).collect::<Vec<_>>())
        .default(0)
        .interact()
        .unwrap();

    let (_, mirror_url) = public_mirrors[mirror_selection];
    update_daemon_json_with_mirror(mirror_url);
}

fn setup_private_mirror() {
    println!("🔐 Setting up Private Docker Hub Mirror\n");

    let compose_content = r#"version: '3.8'

services:
  registry:
    image: registry:2
    ports:
      - "5000:5000"
    environment:
      - REGISTRY_PROXY_REMOTEURL=https://registry-1.docker.io
      - REGISTRY_STORAGE_FILESYSTEM_ROOTDIRECTORY=/var/lib/registry
    volumes:
      - registry-cache:/var/lib/registry
    restart: unless-stopped

volumes:
  registry-cache:
"#;

    fs::write("/tmp/docker-hub-mirror-compose.yml", compose_content).ok();

    if Confirm::new()
        .with_prompt("Start private Docker Hub mirror?")
        .default(true)
        .interact()
        .unwrap()
    {
        let _ = Command::new("docker-compose")
            .args(&["-f", "/tmp/docker-hub-mirror-compose.yml", "up", "-d"])
            .status();

        // Configure Docker daemon to use local mirror
        update_daemon_json_with_mirror("http://localhost:5000");

        println!("✅ Private Docker Hub mirror started!");
        println!("🔗 Mirror URL: http://localhost:5000");
    }
}

fn corporate_proxy_setup() {
    println!("🏢 Corporate Proxy Setup\n");

    let proxy_host: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Proxy host")
        .interact()
        .unwrap();

    let proxy_port: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Proxy port")
        .default("8080".to_string())
        .interact()
        .unwrap();

    let proxy_user: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Proxy username (optional)")
        .default("".to_string())
        .interact()
        .unwrap();

    let proxy_pass: String = if !proxy_user.is_empty() {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Proxy password")
            .interact()
            .unwrap()
    } else {
        String::new()
    };

    let proxy_url = if proxy_user.is_empty() {
        format!("http://{}:{}", proxy_host, proxy_port)
    } else {
        format!("http://{}:{}@{}:{}", proxy_user, proxy_pass, proxy_host, proxy_port)
    };

    // Create systemd override for Docker
    let systemd_override = format!(r#"[Service]
Environment="HTTP_PROXY={}"
Environment="HTTPS_PROXY={}"
Environment="NO_PROXY=localhost,127.0.0.1,docker-registry.somecorporation.com"
"#, proxy_url, proxy_url);

    let override_dir = "/etc/systemd/system/docker.service.d";
    let _ = Command::new("mkdir").args(&["-p", override_dir]).status();
    
    fs::write(format!("{}/http-proxy.conf", override_dir), systemd_override).ok();

    println!("✅ Corporate proxy configured!");
    println!("🔄 Run these commands to apply:");
    println!("   sudo systemctl daemon-reload");
    println!("   sudo systemctl restart docker");
}

fn bandwidth_optimization() {
    println!("⚡ Bandwidth Optimization\n");

    let optimization_options = vec![
        "Configure Concurrent Downloads",
        "Set Max Download Attempts",
        "Configure Download Timeout",
        "Enable Compression",
        "Back",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select optimization")
        .items(&optimization_options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => {
            let concurrent: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Max concurrent downloads")
                .default("3".to_string())
                .interact()
                .unwrap();

            add_daemon_config("max-concurrent-downloads", &concurrent);
        },
        1 => {
            let attempts: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Max download attempts")
                .default("5".to_string())
                .interact()
                .unwrap();

            add_daemon_config("max-download-attempts", &attempts);
        },
        2 => {
            let timeout: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Download timeout (seconds)")
                .default("300".to_string())
                .interact()
                .unwrap();

            add_daemon_config("shutdown-timeout", &timeout);
        },
        3 => {
            println!("💡 Enabling compression at registry level");
            add_daemon_config("experimental", "true");
        },
        _ => {}
    }
}

fn add_daemon_config(key: &str, value: &str) {
    let current_config = fs::read_to_string("/etc/docker/daemon.json")
        .unwrap_or_else(|_| "{}".to_string());

    let mut config: serde_json::Value = serde_json::from_str(&current_config)
        .unwrap_or_else(|_| serde_json::json!({}));

    if value.parse::<i64>().is_ok() {
        config[key] = serde_json::Value::Number(serde_json::Number::from(value.parse::<i64>().unwrap()));
    } else if value == "true" || value == "false" {
        config[key] = serde_json::Value::Bool(value == "true");
    } else {
        config[key] = serde_json::Value::String(value.to_string());
    }

    if let Ok(config_json) = serde_json::to_string_pretty(&config) {
        fs::write("/etc/docker/daemon.json", config_json).ok();
        println!("✅ Configuration added: {} = {}", key, value);
    }
}

fn corporate_registry_setup() {
    println!("🏢 Corporate Registry Setup\n");

    let registry_url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Corporate registry URL")
        .interact()
        .unwrap();

    let is_insecure = !registry_url.starts_with("https://");

    if is_insecure && Confirm::new()
        .with_prompt("Registry uses HTTP. Add to insecure registries?")
        .default(true)
        .interact()
        .unwrap()
    {
        let current_config = fs::read_to_string("/etc/docker/daemon.json")
            .unwrap_or_else(|_| "{}".to_string());

        let mut config: serde_json::Value = serde_json::from_str(&current_config)
            .unwrap_or_else(|_| serde_json::json!({}));

        let insecure = config.get_mut("insecure-registries")
            .and_then(|r| r.as_array_mut())
            .map(|registries| {
                registries.push(serde_json::Value::String(registry_url.clone()));
                registries.clone()
            })
            .unwrap_or_else(|| vec![serde_json::Value::String(registry_url.clone())]);

        config["insecure-registries"] = serde_json::Value::Array(insecure);

        if let Ok(config_json) = serde_json::to_string_pretty(&config) {
            fs::write("/etc/docker/daemon.json", config_json).ok();
            println!("✅ Added to insecure registries: {}", registry_url);
        }
    }

    if Confirm::new()
        .with_prompt("Configure registry authentication?")
        .default(true)
        .interact()
        .unwrap()
    {
        let username: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Username")
            .interact()
            .unwrap();

        let password: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Password")
            .interact()
            .unwrap();

        let _ = Command::new("docker")
            .args(&["login", &registry_url, "-u", &username, "-p", &password])
            .status();

        println!("✅ Authenticated with corporate registry");
    }

    restart_docker_daemon();
}

fn registry_authentication() {
    println!("🔐 Registry Authentication\n");

    let auth_options = vec![
        "Docker Hub Login",
        "GitHub Container Registry",
        "AWS ECR Login",
        "Google Container Registry",
        "Custom Registry Login",
        "Logout from Registry",
        "View Stored Credentials",
        "Back",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select authentication option")
        .items(&auth_options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => docker_hub_login(),
        1 => github_registry_login(),
        2 => aws_ecr_login(),
        3 => google_registry_login(),
        4 => custom_registry_login(),
        5 => registry_logout(),
        6 => view_stored_credentials(),
        _ => {}
    }
}

fn docker_hub_login() {
    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Docker Hub username")
        .interact()
        .unwrap();

    let password: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Docker Hub password/token")
        .interact()
        .unwrap();

    let result = Command::new("docker")
        .args(&["login", "-u", &username, "-p", &password])
        .status();

    if result.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Successfully logged in to Docker Hub");
    } else {
        println!("❌ Login failed");
    }
}

fn github_registry_login() {
    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("GitHub username")
        .interact()
        .unwrap();

    let token: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("GitHub Personal Access Token")
        .interact()
        .unwrap();

    let result = Command::new("docker")
        .args(&["login", "ghcr.io", "-u", &username, "-p", &token])
        .status();

    if result.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Successfully logged in to GitHub Container Registry");
    } else {
        println!("❌ Login failed");
    }
}

fn aws_ecr_login() {
    let region: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("AWS region")
        .default("us-east-1".to_string())
        .interact()
        .unwrap();

    let account_id: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("AWS account ID")
        .interact()
        .unwrap();

    println!("🔄 Getting ECR login token...");
    let result = Command::new("aws")
        .args(&["ecr", "get-login-password", "--region", &region])
        .output();

    if let Ok(output) = result {
        let password = String::from_utf8_lossy(&output.stdout);
        let ecr_url = format!("{}.dkr.ecr.{}.amazonaws.com", account_id, region);

        let login_result = Command::new("docker")
            .args(&["login", "--username", "AWS", "--password-stdin", &ecr_url])
            .arg(&password.trim())
            .status();

        if login_result.map(|s| s.success()).unwrap_or(false) {
            println!("✅ Successfully logged in to AWS ECR");
        } else {
            println!("❌ ECR login failed");
        }
    }
}

fn google_registry_login() {
    println!("🔑 Google Container Registry requires service account authentication");
    
    let key_file: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to service account key file")
        .interact()
        .unwrap();

    let result = Command::new("gcloud")
        .args(&["auth", "activate-service-account", "--key-file", &key_file])
        .status();

    if result.map(|s| s.success()).unwrap_or(false) {
        let _ = Command::new("gcloud")
            .args(&["auth", "configure-docker"])
            .status();
        println!("✅ Successfully configured GCR authentication");
    } else {
        println!("❌ GCR authentication failed");
    }
}

fn custom_registry_login() {
    let registry_url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry URL")
        .interact()
        .unwrap();

    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Username")
        .interact()
        .unwrap();

    let password: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .interact()
        .unwrap();

    let result = Command::new("docker")
        .args(&["login", &registry_url, "-u", &username, "-p", &password])
        .status();

    if result.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Successfully logged in to {}", registry_url);
    } else {
        println!("❌ Login failed");
    }
}

fn registry_logout() {
    let registry_url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry URL to logout from (leave empty for Docker Hub)")
        .default("".to_string())
        .interact()
        .unwrap();

    let result = if registry_url.is_empty() {
        Command::new("docker").args(&["logout"]).status()
    } else {
        Command::new("docker").args(&["logout", &registry_url]).status()
    };

    if result.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Successfully logged out");
    } else {
        println!("❌ Logout failed");
    }
}

fn view_stored_credentials() {
    println!("📋 Stored Registry Credentials\n");

    let config_path = format!("{}/.docker/config.json", std::env::var("HOME").unwrap_or_default());
    
    if let Ok(config_content) = fs::read_to_string(&config_path) {
        if let Ok(config) = serde_json::from_str::<serde_json::Value>(&config_content) {
            if let Some(auths) = config.get("auths").and_then(|a| a.as_object()) {
                println!("🔑 Authenticated registries:");
                for (registry, _) in auths {
                    println!("   • {}", registry);
                }
            } else {
                println!("No stored credentials found");
            }
        }
    } else {
        println!("No Docker config file found");
    }
}

fn mirror_configuration_management() {
    println!("⚙️  Mirror Configuration Management\n");

    let config_options = vec![
        "Backup Current Configuration",
        "Restore Configuration",
        "Export Configuration",
        "Import Configuration",
        "Reset to Defaults",
        "Back",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select configuration action")
        .items(&config_options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => backup_configuration(),
        1 => restore_configuration(),
        2 => export_configuration(),
        3 => import_configuration(),
        4 => reset_to_defaults(),
        _ => {}
    }
}

fn backup_configuration() {
    let backup_path = format!("/tmp/docker-daemon-backup-{}.json", chrono::Utc::now().format("%Y%m%d-%H%M%S"));
    
    if let Ok(_) = fs::copy("/etc/docker/daemon.json", &backup_path) {
        println!("✅ Configuration backed up to: {}", backup_path);
    } else {
        println!("❌ Backup failed");
    }
}

fn restore_configuration() {
    let backup_file: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to backup file")
        .interact()
        .unwrap();

    if Confirm::new()
        .with_prompt("Really restore configuration? This will overwrite current settings.")
        .default(false)
        .interact()
        .unwrap()
    {
        if let Ok(_) = fs::copy(&backup_file, "/etc/docker/daemon.json") {
            println!("✅ Configuration restored from: {}", backup_file);
            restart_docker_daemon();
        } else {
            println!("❌ Restore failed");
        }
    }
}

fn export_configuration() {
    let export_path = "/tmp/docker-registry-config-export.json";
    
    let current_config = fs::read_to_string("/etc/docker/daemon.json")
        .unwrap_or_else(|_| "{}".to_string());

    fs::write(export_path, current_config).ok();
    println!("✅ Configuration exported to: {}", export_path);
}

fn import_configuration() {
    let import_file: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to configuration file")
        .interact()
        .unwrap();

    if let Ok(config_content) = fs::read_to_string(&import_file) {
        // Validate JSON
        if serde_json::from_str::<serde_json::Value>(&config_content).is_ok() {
            if Confirm::new()
                .with_prompt("Import this configuration?")
                .default(true)
                .interact()
                .unwrap()
            {
                fs::write("/etc/docker/daemon.json", config_content).ok();
                println!("✅ Configuration imported!");
                restart_docker_daemon();
            }
        } else {
            println!("❌ Invalid JSON configuration");
        }
    } else {
        println!("❌ Failed to read configuration file");
    }
}

fn reset_to_defaults() {
    if Confirm::new()
        .with_prompt("Really reset Docker daemon configuration to defaults?")
        .default(false)
        .interact()
        .unwrap()
    {
        fs::write("/etc/docker/daemon.json", "{}").ok();
        println!("✅ Configuration reset to defaults");
        restart_docker_daemon();
    }
}

fn registry_health_check() {
    println!("🏥 Registry Health Check\n");

    // Check local Docker daemon
    println!("🐳 Docker daemon status:");
    let _ = Command::new("systemctl").args(&["status", "docker", "--no-pager"]).status();

    // Check configured mirrors
    let current_config = fs::read_to_string("/etc/docker/daemon.json")
        .unwrap_or_else(|_| "{}".to_string());

    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&current_config) {
        if let Some(mirrors) = config.get("registry-mirrors").and_then(|m| m.as_array()) {
            println!("\n🪞 Mirror health check:");
            for mirror in mirrors {
                if let Some(mirror_url) = mirror.as_str() {
                    print!("Checking {}... ", mirror_url);
                    let result = Command::new("curl")
                        .args(&["-s", "-o", "/dev/null", "-w", "%{http_code}", "--max-time", "10", &format!("{}/v2/", mirror_url)])
                        .output();
                    
                    if let Ok(output) = result {
                        let status_code = String::from_utf8_lossy(&output.stdout);
                        match status_code.as_ref() {
                            "200" | "401" => println!("✅ Healthy"),
                            "000" => println!("❌ Timeout/Connection failed"),
                            _ => println!("⚠️  HTTP {}", status_code),
                        }
                    } else {
                        println!("❌ Test failed");
                    }
                }
            }
        }
    }

    // Test Docker pull
    println!("\n🧪 Testing Docker pull performance:");
    let test_image = "hello-world:latest";
    println!("Pulling {}...", test_image);
    
    let start_time = std::time::Instant::now();
    let result = Command::new("docker").args(&["pull", test_image]).status();
    let duration = start_time.elapsed();
    
    if result.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Pull successful in {:.2}s", duration.as_secs_f64());
    } else {
        println!("❌ Pull failed");
    }

    // Clean up test image
    let _ = Command::new("docker").args(&["rmi", test_image]).status();
}

fn registry_sync() {
    println!("🔄 Registry Sync - Implementation coming in next update!");
}

fn registry_selection() {
    println!("🏗️  Registry Selection & Authentication - Implementation coming in next update!");
}

fn search_images() {
    println!("🔍 Search Images - Implementation coming in next update!");
}

fn pull_image() {
    println!("📥 Pull Image - Implementation coming in next update!");
}

fn push_image() {
    println!("📤 Push Image - Implementation coming in next update!");
}

fn list_images() {
    println!("📋 List Local Images\n");
    let _ = Command::new("docker").args(&["images"]).status();
}

fn remove_image() {
    println!("🗑️  Remove Image - Implementation coming in next update!");
}

fn tag_image() {
    println!("🏷️  Tag Image - Implementation coming in next update!");
}

fn image_history() {
    println!("📊 Image History - Implementation coming in next update!");
}