use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::process::Command;

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Zig Development")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let method = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Installation method")
        .items(&install_methods)
        .default(0)
        .interact()
        .unwrap();

    match method {
        0 => install_zig_package_manager(),
        1 => install_zig_official(),
        2 => install_zig_from_source(),
        _ => return,
    }
}

fn install_zig_package_manager() {
    // Try different package managers with reaper priority
    if Command::new("which").arg("reap").status().is_ok() {
        println!("📦 Installing Zig with reaper...");
        let _ = Command::new("reap").arg("zig").status();
    } else if Command::new("which").arg("pacman").status().is_ok() {
        println!("📦 Installing Zig with pacman...");
        let _ = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", "zig"])
            .status();
    } else if Command::new("which").arg("apt").status().is_ok() {
        println!("📦 Installing Zig with apt...");
        // Zig might not be in standard repos, suggest snap
        println!("💡 Zig might not be available in apt. Trying snap...");
        let _ = Command::new("sudo")
            .args(&["snap", "install", "zig", "--classic", "--beta"])
            .status();
    } else if Command::new("which").arg("dnf").status().is_ok() {
        println!("📦 Installing Zig with dnf...");
        let _ = Command::new("sudo")
            .args(&["dnf", "install", "-y", "zig"])
            .status();
    }

    if Command::new("which").arg("zig").status().is_ok() {
        println!("✅ Zig installed successfully");
        show_zig_version();
    } else {
        println!("❌ Package manager installation failed. Try official download.");
    }
}

fn install_zig_official() {
    println!("🌐 Installing Zig from Official Downloads");
    println!("=========================================");

    println!("💡 Visit https://ziglang.org/download/ for the latest version");
    println!("📥 This will download and install the latest development build");

    let confirm = Confirm::new()
        .with_prompt("Download and install latest Zig?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        // Create zig directory
        let zig_dir = "/opt/zig";
        let _ = Command::new("sudo")
            .args(&["mkdir", "-p", zig_dir])
            .status();

        // Download latest (this is simplified - real implementation would get latest URL)
        println!("📥 Downloading Zig...");
        let download_url = "https://ziglang.org/builds/zig-linux-x86_64-0.12.0-dev.latest.tar.xz";

        let _ = Command::new("curl")
            .args(&["-L", download_url, "-o", "/tmp/zig.tar.xz"])
            .status();

        // Extract
        let _ = Command::new("sudo")
            .args(&[
                "tar",
                "-xf",
                "/tmp/zig.tar.xz",
                "-C",
                zig_dir,
                "--strip-components=1",
            ])
            .status();

        // Add to PATH
        add_zig_to_path();

        // Cleanup
        let _ = std::fs::remove_file("/tmp/zig.tar.xz");

        println!("✅ Zig installed to {}", zig_dir);
    }
}

fn install_zig_from_source() {
    println!("🔨 Building Zig from Source");
    println!("===========================");

    println!("⚠️  Building Zig from source requires significant time and resources");

    let confirm = Confirm::new()
        .with_prompt("Continue with source build?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        // This is a simplified version - real build is complex
        println!("📥 Cloning Zig repository...");
        let _ = Command::new("git")
            .args(&[
                "clone",
                "https://github.com/ziglang/zig.git",
                "/tmp/zig-source",
            ])
            .status();

        println!("🔨 Building... (this will take a while)");
        println!(
            "💡 For detailed build instructions, see: https://github.com/ziglang/zig/wiki/Building-Zig-From-Source"
        );
    }
}

fn install_zion_tool() {
    println!("🛠️  Installing Zion (Zig Meta Tool)");
    println!("====================================");

    if Command::new("which").arg("zion").status().is_ok() {
        println!("✅ Zion is already installed");
        return;
    }

    let confirm = Confirm::new()
        .with_prompt("Install Zion meta-tool for Zig development?")
        .default(true)
        .interact()
        .unwrap();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
    let project_name: String = Input::new()
        .with_prompt("Project name")
        .interact_text()
        .unwrap();

    let project_types = ["exe", "lib", "test"];
    let project_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project type")
        .items(&project_types)
        .default(0)
        .interact()
        .unwrap();

    // Use Zion if available, otherwise use zig init
    if Command::new("which").arg("zion").status().is_ok() {
        println!("🛠️  Creating project with Zion...");
        let _ = Command::new("zion")
            .args(&["new", &project_name, "--type", project_types[project_type]])
            .status();
    } else {
        println!("📁 Creating project directory...");
        std::fs::create_dir_all(&project_name).unwrap();

        let _ = Command::new("zig")
            .args(&["init-exe"])
            .current_dir(&project_name)
            .status();
    }

    println!("✅ Zig project '{}' created", project_name);
}

