use dialoguer::{Confirm, Input, MultiSelect, Password, Select, theme::ColorfulTheme};
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

/// Securely log in to a Docker registry using password-stdin to avoid argv exposure.
/// Returns true on success, false on failure.
fn secure_docker_login(registry: Option<&str>, username: &str, password: &str) -> bool {
    let mut cmd = Command::new("docker");
    cmd.args(["login", "--username", username, "--password-stdin"]);

    if let Some(reg) = registry {
        cmd.arg(reg);
    }

    cmd.stdin(Stdio::piped());

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            println!("❌ Failed to start docker login: {}", e);
            return false;
        }
    };

    if let Some(ref mut stdin) = child.stdin {
        if stdin.write_all(password.as_bytes()).is_err() {
            println!("❌ Failed to send password to docker");
            return false;
        }
    }

    match child.wait() {
        Ok(status) => status.success(),
        Err(e) => {
            println!("❌ Docker login failed: {}", e);
            false
        }
    }
}

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

        let choice = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🗄️  Docker Registry Management")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(c)) => c,
            _ => break,
        };

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

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🪞 Registry Mirror Setup")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            _ => break,
        };

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

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select registry type")
        .items(&registry_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        _ => return,
    };

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

    let registry_port: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry port")
        .default("5000".to_string())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let storage_path: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Storage path for registry data")
        .default("/var/lib/registry".to_string())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let docker_compose_content = format!(
        r#"version: '3.8'

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
"#,
        registry_port, storage_path
    );

    // Create registry directory
    if let Err(e) = Command::new("mkdir").args(["-p", &storage_path]).status() {
        println!("⚠️  Could not create storage directory: {}", e);
    }

    // Write docker-compose file
    if let Err(e) = fs::write("/tmp/registry-compose.yml", docker_compose_content) {
        println!("❌ Failed to write compose file: {}", e);
        return;
    }

    println!("📝 Docker Compose file created: /tmp/registry-compose.yml");

    let start_registry = Confirm::new()
        .with_prompt("Start the registry now?")
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if start_registry {
        println!("🚀 Starting registry...");
        match Command::new("docker-compose")
            .args(["-f", "/tmp/registry-compose.yml", "up", "-d"])
            .status()
        {
            Ok(s) if s.success() => println!("✅ Local registry started on port {}", registry_port),
            Ok(_) => println!("⚠️  Registry may have started with warnings"),
            Err(e) => println!("❌ Failed to start registry: {}", e),
        }
        println!("📋 Usage examples:");
        println!(
            "   • Tag image: docker tag myimage localhost:{}/myimage",
            registry_port
        );
        println!(
            "   • Push image: docker push localhost:{}/myimage",
            registry_port
        );
        println!(
            "   • Pull image: docker pull localhost:{}/myimage",
            registry_port
        );
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

    if let Err(e) = fs::write("/tmp/registry-ui-compose.yml", docker_compose_content) {
        println!("❌ Failed to write compose file: {}", e);
        return;
    }

    let start_registry = Confirm::new()
        .with_prompt("Start registry with UI?")
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if start_registry {
        match Command::new("docker-compose")
            .args(["-f", "/tmp/registry-ui-compose.yml", "up", "-d"])
            .status()
        {
            Ok(s) if s.success() => {
                println!("✅ Registry with UI started!");
                println!("🔗 Registry: http://localhost:5000");
                println!("🖥️  Web UI: http://localhost:8080");
            }
            Ok(_) => println!("⚠️  Registry may have started with warnings"),
            Err(e) => println!("❌ Failed to start registry: {}", e),
        }
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

    if let Err(e) = fs::write("/tmp/ha-registry-compose.yml", ha_compose_content) {
        println!("❌ Failed to write compose file: {}", e);
        return;
    }
    if let Err(e) = fs::write("/tmp/nginx.conf", nginx_config) {
        println!("❌ Failed to write nginx config: {}", e);
        return;
    }

    println!("✅ HA Registry configuration created!");
    println!("📝 Files: /tmp/ha-registry-compose.yml, /tmp/nginx.conf");
    println!("💡 This setup provides load balancing across multiple registry instances");
}

fn setup_secured_registry() {
    println!("🔐 Setting up Secured Registry (TLS + Auth)\n");

    let domain: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry domain (e.g., registry.example.com)")
        .interact_text()
    {
        Ok(d) => d,
        Err(_) => return,
    };

    let cert_path: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Certificate file path")
        .default("/etc/ssl/certs/registry.crt".to_string())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let key_path: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Private key file path")
        .default("/etc/ssl/private/registry.key".to_string())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let auth_username: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry username")
        .default("admin".to_string())
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    let auth_password: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry password")
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    // Generate htpasswd file
    if let Err(e) = Command::new("htpasswd")
        .args(["-Bbn", &auth_username, &auth_password])
        .output()
    {
        println!("⚠️  Could not generate htpasswd: {}", e);
    }

    let secured_compose = format!(
        r#"version: '3.8'

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
"#,
        domain, domain, cert_path, domain, key_path, domain
    );

    if let Err(e) = fs::write("/tmp/secured-registry-compose.yml", secured_compose) {
        println!("❌ Failed to write compose file: {}", e);
        return;
    }

    println!("✅ Secured registry configuration created!");
    println!("📋 Next steps:");
    println!("   1. Place SSL certificates in specified paths");
    println!(
        "   2. Create htpasswd file: htpasswd -Bn {} password > htpasswd",
        auth_username
    );
    println!("   3. Start with: docker-compose -f /tmp/secured-registry-compose.yml up -d");
}

fn configure_registry_mirrors() {
    println!("🪞 Configure Docker Registry Mirrors\n");

    let current_config =
        fs::read_to_string("/etc/docker/daemon.json").unwrap_or_else(|_| "{}".to_string());

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

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select mirror action")
        .items(&mirror_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        _ => return,
    };

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

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Docker Hub mirror")
        .items(&popular_mirrors)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        _ => return,
    };

    let mirror_url = if selection == popular_mirrors.len() - 1 {
        match Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter custom mirror URL")
            .interact_text()
        {
            Ok(url) => url,
            Err(_) => return,
        }
    } else {
        popular_mirrors[selection].to_string()
    };

    update_daemon_json_with_mirror(&mirror_url);
}

