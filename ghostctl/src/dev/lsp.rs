use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use std::path::Path;
use std::process::Command;

/// Validate LSP binary name
fn validate_lsp_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("LSP name cannot be empty");
    }
    if name.len() > 50 {
        return Err("LSP name too long");
    }
    // Allow alphanumeric, hyphen, underscore
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err("LSP name contains invalid characters");
    }
    Ok(())
}

pub fn language_server_management() {
    println!("🛠️  Language Server Protocol (LSP) Management");
    println!("==============================================");
    
    let options = [
        "📋 List Installed Language Servers",
        "🔽 Install Language Server",
        "🔧 Configure Language Server",
        "🏥 Health Check All LSPs",
        "📊 Popular Language Servers",
        "🗑️  Uninstall Language Server",
        "⬅️  Back",
    ];
    
    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("LSP Management")
        .items(&options)
        .default(0)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };
    
    match choice {
        0 => list_installed_lsps(),
        1 => install_language_server(),
        2 => configure_language_server(),
        3 => health_check_all_lsps(),
        4 => show_popular_lsps(),
        5 => uninstall_language_server(),
        _ => return,
    }
}

fn list_installed_lsps() {
    println!("Installed Language Servers");
    println!("=============================");

    // Trusted LSP binary names - these are well-known language servers
    let language_servers = [
        ("rust-analyzer", "Rust Language Server"),
        ("zls", "Zig Language Server"),
        ("clangd", "C/C++ Language Server"),
        ("pyright", "Python Language Server"),
        ("pylsp", "Python LSP Server"),
        ("typescript-language-server", "TypeScript/JavaScript LSP"),
        ("lua-language-server", "Lua Language Server"),
        ("gopls", "Go Language Server"),
        ("nil", "Nix Language Server"),
        ("haskell-language-server", "Haskell Language Server"),
        ("jdtls", "Java Language Server"),
        ("omnisharp", "C# Language Server"),
        ("terraform-ls", "Terraform Language Server"),
        ("yaml-language-server", "YAML Language Server"),
        ("vscode-json-language-server", "JSON Language Server"),
        ("bash-language-server", "Bash Language Server"),
        ("docker-langserver", "Dockerfile Language Server"),
    ];

    println!("Checking installed language servers:\n");
    let mut installed_count = 0;

    for (lsp, description) in &language_servers {
        // Validate even trusted names - defense in depth
        if validate_lsp_name(lsp).is_err() {
            continue;
        }

        match Command::new("which").arg(lsp).output() {
            Ok(output) if output.status.success() => {
                let path = String::from_utf8_lossy(&output.stdout);
                println!("[installed] {} - {}", description, path.trim());
                installed_count += 1;
            }
            _ => {
                println!("[missing]   {} - Not installed", description);
            }
        }
    }

    println!(
        "\nSummary: {}/{} language servers installed",
        installed_count,
        language_servers.len()
    );
}

fn install_language_server() {
    println!("🔽 Install Language Server");
    println!("==========================");
    
    let lsp_options = [
        ("rust-analyzer", "🦀 Rust", "pacman -S rust-analyzer"),
        ("zls", "⚡ Zig", "Follow instructions at github.com/zigtools/zls"),
        ("clangd", "🔧 C/C++", "pacman -S clang"),
        ("pyright", "🐍 Python (Microsoft)", "npm install -g pyright"),
        ("pylsp", "🐍 Python (Community)", "pip install python-lsp-server"),
        ("typescript-language-server", "📜 TypeScript/JS", "npm install -g typescript-language-server typescript"),
        ("lua-language-server", "🌙 Lua", "pacman -S lua-language-server"),
        ("gopls", "🐹 Go", "go install golang.org/x/tools/gopls@latest"),
        ("nil", "❄️  Nix", "nix-env -iA nixpkgs.nil"),
        ("haskell-language-server", "λ Haskell", "pacman -S haskell-language-server"),
        ("jdtls", "☕ Java", "pacman -S jdtls"),
        ("omnisharp", "🔷 C#", "pacman -S omnisharp-roslyn"),
        ("terraform-ls", "🏗️  Terraform", "pacman -S terraform-ls"),
        ("yaml-language-server", "📄 YAML", "npm install -g yaml-language-server"),
        ("json-languageserver", "📋 JSON", "npm install -g vscode-langservers-extracted"),
        ("bash-language-server", "🐚 Bash", "npm install -g bash-language-server"),
        ("dockerfile-language-server", "🐳 Dockerfile", "npm install -g dockerfile-language-server-nodejs"),
    ];
    
    let display_options: Vec<String> = lsp_options.iter()
        .map(|(name, lang, _)| format!("{} {} ({})", lang, name, name))
        .collect();
    let mut all_options = display_options;
    all_options.push("⬅️  Back".to_string());
    
    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select language server to install")
        .items(&all_options)
        .default(0)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };
    
    if choice < lsp_options.len() {
        let (name, lang, install_cmd) = lsp_options[choice];
        install_specific_lsp(name, lang, install_cmd);
    }
}

