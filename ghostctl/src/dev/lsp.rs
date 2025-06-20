use dialoguer::{Select, Confirm, theme::ColorfulTheme};
use std::process::Command;
use std::path::Path;

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
    
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("LSP Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    
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
    println!("📋 Installed Language Servers");
    println!("=============================");
    
    let language_servers = [
        ("rust-analyzer", "🦀 Rust Language Server"),
        ("zls", "⚡ Zig Language Server"),
        ("clangd", "🔧 C/C++ Language Server"),
        ("pyright", "🐍 Python Language Server"),
        ("pylsp", "🐍 Python LSP Server"),
        ("typescript-language-server", "📜 TypeScript/JavaScript LSP"),
        ("lua-language-server", "🌙 Lua Language Server"),
        ("gopls", "🐹 Go Language Server"),
        ("nil", "❄️  Nix Language Server"),
        ("haskell-language-server", "λ Haskell Language Server"),
        ("jdtls", "☕ Java Language Server"),
        ("omnisharp", "🔷 C# Language Server"),
        ("terraform-ls", "🏗️  Terraform Language Server"),
        ("yaml-language-server", "📄 YAML Language Server"),
        ("json-languageserver", "📋 JSON Language Server"),
        ("bash-language-server", "🐚 Bash Language Server"),
        ("dockerfile-language-server", "🐳 Dockerfile Language Server"),
    ];
    
    println!("Checking installed language servers:\n");
    let mut installed_count = 0;
    
    for (lsp, description) in &language_servers {
        match Command::new("which").arg(lsp).output() {
            Ok(output) if output.status.success() => {
                let path = String::from_utf8_lossy(&output.stdout).trim();
                println!("✅ {} - {}", description, path);
                installed_count += 1;
            },
            _ => {
                println!("❌ {} - Not installed", description);
            }
        }
    }
    
    println!("\n📊 Summary: {}/{} language servers installed", installed_count, language_servers.len());
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
    
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select language server to install")
        .items(&all_options)
        .default(0)
        .interact()
        .unwrap();
    
    if choice < lsp_options.len() {
        let (name, lang, install_cmd) = lsp_options[choice];
        install_specific_lsp(name, lang, install_cmd);
    }
}

fn install_specific_lsp(name: &str, lang: &str, install_cmd: &str) {
    println!("🔽 Installing {} Language Server for {}", name, lang);
    println!("Install command: {}", install_cmd);
    
    let confirm = Confirm::new()
        .with_prompt(&format!("Install {} language server?", name))
        .default(true)
        .interact()
        .unwrap();
    
    if confirm {
        match name {
            "rust-analyzer" => {
                println!("📦 Installing rust-analyzer...");
                let _ = Command::new("sudo")
                    .args(&["pacman", "-S", "--noconfirm", "rust-analyzer"])
                    .status();
            },
            "zls" => {
                println!("⚡ Installing Zig Language Server...");
                install_zls();
            },
            "clangd" => {
                println!("🔧 Installing clangd...");
                let _ = Command::new("sudo")
                    .args(&["pacman", "-S", "--noconfirm", "clang"])
                    .status();
            },
            "pyright" => {
                println!("🐍 Installing pyright...");
                let _ = Command::new("npm")
                    .args(&["install", "-g", "pyright"])
                    .status();
            },
            "lua-language-server" => {
                println!("🌙 Installing lua-language-server...");
                let _ = Command::new("sudo")
                    .args(&["pacman", "-S", "--noconfirm", "lua-language-server"])
                    .status();
            },
            _ => {
                println!("📝 Manual installation required:");
                println!("Run: {}", install_cmd);
            }
        }
        
        // Verify installation
        println!("🔍 Verifying installation...");
        match Command::new("which").arg(name).output() {
            Ok(output) if output.status.success() => {
                println!("✅ {} installed successfully!", name);
            },
            _ => {
                println!("❌ Installation may have failed. Please check manually.");
            }
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
    
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("ZLS Installation method")
        .items(&methods)
        .default(0)
        .interact()
        .unwrap();
    
    match choice {
        0 => {
            println!("📦 Installing ZLS from AUR...");
            let _ = Command::new("yay")
                .args(&["-S", "--noconfirm", "zls-git"])
                .status();
        },
        1 => {
            println!("🔨 Building ZLS from source...");
            build_zls_from_source();
        },
        2 => {
            println!("📥 Download binary release...");
            println!("Visit: https://github.com/zigtools/zls/releases");
            println!("Download the appropriate binary and place it in your PATH");
        },
        _ => return,
    }
}

fn build_zls_from_source() {
    println!("🔨 Building ZLS from source");
    println!("===========================");
    
    // Check if zig is installed
    match Command::new("zig").arg("version").output() {
        Ok(_) => {
            println!("✅ Zig compiler found");
            
            let build_dir = "/tmp/zls-build";
            println!("📁 Building in: {}", build_dir);
            
            let commands = [
                format!("rm -rf {}", build_dir),
                format!("git clone https://github.com/zigtools/zls.git {}", build_dir),
                format!("cd {} && zig build -Doptimize=ReleaseSafe", build_dir),
                format!("sudo cp {}/zig-out/bin/zls /usr/local/bin/", build_dir),
            ];
            
            for cmd in &commands {
                println!("🔧 Running: {}", cmd);
                let _ = Command::new("bash")
                    .arg("-c")
                    .arg(cmd)
                    .status();
            }
            
            println!("✅ ZLS build complete!");
        },
        Err(_) => {
            println!("❌ Zig compiler not found. Install Zig first:");
            println!("sudo pacman -S zig");
        }
    }
}

fn configure_language_server() {
    println!("🔧 Configure Language Server - TODO: Implement");
    // Implementation for LSP configuration
}

fn health_check_all_lsps() {
    println!("🏥 Language Server Health Check");
    println!("===============================");
    
    list_installed_lsps();
    
    // Run neovim LSP health check
    println!("\n🔍 Running Neovim LSP health check...");
    let _ = Command::new("nvim")
        .args(&["--headless", "+checkhealth lsp", "+qall"])
        .status();
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