fn add_custom_mirror() {
    let registry_url: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry URL to mirror")
        .interact_text()
    {
        Ok(url) => url,
        Err(_) => return,
    };

    let mirror_url: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Mirror URL")
        .interact_text()
    {
        Ok(url) => url,
        Err(_) => return,
    };

    println!(
        "🔄 Adding custom mirror: {} -> {}",
        registry_url, mirror_url
    );
    update_daemon_json_with_custom_mirror(&registry_url, &mirror_url);
}

fn configure_multiple_mirrors() {
    println!("🪞 Configure Multiple Registry Mirrors\n");

    let mirrors = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select mirrors to configure")
        .items(&[
            "Docker Hub (docker.io)",
            "GitHub Container Registry (ghcr.io)",
            "Red Hat Quay (quay.io)",
            "Google Container Registry (gcr.io)",
            "Amazon ECR",
            "Custom Registry",
        ])
        .interact_opt()
    {
        Ok(Some(m)) => m,
        _ => return,
    };

    let mut mirror_config = serde_json::Map::new();
    let mut mirror_list = Vec::new();

    for &mirror_idx in &mirrors {
        match mirror_idx {
            0 => mirror_list.push("https://registry.docker-cn.com".to_string()),
            1 => {
                let ghcr_mirror: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("GHCR mirror URL")
                    .default("https://ghcr.io".to_string())
                    .interact_text()
                {
                    Ok(url) => url,
                    Err(_) => continue,
                };
                mirror_list.push(ghcr_mirror);
            }
            2 => {
                let quay_mirror: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Quay mirror URL")
                    .default("https://quay.io".to_string())
                    .interact_text()
                {
                    Ok(url) => url,
                    Err(_) => continue,
                };
                mirror_list.push(quay_mirror);
            }
            _ => {
                let custom_mirror: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Custom mirror URL")
                    .interact_text()
                {
                    Ok(url) => url,
                    Err(_) => continue,
                };
                mirror_list.push(custom_mirror);
            }
        }
    }

    mirror_config.insert(
        "registry-mirrors".to_string(),
        serde_json::Value::Array(
            mirror_list
                .into_iter()
                .map(serde_json::Value::String)
                .collect(),
        ),
    );

    let daemon_config = serde_json::Value::Object(mirror_config);
    if let Ok(config_json) = serde_json::to_string_pretty(&daemon_config) {
        if let Err(e) = fs::write("/etc/docker/daemon.json", config_json) {
            println!("❌ Failed to write daemon config: {}", e);
            return;
        }
    }

    println!("✅ Multiple mirrors configured!");
    restart_docker_daemon();
}

