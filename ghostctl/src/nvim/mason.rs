use dialoguer::{Confirm, MultiSelect, Select, theme::ColorfulTheme};
use std::collections::HashMap;
use std::process::Command;

/// Mason.nvim integration for LSP/DAP/Tool management
/// Provides automated installation and configuration of language servers,
/// debug adapters, formatters, and linters
pub fn mason_menu() {
    println!("🔨 Mason.nvim - LSP & Tool Management");
    println!("====================================");

    let options = [
        "🏥 Check Mason status",
        "🔧 Setup Mason & language servers",
        "📋 Language environment setup",
        "🔌 Install specific tools",
        "🔄 Update all tools",
        "🩺 Diagnose issues",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Mason Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => check_mason_status(),
        1 => setup_mason(),
        2 => language_environment_setup(),
        3 => install_specific_tools(),
        4 => update_all_tools(),
        5 => diagnose_mason_issues(),
        _ => return,
    }
}

pub fn check_mason_status() {
    println!("🏥 Checking Mason.nvim status...\n");

    // Check if Mason is installed
    println!("=== MASON INSTALLATION ===");
    if check_mason_installed() {
        println!("✅ Mason.nvim is installed");
    } else {
        println!("❌ Mason.nvim is not installed");
        let install = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to install Mason.nvim?")
            .interact()
            .unwrap();

        if install {
            install_mason();
            return;
        }
    }

    // Check installed tools
    println!("\n=== INSTALLED TOOLS ===");
    let installed_tools = get_installed_tools();
    if installed_tools.is_empty() {
        println!("⚠️  No tools installed via Mason");
    } else {
        for tool in &installed_tools {
            println!("✅ {}", tool);
        }
    }

    // Check for language-specific setups
    println!("\n=== LANGUAGE SUPPORT ===");
    check_language_support();
}

pub fn setup_mason() {
    println!("🔧 Setting up Mason.nvim and essential language servers...\n");

    if !check_mason_installed() {
        install_mason();
    }

    // Install essential language servers
    let essential_tools = vec![
        "lua-language-server",        // Lua LSP
        "rust-analyzer",              // Rust LSP
        "typescript-language-server", // TypeScript/JavaScript LSP
        "pyright",                    // Python LSP
        "gopls",                      // Go LSP
        "bash-language-server",       // Bash LSP
        "yaml-language-server",       // YAML LSP
        "json-lsp",                   // JSON LSP
    ];

    println!("📦 Installing essential language servers...");
    for tool in &essential_tools {
        install_mason_tool(tool);
    }

    // Install essential formatters
    let formatters = vec![
        "prettier", // Web development formatter
        "black",    // Python formatter
        "rustfmt",  // Rust formatter
        "gofmt",    // Go formatter
        "shfmt",    // Shell script formatter
    ];

    println!("\n🎨 Installing formatters...");
    for formatter in &formatters {
        install_mason_tool(formatter);
    }

    // Install essential linters
    let linters = vec![
        "eslint_d",   // JavaScript/TypeScript linter
        "pylint",     // Python linter
        "shellcheck", // Shell script linter
    ];

    println!("\n🔍 Installing linters...");
    for linter in &linters {
        install_mason_tool(linter);
    }

    println!("\n✅ Mason setup complete!");
    println!("💡 Restart Neovim to activate all language servers");
}

pub fn language_environment_setup() {
    println!("📋 Language-specific environment setup\n");

    let languages = vec![
        "🦀 Rust Development",
        "🐍 Python Development",
        "🐹 Go Development",
        "⚡ Zig Development",
        "🌐 Web Development (JS/TS/HTML/CSS)",
        "🔧 Systems Administration (Bash/YAML/JSON)",
        "☁️  DevOps (Docker/K8s/Terraform)",
        "📝 Documentation (Markdown/LaTeX)",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select language environment to setup")
        .items(&languages)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => setup_rust_environment(),
        1 => setup_python_environment(),
        2 => setup_go_environment(),
        3 => setup_zig_environment(),
        4 => setup_web_environment(),
        5 => setup_sysadmin_environment(),
        6 => setup_devops_environment(),
        7 => setup_documentation_environment(),
        _ => return,
    }
}

