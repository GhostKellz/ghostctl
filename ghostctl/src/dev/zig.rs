use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

/// Validate Zig project name
fn validate_project_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("Project name cannot be empty");
    }
    if name.len() > 64 {
        return Err("Project name too long");
    }
    // Zig identifiers: alphanumeric and underscore
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err("Project name contains invalid characters");
    }
    if name.starts_with('-') || name.starts_with('_') {
        return Err("Project name cannot start with hyphen or underscore");
    }
    Ok(())
}

pub fn zig_development_menu() {
    println!("⚡ Zig Development Environment");
    println!("=============================");

    let options = [
        "📦 Install Zig Compiler",
        "🛠️  Install Zion (Zig Meta Tool)",
        "📋 Zig Project Management",
        "🔧 Development Tools",
        "📚 Learning Resources",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Zig Development")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => install_zig_compiler(),
        1 => install_zion_tool(),
        2 => zig_project_management(),
        3 => zig_development_tools(),
        4 => zig_learning_resources(),
        _ => return,
    }
}

fn install_zig_compiler() {
    println!("📦 Installing Zig Compiler");
    println!("===========================");

    if Command::new("which").arg("zig").status().is_ok() {
        println!("✅ Zig is already installed");
        show_zig_version();
        return;
    }

    let install_methods = [
        "📦 Package Manager (Recommended)",
        "🌐 Official Download",
        "🔨 Build from Source",
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
        0 => install_zig_package_manager(),
        1 => install_zig_official(),
        2 => install_zig_from_source(),
        _ => return,
    }
}

fn install_zig_package_manager() {
    // Try different package managers with reaper priority
    let install_result = if Command::new("which")
        .arg("reap")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Installing Zig with reaper...");
        Command::new("reap").arg("zig").status()
    } else if Command::new("which")
        .arg("pacman")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Installing Zig with pacman...");
        Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "zig"])
            .status()
    } else if Command::new("which")
        .arg("apt")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Installing Zig with apt...");
        // Zig might not be in standard repos, suggest snap
        println!("Note: Zig might not be available in apt. Trying snap...");
        Command::new("sudo")
            .args(["snap", "install", "zig", "--classic", "--beta"])
            .status()
    } else if Command::new("which")
        .arg("dnf")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Installing Zig with dnf...");
        Command::new("sudo")
            .args(["dnf", "install", "-y", "zig"])
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
        .arg("zig")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Zig installed successfully");
        show_zig_version();
    } else {
        eprintln!("Package manager installation failed. Try official download.");
    }
}

fn install_zig_official() {
    println!("Installing Zig from Official Downloads");
    println!("=========================================");

    println!("Visit https://ziglang.org/download/ for the latest version");
    println!("This will download and install the latest development build");

    let Ok(confirm) = Confirm::new()
        .with_prompt("Download and install latest Zig?")
        .default(true)
        .interact()
    else {
        return;
    };

    if !confirm {
        return;
    }

    // Create zig directory
    let zig_dir = "/opt/zig";
    if let Err(e) = Command::new("sudo")
        .args(["mkdir", "-p", zig_dir])
        .status()
    {
        eprintln!("Failed to create directory: {}", e);
        return;
    }

    // Download latest (using hardcoded trusted URL)
    println!("Downloading Zig...");
    let download_url = "https://ziglang.org/builds/zig-linux-x86_64-0.12.0-dev.latest.tar.xz";

    match Command::new("curl")
        .args(["-L", "-f", "--proto", "=https", download_url, "-o", "/tmp/zig.tar.xz"])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Download completed");
        }
        Ok(_) => {
            eprintln!("Download failed");
            return;
        }
        Err(e) => {
            eprintln!("Failed to download: {}", e);
            return;
        }
    }

    // Extract
    match Command::new("sudo")
        .args([
            "tar",
            "-xf",
            "/tmp/zig.tar.xz",
            "-C",
            zig_dir,
            "--strip-components=1",
        ])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Extraction completed");
        }
        Ok(_) => {
            eprintln!("Extraction failed");
            return;
        }
        Err(e) => {
            eprintln!("Failed to extract: {}", e);
            return;
        }
    }

    // Add to PATH
    add_zig_to_path();

    // Cleanup
    if let Err(e) = std::fs::remove_file("/tmp/zig.tar.xz") {
        eprintln!("Warning: Failed to cleanup download: {}", e);
    }

    println!("Zig installed to {}", zig_dir);
}

fn install_zig_from_source() {
    println!("Building Zig from Source");
    println!("===========================");

    println!("Warning: Building Zig from source requires significant time and resources");

    let Ok(confirm) = Confirm::new()
        .with_prompt("Continue with source build?")
        .default(false)
        .interact()
    else {
        return;
    };

    if !confirm {
        return;
    }

    // Clean up previous attempt
    if std::path::Path::new("/tmp/zig-source").exists() {
        if let Err(e) = std::fs::remove_dir_all("/tmp/zig-source") {
            eprintln!("Warning: Failed to clean previous build: {}", e);
        }
    }

    println!("Cloning Zig repository...");
    match Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "https://github.com/ziglang/zig.git",
            "/tmp/zig-source",
        ])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Clone completed");
        }
        Ok(status) => {
            eprintln!("Clone failed with code: {:?}", status.code());
            return;
        }
        Err(e) => {
            eprintln!("Failed to clone: {}", e);
            return;
        }
    }

    println!("Building... (this will take a while)");
    println!(
        "For detailed build instructions, see: https://github.com/ziglang/zig/wiki/Building-Zig-From-Source"
    );
}