fn remove_mirror() {
    println!("🗑️  Remove Registry Mirror\n");

    // Read current config
    let current_config =
        fs::read_to_string("/etc/docker/daemon.json").unwrap_or_else(|_| "{}".to_string());

    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&current_config)
        && let Some(mirrors) = config.get("registry-mirrors").and_then(|m| m.as_array())
    {
        let mirror_strings: Vec<String> = mirrors
            .iter()
            .filter_map(|m| m.as_str().map(String::from))
            .collect();

        if mirror_strings.is_empty() {
            println!("No mirrors configured");
            return;
        }

        let mirrors_to_remove = match MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select mirrors to remove")
            .items(&mirror_strings)
            .interact_opt()
        {
            Ok(Some(m)) => m,
            _ => return,
        };

        // Remove selected mirrors
        let mut remaining_mirrors = mirror_strings;
        for &idx in mirrors_to_remove.iter().rev() {
            remaining_mirrors.remove(idx);
        }

        // Update config
        if let Some(config_obj) = config.as_object() {
            let mut new_config = config_obj.clone();
            new_config.insert(
                "registry-mirrors".to_string(),
                serde_json::Value::Array(
                    remaining_mirrors
                        .into_iter()
                        .map(serde_json::Value::String)
                        .collect(),
                ),
            );

            if let Ok(config_json) =
                serde_json::to_string_pretty(&serde_json::Value::Object(new_config))
            {
                if let Err(e) = fs::write("/etc/docker/daemon.json", config_json) {
                    println!("❌ Failed to write daemon config: {}", e);
                    return;
                }
            }
            println!("✅ Mirrors removed!");
            restart_docker_daemon();
        }
    }
}

fn show_mirror_status() {
    println!("📊 Registry Mirror Status\n");

    // Show current configuration
    let current_config =
        fs::read_to_string("/etc/docker/daemon.json").unwrap_or_else(|_| "{}".to_string());

    println!("📋 Current mirror configuration:");
    println!("{}", current_config);

    // Test mirror connectivity
    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&current_config)
        && let Some(mirrors) = config.get("registry-mirrors").and_then(|m| m.as_array())
    {
        println!("\n🔍 Testing mirror connectivity:");
        for mirror in mirrors {
            if let Some(mirror_url) = mirror.as_str() {
                println!("Testing {}...", mirror_url);
                let result = Command::new("curl")
                    .args([
                        "-s",
                        "-o",
                        "/dev/null",
                        "-w",
                        "%{http_code}",
                        &format!("{}/v2/", mirror_url),
                    ])
                    .output();

                match result {
                    Ok(output) => {
                        let status_code = String::from_utf8_lossy(&output.stdout);
                        if status_code == "200" || status_code == "401" {
                            println!("  ✅ {} - Healthy", mirror_url);
                        } else {
                            println!("  ❌ {} - Unhealthy (HTTP {})", mirror_url, status_code);
                        }
                    }
                    Err(e) => println!("  ❌ {} - Test failed: {}", mirror_url, e),
                }
            }
        }
    }

    // Show Docker info
    println!("\n🐳 Docker daemon info:");
    if let Err(e) = Command::new("docker")
        .args(["info", "--format", "{{.RegistryConfig}}"])
        .status()
    {
        println!("  Could not get daemon info: {}", e);
    }
}

fn update_daemon_json_with_mirror(mirror_url: &str) {
    let current_config =
        fs::read_to_string("/etc/docker/daemon.json").unwrap_or_else(|_| "{}".to_string());

    let mut config: serde_json::Value =
        serde_json::from_str(&current_config).unwrap_or_else(|_| serde_json::json!({}));

    let mirrors = config
        .get_mut("registry-mirrors")
        .and_then(|m| m.as_array_mut())
        .map(|mirrors| {
            mirrors.push(serde_json::Value::String(mirror_url.to_string()));
            mirrors.clone()
        })
        .unwrap_or_else(|| vec![serde_json::Value::String(mirror_url.to_string())]);

    config["registry-mirrors"] = serde_json::Value::Array(mirrors);

    if let Ok(config_json) = serde_json::to_string_pretty(&config) {
        if let Err(e) = fs::write("/etc/docker/daemon.json", config_json) {
            println!("❌ Failed to write daemon config: {}", e);
            return;
        }
        println!("✅ Mirror added: {}", mirror_url);
        restart_docker_daemon();
    }
}

fn update_daemon_json_with_custom_mirror(registry_url: &str, mirror_url: &str) {
    let current_config =
        fs::read_to_string("/etc/docker/daemon.json").unwrap_or_else(|_| "{}".to_string());

    let mut config: serde_json::Value =
        serde_json::from_str(&current_config).unwrap_or_else(|_| serde_json::json!({}));

    // Add to insecure registries if needed
    if !mirror_url.starts_with("https://") {
        let insecure = config
            .get_mut("insecure-registries")
            .and_then(|r| r.as_array_mut())
            .map(|registries| {
                registries.push(serde_json::Value::String(mirror_url.to_string()));
                registries.clone()
            })
            .unwrap_or_else(|| vec![serde_json::Value::String(mirror_url.to_string())]);

        config["insecure-registries"] = serde_json::Value::Array(insecure);
    }

    if let Ok(config_json) = serde_json::to_string_pretty(&config) {
        if let Err(e) = fs::write("/etc/docker/daemon.json", config_json) {
            println!("❌ Failed to write daemon config: {}", e);
            return;
        }
        println!(
            "✅ Custom mirror configured: {} -> {}",
            registry_url, mirror_url
        );
        restart_docker_daemon();
    }
}