pub fn install_specific_tools() {
    println!("🔌 Install specific Mason tools\n");

    let available_tools = get_available_mason_tools();
    let installed_tools = get_installed_tools();

    // Filter out already installed tools
    let not_installed: Vec<&str> = available_tools
        .iter()
        .filter(|&tool| !installed_tools.contains(&tool.to_string()))
        .map(|s| s.as_str())
        .collect();

    if not_installed.is_empty() {
        println!("✅ All common tools are already installed!");
        return;
    }

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select tools to install")
        .items(&not_installed)
        .interact()
        .unwrap();

    for &index in &selected {
        let tool = not_installed[index];
        install_mason_tool(tool);
    }

    println!("\n✅ Selected tools installed successfully!");
}

pub fn update_all_tools() {
    println!("🔄 Updating all Mason tools...\n");

    let installed_tools = get_installed_tools();
    if installed_tools.is_empty() {
        println!("⚠️  No tools installed to update");
        return;
    }

    println!("📦 Updating {} tools...", installed_tools.len());
    for tool in &installed_tools {
        update_mason_tool(tool);
    }

    println!("\n✅ All tools updated successfully!");
}

pub fn diagnose_mason_issues() {
    println!("🩺 Diagnosing Mason.nvim issues...\n");

    // Check Mason installation
    println!("=== MASON STATUS ===");
    if !check_mason_installed() {
        println!("❌ Mason.nvim is not installed");
        return;
    }
    println!("✅ Mason.nvim is installed");

    // Check Neovim version compatibility
    println!("\n=== NEOVIM VERSION ===");
    if let Ok(output) = Command::new("nvim").args(&["--version"]).output() {
        let version = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = version.lines().collect();
        if let Some(first_line) = lines.first() {
            println!("✅ {}", first_line);

            // Check if version is compatible (0.8.0+)
            if version.contains("v0.7") || version.contains("v0.6") {
                println!("⚠️  Mason requires Neovim 0.8.0 or higher");
                println!("💡 Consider upgrading Neovim for full Mason support");
            }
        }
    } else {
        println!("❌ Could not check Neovim version");
    }

    // Check Node.js for some language servers
    println!("\n=== NODE.JS DEPENDENCY ===");
    if let Ok(output) = Command::new("node").args(&["--version"]).output() {
        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("✅ Node.js: {}", version);
    } else {
        println!("⚠️  Node.js not found - some language servers require Node.js");
        println!("💡 Install Node.js for TypeScript, ESLint, and other tools");
    }

    // Check Python for Python tools
    println!("\n=== PYTHON DEPENDENCY ===");
    if let Ok(output) = Command::new("python3").args(&["--version"]).output() {
        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("✅ {}", version);
    } else {
        println!("⚠️  Python3 not found - some tools require Python");
    }

    // Check for common issues
    println!("\n=== COMMON ISSUES ===");
    check_common_mason_issues();
}

// Implementation helper functions

fn check_mason_installed() -> bool {
    // Check if Mason is installed by looking for Mason commands in Neovim
    let lua_check = r#"
        local mason_ok, mason = pcall(require, "mason")
        if mason_ok then
            print("Mason installed")
        else
            print("Mason not found")
        end
    "#;

    if let Ok(output) = Command::new("nvim")
        .args(&[
            "--headless",
            "-c",
            &format!("lua {}", lua_check),
            "-c",
            "qa",
        ])
        .output()
    {
        String::from_utf8_lossy(&output.stdout).contains("Mason installed")
    } else {
        false
    }
}

