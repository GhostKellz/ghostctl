use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

/// Validate project/crate name
fn validate_project_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("Project name cannot be empty");
    }
    if name.len() > 64 {
        return Err("Project name too long");
    }
    // Cargo crate names: alphanumeric, hyphen, underscore (no leading hyphen/underscore)
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err("Project name contains invalid characters");
    }
    if name.starts_with('-') || name.starts_with('_') {
        return Err("Project name cannot start with hyphen or underscore");
    }
    // Cannot be a Rust keyword
    let keywords = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn",
    ];
    if keywords.contains(&name) {
        return Err("Project name cannot be a Rust keyword");
    }
    Ok(())
}

/// Validate Rust target triple
fn validate_target(target: &str) -> Result<(), &'static str> {
    if target.is_empty() {
        return Err("Target cannot be empty");
    }
    if target.len() > 100 {
        return Err("Target too long");
    }
    // Target triples: alphanumeric, hyphen, underscore
    if !target
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err("Target contains invalid characters");
    }
    Ok(())
}

pub fn rust_development() {
    println!("🦀 Rust Development Environment");
    println!("===============================");

    let options = [
        "📦 Install/Update Rust toolchain",
        "🛠️  Install Oxygen (GhostKellz tool)",
        "🚀 Create new Rust project",
        "🔧 Rust development tools",
        "📊 Cargo utilities",
        "📚 Rust resources",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Rust Development")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => install_rust_toolchain(),
        1 => install_oxygen(),
        2 => create_rust_project(),
        3 => rust_development_tools(),
        4 => cargo_utilities(),
        5 => rust_resources(),
        _ => return,
    }
}

fn install_rust_toolchain() {
    println!("📦 Rust Toolchain Management");
    println!("============================");

    // Check if rustup is installed
    if Command::new("which").arg("rustup").status().is_ok()
        && let Ok(output) = Command::new("rustc").arg("--version").output()
    {
        let version = String::from_utf8_lossy(&output.stdout);
        println!("✅ Rust is installed: {}", version.trim());

        let actions = [
            "🔄 Update Rust",
            "🎯 Add targets",
            "🔧 Add components",
            "📋 Show toolchain info",
        ];

        let Ok(action) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Rust Actions")
            .items(&actions)
            .default(0)
            .interact()
        else {
            return;
        };

        match action {
            0 => update_rust(),
            1 => add_rust_targets(),
            2 => add_rust_components(),
            3 => show_rust_info(),
            _ => return,
        }
        return;
    }

    // Install Rust
    let install_methods = [
        "🌐 Official rustup installer (Recommended)",
        "📦 Package manager",
    ];

    let Ok(method) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Installation method")
        .items(&install_methods)
        .default(0)
        .interact()
    else {
        return;
    };

    match method {
        0 => install_rustup(),
        1 => install_rust_package_manager(),
        _ => return,
    }
}

fn install_rustup() {
    println!("🌐 Installing Rust via rustup...");

    let status = Command::new("bash")
        .arg("-c")
        .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Rust installed successfully!");
            println!("🔄 Sourcing environment...");

            // Source the cargo env
            println!("💡 Run: source ~/.cargo/env");
            println!("💡 Or restart your terminal");

            install_default_rust_tools();
        }
        _ => println!("❌ Failed to install Rust"),
    }
}

fn install_rust_package_manager() {
    println!("Installing Rust via package manager...");

    let install_result = if Command::new("which")
        .arg("pacman")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "rust", "cargo"])
            .status()
    } else if Command::new("which")
        .arg("apt")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        if let Err(e) = Command::new("sudo").args(["apt", "update"]).status() {
            eprintln!("Warning: apt update failed: {}", e);
        }
        Command::new("sudo")
            .args(["apt", "install", "-y", "rustc", "cargo"])
            .status()
    } else if Command::new("which")
        .arg("dnf")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        Command::new("sudo")
            .args(["dnf", "install", "-y", "rust", "cargo"])
            .status()
    } else {
        eprintln!("No supported package manager found");
        return;
    };

    match install_result {
        Ok(status) if status.success() => {
            println!("Installation completed");
        }
        Ok(status) => {
            eprintln!("Installation exited with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run installation: {}", e);
        }
    }

    if Command::new("which")
        .arg("cargo")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Rust installed via package manager");
        install_default_rust_tools();
    } else {
        eprintln!("Rust installation failed");
    }
}