fn restart_docker_daemon() {
    let should_restart = Confirm::new()
        .with_prompt("Restart Docker daemon to apply changes?")
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if should_restart {
        println!("🔄 Restarting Docker daemon...");
        match Command::new("systemctl")
            .args(["restart", "docker"])
            .status()
        {
            Ok(s) if s.success() => println!("✅ Docker daemon restarted!"),
            Ok(_) => println!("⚠️  Docker restart returned non-zero exit"),
            Err(e) => println!("❌ Failed to restart Docker: {}", e),
        }
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

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select setup type")
        .items(&setup_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        _ => return,
    };

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
        (
            "Aliyun Mirror",
            "https://[your-accelerator-url].mirror.aliyuncs.com",
        ),
    ];

    println!("📋 Available public mirrors:");
    for (name, url) in &public_mirrors {
        println!("   • {}: {}", name, url);
    }

    let mirror_selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a public mirror")
        .items(
            &public_mirrors
                .iter()
                .map(|(name, _)| *name)
                .collect::<Vec<_>>(),
        )
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        _ => return,
    };

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

    let start_mirror = Confirm::new()
        .with_prompt("Start private Docker Hub mirror?")
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if start_mirror {
        match Command::new("docker-compose")
            .args(["-f", "/tmp/docker-hub-mirror-compose.yml", "up", "-d"])
            .status()
        {
            Ok(s) if s.success() => {
                // Configure Docker daemon to use local mirror
                update_daemon_json_with_mirror("http://localhost:5000");
                println!("✅ Private Docker Hub mirror started!");
                println!("🔗 Mirror URL: http://localhost:5000");
            }
            Ok(_) => println!("⚠️  Mirror may have started with warnings"),
            Err(e) => println!("❌ Failed to start mirror: {}", e),
        }
    }
}

fn corporate_proxy_setup() {
    println!("🏢 Corporate Proxy Setup\n");

    let proxy_host: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Proxy host")
        .interact_text()
    {
        Ok(h) => h,
        Err(_) => return,
    };

    let proxy_port: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Proxy port")
        .default("8080".to_string())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let proxy_user: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Proxy username (optional)")
        .default("".to_string())
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    let proxy_pass: String = if !proxy_user.is_empty() {
        match Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Proxy password")
            .interact_text()
        {
            Ok(p) => p,
            Err(_) => return,
        }
    } else {
        String::new()
    };

    let proxy_url = if proxy_user.is_empty() {
        format!("http://{}:{}", proxy_host, proxy_port)
    } else {
        format!(
            "http://{}:{}@{}:{}",
            proxy_user, proxy_pass, proxy_host, proxy_port
        )
    };

    // Create systemd override for Docker
    let systemd_override = format!(
        r#"[Service]
Environment="HTTP_PROXY={}"
Environment="HTTPS_PROXY={}"
Environment="NO_PROXY=localhost,127.0.0.1,docker-registry.somecorporation.com"
"#,
        proxy_url, proxy_url
    );

    let override_dir = "/etc/systemd/system/docker.service.d";
    if let Err(e) = Command::new("mkdir").args(["-p", override_dir]).status() {
        println!("⚠️  Could not create override directory: {}", e);
    }

    if let Err(e) = fs::write(
        format!("{}/http-proxy.conf", override_dir),
        systemd_override,
    ) {
        println!("❌ Failed to write proxy config: {}", e);
        return;
    }

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

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select optimization")
        .items(&optimization_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        _ => return,
    };

    match selection {
        0 => {
            let concurrent: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Max concurrent downloads")
                .default("3".to_string())
                .interact_text()
            {
                Ok(c) => c,
                Err(_) => return,
            };

            add_daemon_config("max-concurrent-downloads", &concurrent);
        }
        1 => {
            let attempts: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Max download attempts")
                .default("5".to_string())
                .interact_text()
            {
                Ok(a) => a,
                Err(_) => return,
            };

            add_daemon_config("max-download-attempts", &attempts);
        }
        2 => {
            let timeout: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Download timeout (seconds)")
                .default("300".to_string())
                .interact_text()
            {
                Ok(t) => t,
                Err(_) => return,
            };

            add_daemon_config("shutdown-timeout", &timeout);
        }
        3 => {
            println!("💡 Enabling compression at registry level");
            add_daemon_config("experimental", "true");
        }
        _ => {}
    }
}