fn install_zion_tool() {
    println!("🛠️  Installing Zion (Zig Meta Tool)");
    println!("====================================");

    if Command::new("which").arg("zion").status().is_ok() {
        println!("✅ Zion is already installed");
        return;
    }

    let Ok(confirm) = Confirm::new()
        .with_prompt("Install Zion meta-tool for Zig development?")
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        println!("📥 Installing Zion...");
        let status = Command::new("bash")
            .arg("-c")
            .arg("curl -sSL https://raw.githubusercontent.com/ghostkellz/zion/main/release/install.sh | bash")
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("✅ Zion installed successfully");
                println!("💡 Zion provides project management for Zig");
                println!("🔗 Repository: https://github.com/GhostKellz/zion");
            }
            _ => println!("❌ Failed to install Zion"),
        }
    }
}

fn zig_project_management() {
    println!("📋 Zig Project Management");
    println!("=========================");

    let options = [
        "🆕 Create New Zig Project",
        "🔧 Initialize Existing Project",
        "📦 Manage Dependencies",
        "🏗️  Build Project",
        "🧪 Run Tests",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project Management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => create_zig_project(),
        1 => init_zig_project(),
        2 => manage_zig_dependencies(),
        3 => build_zig_project(),
        4 => run_zig_tests(),
        _ => return,
    }
}

fn create_zig_project() {
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

    let project_types = ["exe", "lib", "test"];
    let Ok(project_type) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project type")
        .items(&project_types)
        .default(0)
        .interact()
    else {
        return;
    };

    // Use Zion if available, otherwise use zig init
    if Command::new("which")
        .arg("zion")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Creating project with Zion...");
        match Command::new("zion")
            .args(["new", &project_name, "--type", project_types[project_type]])
            .status()
        {
            Ok(status) if status.success() => {
                println!("Zig project '{}' created with Zion", project_name);
            }
            Ok(status) => {
                eprintln!("Zion exited with code: {:?}", status.code());
            }
            Err(e) => {
                eprintln!("Failed to run zion: {}", e);
            }
        }
    } else {
        println!("Creating project directory...");
        if let Err(e) = std::fs::create_dir_all(&project_name) {
            eprintln!("Failed to create project directory: {}", e);
            return;
        }

        match Command::new("zig")
            .args(["init-exe"])
            .current_dir(&project_name)
            .status()
        {
            Ok(status) if status.success() => {
                println!("Zig project '{}' created", project_name);
            }
            Ok(status) => {
                eprintln!("zig init-exe failed with code: {:?}", status.code());
            }
            Err(e) => {
                eprintln!("Failed to run zig init: {}", e);
            }
        }
    }
}

fn init_zig_project() {
    println!("Initializing Zig project in current directory...");

    match Command::new("zig").args(["init-exe"]).status() {
        Ok(status) if status.success() => {
            println!("Zig project initialized");
        }
        Ok(status) => {
            eprintln!("zig init-exe failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run zig: {}", e);
        }
    }
}

fn manage_zig_dependencies() {
    println!("Zig Dependency Management");
    println!("============================");

    if !std::path::Path::new("build.zig").exists() {
        eprintln!("No build.zig found. Initialize a Zig project first.");
        return;
    }

    // Check if using Zion for dependency management
    if Command::new("which")
        .arg("zion")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        if let Err(e) = Command::new("zion").args(["deps", "list"]).status() {
            eprintln!("Failed to run zion deps: {}", e);
        }
    } else {
        println!("Zig package management is evolving. Check build.zig for dependencies.");

        match std::fs::read_to_string("build.zig") {
            Ok(content) => {
                println!("Current build.zig:");
                for (i, line) in content.lines().take(20).enumerate() {
                    println!("  {}: {}", i + 1, line);
                }
            }
            Err(e) => {
                eprintln!("Failed to read build.zig: {}", e);
            }
        }
    }
}

fn build_zig_project() {
    println!("Building Zig Project");
    println!("========================");

    if !std::path::Path::new("build.zig").exists() {
        eprintln!("No build.zig found in current directory");
        return;
    }

    println!("Building...");
    match Command::new("zig").args(["build"]).status() {
        Ok(status) if status.success() => {
            println!("Build successful");
        }
        Ok(status) => {
            eprintln!("Build failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run zig build: {}", e);
        }
    }
}

