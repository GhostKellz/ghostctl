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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🐳 Docker Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Docker Health Options")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

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
    match Command::new("docker").args(["system", "df"]).output() {
        Ok(output) if output.status.success() => {
            println!("\n💾 Docker System Usage:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        _ => println!("\n💾 Could not retrieve system usage"),
    }

    // Check for unhealthy containers
    println!("\n🏥 Container Health Status:");
    if let Err(e) = Command::new("docker")
        .args([
            "ps",
            "--filter",
            "health=unhealthy",
            "--format",
            "table {{.Names}}\t{{.Status}}",
        ])
        .status()
    {
        println!("  Could not check container health: {}", e);
    }

    // Resource-hungry containers
    println!("\n🔥 Top Resource Consumers:");
    if let Err(e) = Command::new("docker")
        .args([
            "stats",
            "--no-stream",
            "--format",
            "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}",
        ])
        .status()
    {
        println!("  Could not get stats: {}", e);
    }

    // Check for containers without health checks - do this safely without shell
    println!("\n⚠️  Containers without health checks:");
    let output = Command::new("docker")
        .args(["ps", "--format", "{{.Names}}"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let containers = String::from_utf8_lossy(&output.stdout);
            let mut no_healthcheck_count = 0;
            for container in containers.lines() {
                if container.trim().is_empty() {
                    continue;
                }
                // Validate container name before inspection
                if crate::docker::validate_container_name(container).is_err() {
                    continue;
                }
                // Inspect each container individually
                if let Ok(inspect) = Command::new("docker")
                    .args([
                        "inspect",
                        "--format",
                        "{{json .Config.Healthcheck}}",
                        container,
                    ])
                    .output()
                {
                    let healthcheck = String::from_utf8_lossy(&inspect.stdout);
                    if healthcheck.trim() == "null" || healthcheck.trim().is_empty() {
                        println!("  {}", container);
                        no_healthcheck_count += 1;
                    }
                }
            }
            if no_healthcheck_count == 0 {
                println!("  All containers have health checks configured");
            }
        }
        _ => println!("  Could not list containers"),
    }
}

pub fn docker_resource_report() {
    println!("📊 Docker Resource Report");
    println!("========================");

    println!("🐳 Running Containers:");
    if let Err(e) = Command::new("docker")
        .args([
            "ps",
            "--format",
            "table {{.Names}}\t{{.CPU}}\t{{.MemUsage}}\t{{.NetIO}}\t{{.BlockIO}}",
        ])
        .status()
    {
        println!("  Could not list containers: {}", e);
    }

    println!("\n💾 Image Storage:");
    if let Err(e) = Command::new("docker")
        .args([
            "images",
            "--format",
            "table {{.Repository}}\t{{.Tag}}\t{{.Size}}",
        ])
        .status()
    {
        println!("  Could not list images: {}", e);
    }

    println!("\n🔗 Network Usage:");
    if let Err(e) = Command::new("docker").args(["network", "ls"]).status() {
        println!("  Could not list networks: {}", e);
    }

    println!("\n💿 Volume Usage:");
    if let Err(e) = Command::new("docker").args(["volume", "ls"]).status() {
        println!("  Could not list volumes: {}", e);
    }
}

pub fn docker_system_cleanup() {
    println!("🧹 Docker System Cleanup");
    println!("========================");

    let cleanup_options = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select cleanup operations")
        .items(&[
            "🗑️  Remove stopped containers",
            "🖼️  Remove unused images",
            "💿 Remove unused volumes",
            "🔗 Remove unused networks",
            "🧹 Full system prune",
        ])
        .interact_opt()
    {
        Ok(Some(ops)) => ops,
        Ok(None) | Err(_) => return,
    };

    if cleanup_options.is_empty() {
        println!("❌ No cleanup operations selected");
        return;
    }

    for operation in cleanup_options {
        match operation {
            0 => {
                print!("🗑️  Removing stopped containers... ");
                match Command::new("docker")
                    .args(["container", "prune", "-f"])
                    .status()
                {
                    Ok(s) if s.success() => println!("Done"),
                    Ok(_) => println!("Warning"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            1 => {
                print!("🖼️  Removing unused images... ");
                match Command::new("docker")
                    .args(["image", "prune", "-f"])
                    .status()
                {
                    Ok(s) if s.success() => println!("Done"),
                    Ok(_) => println!("Warning"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            2 => {
                print!("💿 Removing unused volumes... ");
                match Command::new("docker")
                    .args(["volume", "prune", "-f"])
                    .status()
                {
                    Ok(s) if s.success() => println!("Done"),
                    Ok(_) => println!("Warning"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            3 => {
                print!("🔗 Removing unused networks... ");
                match Command::new("docker")
                    .args(["network", "prune", "-f"])
                    .status()
                {
                    Ok(s) if s.success() => println!("Done"),
                    Ok(_) => println!("Warning"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            4 => {
                let confirm = match Confirm::new()
                    .with_prompt("⚠️  This will remove ALL unused data. Continue?")
                    .default(false)
                    .interact_opt()
                {
                    Ok(Some(c)) => c,
                    Ok(None) | Err(_) => continue,
                };

                if confirm {
                    print!("🧹 Running full system prune... ");
                    match Command::new("docker")
                        .args(["system", "prune", "-af", "--volumes"])
                        .status()
                    {
                        Ok(s) if s.success() => println!("Done"),
                        Ok(_) => println!("Warning"),
                        Err(e) => println!("Error: {}", e),
                    }
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

    let registry: String = match Input::new()
        .with_prompt("Registry URL (e.g., docker.cktechx.io)")
        .default("docker.cktechx.io".into())
        .interact_text()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    println!("🔍 Listing images from {}...", registry);

    // This would need registry API integration
    println!("💡 Use: docker search {} or registry API", registry);
}

#[allow(dead_code)]
pub fn push_to_registry() {
    println!("📤 Push Image to Registry");
    println!("========================");

    let image: String = match Input::new().with_prompt("Local image name").interact_text() {
        Ok(i) => i,
        Err(_) => return,
    };

    let registry: String = match Input::new()
        .with_prompt("Registry URL")
        .default("docker.cktechx.io".into())
        .interact_text()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    let tag: String = match Input::new()
        .with_prompt("Tag")
        .default("latest".into())
        .interact_text()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    let full_name = format!("{}/{}:{}", registry, image, tag);

    println!("🏷️  Tagging image...");
    let _ = Command::new("docker")
        .args(["tag", &image, &full_name])
        .status();

    println!("📤 Pushing to registry...");
    let _ = Command::new("docker").args(["push", &full_name]).status();
}

#[allow(dead_code)]
pub fn pull_from_registry() {
    println!("📥 Pull Image from Registry");
    println!("===========================");

    let image: String = match Input::new()
        .with_prompt("Image name (registry/image:tag)")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    println!("📥 Pulling {}...", image);
    let _ = Command::new("docker").args(["pull", &image]).status();
}

#[allow(dead_code)]
pub fn registry_authentication() {
    println!("🔑 Registry Authentication");
    println!("==========================");

    let registry: String = match Input::new()
        .with_prompt("Registry URL")
        .default("docker.cktechx.io".into())
        .interact_text()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    let username: String = match Input::new().with_prompt("Username").interact_text() {
        Ok(u) => u,
        Err(_) => return,
    };

    println!("🔑 Logging into {}...", registry);
    let _ = Command::new("docker")
        .args(["login", &registry, "-u", &username])
        .status();
}

#[allow(dead_code)]
pub fn dockerhub_signin() {
    println!("🐳 DockerHub Sign-in");
    println!("===================");

    let username: String = match Input::new()
        .with_prompt("DockerHub Username")
        .interact_text()
    {
        Ok(u) => u,
        Err(_) => return,
    };

    println!("🔑 Logging into DockerHub...");
    let _ = Command::new("docker")
        .args(["login", "-u", &username])
        .status();
}

#[allow(dead_code)]
pub fn delete_registry_image() {
    println!("🗑️  Delete Registry Image");
    println!("========================");

    let image: String = match Input::new().with_prompt("Image to delete").interact_text() {
        Ok(i) => i,
        Err(_) => return,
    };

    let confirm = match Confirm::new()
        .with_prompt(format!("Delete image {}?", image))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let _ = Command::new("docker").args(["rmi", &image]).status();
    }
}

#[allow(dead_code)]
pub fn registry_statistics() {
    println!("📊 Registry Statistics");
    println!("======================");

    let _ = Command::new("docker")
        .args([
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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🔄 CI/CD Helpers")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

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

    let project_name: String = match Input::new().with_prompt("Project name").interact_text() {
        Ok(n) => n,
        Err(_) => return,
    };

    let features = match MultiSelect::with_theme(&ColorfulTheme::default())
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
        .interact_opt()
    {
        Ok(Some(f)) => f,
        Ok(None) | Err(_) => return,
    };

    let template = generate_rust_workflow_template(&project_name, &features);

    let save_path = format!(".github/workflows/{}-ci.yml", project_name);

    let save = match Confirm::new()
        .with_prompt(format!("Save workflow to {}?", save_path))
        .default(true)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    if save {
        if fs::create_dir_all(".github/workflows").is_err() {
            println!("❌ Failed to create workflow directory");
            return;
        }
        if fs::write(&save_path, template).is_err() {
            println!("❌ Failed to write workflow file");
            return;
        }
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
    println!("⚡ Generating Zig CI/CD Template");
    println!("================================");

    let project_name: String = match Input::new().with_prompt("Project name").interact_text() {
        Ok(n) => n,
        Err(_) => return,
    };

    let features = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select features")
        .items(&[
            "🧪 Unit Tests",
            "📦 Build Release",
            "🔄 Cross-compilation",
            "🐳 Docker Build",
            "📝 Caching",
            "🚀 Auto Release",
        ])
        .interact_opt()
    {
        Ok(Some(f)) => f,
        Ok(None) | Err(_) => return,
    };

    let targets = if features.contains(&2) {
        match MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select target platforms")
            .items(&[
                "x86_64-linux-gnu",
                "x86_64-linux-musl",
                "aarch64-linux-gnu",
                "x86_64-windows",
                "x86_64-macos",
                "aarch64-macos",
            ])
            .interact_opt()
        {
            Ok(Some(t)) => t,
            Ok(None) | Err(_) => vec![0],
        }
    } else {
        vec![]
    };

    let template = generate_zig_workflow_template(&project_name, &features, &targets);

    let save_path = format!(".github/workflows/{}-ci.yml", project_name);

    let save = Confirm::new()
        .with_prompt(format!("Save workflow to {}?", save_path))
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if save {
        if fs::create_dir_all(".github/workflows").is_err() {
            println!("❌ Failed to create workflow directory");
            return;
        }
        if fs::write(&save_path, template).is_err() {
            println!("❌ Failed to write workflow file");
            return;
        }
        println!("✅ Workflow saved to {}", save_path);
    } else {
        println!("📋 Template generated (not saved)");
    }
}

fn generate_zig_workflow_template(
    project_name: &str,
    features: &[usize],
    targets: &[usize],
) -> String {
    let target_names = [
        "x86_64-linux-gnu",
        "x86_64-linux-musl",
        "aarch64-linux-gnu",
        "x86_64-windows",
        "x86_64-macos",
        "aarch64-macos",
    ];

    let mut workflow = format!(
        r#"name: {project_name} CI/CD

on:
  push:
    branches: [main, master]
    tags: ['v*']
  pull_request:

jobs:
  build:
    name: Build & Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Zig
        uses: goto-bus-stop/setup-zig@v2
        with:
          version: 0.13.0
"#,
        project_name = project_name
    );

    if features.contains(&4) {
        // Caching
        workflow.push_str(
            r#"
      - name: Cache Zig
        uses: actions/cache@v4
        with:
          path: |
            ~/.cache/zig
            zig-cache
          key: ${{ runner.os }}-zig-${{ hashFiles('build.zig') }}
"#,
        );
    }

    if features.contains(&0) {
        // Unit tests
        workflow.push_str(
            r#"
      - name: Run tests
        run: zig build test
"#,
        );
    }

    if features.contains(&1) {
        // Build release
        workflow.push_str(
            r#"
      - name: Build release
        run: zig build -Doptimize=ReleaseSafe
"#,
        );
    }

    if features.contains(&2) && !targets.is_empty() {
        // Cross-compilation
        workflow.push_str("\n  cross-compile:\n    name: Cross Compile\n    runs-on: ubuntu-latest\n    strategy:\n      matrix:\n        target:\n");
        for &idx in targets {
            if idx < target_names.len() {
                workflow.push_str(&format!("          - {}\n", target_names[idx]));
            }
        }
        workflow.push_str(
            r#"    steps:
      - uses: actions/checkout@v4
      - uses: goto-bus-stop/setup-zig@v2
        with:
          version: 0.13.0
      - name: Build for ${{ matrix.target }}
        run: zig build -Dtarget=${{ matrix.target }} -Doptimize=ReleaseSafe
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: build-${{ matrix.target }}
          path: zig-out/
"#,
        );
    }

    if features.contains(&3) {
        // Docker build
        workflow.push_str(
            r#"
  docker:
    name: Docker Build
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v4
      - name: Build Docker image
        run: |
          docker build -t ${{ github.repository }}:${{ github.sha }} .
      - name: Login to GHCR
        if: github.event_name != 'pull_request'
        uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}
      - name: Push image
        if: github.event_name != 'pull_request'
        run: |
          docker tag ${{ github.repository }}:${{ github.sha }} ghcr.io/${{ github.repository }}:latest
          docker push ghcr.io/${{ github.repository }}:latest
"#,
        );
    }

    if features.contains(&5) {
        // Auto release
        workflow.push_str(
            r#"
  release:
    name: Create Release
    runs-on: ubuntu-latest
    needs: [build]
    if: startsWith(github.ref, 'refs/tags/v')
    steps:
      - uses: actions/checkout@v4
      - uses: goto-bus-stop/setup-zig@v2
        with:
          version: 0.13.0
      - name: Build release binaries
        run: zig build -Doptimize=ReleaseSafe
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: zig-out/bin/*
          generate_release_notes: true
"#,
        );
    }

    workflow
}

fn docker_multiarch_build() {
    println!("🐳 Docker Multi-architecture Build Setup");
    println!("========================================");

    let options = [
        "🔧 Setup Docker Buildx",
        "📦 Build Multi-arch Image",
        "📝 Generate Buildx Workflow",
        "📋 Show Available Platforms",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multi-arch Build Options")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => setup_docker_buildx(),
        1 => build_multiarch_image(),
        2 => generate_multiarch_workflow(),
        3 => show_buildx_platforms(),
        _ => return,
    }
}

fn setup_docker_buildx() {
    println!("🔧 Setting up Docker Buildx...");

    // Check if buildx is available
    let status = Command::new("docker").args(["buildx", "version"]).status();

    match status {
        Ok(s) if s.success() => println!("✅ Docker Buildx is available"),
        _ => {
            println!("❌ Docker Buildx not available");
            println!("💡 Install Docker Desktop or upgrade Docker Engine");
            return;
        }
    }

    // Create and use a new builder
    let builder_name = "multiarch-builder";

    println!("📦 Creating builder: {}", builder_name);
    let _ = Command::new("docker")
        .args(["buildx", "create", "--name", builder_name, "--use"])
        .status();

    // Bootstrap the builder
    println!("🚀 Bootstrapping builder...");
    let _ = Command::new("docker")
        .args(["buildx", "inspect", "--bootstrap"])
        .status();

    // Install QEMU for cross-platform emulation
    println!("🔄 Installing QEMU for cross-platform builds...");
    let _ = Command::new("docker")
        .args([
            "run",
            "--privileged",
            "--rm",
            "tonistiigi/binfmt",
            "--install",
            "all",
        ])
        .status();

    println!("✅ Docker Buildx setup complete");
    println!("💡 You can now build multi-architecture images");
}

fn build_multiarch_image() {
    println!("📦 Building Multi-architecture Image");

    let image_name: String = match Input::new()
        .with_prompt("Image name (e.g., myapp:latest)")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let platforms = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select target platforms")
        .items(&[
            "linux/amd64",
            "linux/arm64",
            "linux/arm/v7",
            "linux/arm/v6",
            "linux/386",
            "linux/ppc64le",
            "linux/s390x",
        ])
        .interact_opt()
    {
        Ok(Some(p)) if !p.is_empty() => p,
        _ => {
            println!("❌ No platforms selected");
            return;
        }
    };

    let platform_names = [
        "linux/amd64",
        "linux/arm64",
        "linux/arm/v7",
        "linux/arm/v6",
        "linux/386",
        "linux/ppc64le",
        "linux/s390x",
    ];

    let selected_platforms: Vec<&str> = platforms
        .iter()
        .filter_map(|&i| platform_names.get(i).copied())
        .collect();

    let platform_arg = selected_platforms.join(",");

    let push = Confirm::new()
        .with_prompt("Push to registry?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    println!("🔨 Building for platforms: {}", platform_arg);

    let mut args = vec![
        "buildx",
        "build",
        "--platform",
        &platform_arg,
        "-t",
        &image_name,
    ];

    if push {
        args.push("--push");
    } else {
        args.push("--load");
    }
    args.push(".");

    let status = Command::new("docker").args(&args).status();

    match status {
        Ok(s) if s.success() => println!("✅ Multi-arch build complete"),
        _ => println!("❌ Build failed"),
    }
}

fn generate_multiarch_workflow() {
    println!("📝 Generating Multi-arch Build Workflow");

    let image_name: String = match Input::new()
        .with_prompt("Image name (without tag)")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let platforms = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select target platforms")
        .items(&["linux/amd64", "linux/arm64", "linux/arm/v7"])
        .interact_opt()
    {
        Ok(Some(p)) if !p.is_empty() => p,
        _ => vec![0, 1], // Default to amd64 and arm64
    };

    let platform_names = ["linux/amd64", "linux/arm64", "linux/arm/v7"];
    let selected_platforms: Vec<&str> = platforms
        .iter()
        .filter_map(|&i| platform_names.get(i).copied())
        .collect();

    let workflow = format!(
        r#"name: Multi-arch Docker Build

on:
  push:
    branches: [main, master]
    tags: ['v*']
  pull_request:

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{{{ github.repository }}}}

jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Set up QEMU
        uses: docker/setup-qemu-action@v3

      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3

      - name: Login to GHCR
        if: github.event_name != 'pull_request'
        uses: docker/login-action@v3
        with:
          registry: ${{{{ env.REGISTRY }}}}
          username: ${{{{ github.actor }}}}
          password: ${{{{ secrets.GITHUB_TOKEN }}}}

      - name: Extract metadata
        id: meta
        uses: docker/metadata-action@v5
        with:
          images: ${{{{ env.REGISTRY }}}}/{image_name}
          tags: |
            type=ref,event=branch
            type=ref,event=pr
            type=semver,pattern={{{{version}}}}
            type=sha

      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          platforms: {platforms}
          push: ${{{{ github.event_name != 'pull_request' }}}}
          tags: ${{{{ steps.meta.outputs.tags }}}}
          labels: ${{{{ steps.meta.outputs.labels }}}}
          cache-from: type=gha
          cache-to: type=gha,mode=max
"#,
        image_name = image_name,
        platforms = selected_platforms.join(",")
    );

    let save_path = ".github/workflows/docker-multiarch.yml";

    let save = Confirm::new()
        .with_prompt(format!("Save workflow to {}?", save_path))
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if save {
        if fs::create_dir_all(".github/workflows").is_err() {
            println!("❌ Failed to create workflow directory");
            return;
        }
        if fs::write(save_path, workflow).is_err() {
            println!("❌ Failed to write workflow file");
            return;
        }
        println!("✅ Workflow saved to {}", save_path);
    }
}

fn show_buildx_platforms() {
    println!("📋 Available Buildx Platforms");
    println!("=============================\n");

    let _ = Command::new("docker").args(["buildx", "ls"]).status();

    println!("\n💡 Common platforms:");
    println!("  • linux/amd64  - Standard x86_64 Linux");
    println!("  • linux/arm64  - ARM64 (Apple Silicon, AWS Graviton)");
    println!("  • linux/arm/v7 - ARMv7 (Raspberry Pi 3/4 32-bit)");
    println!("  • linux/arm/v6 - ARMv6 (Raspberry Pi Zero)");
    println!("  • linux/386    - x86 32-bit");
}

fn release_automation() {
    println!("🚀 Release Automation Setup");
    println!("===========================");

    let options = [
        "📝 Generate Release Workflow",
        "🏷️  Create Semantic Release Config",
        "📋 Generate Changelog Template",
        "🔧 Setup Version Bumping",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Release Automation Options")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => generate_release_workflow(),
        1 => create_semantic_release_config(),
        2 => generate_changelog_template(),
        3 => setup_version_bumping(),
        _ => return,
    }
}

fn generate_release_workflow() {
    println!("📝 Generating Release Workflow");

    let project_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project type")
        .items(&["🦀 Rust", "⚡ Zig", "🐳 Docker", "📦 Generic"])
        .default(0)
        .interact_opt()
    {
        Ok(Some(t)) => t,
        Ok(None) | Err(_) => return,
    };

    let workflow = match project_type {
        0 => generate_rust_release_workflow(),
        1 => generate_zig_release_workflow(),
        2 => generate_docker_release_workflow(),
        _ => generate_generic_release_workflow(),
    };

    let save_path = ".github/workflows/release.yml";

    let save = Confirm::new()
        .with_prompt(format!("Save workflow to {}?", save_path))
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if save {
        if fs::create_dir_all(".github/workflows").is_err() {
            println!("❌ Failed to create workflow directory");
            return;
        }
        if fs::write(save_path, workflow).is_err() {
            println!("❌ Failed to write workflow file");
            return;
        }
        println!("✅ Workflow saved to {}", save_path);
    }
}

fn generate_rust_release_workflow() -> String {
    r#"name: Release

on:
  push:
    tags: ['v*']

permissions:
  contents: write

jobs:
  build:
    name: Build ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-unknown-linux-musl
            os: ubuntu-latest
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: aarch64-apple-darwin
            os: macos-latest
          - target: x86_64-pc-windows-msvc
            os: windows-latest

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install cross-compilation tools
        if: matrix.target == 'aarch64-unknown-linux-gnu'
        run: |
          sudo apt-get update
          sudo apt-get install -y gcc-aarch64-linux-gnu

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Package (Unix)
        if: runner.os != 'Windows'
        run: |
          cd target/${{ matrix.target }}/release
          tar czvf ../../../${{ github.event.repository.name }}-${{ matrix.target }}.tar.gz *
        shell: bash

      - name: Package (Windows)
        if: runner.os == 'Windows'
        run: |
          cd target/${{ matrix.target }}/release
          7z a ../../../${{ github.event.repository.name }}-${{ matrix.target }}.zip *

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.target }}
          path: ${{ github.event.repository.name }}-${{ matrix.target }}.*

  release:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/download-artifact@v4
        with:
          path: artifacts

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: artifacts/**/*
          generate_release_notes: true
"#
    .to_string()
}

fn generate_zig_release_workflow() -> String {
    r#"name: Release

on:
  push:
    tags: ['v*']

permissions:
  contents: write

jobs:
  build:
    name: Build ${{ matrix.target }}
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target:
          - x86_64-linux-gnu
          - x86_64-linux-musl
          - aarch64-linux-gnu
          - x86_64-windows
          - x86_64-macos
          - aarch64-macos

    steps:
      - uses: actions/checkout@v4
      - uses: goto-bus-stop/setup-zig@v2
        with:
          version: 0.13.0

      - name: Build
        run: zig build -Dtarget=${{ matrix.target }} -Doptimize=ReleaseSafe

      - name: Package
        run: |
          cd zig-out
          tar czvf ../${{ github.event.repository.name }}-${{ matrix.target }}.tar.gz *

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.target }}
          path: ${{ github.event.repository.name }}-${{ matrix.target }}.tar.gz

  release:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/download-artifact@v4
        with:
          path: artifacts

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: artifacts/**/*
          generate_release_notes: true
"#
    .to_string()
}

fn generate_docker_release_workflow() -> String {
    r#"name: Release

on:
  push:
    tags: ['v*']

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  release:
    runs-on: ubuntu-latest
    permissions:
      contents: write
      packages: write

    steps:
      - uses: actions/checkout@v4

      - name: Set up QEMU
        uses: docker/setup-qemu-action@v3

      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3

      - name: Login to GHCR
        uses: docker/login-action@v3
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Extract version
        id: version
        run: echo "VERSION=${GITHUB_REF#refs/tags/v}" >> $GITHUB_OUTPUT

      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          platforms: linux/amd64,linux/arm64
          push: true
          tags: |
            ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:${{ steps.version.outputs.VERSION }}
            ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:latest
          cache-from: type=gha
          cache-to: type=gha,mode=max

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          generate_release_notes: true
"#
    .to_string()
}

fn generate_generic_release_workflow() -> String {
    r#"name: Release

on:
  push:
    tags: ['v*']

permissions:
  contents: write

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build
        run: |
          # Add your build commands here
          echo "Building..."

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          generate_release_notes: true
          # files: |
          #   dist/*
"#
    .to_string()
}

fn create_semantic_release_config() {
    println!("🏷️  Creating Semantic Release Configuration");

    let config = r#"{
  "branches": ["main", "master"],
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    "@semantic-release/github"
  ]
}
"#;

    let save = Confirm::new()
        .with_prompt("Save .releaserc.json?")
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if save {
        if fs::write(".releaserc.json", config).is_err() {
            println!("❌ Failed to write config file");
            return;
        }
        println!("✅ Semantic release config saved to .releaserc.json");
        println!("💡 Install: npm install -D semantic-release @semantic-release/changelog");
    }
}

fn generate_changelog_template() {
    println!("📋 Generating Changelog Template");

    let template = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Vulnerability fixes

## [0.1.0] - YYYY-MM-DD

### Added
- Initial release
"#;

    let save = Confirm::new()
        .with_prompt("Save CHANGELOG.md?")
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if save {
        if fs::write("CHANGELOG.md", template).is_err() {
            println!("❌ Failed to write changelog file");
            return;
        }
        println!("✅ Changelog template saved to CHANGELOG.md");
    }
}

fn setup_version_bumping() {
    println!("🔧 Setting up Version Bumping");

    let project_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project type")
        .items(&[
            "🦀 Rust (Cargo.toml)",
            "📦 Node.js (package.json)",
            "🐍 Python (pyproject.toml)",
        ])
        .default(0)
        .interact_opt()
    {
        Ok(Some(t)) => t,
        Ok(None) | Err(_) => return,
    };

    match project_type {
        0 => {
            println!("💡 Rust version bumping:");
            println!("  Install: cargo install cargo-release");
            println!("  Bump: cargo release patch/minor/major");
        }
        1 => {
            println!("💡 Node.js version bumping:");
            println!("  Patch: npm version patch");
            println!("  Minor: npm version minor");
            println!("  Major: npm version major");
        }
        2 => {
            println!("💡 Python version bumping:");
            println!("  Install: pip install bump2version");
            println!("  Bump: bump2version patch/minor/major");
        }
        _ => {}
    }
}

fn test_coverage_setup() {
    println!("🧪 Test Coverage Setup");
    println!("======================");

    let options = [
        "🦀 Rust (cargo-tarpaulin/llvm-cov)",
        "⚡ Zig (built-in coverage)",
        "🐳 Docker (container coverage)",
        "📝 Generate Coverage Workflow",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Coverage Setup Options")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => setup_rust_coverage(),
        1 => setup_zig_coverage(),
        2 => setup_docker_coverage(),
        3 => generate_coverage_workflow(),
        _ => return,
    }
}

fn setup_rust_coverage() {
    println!("🦀 Setting up Rust Code Coverage");

    let tool = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select coverage tool")
        .items(&[
            "cargo-tarpaulin (Recommended)",
            "cargo-llvm-cov (LLVM-based)",
            "grcov (Mozilla)",
        ])
        .default(0)
        .interact_opt()
    {
        Ok(Some(t)) => t,
        Ok(None) | Err(_) => return,
    };

    match tool {
        0 => {
            println!("📦 Installing cargo-tarpaulin...");
            let _ = Command::new("cargo")
                .args(["install", "cargo-tarpaulin"])
                .status();
            println!("\n💡 Usage:");
            println!("  cargo tarpaulin --out Html");
            println!("  cargo tarpaulin --out Xml --output-dir coverage");
        }
        1 => {
            println!("📦 Installing cargo-llvm-cov...");
            let _ = Command::new("cargo")
                .args(["install", "cargo-llvm-cov"])
                .status();
            println!("\n💡 Usage:");
            println!("  cargo llvm-cov --html");
            println!("  cargo llvm-cov --lcov --output-path lcov.info");
        }
        2 => {
            println!("📦 Installing grcov...");
            let _ = Command::new("cargo").args(["install", "grcov"]).status();
            println!("\n💡 Usage:");
            println!("  CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' cargo test");
            println!(
                "  grcov . -s . --binary-path ./target/debug/ -t html --branch -o ./coverage/"
            );
        }
        _ => {}
    }
}

fn setup_zig_coverage() {
    println!("⚡ Zig Code Coverage");
    println!("====================\n");

    println!("Zig has built-in code coverage support:\n");
    println!("💡 Generate coverage:");
    println!("  zig build test -Dcoverage");
    println!("  zig build -Dcoverage\n");

    println!("💡 View coverage report:");
    println!("  kcov --include-path=src target/coverage ./zig-out/bin/test\n");

    println!("💡 For CI, add to build.zig:");
    println!("  const coverage = b.option(bool, \"coverage\", \"Enable coverage\") orelse false;");
}

fn setup_docker_coverage() {
    println!("🐳 Docker Container Coverage");
    println!("============================\n");

    println!("💡 For containerized tests with coverage:\n");

    let dockerfile = r#"# Coverage Dockerfile
FROM rust:latest as builder

WORKDIR /app
COPY . .

# Install coverage tool
RUN cargo install cargo-tarpaulin

# Run tests with coverage
RUN cargo tarpaulin --out Xml --output-dir /coverage

# Export coverage
FROM scratch as coverage
COPY --from=builder /coverage /
"#;

    println!("Example Dockerfile:\n{}", dockerfile);

    let save = Confirm::new()
        .with_prompt("Save Dockerfile.coverage?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if save {
        if fs::write("Dockerfile.coverage", dockerfile).is_err() {
            println!("❌ Failed to write Dockerfile");
            return;
        }
        println!("✅ Saved to Dockerfile.coverage");
    }
}

fn generate_coverage_workflow() {
    println!("📝 Generating Coverage Workflow");

    let workflow = r#"name: Code Coverage

on:
  push:
    branches: [main, master]
  pull_request:

jobs:
  coverage:
    name: Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          components: llvm-tools-preview

      - name: Install cargo-llvm-cov
        uses: taiki-e/install-action@cargo-llvm-cov

      - name: Generate coverage
        run: cargo llvm-cov --all-features --lcov --output-path lcov.info

      - name: Upload to Codecov
        uses: codecov/codecov-action@v4
        with:
          files: lcov.info
          fail_ci_if_error: true
"#;

    let save_path = ".github/workflows/coverage.yml";

    let save = Confirm::new()
        .with_prompt(format!("Save workflow to {}?", save_path))
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if save {
        if fs::create_dir_all(".github/workflows").is_err() {
            println!("❌ Failed to create workflow directory");
            return;
        }
        if fs::write(save_path, workflow).is_err() {
            println!("❌ Failed to write workflow file");
            return;
        }
        println!("✅ Workflow saved to {}", save_path);
    }
}

fn security_scanning_setup() {
    println!("🛡️  Security Scanning Setup");
    println!("===========================");

    let options = [
        "🔍 Container Vulnerability Scanning (Trivy)",
        "🔐 Dependency Audit (Rust/Node)",
        "📋 Generate Security Workflow",
        "🛡️  Setup SAST (Static Analysis)",
        "🔑 Secret Scanning Setup",
        "⬅️  Back",
    ];

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Security Scanning Options")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => setup_trivy_scanning(),
        1 => setup_dependency_audit(),
        2 => generate_security_workflow(),
        3 => setup_sast(),
        4 => setup_secret_scanning(),
        _ => return,
    }
}

fn setup_trivy_scanning() {
    println!("🔍 Setting up Trivy Container Scanning");

    // Check if Trivy is installed
    let trivy_check = Command::new("which")
        .arg("trivy")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !trivy_check {
        let install = Confirm::new()
            .with_prompt("Trivy not found. Install it?")
            .default(true)
            .interact_opt()
            .ok()
            .flatten()
            .unwrap_or(false);

        if install {
            println!("📦 Installing Trivy...");
            let _ = Command::new("sh")
                .arg("-c")
                .arg("curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh -s -- -b /usr/local/bin")
                .status();
        }
    }

    println!("\n💡 Trivy Usage:");
    println!("  Scan image:     trivy image nginx:latest");
    println!("  Scan filesystem: trivy fs .");
    println!("  Scan config:    trivy config .");
    println!("  High/Critical:  trivy image --severity HIGH,CRITICAL nginx:latest");
}

fn setup_dependency_audit() {
    println!("🔐 Setting up Dependency Audit");

    let project_type = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project type")
        .items(&["🦀 Rust", "📦 Node.js", "🐍 Python"])
        .default(0)
        .interact_opt()
    {
        Ok(Some(t)) => t,
        Ok(None) | Err(_) => return,
    };

    match project_type {
        0 => {
            println!("📦 Installing cargo-audit...");
            let _ = Command::new("cargo")
                .args(["install", "cargo-audit"])
                .status();
            println!("\n💡 Usage: cargo audit");
        }
        1 => {
            println!("\n💡 Node.js audit commands:");
            println!("  npm audit");
            println!("  npm audit fix");
            println!("  npx audit-ci --moderate");
        }
        2 => {
            println!("\n💡 Python audit:");
            println!("  pip install safety");
            println!("  safety check");
            println!("  pip-audit");
        }
        _ => {}
    }
}

fn generate_security_workflow() {
    println!("📋 Generating Security Scanning Workflow");

    let workflow = r#"name: Security Scanning

on:
  push:
    branches: [main, master]
  pull_request:
  schedule:
    - cron: '0 0 * * 0'  # Weekly on Sunday

jobs:
  trivy:
    name: Trivy Container Scan
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build Docker image
        run: docker build -t scan-target:${{ github.sha }} .

      - name: Run Trivy vulnerability scanner
        uses: aquasecurity/trivy-action@master
        with:
          image-ref: 'scan-target:${{ github.sha }}'
          format: 'sarif'
          output: 'trivy-results.sarif'
          severity: 'CRITICAL,HIGH'

      - name: Upload Trivy scan results
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: 'trivy-results.sarif'

  dependency-audit:
    name: Dependency Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-audit
        run: cargo install cargo-audit

      - name: Run audit
        run: cargo audit

  codeql:
    name: CodeQL Analysis
    runs-on: ubuntu-latest
    permissions:
      security-events: write
    steps:
      - uses: actions/checkout@v4

      - name: Initialize CodeQL
        uses: github/codeql-action/init@v2
        with:
          languages: 'python,javascript'

      - name: Perform CodeQL Analysis
        uses: github/codeql-action/analyze@v2
"#;

    let save_path = ".github/workflows/security.yml";

    let save = Confirm::new()
        .with_prompt(format!("Save workflow to {}?", save_path))
        .default(true)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if save {
        if fs::create_dir_all(".github/workflows").is_err() {
            println!("❌ Failed to create workflow directory");
            return;
        }
        if fs::write(save_path, workflow).is_err() {
            println!("❌ Failed to write workflow file");
            return;
        }
        println!("✅ Workflow saved to {}", save_path);
    }
}

fn setup_sast() {
    println!("🛡️  Setting up Static Application Security Testing");

    println!("\n💡 SAST Tools:");
    println!("  • Semgrep - Multi-language static analysis");
    println!("  • CodeQL - GitHub's semantic code analysis");
    println!("  • Clippy - Rust lints (cargo clippy)");
    println!("  • Bandit - Python security linter\n");

    let install_semgrep = Confirm::new()
        .with_prompt("Install Semgrep?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if install_semgrep {
        println!("📦 Installing Semgrep...");
        let _ = Command::new("pip").args(["install", "semgrep"]).status();
        println!("\n💡 Usage: semgrep --config auto .");
    }
}

fn setup_secret_scanning() {
    println!("🔑 Setting up Secret Scanning");

    println!("\n💡 Secret scanning options:");
    println!("  • GitHub Secret Scanning (built-in for public repos)");
    println!("  • Gitleaks - Detect secrets in git repos");
    println!("  • TruffleHog - Find secrets in commits\n");

    let install_gitleaks = Confirm::new()
        .with_prompt("Install Gitleaks?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if install_gitleaks {
        println!("📦 Installing Gitleaks...");
        let _ = Command::new("sh")
            .arg("-c")
            .arg("curl -sSfL https://github.com/gitleaks/gitleaks/releases/latest/download/gitleaks_$(uname -s)_$(uname -m).tar.gz | tar xz && sudo mv gitleaks /usr/local/bin/")
            .status();
        println!("\n💡 Usage: gitleaks detect");
    }

    // Create pre-commit hook option
    let create_hook = Confirm::new()
        .with_prompt("Create pre-commit hook for secret scanning?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    if create_hook {
        let hook_content = r#"#!/bin/bash
# Pre-commit hook for secret scanning

if command -v gitleaks &> /dev/null; then
    gitleaks protect --staged
    if [ $? -ne 0 ]; then
        echo "❌ Secrets detected! Commit blocked."
        exit 1
    fi
fi
"#;

        if fs::create_dir_all(".git/hooks").is_ok()
            && fs::write(".git/hooks/pre-commit", hook_content).is_ok()
        {
            let _ = Command::new("chmod")
                .args(["+x", ".git/hooks/pre-commit"])
                .status();
            println!("✅ Pre-commit hook installed");
        }
    }
}

pub fn monitoring_tools() {
    println!("📊 Setting up Infrastructure Monitoring");

    let tools = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select monitoring tools")
        .items(&[
            "📈 Prometheus + Grafana",
            "📊 ELK Stack",
            "🔍 Loki + Grafana",
            "📡 Node Exporter",
            "🐳 cAdvisor",
        ])
        .interact_opt()
    {
        Ok(Some(t)) => t,
        Ok(None) | Err(_) => return,
    };

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

    if std::fs::create_dir_all("monitoring/prometheus").is_err() {
        println!("❌ Failed to create monitoring directory");
        return;
    }
    if std::fs::write("monitoring/prometheus/prometheus.yml", prometheus_config).is_err() {
        println!("❌ Failed to write Prometheus config");
        return;
    }

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

    if std::fs::write("docker-compose.monitoring.yml", &compose).is_err() {
        println!("❌ Failed to write docker-compose.monitoring.yml");
        return compose;
    }
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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Security Scanning Options")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

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
    let image: String = match Input::new()
        .with_prompt("Image name to scan")
        .interact_text()
    {
        Ok(i) => i,
        Err(_) => return,
    };

    println!("🔍 Scanning image: {}", image);
    let _ = Command::new("trivy").args(["image", &image]).status();
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
        .args([
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
    let query: String = match Input::new().with_prompt("Search query").interact_text() {
        Ok(q) => q,
        Err(_) => return,
    };

    println!("🔍 Searching for: {}", query);
    let _ = Command::new("docker").args(["search", &query]).status();
}

// Wrapper functions for CLI usage
pub fn scan_local_image_with_name(image: &str) {
    println!("🔍 Scanning image: {}", image);
    let _ = Command::new("trivy").args(["image", image]).status();
}

pub fn search_registry_with_query(query: &str) {
    println!("🔍 Searching for: {}", query);
    let _ = Command::new("docker").args(["search", query]).status();
}

/// Generate a GitHub Actions workflow for Rust projects
pub fn generate_rust_workflow(project_name: &str, features: &RustWorkflowFeatures) -> String {
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

    if features.caching {
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

    if features.security_audit {
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

    if features.coverage {
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

/// Features for Rust CI/CD workflow generation
#[derive(Debug, Clone, Default)]
pub struct RustWorkflowFeatures {
    pub unit_tests: bool,
    pub security_audit: bool,
    pub coverage: bool,
    pub multi_target: bool,
    pub auto_release: bool,
    pub docker_build: bool,
    pub caching: bool,
}

/// Parse Docker stats output into structured data
pub fn parse_docker_stats(output: &str) -> Vec<ContainerStats> {
    let mut stats = Vec::new();

    for line in output.lines().skip(1) {
        // Skip header
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 7 {
            stats.push(ContainerStats {
                container_id: parts[0].to_string(),
                name: parts[1].to_string(),
                cpu_percent: parts[2].trim_end_matches('%').parse().unwrap_or(0.0),
                mem_usage: parts[3].to_string(),
                mem_limit: parts.get(5).unwrap_or(&"").to_string(),
                net_io: parts.get(6).unwrap_or(&"").to_string(),
            });
        }
    }

    stats
}

/// Container statistics
#[derive(Debug, Clone)]
pub struct ContainerStats {
    pub container_id: String,
    pub name: String,
    pub cpu_percent: f64,
    pub mem_usage: String,
    pub mem_limit: String,
    pub net_io: String,
}

/// Validate Prometheus configuration YAML
pub fn validate_prometheus_config(config: &str) -> Result<(), String> {
    // Basic validation of Prometheus config structure
    if !config.contains("scrape_configs:") {
        return Err("Missing 'scrape_configs' section".to_string());
    }

    if !config.contains("job_name:") {
        return Err("No job definitions found".to_string());
    }

    Ok(())
}

/// Extract job names from Prometheus config
pub fn extract_prometheus_jobs(config: &str) -> Vec<String> {
    let mut jobs = Vec::new();

    for line in config.lines() {
        let trimmed = line.trim();
        if (trimmed.starts_with("- job_name:") || trimmed.starts_with("-job_name:"))
            && let Some(name) = trimmed
                .split(':')
                .nth(1)
                .map(|s| s.trim().trim_matches('\'').trim_matches('"'))
            && !name.is_empty()
        {
            jobs.push(name.to_string());
        }
    }

    jobs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_rust_workflow_basic() {
        let features = RustWorkflowFeatures::default();
        let workflow = generate_rust_workflow("test-project", &features);

        assert!(workflow.contains("name: test-project CI/CD"));
        assert!(workflow.contains("cargo test"));
        assert!(workflow.contains("cargo fmt"));
        assert!(workflow.contains("cargo clippy"));
    }

    #[test]
    fn test_generate_rust_workflow_with_caching() {
        let features = RustWorkflowFeatures {
            caching: true,
            ..Default::default()
        };
        let workflow = generate_rust_workflow("test-project", &features);

        assert!(workflow.contains("actions/cache@v3"));
        assert!(workflow.contains("Cargo.lock"));
    }

    #[test]
    fn test_generate_rust_workflow_with_security() {
        let features = RustWorkflowFeatures {
            security_audit: true,
            ..Default::default()
        };
        let workflow = generate_rust_workflow("test-project", &features);

        assert!(workflow.contains("Security Audit"));
        assert!(workflow.contains("rustsec/audit-check"));
    }

    #[test]
    fn test_generate_rust_workflow_with_coverage() {
        let features = RustWorkflowFeatures {
            coverage: true,
            ..Default::default()
        };
        let workflow = generate_rust_workflow("test-project", &features);

        assert!(workflow.contains("Code Coverage"));
        assert!(workflow.contains("cargo-tarpaulin"));
        assert!(workflow.contains("codecov"));
    }

    #[test]
    fn test_rust_workflow_features_default() {
        let features = RustWorkflowFeatures::default();
        assert!(!features.unit_tests);
        assert!(!features.security_audit);
        assert!(!features.coverage);
        assert!(!features.multi_target);
        assert!(!features.auto_release);
        assert!(!features.docker_build);
        assert!(!features.caching);
    }

    #[test]
    fn test_validate_prometheus_config_valid() {
        let config = r#"
scrape_configs:
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']
"#;
        assert!(validate_prometheus_config(config).is_ok());
    }

    #[test]
    fn test_validate_prometheus_config_missing_scrape() {
        let config = r#"
global:
  scrape_interval: 15s
"#;
        let result = validate_prometheus_config(config);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("scrape_configs"));
    }

    #[test]
    fn test_validate_prometheus_config_no_jobs() {
        let config = r#"
scrape_configs:
"#;
        let result = validate_prometheus_config(config);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("job"));
    }

    #[test]
    fn test_extract_prometheus_jobs() {
        let config = r#"
scrape_configs:
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']
  - job_name: 'node'
    static_configs:
      - targets: ['localhost:9100']
  - job_name: "cadvisor"
    static_configs:
      - targets: ['localhost:8080']
"#;
        let jobs = extract_prometheus_jobs(config);
        assert_eq!(jobs.len(), 3);
        assert!(jobs.contains(&"prometheus".to_string()));
        assert!(jobs.contains(&"node".to_string()));
        assert!(jobs.contains(&"cadvisor".to_string()));
    }

    #[test]
    fn test_extract_prometheus_jobs_empty() {
        let config = "";
        let jobs = extract_prometheus_jobs(config);
        assert!(jobs.is_empty());
    }

    #[test]
    fn test_container_stats_struct() {
        let stats = ContainerStats {
            container_id: "abc123".to_string(),
            name: "web".to_string(),
            cpu_percent: 5.5,
            mem_usage: "100MiB".to_string(),
            mem_limit: "512MiB".to_string(),
            net_io: "1.2kB / 3.4kB".to_string(),
        };

        assert_eq!(stats.container_id, "abc123");
        assert_eq!(stats.name, "web");
        assert_eq!(stats.cpu_percent, 5.5);
    }
}
