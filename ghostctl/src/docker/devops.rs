use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::fs;
use std::process::Command;

pub fn docker_management() {
    let options = [
        "🔍 Docker Health Check",
        "📦 Container Management",
        "🛡️  Container Security",
        "📦 Stack Management",
        "📊 Resource Monitoring",
        "📈 Infrastructure Monitoring",
        "🚀 CI/CD Tools",
        "🧹 System Cleanup",
        "🏗️  Registry Management",
        "🌍 Environment Manager",
        "☸️  Kubernetes Tools",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🐳 Docker Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => docker_health_comprehensive(),
        1 => crate::docker::container::container_management(),
        2 => crate::docker::security::container_security(),
        3 => compose_stack_manager(),
        4 => docker_resource_report(),
        5 => monitoring_tools(),
        6 => cicd_helpers(),
        7 => docker_system_cleanup(),
        8 => crate::docker::registry::registry_management(),
        9 => environment_manager(),
        10 => kubernetes_tools(),
        _ => return,
    }
}

pub fn docker_health_comprehensive() {
    println!("🔍 Comprehensive Docker Health Check");
    println!("====================================");

    let options = [
        "📊 System Health Report",
        "🔍 Scan Specific Image",
        "🔍 Search Registry Images",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Docker Health Options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => show_system_health(),
        1 => {
            let image = "nginx:latest";
            scan_local_image_with_name(image);
        }
        2 => {
            let query = "nginx";
            search_registry_with_query(query);
        }
        _ => return,
    }
}