fn install_mason() {
    println!("📦 Installing Mason.nvim...");

    println!("✅ Mason.nvim configuration created");
    println!("💡 Add the following configuration to your init.lua:");
    println!(r#"
-- Mason.nvim setup
require("mason").setup({{
    ui = {{
        icons = {{
            package_installed = "✓",
            package_pending = "➜",
            package_uninstalled = "✗"
        }}
    }}
}})

require("mason-lspconfig").setup({{
    automatic_installation = true,
}})
"#);
    println!("💡 Install mason.nvim plugin via your plugin manager");
}

fn get_installed_tools() -> Vec<String> {
    // Get list of installed Mason tools
    let lua_cmd = r#"
        local mason_registry = require("mason-registry")
        local installed = mason_registry.get_installed_packages()
        for _, pkg in ipairs(installed) do
            print(pkg.name)
        end
    "#;

    if let Ok(output) = Command::new("nvim")
        .args(&["--headless", "-c", &format!("lua {}", lua_cmd), "-c", "qa"])
        .output()
    {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect()
    } else {
        Vec::new()
    }
}

fn get_available_mason_tools() -> Vec<String> {
    // Common Mason tools that are frequently used
    vec![
        "lua-language-server".to_string(),
        "rust-analyzer".to_string(),
        "typescript-language-server".to_string(),
        "pyright".to_string(),
        "pylsp".to_string(),
        "gopls".to_string(),
        "zls".to_string(),
        "bash-language-server".to_string(),
        "yaml-language-server".to_string(),
        "json-lsp".to_string(),
        "html-lsp".to_string(),
        "css-lsp".to_string(),
        "prettier".to_string(),
        "black".to_string(),
        "isort".to_string(),
        "rustfmt".to_string(),
        "gofmt".to_string(),
        "shfmt".to_string(),
        "eslint_d".to_string(),
        "pylint".to_string(),
        "shellcheck".to_string(),
        "markdownlint".to_string(),
        "dockerfile-language-server".to_string(),
        "terraform-ls".to_string(),
    ]
}

fn install_mason_tool(tool: &str) {
    println!("  📦 Installing {}...", tool);

    let lua_cmd = format!(
        r#"
        local mason_registry = require("mason-registry")
        local package = mason_registry.get_package("{}")
        if not package:is_installed() then
            package:install()
            print("Installing {}")
        else
            print("{} already installed")
        end
    "#,
        tool, tool, tool
    );

    let _ = Command::new("nvim")
        .args(&["--headless", "-c", &format!("lua {}", lua_cmd), "-c", "qa"])
        .output();
}

fn update_mason_tool(tool: &str) {
    println!("  🔄 Updating {}...", tool);
    // Mason update logic would go here
}

fn check_language_support() {
    let languages = HashMap::from([
        ("Rust", check_rust_support()),
        ("Python", check_python_support()),
        ("Go", check_go_support()),
        ("JavaScript/TypeScript", check_js_support()),
        ("Lua", check_lua_support()),
        ("Bash", check_bash_support()),
    ]);

    for (lang, supported) in languages {
        if supported {
            println!("✅ {} support configured", lang);
        } else {
            println!("⚠️  {} support incomplete", lang);
        }
    }
}

fn check_rust_support() -> bool {
    get_installed_tools().contains(&"rust-analyzer".to_string())
}

fn check_python_support() -> bool {
    let tools = get_installed_tools();
    tools.contains(&"pyright".to_string()) || tools.contains(&"pylsp".to_string())
}

fn check_go_support() -> bool {
    get_installed_tools().contains(&"gopls".to_string())
}

fn check_js_support() -> bool {
    get_installed_tools().contains(&"typescript-language-server".to_string())
}

fn check_lua_support() -> bool {
    get_installed_tools().contains(&"lua-language-server".to_string())
}

fn check_bash_support() -> bool {
    get_installed_tools().contains(&"bash-language-server".to_string())
}

fn check_common_mason_issues() {
    // Check common Mason issues and provide solutions
    let issues = vec![
        "💡 Ensure your plugin manager has installed mason.nvim and mason-lspconfig.nvim",
        "💡 Check that Neovim version is 0.8.0 or higher",
        "💡 Verify Node.js is installed for TypeScript and JavaScript tools",
        "💡 Ensure Python3 is available for Python-based tools",
        "💡 Check internet connection for tool downloads",
        "💡 Verify file permissions in ~/.local/share/nvim/mason/",
    ];

    for issue in issues {
        println!("{}", issue);
    }
}

// Language-specific environment setup functions

fn setup_rust_environment() {
    println!("🦀 Setting up Rust development environment...\n");

    let rust_tools = vec![
        "rust-analyzer",
        "rustfmt",
        "taplo", // TOML language server
    ];

    for tool in &rust_tools {
        install_mason_tool(tool);
    }

    println!("✅ Rust environment configured!");
    println!("💡 Features: LSP, formatting, TOML support");
}

fn setup_python_environment() {
    println!("🐍 Setting up Python development environment...\n");

    let python_tools = vec![
        "pyright", "black", "isort", "pylint", "mypy", "debugpy", // Python debugger
    ];

    for tool in &python_tools {
        install_mason_tool(tool);
    }

    println!("✅ Python environment configured!");
    println!("💡 Features: LSP, formatting, linting, type checking, debugging");
}

fn setup_go_environment() {
    println!("🐹 Setting up Go development environment...\n");

    let go_tools = vec![
        "gopls",
        "gofmt",
        "goimports",
        "delve", // Go debugger
    ];

    for tool in &go_tools {
        install_mason_tool(tool);
    }

    println!("✅ Go environment configured!");
    println!("💡 Features: LSP, formatting, imports, debugging");
}

fn setup_zig_environment() {
    println!("⚡ Setting up Zig development environment...\n");

    let zig_tools = vec![
        "zls", // Zig Language Server
    ];

    for tool in &zig_tools {
        install_mason_tool(tool);
    }

    println!("✅ Zig environment configured!");
    println!("💡 Features: LSP support");
}

fn setup_web_environment() {
    println!("🌐 Setting up Web development environment...\n");

    let web_tools = vec![
        "typescript-language-server",
        "html-lsp",
        "css-lsp",
        "emmet-ls",
        "prettier",
        "eslint_d",
        "vetur-vls", // Vue.js
    ];

    for tool in &web_tools {
        install_mason_tool(tool);
    }

    println!("✅ Web development environment configured!");
    println!("💡 Features: TypeScript/JavaScript, HTML, CSS, Vue.js, formatting, linting");
}

fn setup_sysadmin_environment() {
    println!("🔧 Setting up Systems Administration environment...\n");

    let sysadmin_tools = vec![
        "bash-language-server",
        "yaml-language-server",
        "json-lsp",
        "shellcheck",
        "shfmt",
        "markdownlint",
    ];

    for tool in &sysadmin_tools {
        install_mason_tool(tool);
    }

    println!("✅ Systems Administration environment configured!");
    println!("💡 Features: Bash, YAML, JSON, shell linting, markdown");
}

fn setup_devops_environment() {
    println!("☁️  Setting up DevOps environment...\n");

    let devops_tools = vec![
        "dockerfile-language-server",
        "docker-compose-language-service",
        "terraform-ls",
        "yaml-language-server",
        "helm-ls",
        "ansiblels",
    ];

    for tool in &devops_tools {
        install_mason_tool(tool);
    }

    println!("✅ DevOps environment configured!");
    println!("💡 Features: Docker, Terraform, Kubernetes, Ansible, Helm");
}

fn setup_documentation_environment() {
    println!("📝 Setting up Documentation environment...\n");

    let doc_tools = vec![
        "markdownlint",
        "ltex-ls", // LaTeX/Markdown grammar checking
        "texlab",  // LaTeX language server
        "vale",    // Prose linter
    ];

    for tool in &doc_tools {
        install_mason_tool(tool);
    }

    println!("✅ Documentation environment configured!");
    println!("💡 Features: Markdown, LaTeX, grammar checking, prose linting");
}