fn add_daemon_config(key: &str, value: &str) {
    let current_config =
        fs::read_to_string("/etc/docker/daemon.json").unwrap_or_else(|_| "{}".to_string());

    let mut config: serde_json::Value =
        serde_json::from_str(&current_config).unwrap_or_else(|_| serde_json::json!({}));

    if let Ok(num) = value.parse::<i64>() {
        config[key] = serde_json::Value::Number(serde_json::Number::from(num));
    } else if value == "true" || value == "false" {
        config[key] = serde_json::Value::Bool(value == "true");
    } else {
        config[key] = serde_json::Value::String(value.to_string());
    }

    if let Ok(config_json) = serde_json::to_string_pretty(&config) {
        if let Err(e) = fs::write("/etc/docker/daemon.json", config_json) {
            println!("❌ Failed to write daemon config: {}", e);
            return;
        }
        println!("✅ Configuration added: {} = {}", key, value);
    }
}

fn corporate_registry_setup() {
    println!("🏢 Corporate Registry Setup\n");

    let registry_url: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Corporate registry URL")
        .interact_text()
    {
        Ok(url) => url,
        Err(_) => return,
    };

    let is_insecure = !registry_url.starts_with("https://");

    let add_to_insecure = is_insecure
        && Confirm::new()
            .with_prompt("Registry uses HTTP. Add to insecure registries?")
            .default(true)
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(false);

    if add_to_insecure {
        let current_config =
            fs::read_to_string("/etc/docker/daemon.json").unwrap_or_else(|_| "{}".to_string());

        let mut config: serde_json::Value =
            serde_json::from_str(&current_config).unwrap_or_else(|_| serde_json::json!({}));

        let insecure = config
            .get_mut("insecure-registries")
            .and_then(|r| r.as_array_mut())
            .map(|registries| {
                registries.push(serde_json::Value::String(registry_url.clone()));
                registries.clone()
            })
            .unwrap_or_else(|| vec![serde_json::Value::String(registry_url.clone())]);

        config["insecure-registries"] = serde_json::Value::Array(insecure);

        if let Ok(config_json) = serde_json::to_string_pretty(&config) {
            if let Err(e) = fs::write("/etc/docker/daemon.json", config_json) {
                println!("❌ Failed to write daemon config: {}", e);
                return;
            }
            println!("✅ Added to insecure registries: {}", registry_url);
        }
    }

    let configure_auth = Confirm::new()
        .with_prompt("Configure registry authentication?")
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if configure_auth {
        let username: String = match Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Username")
            .interact_text()
        {
            Ok(u) => u,
            Err(_) => return,
        };

        // Use masked password input
        let password: String = match Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password")
            .interact()
        {
            Ok(p) => p,
            Err(_) => return,
        };

        // Use secure login helper (password via stdin, not argv)
        if secure_docker_login(Some(&registry_url), &username, &password) {
            println!("✅ Authenticated with corporate registry");
        } else {
            println!("❌ Failed to authenticate with corporate registry");
        }
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

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select authentication option")
        .items(&auth_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        _ => return,
    };

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
    let username: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Docker Hub username")
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    // Use masked password input
    let password: String = match Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Docker Hub password/token")
        .interact()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    // Use secure login helper (password via stdin, not argv)
    if secure_docker_login(None, &username, &password) {
        println!("✅ Successfully logged in to Docker Hub");
    } else {
        println!("❌ Login failed");
    }
}

fn github_registry_login() {
    let username: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("GitHub username")
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    // Use masked input for token
    let token: String = match Password::with_theme(&ColorfulTheme::default())
        .with_prompt("GitHub Personal Access Token")
        .interact()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    // Use secure login helper (token via stdin, not argv)
    if secure_docker_login(Some("ghcr.io"), &username, &token) {
        println!("✅ Successfully logged in to GitHub Container Registry");
    } else {
        println!("❌ Login failed");
    }
}

fn aws_ecr_login() {
    let region: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("AWS region")
        .default("us-east-1".to_string())
        .interact_text()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    let account_id: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("AWS account ID")
        .interact_text()
    {
        Ok(id) => id,
        Err(_) => return,
    };

    println!("🔄 Getting ECR login token...");
    let result = Command::new("aws")
        .args(["ecr", "get-login-password", "--region", &region])
        .output();

    if let Ok(output) = result {
        let password = String::from_utf8_lossy(&output.stdout);
        let ecr_url = format!("{}.dkr.ecr.{}.amazonaws.com", account_id, region);

        let login_result = Command::new("docker")
            .args(["login", "--username", "AWS", "--password-stdin", &ecr_url])
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

    let key_file: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to service account key file")
        .interact_text()
    {
        Ok(k) => k,
        Err(_) => return,
    };

    // Validate key file path
    if key_file.contains(|c: char| {
        matches!(
            c,
            '`' | '$' | '(' | ')' | '{' | '}' | ';' | '&' | '|' | '<' | '>' | '\n' | '\r'
        )
    }) {
        println!("❌ Key file path contains invalid characters");
        return;
    }

    if !std::path::Path::new(&key_file).exists() {
        println!("❌ Key file does not exist: {}", key_file);
        return;
    }

    let result = Command::new("gcloud")
        .args(["auth", "activate-service-account", "--key-file", &key_file])
        .status();

    match result {
        Ok(s) if s.success() => {
            match Command::new("gcloud")
                .args(["auth", "configure-docker"])
                .status()
            {
                Ok(s) if s.success() => println!("✅ Successfully configured GCR authentication"),
                Ok(_) => println!("⚠️  GCR auth may have warnings"),
                Err(e) => println!("❌ Failed to configure docker: {}", e),
            }
        }
        Ok(_) => println!("❌ GCR authentication failed"),
        Err(e) => println!("❌ Failed to authenticate: {}", e),
    }
}

fn custom_registry_login() {
    let registry_url: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry URL")
        .interact_text()
    {
        Ok(url) => url,
        Err(_) => return,
    };

    let username: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Username")
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    // Use masked password input
    let password: String = match Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .interact()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    // Use secure login helper (password via stdin, not argv)
    if secure_docker_login(Some(&registry_url), &username, &password) {
        println!("✅ Successfully logged in to {}", registry_url);
    } else {
        println!("❌ Login failed");
    }
}

fn registry_logout() {
    let registry_url: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry URL to logout from (leave empty for Docker Hub)")
        .default("".to_string())
        .interact_text()
    {
        Ok(url) => url,
        Err(_) => return,
    };

    let result = if registry_url.is_empty() {
        Command::new("docker").args(["logout"]).status()
    } else {
        Command::new("docker")
            .args(["logout", &registry_url])
            .status()
    };

    if result.map(|s| s.success()).unwrap_or(false) {
        println!("✅ Successfully logged out");
    } else {
        println!("❌ Logout failed");
    }
}

fn view_stored_credentials() {
    println!("📋 Stored Registry Credentials\n");

    let config_path = format!(
        "{}/.docker/config.json",
        std::env::var("HOME").unwrap_or_default()
    );

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

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select configuration action")
        .items(&config_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        _ => return,
    };

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
    let backup_path = format!(
        "/tmp/docker-daemon-backup-{}.json",
        chrono::Utc::now().format("%Y%m%d-%H%M%S")
    );

    if fs::copy("/etc/docker/daemon.json", &backup_path).is_ok() {
        println!("✅ Configuration backed up to: {}", backup_path);
    } else {
        println!("❌ Backup failed");
    }
}

fn restore_configuration() {
    let backup_file: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to backup file")
        .interact_text()
    {
        Ok(f) => f,
        Err(_) => return,
    };

    let should_restore = Confirm::new()
        .with_prompt("Really restore configuration? This will overwrite current settings.")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if should_restore {
        if fs::copy(&backup_file, "/etc/docker/daemon.json").is_ok() {
            println!("✅ Configuration restored from: {}", backup_file);
            restart_docker_daemon();
        } else {
            println!("❌ Restore failed");
        }
    }
}

fn export_configuration() {
    let export_path = "/tmp/docker-registry-config-export.json";

    let current_config =
        fs::read_to_string("/etc/docker/daemon.json").unwrap_or_else(|_| "{}".to_string());

    fs::write(export_path, current_config).ok();
    println!("✅ Configuration exported to: {}", export_path);
}

fn import_configuration() {
    let import_file: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to configuration file")
        .interact_text()
    {
        Ok(f) => f,
        Err(_) => return,
    };

    if let Ok(config_content) = fs::read_to_string(&import_file) {
        // Validate JSON
        if serde_json::from_str::<serde_json::Value>(&config_content).is_ok() {
            let should_import = Confirm::new()
                .with_prompt("Import this configuration?")
                .default(true)
                .interact_opt()
                .ok()
                .flatten()
                .unwrap_or(false);

            if should_import {
                if let Err(e) = fs::write("/etc/docker/daemon.json", config_content) {
                    println!("❌ Failed to write daemon config: {}", e);
                    return;
                }
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
    let should_reset = Confirm::new()
        .with_prompt("Really reset Docker daemon configuration to defaults?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if should_reset {
        if let Err(e) = fs::write("/etc/docker/daemon.json", "{}") {
            println!("❌ Failed to write daemon config: {}", e);
            return;
        }
        println!("✅ Configuration reset to defaults");
        restart_docker_daemon();
    }
}

fn registry_health_check() {
    println!("🏥 Registry Health Check\n");

    // Check local Docker daemon
    println!("🐳 Docker daemon status:");
    if let Err(e) = Command::new("systemctl")
        .args(["status", "docker", "--no-pager"])
        .status()
    {
        println!("  Could not check daemon status: {}", e);
    }

    // Check configured mirrors
    let current_config =
        fs::read_to_string("/etc/docker/daemon.json").unwrap_or_else(|_| "{}".to_string());

    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&current_config)
        && let Some(mirrors) = config.get("registry-mirrors").and_then(|m| m.as_array())
    {
        println!("\n🪞 Mirror health check:");
        for mirror in mirrors {
            if let Some(mirror_url) = mirror.as_str() {
                print!("Checking {}... ", mirror_url);
                let result = Command::new("curl")
                    .args([
                        "-s",
                        "-o",
                        "/dev/null",
                        "-w",
                        "%{http_code}",
                        "--max-time",
                        "10",
                        &format!("{}/v2/", mirror_url),
                    ])
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

    // Test Docker pull
    println!("\n🧪 Testing Docker pull performance:");
    let test_image = "hello-world:latest";
    println!("Pulling {}...", test_image);

    let start_time = std::time::Instant::now();
    let result = Command::new("docker").args(["pull", test_image]).status();
    let duration = start_time.elapsed();

    match result {
        Ok(s) if s.success() => println!("✅ Pull successful in {:.2}s", duration.as_secs_f64()),
        Ok(_) => println!("⚠️  Pull completed with warnings"),
        Err(e) => println!("❌ Pull failed: {}", e),
    }

    // Clean up test image
    if let Err(e) = Command::new("docker").args(["rmi", test_image]).status() {
        println!("  Note: Could not remove test image: {}", e);
    }
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
    if let Err(e) = Command::new("docker").args(["images"]).status() {
        println!("❌ Failed to list images: {}", e);
    }
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

/// Parse an image reference into its components (registry, repository, tag)
pub fn parse_image_reference(image: &str) -> ImageReference {
    let mut parts = image.splitn(2, '/');
    let first = parts.next().unwrap_or("");
    let rest = parts.next();

    // Check if first part is a registry:
    // - Contains '.' (domain like registry.example.com)
    // - Contains ':' followed by digits (port like localhost:5000)
    // - Is 'localhost'
    let is_registry = first.contains('.')
        || first == "localhost"
        || (first.contains(':')
            && first
                .split(':')
                .nth(1)
                .map_or(false, |p| p.chars().all(|c| c.is_ascii_digit())));

    let (registry, repo_tag) = if is_registry && rest.is_some() {
        (Some(first.to_string()), rest.unwrap_or(""))
    } else if rest.is_some() {
        // It's user/repo format from Docker Hub
        (None, image)
    } else {
        // Just a repo name (possibly with tag), no registry
        (None, image)
    };

    // Split repo:tag
    let (repository, tag) = if let Some(colon_pos) = repo_tag.rfind(':') {
        // Make sure colon is not in a port (check if after last /)
        let last_slash = repo_tag.rfind('/').unwrap_or(0);
        if colon_pos > last_slash {
            (&repo_tag[..colon_pos], Some(&repo_tag[colon_pos + 1..]))
        } else {
            (repo_tag, None)
        }
    } else {
        (repo_tag, None)
    };

    ImageReference {
        registry,
        repository: repository.to_string(),
        tag: tag.map(|s| s.to_string()),
    }
}

/// Represents a parsed Docker image reference
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageReference {
    pub registry: Option<String>,
    pub repository: String,
    pub tag: Option<String>,
}

impl ImageReference {
    /// Get the full image reference string
    pub fn full_name(&self) -> String {
        let mut name = String::new();
        if let Some(ref reg) = self.registry {
            name.push_str(reg);
            name.push('/');
        }
        name.push_str(&self.repository);
        if let Some(ref tag) = self.tag {
            name.push(':');
            name.push_str(tag);
        }
        name
    }

    /// Get the tag or "latest" as default
    pub fn tag_or_latest(&self) -> &str {
        self.tag.as_deref().unwrap_or("latest")
    }
}

/// Validate a registry URL format
pub fn validate_registry_url(url: &str) -> Result<(), String> {
    if url.is_empty() {
        return Err("Registry URL cannot be empty".to_string());
    }

    // Check for valid protocol or hostname format
    if url.starts_with("http://") || url.starts_with("https://") {
        // Full URL format
        if url.len() < 10 {
            return Err("Registry URL too short".to_string());
        }
    } else {
        // Hostname format (e.g., docker.io, registry.example.com)
        if !url.contains('.') && url != "localhost" && !url.contains(':') {
            return Err("Invalid registry format - expected hostname with domain".to_string());
        }
    }

    Ok(())
}

/// Parse a Docker config.json auth section to extract registry URLs
pub fn parse_docker_auths(config: &serde_json::Value) -> Vec<String> {
    let mut registries = Vec::new();

    if let Some(auths) = config.get("auths").and_then(|a| a.as_object()) {
        for (registry, _) in auths {
            registries.push(registry.clone());
        }
    }

    registries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_image_reference_simple() {
        let ref1 = parse_image_reference("nginx");
        assert_eq!(ref1.registry, None);
        assert_eq!(ref1.repository, "nginx");
        assert_eq!(ref1.tag, None);
    }

    #[test]
    fn test_parse_image_reference_with_tag() {
        let ref1 = parse_image_reference("nginx:latest");
        assert_eq!(ref1.registry, None);
        assert_eq!(ref1.repository, "nginx");
        assert_eq!(ref1.tag, Some("latest".to_string()));
    }

    #[test]
    fn test_parse_image_reference_user_repo() {
        let ref1 = parse_image_reference("myuser/myimage:v1.0");
        assert_eq!(ref1.registry, None);
        assert_eq!(ref1.repository, "myuser/myimage");
        assert_eq!(ref1.tag, Some("v1.0".to_string()));
    }

    #[test]
    fn test_parse_image_reference_with_registry() {
        let ref1 = parse_image_reference("registry.example.com/myimage:v1.0");
        assert_eq!(ref1.registry, Some("registry.example.com".to_string()));
        assert_eq!(ref1.repository, "myimage");
        assert_eq!(ref1.tag, Some("v1.0".to_string()));
    }

    #[test]
    fn test_parse_image_reference_localhost() {
        let ref1 = parse_image_reference("localhost:5000/myimage:v1.0");
        assert_eq!(ref1.registry, Some("localhost:5000".to_string()));
        assert_eq!(ref1.repository, "myimage");
        assert_eq!(ref1.tag, Some("v1.0".to_string()));
    }

    #[test]
    fn test_parse_image_reference_gcr() {
        let ref1 = parse_image_reference("gcr.io/project/image:tag");
        assert_eq!(ref1.registry, Some("gcr.io".to_string()));
        assert_eq!(ref1.repository, "project/image");
        assert_eq!(ref1.tag, Some("tag".to_string()));
    }

    #[test]
    fn test_image_reference_full_name() {
        let ref1 = ImageReference {
            registry: Some("docker.io".to_string()),
            repository: "library/nginx".to_string(),
            tag: Some("1.21".to_string()),
        };
        assert_eq!(ref1.full_name(), "docker.io/library/nginx:1.21");
    }

    #[test]
    fn test_image_reference_full_name_no_registry() {
        let ref1 = ImageReference {
            registry: None,
            repository: "nginx".to_string(),
            tag: Some("latest".to_string()),
        };
        assert_eq!(ref1.full_name(), "nginx:latest");
    }

    #[test]
    fn test_image_reference_full_name_no_tag() {
        let ref1 = ImageReference {
            registry: None,
            repository: "nginx".to_string(),
            tag: None,
        };
        assert_eq!(ref1.full_name(), "nginx");
    }

    #[test]
    fn test_image_reference_tag_or_latest() {
        let ref1 = ImageReference {
            registry: None,
            repository: "nginx".to_string(),
            tag: None,
        };
        assert_eq!(ref1.tag_or_latest(), "latest");

        let ref2 = ImageReference {
            registry: None,
            repository: "nginx".to_string(),
            tag: Some("1.21".to_string()),
        };
        assert_eq!(ref2.tag_or_latest(), "1.21");
    }

    #[test]
    fn test_validate_registry_url_valid() {
        assert!(validate_registry_url("docker.io").is_ok());
        assert!(validate_registry_url("registry.example.com").is_ok());
        assert!(validate_registry_url("https://registry.example.com").is_ok());
        assert!(validate_registry_url("localhost:5000").is_ok());
        assert!(validate_registry_url("localhost").is_ok());
    }

    #[test]
    fn test_validate_registry_url_empty() {
        let result = validate_registry_url("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_validate_registry_url_invalid() {
        let result = validate_registry_url("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_docker_auths() {
        let config = serde_json::json!({
            "auths": {
                "docker.io": {"auth": "xxx"},
                "ghcr.io": {"auth": "yyy"}
            }
        });

        let registries = parse_docker_auths(&config);
        assert_eq!(registries.len(), 2);
        assert!(registries.contains(&"docker.io".to_string()));
        assert!(registries.contains(&"ghcr.io".to_string()));
    }

    #[test]
    fn test_parse_docker_auths_empty() {
        let config = serde_json::json!({});
        let registries = parse_docker_auths(&config);
        assert!(registries.is_empty());
    }

    #[test]
    fn test_parse_docker_auths_no_auths_key() {
        let config = serde_json::json!({
            "credsStore": "desktop"
        });
        let registries = parse_docker_auths(&config);
        assert!(registries.is_empty());
    }
}