fn show_system_health() {
    // Check Docker daemon
    print!("🐳 Docker Daemon: ");
    match Command::new("docker").arg("info").output() {
        Ok(output) if output.status.success() => println!("✅ Running"),
        _ => println!("❌ Not running or not accessible"),
    }

    // System info
    if let Ok(output) = Command::new("docker").arg("system").arg("df").output() {
        println!("\n💾 Docker System Usage:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    // Check for unhealthy containers
    println!("\n🏥 Container Health Status:");
    let _ = Command::new("docker")
        .args(&[
            "ps",
            "--filter",
            "health=unhealthy",
            "--format",
            "table {{.Names}}\\t{{.Status}}",
        ])
        .status();

    // Resource-hungry containers
    println!("\n🔥 Top Resource Consumers:");
    let _ = Command::new("docker")
        .args(&[
            "stats",
            "--no-stream",
            "--format",
            "table {{.Container}}\\t{{.CPUPerc}}\\t{{.MemUsage}}",
        ])
        .status();

    // Check for containers without health checks
    println!("\n⚠️  Containers without health checks:");
    let _ = Command::new("bash")
        .arg("-c")
        .arg("docker ps --format '{{.Names}}' | xargs -I {} sh -c 'docker inspect {} | jq -r \".[0].Config.Healthcheck // \\\"No healthcheck\\\"\" | grep -q \"No healthcheck\" && echo \"{}\"'")
        .status();
}

pub fn docker_resource_report() {
    println!("📊 Docker Resource Report");
    println!("========================");

    println!("🐳 Running Containers:");
    let _ = Command::new("docker")
        .args(&[
            "ps",
            "--format",
            "table {{.Names}}\\t{{.CPU}}\\t{{.MemUsage}}\\t{{.NetIO}}\\t{{.BlockIO}}",
        ])
        .status();

    println!("\n💾 Image Storage:");
    let _ = Command::new("docker")
        .args(&[
            "images",
            "--format",
            "table {{.Repository}}\\t{{.Tag}}\\t{{.Size}}",
        ])
        .status();

    println!("\n🔗 Network Usage:");
    let _ = Command::new("docker").args(&["network", "ls"]).status();

    println!("\n💿 Volume Usage:");
    let _ = Command::new("docker").args(&["volume", "ls"]).status();
}

pub fn docker_system_cleanup() {
    println!("🧹 Docker System Cleanup");
    println!("========================");

    let cleanup_options = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup operations")
        .items(&[
            "🗑️  Remove stopped containers",
            "🖼️  Remove unused images",
            "💿 Remove unused volumes",
            "🔗 Remove unused networks",
            "🧹 Full system prune",
        ])
        .interact()
        .unwrap();

    if cleanup_options.is_empty() {
        println!("❌ No cleanup operations selected");
        return;
    }

    for operation in cleanup_options {
        match operation {
            0 => {
                println!("🗑️  Removing stopped containers...");
                let _ = Command::new("docker")
                    .args(&["container", "prune", "-f"])
                    .status();
            }
            1 => {
                println!("🖼️  Removing unused images...");
                let _ = Command::new("docker")
                    .args(&["image", "prune", "-f"])
                    .status();
            }
            2 => {
                println!("💿 Removing unused volumes...");
                let _ = Command::new("docker")
                    .args(&["volume", "prune", "-f"])
                    .status();
            }
            3 => {
                println!("🔗 Removing unused networks...");
                let _ = Command::new("docker")
                    .args(&["network", "prune", "-f"])
                    .status();
            }
            4 => {
                let confirm = Confirm::new()
                    .with_prompt("⚠️  This will remove ALL unused data. Continue?")
                    .default(false)
                    .interact()
                    .unwrap();

                if confirm {
                    println!("🧹 Running full system prune...");
                    let _ = Command::new("docker")
                        .args(&["system", "prune", "-af", "--volumes"])
                        .status();
                }
            }
            _ => {}
        }
    }

    println!("✅ Cleanup operations completed");
}

#[allow(dead_code)]
#[allow(dead_code)]
pub fn list_registry_images() {
    println!("📋 Registry Images");
    println!("==================");

    let registry: String = Input::new()
        .with_prompt("Registry URL (e.g., docker.cktechx.io)")
        .default("docker.cktechx.io".into())
        .interact_text()
        .unwrap();

    println!("🔍 Listing images from {}...", registry);

    // This would need registry API integration
    println!("💡 Use: docker search {} or registry API", registry);
}

#[allow(dead_code)]
pub fn push_to_registry() {
    println!("📤 Push Image to Registry");
    println!("========================");

    let image: String = Input::new()
        .with_prompt("Local image name")
        .interact_text()
        .unwrap();

    let registry: String = Input::new()
        .with_prompt("Registry URL")
        .default("docker.cktechx.io".into())
        .interact_text()
        .unwrap();

    let tag: String = Input::new()
        .with_prompt("Tag")
        .default("latest".into())
        .interact_text()
        .unwrap();

    let full_name = format!("{}/{}:{}", registry, image, tag);

    println!("🏷️  Tagging image...");
    let _ = Command::new("docker")
        .args(&["tag", &image, &full_name])
        .status();

    println!("📤 Pushing to registry...");
    let _ = Command::new("docker").args(&["push", &full_name]).status();
}

#[allow(dead_code)]
pub fn pull_from_registry() {
    println!("📥 Pull Image from Registry");
    println!("===========================");

    let image: String = Input::new()
        .with_prompt("Image name (registry/image:tag)")
        .interact_text()
        .unwrap();

    println!("📥 Pulling {}...", image);
    let _ = Command::new("docker").args(&["pull", &image]).status();
}

#[allow(dead_code)]
pub fn registry_authentication() {
    println!("🔑 Registry Authentication");
    println!("==========================");

    let registry: String = Input::new()
        .with_prompt("Registry URL")
        .default("docker.cktechx.io".into())
        .interact_text()
        .unwrap();

    let username: String = Input::new()
        .with_prompt("Username")
        .interact_text()
        .unwrap();

    println!("🔑 Logging into {}...", registry);
    let _ = Command::new("docker")
        .args(&["login", &registry, "-u", &username])
        .status();
}

#[allow(dead_code)]
pub fn dockerhub_signin() {
    println!("🐳 DockerHub Sign-in");
    println!("===================");

    let username: String = Input::new()
        .with_prompt("DockerHub Username")
        .interact_text()
        .unwrap();

    println!("🔑 Logging into DockerHub...");
    let _ = Command::new("docker")
        .args(&["login", "-u", &username])
        .status();
}

#[allow(dead_code)]
pub fn delete_registry_image() {
    println!("🗑️  Delete Registry Image");
    println!("========================");

    let image: String = Input::new()
        .with_prompt("Image to delete")
        .interact_text()
        .unwrap();

    let confirm = Confirm::new()
        .with_prompt(format!("Delete image {}?", image))
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let _ = Command::new("docker").args(&["rmi", &image]).status();
    }
}

#[allow(dead_code)]
pub fn registry_statistics() {
    println!("📊 Registry Statistics");
    println!("======================");

    let _ = Command::new("docker")
        .args(&[
            "images",
            "--format",
            "table {{.Repository}}\t{{.Tag}}\t{{.Size}}",
        ])
        .status();
}

// CI/CD Helper Functions
pub fn cicd_helpers() {
    let options = [
        "🦀 Rust CI/CD Template",
        "⚡ Zig CI/CD Template",
        "🐳 Docker Multi-arch Build",
        "⚡ Docker Build Optimizer",
        "🚀 Release Automation",
        "🧪 Test Coverage Setup",
        "🛡️  Security Scanning",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🔄 CI/CD Helpers")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => rust_cicd_template(),
        1 => zig_cicd_template(),
        2 => docker_multiarch_build(),
        3 => docker_build_optimizer(),
        4 => release_automation(),
        5 => test_coverage_setup(),
        6 => security_scanning_setup(),
        _ => return,
    }
}

fn rust_cicd_template() {
    println!("🦀 Generating Rust CI/CD Template");

    let project_name: String = Input::new()
        .with_prompt("Project name")
        .interact_text()
        .unwrap();

    let features = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select features")
        .items(&[
            "🧪 Unit Tests",
            "🛡️  Security Audit",
            "📊 Code Coverage",
            "📦 Multi-target Build",
            "🚀 Auto Release",
            "🐳 Docker Build",
            "📝 Dependency Caching",
        ])
        .interact()
        .unwrap();

    let template = generate_rust_workflow_template(&project_name, &features);

    let save_path = format!(".github/workflows/{}-ci.yml", project_name);

    let save = Confirm::new()
        .with_prompt(format!("Save workflow to {}?", save_path))
        .default(true)
        .interact()
        .unwrap();

    if save {
        fs::create_dir_all(".github/workflows").unwrap();
        fs::write(&save_path, template).unwrap();
        println!("✅ Workflow saved to {}", save_path);
    } else {
        println!("📋 Template generated (not saved)");
    }
}

fn generate_rust_workflow_template(project_name: &str, features: &[usize]) -> String {
    let mut workflow = format!(
        r#"name: {project_name} CI/CD

on:
  push:
    tags: ['v*']
  pull_request:

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  test:
    name: Test Suite
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust: [stable, beta]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{{{ matrix.rust }}}}
          components: clippy, rustfmt
"#,
        project_name = project_name
    );

    if features.contains(&6) {
        // Dependency caching
        workflow.push_str(
            r#"
      - uses: actions/cache@v3
        with:
          path: target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
"#,
        );
    }

    workflow.push_str(
        r#"
      - name: Run tests
        run: cargo test --verbose
      - name: Check formatting  
        run: cargo fmt --check
      - name: Clippy check
        run: cargo clippy -- -D warnings
"#,
    );

    if features.contains(&1) {
        // Security audit
        workflow.push_str(
            r#"
  security:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v1.4.1
"#,
        );
    }

    if features.contains(&2) {
        // Code coverage
        workflow.push_str(
            r#"
  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@cargo-tarpaulin
      - run: cargo tarpaulin --out xml
      - uses: codecov/codecov-action@v3
"#,
        );
    }

    workflow
}