fn install_specific_lsp(name: &str, lang: &str, install_cmd: &str) {
    // Validate LSP name
    if let Err(e) = validate_lsp_name(name) {
        eprintln!("Invalid LSP name: {}", e);
        return;
    }

    println!("Installing {} Language Server for {}", name, lang);
    println!("Install command: {}", install_cmd);

    let confirm = match Confirm::new()
        .with_prompt(&format!("Install {} language server?", name))
        .default(true)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    if !confirm {
        return;
    }

    let install_result = match name {
        "rust-analyzer" => {
            println!("Installing rust-analyzer...");
            Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "rust-analyzer"])
                .status()
        }
        "zls" => {
            println!("Installing Zig Language Server...");
            install_zls();
            return; // install_zls handles its own verification
        }
        "clangd" => {
            println!("Installing clangd...");
            Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "clang"])
                .status()
        }
        "pyright" => {
            println!("Installing pyright...");
            Command::new("npm").args(["install", "-g", "pyright"]).status()
        }
        "lua-language-server" => {
            println!("Installing lua-language-server...");
            Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "lua-language-server"])
                .status()
        }
        "gopls" => {
            println!("Installing gopls...");
            Command::new("go")
                .args(["install", "golang.org/x/tools/gopls@latest"])
                .status()
        }
        "typescript-language-server" => {
            println!("Installing typescript-language-server...");
            Command::new("npm")
                .args(["install", "-g", "typescript-language-server", "typescript"])
                .status()
        }
        "bash-language-server" => {
            println!("Installing bash-language-server...");
            Command::new("npm")
                .args(["install", "-g", "bash-language-server"])
                .status()
        }
        _ => {
            println!("Manual installation required:");
            println!("Run: {}", install_cmd);
            return;
        }
    };

    // Check installation result
    match install_result {
        Ok(status) if status.success() => {
            println!("Installation command completed");
        }
        Ok(status) => {
            eprintln!(
                "Installation command exited with code: {:?}",
                status.code()
            );
        }
        Err(e) => {
            eprintln!("Failed to run installation command: {}", e);
            return;
        }
    }

    // Verify installation
    println!("Verifying installation...");
    match Command::new("which").arg(name).output() {
        Ok(output) if output.status.success() => {
            println!("{} installed successfully!", name);
        }
        _ => {
            eprintln!("Installation may have failed. Please check manually.");
        }
    }
}

pub fn install_zls() {
    println!("⚡ Installing Zig Language Server (ZLS)");
    println!("======================================");
    
    let methods = [
        "📦 Install from AUR (zls-git)",
        "🔨 Build from source",
        "📥 Download binary release",
        "⬅️  Back",
    ];
    
    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("ZLS Installation method")
        .items(&methods)
        .default(0)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };
    
    match choice {
        0 => {
            println!("Installing ZLS from AUR...");
            match Command::new("yay")
                .args(["-S", "--noconfirm", "zls-git"])
                .status()
            {
                Ok(status) if status.success() => {
                    println!("ZLS installed from AUR");
                }
                Ok(status) => {
                    eprintln!("AUR installation failed with code: {:?}", status.code());
                }
                Err(e) => {
                    eprintln!("Failed to run yay: {}", e);
                }
            }
        }
        1 => {
            println!("Building ZLS from source...");
            build_zls_from_source();
        }
        2 => {
            println!("Download binary release...");
            println!("Visit: https://github.com/zigtools/zls/releases");
            println!("Download the appropriate binary and place it in your PATH");
        }
        _ => return,
    }
}