fn install_default_rust_tools() {
    println!("Installing essential Rust tools...");

    // Trusted tool names from crates.io
    let tools = [
        ("cargo-edit", "Add/remove dependencies"),
        ("cargo-watch", "Watch for changes"),
    ];

    for (tool, description) in &tools {
        println!("Installing {} - {}", tool, description);
        match Command::new("cargo").args(["install", tool]).status() {
            Ok(status) if status.success() => {
                println!("{} installed", tool);
            }
            Ok(status) => {
                eprintln!("Failed to install {} (code: {:?})", tool, status.code());
            }
            Err(e) => {
                eprintln!("Failed to run cargo install {}: {}", tool, e);
            }
        }
    }

    // Install clippy and rustfmt via rustup (they're components, not crates)
    println!("Installing clippy and rustfmt via rustup...");
    if let Err(e) = Command::new("rustup")
        .args(["component", "add", "clippy", "rustfmt"])
        .status()
    {
        eprintln!("Warning: Failed to add components: {}", e);
    }
}

fn update_rust() {
    println!("Updating Rust toolchain...");

    match Command::new("rustup").args(["update"]).status() {
        Ok(status) if status.success() => {
            println!("Rust updated successfully");
        }
        Ok(status) => {
            eprintln!("rustup update failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run rustup: {}", e);
        }
    }
}

fn add_rust_targets() {
    println!("Add Rust Targets");
    println!("===================");

    // Known valid target triples
    let targets = [
        "wasm32-unknown-unknown (WebAssembly)",
        "x86_64-pc-windows-gnu (Windows cross-compile)",
        "aarch64-unknown-linux-gnu (ARM64 Linux)",
        "x86_64-apple-darwin (macOS cross-compile)",
        "Custom target",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select target to add")
        .items(&targets)
        .default(0)
        .interact()
    else {
        return;
    };

    let target = match choice {
        0 => "wasm32-unknown-unknown".to_string(),
        1 => "x86_64-pc-windows-gnu".to_string(),
        2 => "aarch64-unknown-linux-gnu".to_string(),
        3 => "x86_64-apple-darwin".to_string(),
        4 => {
            let Ok(custom) = Input::<String>::new()
                .with_prompt("Enter target triple")
                .interact_text()
            else {
                return;
            };
            // Validate custom target
            if let Err(e) = validate_target(&custom) {
                eprintln!("Invalid target: {}", e);
                return;
            }
            custom
        }
        _ => return,
    };

    println!("Adding target: {}", target);
    match Command::new("rustup")
        .args(["target", "add", &target])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Target {} added", target);
        }
        Ok(status) => {
            eprintln!("Failed to add target (code: {:?})", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run rustup: {}", e);
        }
    }
}

fn add_rust_components() {
    println!("Add Rust Components");
    println!("======================");

    // Known valid rustup components
    let components = [
        "clippy (Linter)",
        "rustfmt (Formatter)",
        "rust-src (Source code)",
        "rust-analysis (IDE support)",
        "rls (Language server - legacy)",
        "rust-analyzer (Language server - modern)",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select component to add")
        .items(&components)
        .default(0)
        .interact()
    else {
        return;
    };

    let component = match choice {
        0 => "clippy",
        1 => "rustfmt",
        2 => "rust-src",
        3 => "rust-analysis",
        4 => "rls",
        5 => "rust-analyzer",
        _ => return,
    };

    println!("Adding component: {}", component);
    match Command::new("rustup")
        .args(["component", "add", component])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Component {} added", component);
        }
        Ok(status) => {
            eprintln!("Failed to add component (code: {:?})", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run rustup: {}", e);
        }
    }
}