fn zig_cicd_template() {
    println!("⚡ Zig CI/CD Template - TODO: Implement");
}

fn docker_multiarch_build() {
    println!("🐳 Docker Multi-arch Build - TODO: Implement");
}

fn release_automation() {
    println!("🚀 Release Automation - TODO: Implement");
}

fn test_coverage_setup() {
    println!("🧪 Test Coverage Setup - TODO: Implement");
}

fn security_scanning_setup() {
    println!("🛡️  Security Scanning Setup - TODO: Implement");
}

pub fn monitoring_tools() {
    println!("📊 Setting up Infrastructure Monitoring");

    let tools = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select monitoring tools")
        .items(&[
            "📈 Prometheus + Grafana",
            "📊 ELK Stack",
            "🔍 Loki + Grafana",
            "📡 Node Exporter",
            "🐳 cAdvisor",
        ])
        .interact()
        .unwrap();

    if tools.contains(&0) {
        setup_prometheus_grafana();
    }

    if tools.contains(&1) {
        setup_elk_stack();
    }

    // Generate docker-compose.yml for selected tools
    generate_monitoring_compose(&tools);
}

fn setup_prometheus_grafana() {
    println!("🔧 Setting up Prometheus + Grafana");

    let prometheus_config = r#"global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  # - "first_rules.yml"

scrape_configs:
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']
  
  - job_name: 'node'
    static_configs:
      - targets: ['localhost:9100']
      
  - job_name: 'cadvisor'
    static_configs:
      - targets: ['localhost:8080']
"#;

    std::fs::create_dir_all("monitoring/prometheus").unwrap();
    std::fs::write("monitoring/prometheus/prometheus.yml", prometheus_config).unwrap();

    println!("✅ Prometheus config saved to monitoring/prometheus/prometheus.yml");
}

fn setup_elk_stack() {
    println!("📊 ELK Stack setup - TODO: Implement");
}