fn init_zig_project() {
    println!("🔧 Initializing Zig project in current directory...");

    let _ = Command::new("zig").args(&["init-exe"]).status();

    println!("✅ Zig project initialized");
}

fn manage_zig_dependencies() {
    println!("📦 Zig Dependency Management");
    println!("============================");

    if !std::path::Path::new("build.zig").exists() {
        println!("❌ No build.zig found. Initialize a Zig project first.");
        return;
    }

    // Check if using Zion for dependency management
    if Command::new("which").arg("zion").status().is_ok() {
        let _ = Command::new("zion").args(&["deps", "list"]).status();
    } else {
        println!("💡 Zig package management is evolving. Check build.zig for dependencies.");

        if let Ok(content) = std::fs::read_to_string("build.zig") {
            println!("📋 Current build.zig:");
            for (i, line) in content.lines().take(20).enumerate() {
                println!("  {}: {}", i + 1, line);
            }
        }
    }
}

fn build_zig_project() {
    println!("🏗️  Building Zig Project");
    println!("========================");

    if !std::path::Path::new("build.zig").exists() {
        println!("❌ No build.zig found in current directory");
        return;
    }

    println!("🔨 Building...");
    let status = Command::new("zig").args(&["build"]).status();

    match status {
        Ok(s) if s.success() => println!("✅ Build successful"),
        _ => println!("❌ Build failed"),
    }
}

fn run_zig_tests() {
    println!("🧪 Running Zig Tests");
    println!("====================");

    let status = Command::new("zig").args(&["build", "test"]).status();

    match status {
        Ok(s) if s.success() => println!("✅ All tests passed"),
        _ => println!("❌ Tests failed"),
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Development Tools")
        .items(&tools)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_zls(),
        1 => install_zig_emacs(),
        2 => install_zig_vim(),
        3 => install_zigtools(),
        _ => return,
    }
}

fn install_zls() {
    println!("📝 Installing zls (Zig Language Server)");
    println!("========================================");

    if Command::new("which").arg("zls").status().is_ok() {
        println!("✅ zls is already installed");
        return;
    }

    // Try package manager first
    if Command::new("which").arg("reap").status().is_ok() {
        let _ = Command::new("reap").arg("zls").status();
    } else {
        // Build from source
        println!("🔨 Building zls from source...");
        let _ = Command::new("git")
            .args(&["clone", "https://github.com/zigtools/zls.git", "/tmp/zls"])
            .status();

        let build_status = Command::new("zig")
            .args(&["build", "-Doptimize=ReleaseSafe"])
            .current_dir("/tmp/zls")
            .status();

        if build_status.is_ok() {
            // Install binary
            let _ = Command::new("sudo")
                .args(&["cp", "/tmp/zls/zig-out/bin/zls", "/usr/local/bin/"])
                .status();

            println!("✅ zls installed successfully");
        }
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
    let shell_files = [
        format!("{}/.bashrc", dirs::home_dir().unwrap().display()),
        format!("{}/.zshrc", dirs::home_dir().unwrap().display()),
    ];

    let zig_path_export = "export PATH=\"/opt/zig:$PATH\"";

    for shell_file in &shell_files {
        if std::path::Path::new(shell_file).exists() {
            if let Ok(content) = std::fs::read_to_string(shell_file) {
                if !content.contains("/opt/zig") {
                    let mut file = std::fs::OpenOptions::new()
                        .append(true)
                        .open(shell_file)
                        .unwrap();

                    use std::io::Write;
                    writeln!(file, "\n# Zig compiler").unwrap();
                    writeln!(file, "{}", zig_path_export).unwrap();

                    println!("✅ Added Zig to PATH in {}", shell_file);
                }
            }
        }
    }
}