fn show_rust_info() {
    println!("Rust Toolchain Information");
    println!("=============================");

    println!("Rust version:");
    if let Err(e) = Command::new("rustc").arg("--version").status() {
        eprintln!("Failed to get rustc version: {}", e);
    }

    println!("\nCargo version:");
    if let Err(e) = Command::new("cargo").arg("--version").status() {
        eprintln!("Failed to get cargo version: {}", e);
    }

    println!("\nInstalled toolchains:");
    if let Err(e) = Command::new("rustup").args(["toolchain", "list"]).status() {
        eprintln!("Failed to list toolchains: {}", e);
    }

    println!("\nInstalled targets:");
    if let Err(e) = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .status()
    {
        eprintln!("Failed to list targets: {}", e);
    }

    println!("\nInstalled components:");
    if let Err(e) = Command::new("rustup")
        .args(["component", "list", "--installed"])
        .status()
    {
        eprintln!("Failed to list components: {}", e);
    }
}

fn install_oxygen() {
    println!("🛠️  Installing Oxygen (GhostKellz Tool)");
    println!("=======================================");

    // Check if already installed
    if Command::new("which").arg("oxygen").status().is_ok() {
        println!("✅ Oxygen is already installed");

        let Ok(action) = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Oxygen Options")
            .items(&["🔄 Update Oxygen", "📋 Show Oxygen Info", "⬅️  Back"])
            .default(0)
            .interact()
        else {
            return;
        };

        match action {
            0 => {
                println!("🔄 Updating Oxygen...");
                install_oxygen_script();
            }
            1 => {
                if let Ok(output) = Command::new("oxygen").arg("--version").output() {
                    println!(
                        "📋 Oxygen version: {}",
                        String::from_utf8_lossy(&output.stdout)
                    );
                }
                println!("🔗 Repository: https://github.com/GhostKellz/oxygen");
            }
            _ => return,
        }
        return;
    }

    let Ok(confirm) = Confirm::new()
        .with_prompt("Install Oxygen via official installer?")
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        install_oxygen_script();
    }
}

fn install_oxygen_script() {
    println!("📥 Installing Oxygen via official installer...");

    let status = Command::new("bash")
        .arg("-c")
        .arg("curl -sSL https://raw.githubusercontent.com/GhostKellz/oxygen/main/install.sh | bash")
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Oxygen installed successfully!");
            println!("💡 Oxygen is GhostKellz's Rust development toolkit");
            println!("📚 Use 'oxygen --help' for available commands");
        }
        _ => println!("❌ Failed to install Oxygen"),
    }
}

fn create_rust_project() {
    println!("Create New Rust Project");
    println!("==========================");

    let Ok(project_name) = Input::<String>::new()
        .with_prompt("Project name")
        .interact_text()
    else {
        return;
    };

    // Validate project name
    if let Err(e) = validate_project_name(&project_name) {
        eprintln!("Invalid project name: {}", e);
        return;
    }

    let project_types = [
        "Binary (executable)",
        "Library",
        "Web project (with framework)",
        "Game project",
        "CLI tool",
    ];

    let Ok(project_type) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project type")
        .items(&project_types)
        .default(0)
        .interact()
    else {
        return;
    };

    let mut args = vec!["new"];

    match project_type {
        1 => args.push("--lib"),
        _ => args.push("--bin"),
    }

    args.push(&project_name);

    println!("Creating Rust project: {}", project_name);
    match Command::new("cargo").args(&args).status() {
        Ok(s) if s.success() => {
            println!("Rust project '{}' created successfully!", project_name);

            // Add framework-specific setup
            match project_type {
                2 => setup_web_project(&project_name),
                3 => setup_game_project(&project_name),
                4 => setup_cli_project(&project_name),
                _ => {}
            }

            println!("Project directory: ./{}", project_name);
            println!("Build with: cd {} && cargo build", project_name);
            println!("Run with: cd {} && cargo run", project_name);
        }
        Ok(status) => {
            eprintln!("cargo new failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run cargo: {}", e);
        }
    }
}

fn setup_web_project(project_name: &str) {
    println!("🌐 Setting up web project dependencies...");

    let frameworks = [
        "🚀 Axum (async web framework)",
        "⚡ Actix-web (actor-based)",
        "🌊 Warp (filter-based)",
        "📡 Rocket (type-safe)",
    ];

    let Ok(framework) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Web framework")
        .items(&frameworks)
        .default(0)
        .interact()
    else {
        return;
    };

    // Trusted crate names from crates.io
    let dependency = match framework {
        0 => "axum",
        1 => "actix-web",
        2 => "warp",
        3 => "rocket",
        _ => return,
    };

    if let Err(e) = Command::new("cargo")
        .args(["add", dependency, "tokio", "serde"])
        .current_dir(project_name)
        .status()
    {
        eprintln!("Failed to add dependencies: {}", e);
    }
}