fn run_zig_tests() {
    println!("Running Zig Tests");
    println!("====================");

    match Command::new("zig").args(["build", "test"]).status() {
        Ok(status) if status.success() => {
            println!("All tests passed");
        }
        Ok(status) => {
            eprintln!("Tests failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run tests: {}", e);
        }
    }
}

fn zig_development_tools() {
    println!("🔧 Zig Development Tools");
    println!("========================");

    let tools = [
        "📝 Install zls (Zig Language Server)",
        "🎨 Install zig-mode for Emacs",
        "⚡ Install vim-zig plugin",
        "🔍 Install zigtools extensions",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Development Tools")
        .items(&tools)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => install_zls(),
        1 => install_zig_emacs(),
        2 => install_zig_vim(),
        3 => install_zigtools(),
        _ => return,
    }
}

fn install_zls() {
    println!("Installing zls (Zig Language Server)");
    println!("========================================");

    if Command::new("which")
        .arg("zls")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("zls is already installed");
        return;
    }

    // Try package manager first
    if Command::new("which")
        .arg("reap")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        match Command::new("reap").arg("zls").status() {
            Ok(status) if status.success() => {
                println!("zls installed via reaper");
                return;
            }
            _ => {
                println!("Package manager install failed, building from source...");
            }
        }
    }

    // Build from source
    println!("Building zls from source...");
    let zls_dir = "/tmp/zls-build";

    // Clean up previous attempt
    if std::path::Path::new(zls_dir).exists() {
        if let Err(e) = std::fs::remove_dir_all(zls_dir) {
            eprintln!("Warning: Failed to clean up {}: {}", zls_dir, e);
        }
    }

    match Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "https://github.com/zigtools/zls.git",
            zls_dir,
        ])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Repository cloned");
        }
        Ok(_) => {
            eprintln!("Failed to clone zls repository");
            return;
        }
        Err(e) => {
            eprintln!("Failed to run git: {}", e);
            return;
        }
    }

    match Command::new("zig")
        .args(["build", "-Doptimize=ReleaseSafe"])
        .current_dir(zls_dir)
        .status()
    {
        Ok(status) if status.success() => {
            println!("Build completed");
        }
        Ok(status) => {
            eprintln!("Build failed with code: {:?}", status.code());
            return;
        }
        Err(e) => {
            eprintln!("Failed to run zig build: {}", e);
            return;
        }
    }

    // Install binary
    let zls_binary = format!("{}/zig-out/bin/zls", zls_dir);
    match Command::new("sudo")
        .args(["install", "-Dm755", &zls_binary, "/usr/local/bin/zls"])
        .status()
    {
        Ok(status) if status.success() => {
            println!("zls installed successfully to /usr/local/bin/zls");
        }
        Ok(status) => {
            eprintln!("Install failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to install binary: {}", e);
        }
    }

    // Clean up
    if let Err(e) = std::fs::remove_dir_all(zls_dir) {
        eprintln!("Warning: Failed to clean up {}: {}", zls_dir, e);
    }
}

fn install_zig_emacs() {
    println!("🎨 Installing zig-mode for Emacs");
    println!("=================================");

    println!("💡 Add to your Emacs config:");
    println!("(use-package zig-mode)");
}

fn install_zig_vim() {
    println!("⚡ Installing vim-zig plugin");
    println!("===========================");

    println!("💡 For vim-plug users, add to .vimrc:");
    println!("Plug 'ziglang/zig.vim'");
}

fn install_zigtools() {
    println!("🔍 Installing Zig development tools");
    println!("===================================");

    println!("💡 Visit https://github.com/zigtools for more tools");
}

fn zig_learning_resources() {
    println!("📚 Zig Learning Resources");
    println!("=========================");

    println!("🌐 Official Documentation: https://ziglang.org/documentation/");
    println!("📖 Zig Guide: https://zig.guide/");
    println!("💡 Learn Zig: https://ziglearn.org/");
    println!("👥 Community: https://github.com/ziglang/zig/wiki/Community");
    println!("📺 Videos: https://www.youtube.com/c/AndrewKelley");
}

fn show_zig_version() {
    if let Ok(output) = Command::new("zig").arg("version").output() {
        let version = String::from_utf8_lossy(&output.stdout);
        println!("📋 Zig version: {}", version.trim());
    }
}

fn add_zig_to_path() {
    let Some(home_dir) = dirs::home_dir() else {
        println!("❌ Could not determine home directory");
        return;
    };

    let shell_files = [
        format!("{}/.bashrc", home_dir.display()),
        format!("{}/.zshrc", home_dir.display()),
    ];

    let zig_path_export = "export PATH=\"/opt/zig:$PATH\"";

    for shell_file in &shell_files {
        if std::path::Path::new(shell_file).exists()
            && let Ok(content) = std::fs::read_to_string(shell_file)
            && !content.contains("/opt/zig")
        {
            let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(shell_file) else {
                continue;
            };

            use std::io::Write;
            let _ = writeln!(file, "\n# Zig compiler");
            let _ = writeln!(file, "{}", zig_path_export);

            println!("✅ Added Zig to PATH in {}", shell_file);
        }
    }
}