fn build_zls_from_source() {
    println!("Building ZLS from source");
    println!("===========================");

    // Check if zig is installed
    match Command::new("zig").arg("version").output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("Zig compiler found: {}", version.trim());
        }
        _ => {
            eprintln!("Zig compiler not found. Install Zig first:");
            eprintln!("  sudo pacman -S zig");
            return;
        }
    }

    let build_dir = "/tmp/zls-build";
    println!("Building in: {}", build_dir);

    // Clean up previous build
    if Path::new(build_dir).exists() {
        if let Err(e) = std::fs::remove_dir_all(build_dir) {
            eprintln!("Warning: Failed to clean previous build: {}", e);
        }
    }

    // Clone repository
    println!("Cloning ZLS repository...");
    match Command::new("git")
        .args(["clone", "--depth", "1", "https://github.com/zigtools/zls.git", build_dir])
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

    // Build ZLS
    println!("Building ZLS...");
    match Command::new("zig")
        .args(["build", "-Doptimize=ReleaseSafe"])
        .current_dir(build_dir)
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
            eprintln!("Failed to run build: {}", e);
            return;
        }
    }

    // Install binary
    let zls_binary = format!("{}/zig-out/bin/zls", build_dir);
    println!("Installing ZLS...");
    match Command::new("sudo")
        .args(["install", "-Dm755", &zls_binary, "/usr/local/bin/zls"])
        .status()
    {
        Ok(status) if status.success() => {
            println!("ZLS installed to /usr/local/bin/zls");
        }
        Ok(status) => {
            eprintln!("Install failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to install: {}", e);
        }
    }

    // Clean up
    if let Err(e) = std::fs::remove_dir_all(build_dir) {
        eprintln!("Warning: Failed to clean up build directory: {}", e);
    }
}

fn configure_language_server() {
    println!("🔧 Configure Language Server - TODO: Implement");
    // Implementation for LSP configuration
}

fn health_check_all_lsps() {
    println!("Language Server Health Check");
    println!("===============================");

    list_installed_lsps();

    // Run neovim LSP health check if nvim is available
    if Command::new("which")
        .arg("nvim")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("\nRunning Neovim LSP health check...");
        match Command::new("nvim")
            .args(["--headless", "+checkhealth", "lsp", "+qall"])
            .status()
        {
            Ok(status) if !status.success() => {
                eprintln!(
                    "Neovim health check exited with code: {:?}",
                    status.code()
                );
            }
            Err(e) => {
                eprintln!("Failed to run Neovim health check: {}", e);
            }
            _ => {}
        }
    } else {
        println!("\nNeovim not found - skipping Neovim LSP health check");
    }
}

fn show_popular_lsps() {
    println!("📊 Popular Language Servers by Category");
    println!("=======================================");
    
    let categories = [
        ("🦀 Systems Programming", vec![
            "rust-analyzer - Rust",
            "clangd - C/C++",
            "zls - Zig",
            "gopls - Go",
        ]),
        ("🌐 Web Development", vec![
            "typescript-language-server - TypeScript/JavaScript",
            "volar - Vue.js",
            "svelte-language-server - Svelte",
            "html-languageserver - HTML",
            "css-languageserver - CSS",
        ]),
        ("🐍 Scripting & Dynamic", vec![
            "pyright - Python (Microsoft)",
            "pylsp - Python (Community)",
            "lua-language-server - Lua",
            "ruby-lsp - Ruby",
            "bash-language-server - Bash",
        ]),
        ("☁️  DevOps & Config", vec![
            "terraform-ls - Terraform",
            "yaml-language-server - YAML",
            "dockerfile-language-server - Docker",
            "helm-ls - Helm Charts",
        ]),
        ("🎓 Academic & Functional", vec![
            "haskell-language-server - Haskell",
            "ocaml-lsp - OCaml",
            "erlang-ls - Erlang",
            "elixir-ls - Elixir",
        ]),
    ];
    
    for (category, servers) in &categories {
        println!("\n{}", category);
        println!("{}", "=".repeat(category.len() - 2));
        for server in servers {
            println!("  • {}", server);
        }
    }
    
    println!("\n💡 Use 'Install Language Server' to install any of these servers.");
}

fn uninstall_language_server() {
    println!("🗑️  Uninstall Language Server - TODO: Implement");
}