fn setup_game_project(project_name: &str) {
    println!("Setting up game project dependencies...");

    let engines = [
        "Bevy (ECS-based)",
        "Macroquad (simple 2D)",
        "ggez (2D game library)",
    ];

    let Ok(engine) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Game engine")
        .items(&engines)
        .default(0)
        .interact()
    else {
        return;
    };

    // Trusted crate names from crates.io
    let dependency = match engine {
        0 => "bevy",
        1 => "macroquad",
        2 => "ggez",
        _ => return,
    };

    if let Err(e) = Command::new("cargo")
        .args(["add", dependency])
        .current_dir(project_name)
        .status()
    {
        eprintln!("Failed to add game engine dependency: {}", e);
    }
}

fn setup_cli_project(project_name: &str) {
    println!("Setting up CLI project dependencies...");

    // Trusted crate names from crates.io
    if let Err(e) = Command::new("cargo")
        .args(["add", "clap", "anyhow", "serde"])
        .current_dir(project_name)
        .status()
    {
        eprintln!("Failed to add CLI dependencies: {}", e);
    }
}

fn rust_development_tools() {
    println!("🛠️  Rust Development Tools");
    println!("==========================");

    let tools = [
        "cargo-watch - Watch for changes and rebuild",
        "cargo-edit - Add/remove dependencies from CLI",
        "cargo-audit - Security vulnerability scanner",
        "cargo-outdated - Check for outdated dependencies",
        "cargo-tree - Display dependency tree",
    ];

    for tool in &tools {
        println!("  • {}", tool);
    }

    println!("\n💡 Use 'cargo install <tool-name>' to install");
}

fn cargo_utilities() {
    println!("Cargo Utilities");
    println!("==================");

    // Trusted cargo tool names from crates.io
    let utilities = [
        "cargo-audit (Security audit)",
        "cargo-bloat (Binary size analysis)",
        "cargo-sweep (Clean old builds)",
        "cargo-nextest (Next-gen test runner)",
        "cargo-edit (Dependency management)",
        "cargo-watch (Auto rebuild)",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Install utility")
        .items(&utilities)
        .default(0)
        .interact()
    else {
        return;
    };

    let tool = match choice {
        0 => "cargo-audit",
        1 => "cargo-bloat",
        2 => "cargo-sweep",
        3 => "cargo-nextest",
        4 => "cargo-edit",
        5 => "cargo-watch",
        _ => return,
    };

    println!("Installing {}...", tool);
    match Command::new("cargo").args(["install", tool]).status() {
        Ok(status) if status.success() => {
            println!("{} installed successfully", tool);
        }
        Ok(status) => {
            eprintln!("Installation failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run cargo install: {}", e);
        }
    }
}

fn rust_resources() {
    println!("📚 Rust Learning Resources");
    println!("==========================");

    println!("🌐 Official Resources:");
    println!("  • https://rust-lang.org/ - Official website");
    println!("  • https://doc.rust-lang.org/book/ - The Rust Book");
    println!("  • https://doc.rust-lang.org/rust-by-example/ - Rust by Example");

    println!("\n📖 Learning Resources:");
    println!("  • https://rustlings.cool/ - Interactive exercises");
    println!("  • https://exercism.org/tracks/rust - Coding exercises");
    println!("  • https://rust-unofficial.github.io/too-many-lists/ - Data structures");

    println!("\n🛠️  GhostKellz Tools:");
    println!("  • https://github.com/GhostKellz/oxygen - Oxygen development toolkit");

    println!("\n💡 Quick Start:");
    println!("  1. Install Rust with rustup");
    println!("  2. Install Oxygen for enhanced development");
    println!("  3. Create a new project with 'cargo new'");
    println!("  4. Build with 'cargo build'");
    println!("  5. Run with 'cargo run'");
}