fn generate_monitoring_compose(selected_tools: &[usize]) -> String {
    let mut compose = String::from(
        r#"version: '3.8'

services:
"#,
    );

    if selected_tools.contains(&0) {
        // Prometheus + Grafana
        compose.push_str(
            r#"
  prometheus:
    image: prom/prometheus:latest
    container_name: prometheus
    ports:
      - "9090:9090"
    volumes:
      - ./monitoring/prometheus:/etc/prometheus
      - prometheus_data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      
  grafana:
    image: grafana/grafana:latest
    container_name: grafana
    ports:
      - "3000:3000"
    volumes:
      - grafana_data:/var/lib/grafana
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
"#,
        );
    }

    compose.push_str(
        r#"
volumes:
  prometheus_data:
  grafana_data:

networks:
  monitoring:
    driver: bridge
"#,
    );

    std::fs::write("docker-compose.monitoring.yml", &compose).unwrap();
    println!("✅ Monitoring compose saved to docker-compose.monitoring.yml");

    compose
}

// Complete missing function implementations
pub fn container_security_scanning() {
    println!("🔍 Container Security Scanning");
    println!("==============================");

    let options = [
        "🔍 Scan specific image",
        "📊 Scan all local images",
        "🔧 Install/Update Trivy",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Security Scanning Options")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => scan_local_image(),
        1 => scan_all_local_images(),
        2 => install_trivy(),
        _ => return,
    }
}

fn scan_all_local_images() {
    println!("🚀 Scanning all local images...");
    let _ = Command::new("bash")
        .arg("-c")
        .arg("docker images --format '{{.Repository}}:{{.Tag}}' | xargs -I {} trivy image {}")
        .status();
}

fn install_trivy() {
    println!("🔧 Installing Trivy...");
    let _ = Command::new("bash")
        .arg("-c")
        .arg("curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh -s -- -b /usr/local/bin")
        .status();
}

pub fn scan_local_image() {
    let image: String = Input::new()
        .with_prompt("Image name to scan")
        .interact_text()
        .unwrap();

    println!("🔍 Scanning image: {}", image);
    let _ = Command::new("trivy").args(&["image", &image]).status();
}

pub fn compose_stack_manager() {
    crate::docker::compose::compose_stack_manager();
}

#[allow(dead_code)]
pub fn list_compose_stacks(stack_dir: &str) {
    println!("📋 Compose stacks in: {}", stack_dir);
    if let Ok(entries) = std::fs::read_dir(stack_dir) {
        for entry in entries.flatten() {
            if entry.path().join("docker-compose.yml").exists()
                || entry.path().join("docker-compose.yaml").exists()
            {
                println!("  📦 {}", entry.file_name().to_string_lossy());
            }
        }
    }
}

#[allow(dead_code)]
pub fn deploy_new_stack(stack_dir: &str) {
    println!("🚀 Deploying new stack in: {}", stack_dir);
    let _ = Command::new("docker-compose")
        .args(&[
            "-f",
            &format!("{}/docker-compose.yml", stack_dir),
            "up",
            "-d",
        ])
        .status();
}

#[allow(dead_code)]
pub fn registry_tools() {
    crate::docker::registry::registry_management();
}

pub fn kubernetes_tools() {
    println!("☸️ Kubernetes Tools");
    println!("===================");
    println!("Feature not yet implemented");
}

#[allow(dead_code)]
pub fn generate_github_workflow() {
    println!("🔄 Generating GitHub workflow...");
    rust_cicd_template();
}

pub fn docker_build_optimizer() {
    println!("⚡ Docker Build Optimizer");
    println!("========================");

    let optimizations = [
        "🔧 Multi-stage builds",
        "📦 Layer caching",
        "🗜️ Image size reduction",
        "⚡ Build speed optimization",
    ];

    for opt in &optimizations {
        println!("  {}", opt);
    }
}

pub fn environment_manager() {
    println!("🌍 Environment Manager");
    println!("======================");

    let envs = ["development", "staging", "production"];
    for env in &envs {
        println!("  📁 {}", env);
    }
}

#[allow(dead_code)]
pub fn search_registry() {
    let query: String = Input::new()
        .with_prompt("Search query")
        .interact_text()
        .unwrap();

    println!("🔍 Searching for: {}", query);
    let _ = Command::new("docker").args(&["search", &query]).status();
}

// Wrapper functions for CLI usage
pub fn scan_local_image_with_name(image: &str) {
    println!("🔍 Scanning image: {}", image);
    let _ = Command::new("trivy").args(&["image", image]).status();
}

pub fn search_registry_with_query(query: &str) {
    println!("🔍 Searching for: {}", query);
    let _ = Command::new("docker").args(&["search", query]).status();
}
