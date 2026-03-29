use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use std::process::Command;

/// Validate Python package name
fn validate_package_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("Package name cannot be empty");
    }
    if name.len() > 200 {
        return Err("Package name too long");
    }
    // PyPI package names: alphanumeric, hyphen, underscore, dot, brackets for extras
    // Also allow comparison operators and version specifiers
    if !name.chars().all(|c| {
        c.is_alphanumeric()
            || c == '-'
            || c == '_'
            || c == '.'
            || c == '['
            || c == ']'
            || c == ','
            || c == '>'
            || c == '<'
            || c == '='
            || c == '!'
            || c == '~'
    }) {
        return Err("Package name contains invalid characters");
    }
    // Prevent path traversal attempts
    if name.contains("..") || name.starts_with('/') || name.starts_with('-') {
        return Err("Invalid package name format");
    }
    Ok(())
}

/// Validate environment/directory name
fn validate_env_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("Environment name cannot be empty");
    }
    if name.len() > 100 {
        return Err("Environment name too long");
    }
    // Only allow safe directory names
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err("Environment name contains invalid characters");
    }
    if name.starts_with('-') {
        return Err("Environment name cannot start with hyphen");
    }
    Ok(())
}

pub fn python_development_menu() {
    println!("🐍 Python Development Environment");
    println!("=================================");

    let options = [
        "📦 Install Python & pip",
        "🌿 Virtual Environment Management",
        "📦 Package Management",
        "🛠️  Development Tools",
        "🧪 Testing & Quality",
        "📚 Learning Resources",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Python Development")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => install_python(),
        1 => virtual_environment_management(),
        2 => package_management(),
        3 => development_tools(),
        4 => testing_quality(),
        5 => learning_resources(),
        _ => return,
    }
}

fn install_python() {
    println!("📦 Python Installation");
    println!("======================");

    if Command::new("which").arg("python3").status().is_ok() {
        println!("✅ Python3 is already installed");
        show_python_version();
        check_pip_installation();
        return;
    }

    let install_methods = [
        "📦 Package Manager",
        "🐍 pyenv (Recommended for multiple versions)",
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
        0 => install_python_package_manager(),
        1 => install_pyenv(),
        2 => install_python_official(),
        3 => install_python_from_source(),
        _ => return,
    }
}

fn install_python_package_manager() {
    // Try different package managers with reaper priority
    let install_result = if Command::new("which")
        .arg("reap")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Installing Python with reaper...");
        if let Err(e) = Command::new("reap").arg("python").status() {
            eprintln!("Warning: Failed to install python: {}", e);
        }
        Command::new("reap").arg("python-pip").status()
    } else if Command::new("which")
        .arg("pacman")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Installing Python with pacman...");
        Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "python", "python-pip"])
            .status()
    } else if Command::new("which")
        .arg("apt")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Installing Python with apt...");
        if let Err(e) = Command::new("sudo").args(["apt", "update"]).status() {
            eprintln!("Warning: apt update failed: {}", e);
        }
        Command::new("sudo")
            .args([
                "apt",
                "install",
                "-y",
                "python3",
                "python3-pip",
                "python3-venv",
            ])
            .status()
    } else if Command::new("which")
        .arg("dnf")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Installing Python with dnf...");
        Command::new("sudo")
            .args(["dnf", "install", "-y", "python3", "python3-pip"])
            .status()
    } else {
        eprintln!("No supported package manager found");
        return;
    };

    match install_result {
        Ok(status) if status.success() => {
            println!("Installation command completed");
        }
        Ok(status) => {
            eprintln!("Installation exited with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run installation: {}", e);
        }
    }

    if Command::new("which")
        .arg("python3")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Python installed successfully");
        show_python_version();
        setup_python_environment();
    } else {
        eprintln!("Package manager installation failed");
    }
}

fn install_pyenv() {
    println!("🐍 Installing pyenv (Python Version Manager)");
    println!("=============================================");

    if Command::new("which").arg("pyenv").status().is_ok() {
        println!("✅ pyenv is already installed");
        return;
    }

    let Ok(confirm) = Confirm::new()
        .with_prompt("Install pyenv for Python version management?")
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        println!("📥 Installing pyenv...");
        let status = Command::new("curl")
            .args(&["https://pyenv.run", "|", "bash"])
            .status();

        if status.is_ok() {
            setup_pyenv_environment();
            println!("✅ pyenv installed. Restart your shell and run 'pyenv install 3.11.0'");
        }
    }
}

fn install_python_official() {
    println!("🌐 Installing Python from Official Source");
    println!("==========================================");
    println!("💡 Visit https://www.python.org/downloads/ for the latest version");
    println!("💡 Download and install the appropriate package for your system");
}

fn install_python_from_source() {
    println!("🔨 Building Python from Source");
    println!("==============================");

    println!("⚠️  Building Python from source requires development tools");

    let Ok(confirm) = Confirm::new()
        .with_prompt("Continue with source build?")
        .default(false)
        .interact()
    else {
        return;
    };

    if confirm {
        // Install build dependencies first
        if Command::new("which").arg("pacman").status().is_ok() {
            if let Err(e) = Command::new("sudo")
                .args([
                    "pacman",
                    "-S",
                    "--noconfirm",
                    "base-devel",
                    "openssl",
                    "zlib",
                    "bzip2",
                ])
                .status()
            {
                eprintln!("Failed to install build dependencies: {}", e);
            }
        }

        println!("📥 Downloading Python source...");
        if let Err(e) = Command::new("wget")
            .args(["https://www.python.org/ftp/python/3.11.0/Python-3.11.0.tgz"])
            .status()
        {
            eprintln!("Failed to download Python source: {}", e);
        }

        println!("🔨 Building... (this will take a while)");
        println!(
            "💡 For detailed instructions, see: https://devguide.python.org/getting-started/setup-building/"
        );
    }
}

fn virtual_environment_management() {
    println!("🌿 Virtual Environment Management");
    println!("=================================");

    let options = [
        "🆕 Create Virtual Environment",
        "🔌 Activate Environment",
        "📋 List Environments",
        "🗑️  Remove Environment",
        "📦 Install virtualenv/venv",
        "🐍 Install conda/miniconda",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Virtual Environment")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => create_virtual_environment(),
        1 => activate_environment(),
        2 => list_environments(),
        3 => remove_environment(),
        4 => install_virtualenv(),
        5 => install_conda(),
        _ => return,
    }
}

fn create_virtual_environment() {
    let Ok(env_name): Result<String, _> =
        Input::new().with_prompt("Environment name").interact_text()
    else {
        return;
    };

    // Validate environment name
    if let Err(e) = validate_env_name(&env_name) {
        eprintln!("Invalid environment name: {}", e);
        return;
    }

    let tools = ["venv (built-in)", "virtualenv", "conda"];
    let Ok(tool) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Environment tool")
        .items(&tools)
        .default(0)
        .interact()
    else {
        return;
    };

    match tool {
        0 => {
            match Command::new("python3")
                .args(["-m", "venv", &env_name])
                .status()
            {
                Ok(status) if status.success() => {
                    println!("Virtual environment '{}' created with venv", env_name);
                    println!("Activate with: source {}/bin/activate", env_name);
                }
                Ok(status) => {
                    eprintln!("venv creation failed with code: {:?}", status.code());
                }
                Err(e) => {
                    eprintln!("Failed to create venv: {}", e);
                }
            }
        }
        1 => match Command::new("virtualenv").arg(&env_name).status() {
            Ok(status) if status.success() => {
                println!("Virtual environment '{}' created with virtualenv", env_name);
            }
            Ok(status) => {
                eprintln!("virtualenv failed with code: {:?}", status.code());
            }
            Err(e) => {
                eprintln!("Failed to run virtualenv: {}", e);
            }
        },
        2 => {
            match Command::new("conda")
                .args(["create", "-n", &env_name, "python", "-y"])
                .status()
            {
                Ok(status) if status.success() => {
                    println!("Conda environment '{}' created", env_name);
                    println!("Activate with: conda activate {}", env_name);
                }
                Ok(status) => {
                    eprintln!("conda create failed with code: {:?}", status.code());
                }
                Err(e) => {
                    eprintln!("Failed to run conda: {}", e);
                }
            }
        }
        _ => return,
    }
}

fn activate_environment() {
    println!("🔌 Activating Virtual Environment");
    println!("=================================");

    println!("💡 To activate a virtual environment:");
    println!("  venv/virtualenv: source env_name/bin/activate");
    println!("  conda: conda activate env_name");
    println!("  pyenv: pyenv activate env_name");
}

fn list_environments() {
    println!("📋 Listing Virtual Environments");
    println!("===============================");

    // List conda environments
    if Command::new("which").arg("conda").status().is_ok() {
        println!("🐍 Conda environments:");
        if let Err(e) = Command::new("conda").args(["env", "list"]).status() {
            eprintln!("Failed to list conda environments: {}", e);
        }
    }

    // List pyenv environments
    if Command::new("which").arg("pyenv").status().is_ok() {
        println!("\n🐍 Pyenv versions:");
        if let Err(e) = Command::new("pyenv").args(["versions"]).status() {
            eprintln!("Failed to list pyenv versions: {}", e);
        }
    }

    // List local venv directories
    println!("\n📁 Local venv directories:");
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join("pyvenv.cfg").exists() {
                println!("  📂 {}", path.display());
            }
        }
    }
}

fn remove_environment() {
    let Ok(env_name): Result<String, _> = Input::new()
        .with_prompt("Environment name to remove")
        .interact_text()
    else {
        return;
    };

    // Validate environment name to prevent path traversal
    if let Err(e) = validate_env_name(&env_name) {
        eprintln!("Invalid environment name: {}", e);
        return;
    }

    let tools = ["venv/virtualenv (delete directory)", "conda"];
    let Ok(tool) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Environment type")
        .items(&tools)
        .default(0)
        .interact()
    else {
        return;
    };

    let Ok(confirm) = Confirm::new()
        .with_prompt(&format!("Remove environment '{}'?", env_name))
        .default(false)
        .interact()
    else {
        return;
    };

    if !confirm {
        println!("Cancelled");
        return;
    }

    match tool {
        0 => {
            // Check if path exists and is a directory
            let path = std::path::Path::new(&env_name);
            if !path.exists() {
                eprintln!("Environment directory does not exist: {}", env_name);
                return;
            }
            if !path.is_dir() {
                eprintln!("Path is not a directory: {}", env_name);
                return;
            }
            // Verify it looks like a venv (has pyvenv.cfg)
            if !path.join("pyvenv.cfg").exists() {
                eprintln!(
                    "Warning: {} does not look like a Python virtual environment",
                    env_name
                );
                let Ok(really_confirm) = Confirm::new()
                    .with_prompt("Delete anyway?")
                    .default(false)
                    .interact()
                else {
                    return;
                };
                if !really_confirm {
                    return;
                }
            }

            match std::fs::remove_dir_all(&env_name) {
                Ok(()) => {
                    println!("Removed directory: {}", env_name);
                }
                Err(e) => {
                    eprintln!("Failed to remove directory: {}", e);
                }
            }
        }
        1 => {
            match Command::new("conda")
                .args(["env", "remove", "-n", &env_name, "-y"])
                .status()
            {
                Ok(status) if status.success() => {
                    println!("Removed conda environment: {}", env_name);
                }
                Ok(status) => {
                    eprintln!("conda env remove failed with code: {:?}", status.code());
                }
                Err(e) => {
                    eprintln!("Failed to run conda: {}", e);
                }
            }
        }
        _ => return,
    }
}

fn install_virtualenv() {
    println!("📦 Installing virtualenv");
    println!("========================");

    if let Err(e) = Command::new("pip3")
        .args(["install", "--user", "virtualenv"])
        .status()
    {
        eprintln!("Failed to install virtualenv: {}", e);
        return;
    }

    println!("✅ virtualenv installed");
}

fn install_conda() {
    println!("🐍 Installing Miniconda");
    println!("=======================");

    if Command::new("which").arg("conda").status().is_ok() {
        println!("✅ Conda is already installed");
        return;
    }

    let Ok(confirm) = Confirm::new()
        .with_prompt("Download and install Miniconda?")
        .default(true)
        .interact()
    else {
        return;
    };

    if confirm {
        println!("📥 Downloading Miniconda...");
        if let Err(e) = Command::new("wget")
            .args(["https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh"])
            .status()
        {
            eprintln!("Failed to download Miniconda: {}", e);
            return;
        }

        println!("🔧 Installing Miniconda...");
        if let Err(e) = Command::new("bash")
            .args(["Miniconda3-latest-Linux-x86_64.sh", "-b"])
            .status()
        {
            eprintln!("Failed to install Miniconda: {}", e);
            return;
        }

        // Add to PATH
        setup_conda_environment();

        // Cleanup
        let _ = std::fs::remove_file("Miniconda3-latest-Linux-x86_64.sh");

        println!("✅ Miniconda installed");
    }
}

fn package_management() {
    println!("📦 Python Package Management");
    println!("============================");

    let options = [
        "📋 List Installed Packages",
        "🔍 Search Packages",
        "📦 Install Package",
        "🗑️  Uninstall Package",
        "🔄 Update Packages",
        "📄 Requirements Management",
        "🔧 pip Configuration",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Package Management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => list_packages(),
        1 => search_packages(),
        2 => install_package(),
        3 => uninstall_package(),
        4 => update_packages(),
        5 => requirements_management(),
        6 => pip_configuration(),
        _ => return,
    }
}

fn list_packages() {
    println!("📋 Installed Python Packages");
    println!("=============================");

    if let Err(e) = Command::new("pip3").args(["list"]).status() {
        eprintln!("Failed to list packages: {}", e);
    }
}

fn search_packages() {
    let Ok(query): Result<String, _> = Input::new().with_prompt("Search query").interact_text()
    else {
        return;
    };

    println!("🔍 Searching for: {}", query);
    println!("💡 Visit https://pypi.org/search/?q={}", query);
}

fn install_package() {
    let Ok(package): Result<String, _> = Input::new().with_prompt("Package name").interact_text()
    else {
        return;
    };

    // Validate package name
    if let Err(e) = validate_package_name(&package) {
        eprintln!("Invalid package name: {}", e);
        return;
    }

    let options = [
        "Install normally",
        "Install for user only",
        "Install in development mode",
    ];
    let Ok(install_type) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Installation type")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    let result = match install_type {
        0 => Command::new("pip3").args(["install", &package]).status(),
        1 => Command::new("pip3")
            .args(["install", "--user", &package])
            .status(),
        2 => Command::new("pip3")
            .args(["install", "-e", &package])
            .status(),
        _ => return,
    };

    match result {
        Ok(status) if status.success() => {
            println!("Package '{}' installed", package);
        }
        Ok(status) => {
            eprintln!("pip install failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run pip: {}", e);
        }
    }
}

fn uninstall_package() {
    let Ok(package): Result<String, _> = Input::new()
        .with_prompt("Package name to uninstall")
        .interact_text()
    else {
        return;
    };

    // Validate package name
    if let Err(e) = validate_package_name(&package) {
        eprintln!("Invalid package name: {}", e);
        return;
    }

    match Command::new("pip3")
        .args(["uninstall", "-y", &package])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Package '{}' uninstalled", package);
        }
        Ok(status) => {
            eprintln!("pip uninstall failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run pip: {}", e);
        }
    }
}

fn update_packages() {
    println!("🔄 Updating Python Packages");
    println!("===========================");

    // Update pip first
    if let Err(e) = Command::new("pip3")
        .args(["install", "--upgrade", "pip"])
        .status()
    {
        eprintln!("Failed to upgrade pip: {}", e);
    }

    // List outdated packages
    println!("📋 Checking for outdated packages...");
    if let Err(e) = Command::new("pip3").args(["list", "--outdated"]).status() {
        eprintln!("Failed to list outdated packages: {}", e);
    }

    println!("💡 To update all packages: pip-review --auto");
    println!("💡 Install pip-review with: pip install pip-review");
}

fn requirements_management() {
    println!("📄 Requirements Management");
    println!("==========================");

    let options = [
        "📋 Generate requirements.txt",
        "📦 Install from requirements.txt",
        "🔒 Generate locked requirements",
        "📊 Show dependency tree",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Requirements")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => match Command::new("pip3").args(["freeze"]).output() {
            Ok(output) => {
                if let Err(e) = std::fs::write("requirements.txt", output.stdout) {
                    eprintln!("Failed to write requirements.txt: {}", e);
                } else {
                    println!("✅ requirements.txt generated");
                }
            }
            Err(e) => {
                eprintln!("Failed to run pip freeze: {}", e);
            }
        },
        1 => {
            if let Err(e) = Command::new("pip3")
                .args(["install", "-r", "requirements.txt"])
                .status()
            {
                eprintln!("Failed to install from requirements.txt: {}", e);
            }
        }
        2 => {
            println!("💡 Use pip-tools: pip install pip-tools");
            println!("💡 Then: pip-compile requirements.in");
        }
        3 => {
            println!("💡 Install pipdeptree: pip install pipdeptree");
            if let Err(e) = Command::new("pipdeptree").status() {
                eprintln!("Failed to run pipdeptree: {}", e);
            }
        }
        _ => return,
    }
}

fn pip_configuration() {
    println!("🔧 pip Configuration");
    println!("====================");

    println!("📋 Current pip configuration:");
    if let Err(e) = Command::new("pip3").args(["config", "list"]).status() {
        eprintln!("Failed to list pip config: {}", e);
    }

    println!("\n💡 Common pip configurations:");
    println!("  pip config set global.timeout 60");
    println!("  pip config set global.index-url https://pypi.org/simple/");
}

fn development_tools() {
    println!("🛠️  Python Development Tools");
    println!("=============================");

    let tools = [
        "📝 Install Python Language Server",
        "🎨 Install Code Formatters",
        "🔍 Install Linters",
        "⚡ Install IPython/Jupyter",
        "🔧 Install Development Utilities",
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
        0 => install_language_server(),
        1 => install_formatters(),
        2 => install_linters(),
        3 => install_jupyter(),
        4 => install_dev_utilities(),
        _ => return,
    }
}

fn install_language_server() {
    println!("📝 Installing Python Language Servers");
    println!("======================================");

    let servers = [
        "pylsp (Python LSP Server)",
        "pyright (Microsoft)",
        "jedi-language-server",
    ];

    let Ok(selected) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select language servers to install")
        .items(&servers)
        .interact()
    else {
        return;
    };

    for &index in &selected {
        match index {
            0 => {
                if let Err(e) = Command::new("pip3")
                    .args(["install", "python-lsp-server"])
                    .status()
                {
                    eprintln!("Failed to install pylsp: {}", e);
                } else {
                    println!("✅ pylsp installed");
                }
            }
            1 => {
                if let Err(e) = Command::new("pip3").args(["install", "pyright"]).status() {
                    eprintln!("Failed to install pyright: {}", e);
                } else {
                    println!("✅ pyright installed");
                }
            }
            2 => {
                if let Err(e) = Command::new("pip3")
                    .args(["install", "jedi-language-server"])
                    .status()
                {
                    eprintln!("Failed to install jedi-language-server: {}", e);
                } else {
                    println!("✅ jedi-language-server installed");
                }
            }
            _ => {}
        }
    }
}

fn install_formatters() {
    println!("🎨 Installing Python Code Formatters");
    println!("=====================================");

    let formatters = [
        "black (The uncompromising formatter)",
        "autopep8 (PEP 8 formatter)",
        "isort (Import sorter)",
        "yapf (Google's formatter)",
    ];

    let Ok(selected) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select formatters to install")
        .items(&formatters)
        .interact()
    else {
        return;
    };

    for &index in &selected {
        match index {
            0 => {
                if let Err(e) = Command::new("pip3").args(["install", "black"]).status() {
                    eprintln!("Failed to install black: {}", e);
                } else {
                    println!("✅ black installed");
                }
            }
            1 => {
                if let Err(e) = Command::new("pip3").args(["install", "autopep8"]).status() {
                    eprintln!("Failed to install autopep8: {}", e);
                } else {
                    println!("✅ autopep8 installed");
                }
            }
            2 => {
                if let Err(e) = Command::new("pip3").args(["install", "isort"]).status() {
                    eprintln!("Failed to install isort: {}", e);
                } else {
                    println!("✅ isort installed");
                }
            }
            3 => {
                if let Err(e) = Command::new("pip3").args(["install", "yapf"]).status() {
                    eprintln!("Failed to install yapf: {}", e);
                } else {
                    println!("✅ yapf installed");
                }
            }
            _ => {}
        }
    }
}

fn install_linters() {
    println!("🔍 Installing Python Linters");
    println!("=============================");

    let linters = [
        "flake8 (Style guide enforcement)",
        "pylint (Comprehensive linter)",
        "mypy (Static type checker)",
        "bandit (Security linter)",
    ];

    let Ok(selected) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select linters to install")
        .items(&linters)
        .interact()
    else {
        return;
    };

    for &index in &selected {
        match index {
            0 => {
                if let Err(e) = Command::new("pip3").args(["install", "flake8"]).status() {
                    eprintln!("Failed to install flake8: {}", e);
                } else {
                    println!("✅ flake8 installed");
                }
            }
            1 => {
                if let Err(e) = Command::new("pip3").args(["install", "pylint"]).status() {
                    eprintln!("Failed to install pylint: {}", e);
                } else {
                    println!("✅ pylint installed");
                }
            }
            2 => {
                if let Err(e) = Command::new("pip3").args(["install", "mypy"]).status() {
                    eprintln!("Failed to install mypy: {}", e);
                } else {
                    println!("✅ mypy installed");
                }
            }
            3 => {
                if let Err(e) = Command::new("pip3").args(["install", "bandit"]).status() {
                    eprintln!("Failed to install bandit: {}", e);
                } else {
                    println!("✅ bandit installed");
                }
            }
            _ => {}
        }
    }
}

fn install_jupyter() {
    println!("⚡ Installing IPython/Jupyter");
    println!("==============================");

    let tools = [
        "IPython (Enhanced interactive shell)",
        "Jupyter Notebook",
        "JupyterLab",
        "All of the above",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Jupyter tools")
        .items(&tools)
        .default(3)
        .interact()
    else {
        return;
    };

    match choice {
        0 => {
            if let Err(e) = Command::new("pip3").args(["install", "ipython"]).status() {
                eprintln!("Failed to install ipython: {}", e);
            } else {
                println!("✅ IPython installed");
            }
        }
        1 => {
            if let Err(e) = Command::new("pip3").args(["install", "notebook"]).status() {
                eprintln!("Failed to install notebook: {}", e);
            } else {
                println!("✅ Jupyter Notebook installed");
            }
        }
        2 => {
            if let Err(e) = Command::new("pip3")
                .args(["install", "jupyterlab"])
                .status()
            {
                eprintln!("Failed to install jupyterlab: {}", e);
            } else {
                println!("✅ JupyterLab installed");
            }
        }
        3 => {
            if let Err(e) = Command::new("pip3")
                .args(["install", "ipython", "jupyterlab"])
                .status()
            {
                eprintln!("Failed to install IPython/JupyterLab: {}", e);
            } else {
                println!("✅ IPython and JupyterLab installed");
            }
        }
        _ => return,
    }
}

fn install_dev_utilities() {
    println!("🔧 Installing Development Utilities");
    println!("===================================");

    let utilities = [
        "cookiecutter (Project templates)",
        "pre-commit (Git hooks)",
        "tox (Testing across environments)",
        "poetry (Dependency management)",
        "pipenv (Pip + virtualenv)",
    ];

    let Ok(selected) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select utilities to install")
        .items(&utilities)
        .interact()
    else {
        return;
    };

    for &index in &selected {
        match index {
            0 => {
                if let Err(e) = Command::new("pip3")
                    .args(["install", "cookiecutter"])
                    .status()
                {
                    eprintln!("Failed to install cookiecutter: {}", e);
                } else {
                    println!("✅ cookiecutter installed");
                }
            }
            1 => {
                if let Err(e) = Command::new("pip3")
                    .args(["install", "pre-commit"])
                    .status()
                {
                    eprintln!("Failed to install pre-commit: {}", e);
                } else {
                    println!("✅ pre-commit installed");
                }
            }
            2 => {
                if let Err(e) = Command::new("pip3").args(["install", "tox"]).status() {
                    eprintln!("Failed to install tox: {}", e);
                } else {
                    println!("✅ tox installed");
                }
            }
            3 => {
                if let Err(e) = Command::new("pip3").args(["install", "poetry"]).status() {
                    eprintln!("Failed to install poetry: {}", e);
                } else {
                    println!("✅ poetry installed");
                }
            }
            4 => {
                if let Err(e) = Command::new("pip3").args(["install", "pipenv"]).status() {
                    eprintln!("Failed to install pipenv: {}", e);
                } else {
                    println!("✅ pipenv installed");
                }
            }
            _ => {}
        }
    }
}

fn testing_quality() {
    println!("🧪 Testing & Code Quality");
    println!("=========================");

    let options = [
        "🧪 Install Testing Frameworks",
        "📊 Install Coverage Tools",
        "🔍 Install Quality Tools",
        "🚀 Run Tests",
        "📋 Generate Coverage Report",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Testing & Quality")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => install_testing_frameworks(),
        1 => install_coverage_tools(),
        2 => install_quality_tools(),
        3 => run_tests(),
        4 => generate_coverage_report(),
        _ => return,
    }
}

fn install_testing_frameworks() {
    let frameworks = [
        "pytest (Recommended)",
        "unittest (Built-in)",
        "nose2",
        "hypothesis (Property-based testing)",
    ];

    let Ok(selected) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select testing frameworks")
        .items(&frameworks)
        .interact()
    else {
        return;
    };

    for &index in &selected {
        match index {
            0 => {
                if let Err(e) = Command::new("pip3").args(["install", "pytest"]).status() {
                    eprintln!("Failed to install pytest: {}", e);
                } else {
                    println!("✅ pytest installed");
                }
            }
            1 => {
                println!("✅ unittest is built-in with Python");
            }
            2 => {
                if let Err(e) = Command::new("pip3").args(["install", "nose2"]).status() {
                    eprintln!("Failed to install nose2: {}", e);
                } else {
                    println!("✅ nose2 installed");
                }
            }
            3 => {
                if let Err(e) = Command::new("pip3")
                    .args(["install", "hypothesis"])
                    .status()
                {
                    eprintln!("Failed to install hypothesis: {}", e);
                } else {
                    println!("✅ hypothesis installed");
                }
            }
            _ => {}
        }
    }
}

fn install_coverage_tools() {
    if let Err(e) = Command::new("pip3")
        .args(["install", "coverage", "pytest-cov"])
        .status()
    {
        eprintln!("Failed to install coverage tools: {}", e);
    } else {
        println!("✅ Coverage tools installed");
    }
}

fn install_quality_tools() {
    let tools = ["flake8", "pylint", "mypy", "bandit", "safety", "pydocstyle"];

    for tool in &tools {
        if let Err(e) = Command::new("pip3").args(["install", tool]).status() {
            eprintln!("Failed to install {}: {}", tool, e);
        }
    }

    println!("✅ Quality tools installed");
}

fn run_tests() {
    println!("🧪 Running Python Tests");
    println!("=======================");

    if Command::new("which").arg("pytest").status().is_ok() {
        if let Err(e) = Command::new("pytest").status() {
            eprintln!("Failed to run pytest: {}", e);
        }
    } else if let Err(e) = Command::new("python3")
        .args(["-m", "unittest", "discover"])
        .status()
    {
        eprintln!("Failed to run unittest: {}", e);
    }
}

fn generate_coverage_report() {
    println!("📋 Generating Coverage Report");
    println!("=============================");

    if let Err(e) = Command::new("coverage")
        .args(["run", "-m", "pytest"])
        .status()
    {
        eprintln!("Failed to run coverage: {}", e);
        return;
    }
    if let Err(e) = Command::new("coverage").args(["report"]).status() {
        eprintln!("Failed to generate report: {}", e);
    }
    if let Err(e) = Command::new("coverage").args(["html"]).status() {
        eprintln!("Failed to generate HTML report: {}", e);
    }

    println!("✅ Coverage report generated in htmlcov/");
}

fn learning_resources() {
    println!("📚 Python Learning Resources");
    println!("============================");

    println!("🌐 Official Documentation: https://docs.python.org/");
    println!("📖 Python Tutorial: https://docs.python.org/tutorial/");
    println!("💡 Real Python: https://realpython.com/");
    println!("📚 Automate the Boring Stuff: https://automatetheboringstuff.com/");
    println!("🎓 Python.org Beginner's Guide: https://wiki.python.org/moin/BeginnersGuide");
    println!("📺 Python YouTube Channels: Corey Schafer, sentdex, Tech With Tim");
}

fn show_python_version() {
    if let Ok(output) = Command::new("python3").arg("--version").output() {
        let version = String::from_utf8_lossy(&output.stdout);
        println!("📋 Python version: {}", version.trim());
    }
}

fn check_pip_installation() {
    if Command::new("which").arg("pip3").status().is_ok() {
        println!("✅ pip3 is installed");
    } else {
        println!("❌ pip3 not found. Installing...");
        install_pip();
    }
}

fn install_pip() {
    if Command::new("which").arg("pacman").status().is_ok() {
        if let Err(e) = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "python-pip"])
            .status()
        {
            eprintln!("Failed to install python-pip: {}", e);
        }
    } else if Command::new("which").arg("apt").status().is_ok() {
        if let Err(e) = Command::new("sudo")
            .args(["apt", "install", "-y", "python3-pip"])
            .status()
        {
            eprintln!("Failed to install python3-pip: {}", e);
        }
    } else {
        println!("💡 Install pip manually: python3 -m ensurepip");
    }
}

fn setup_python_environment() {
    println!("⚙️  Setting up Python environment...");

    // Upgrade pip
    if let Err(e) = Command::new("pip3")
        .args(["install", "--upgrade", "pip"])
        .status()
    {
        eprintln!("Failed to upgrade pip: {}", e);
    }

    // Install essential packages
    let essential_packages = ["wheel", "setuptools", "virtualenv"];
    for package in &essential_packages {
        if let Err(e) = Command::new("pip3")
            .args(["install", "--user", package])
            .status()
        {
            eprintln!("Failed to install {}: {}", package, e);
        }
    }

    println!("✅ Python environment setup completed");
}

fn setup_pyenv_environment() {
    let Some(home_dir) = dirs::home_dir() else {
        return;
    };

    let shell_files = [
        format!("{}/.bashrc", home_dir.display()),
        format!("{}/.zshrc", home_dir.display()),
    ];

    let pyenv_setup = vec![
        "export PYENV_ROOT=\"$HOME/.pyenv\"",
        "command -v pyenv >/dev/null || export PATH=\"$PYENV_ROOT/bin:$PATH\"",
        "eval \"$(pyenv init -)\"",
    ];

    for shell_file in &shell_files {
        if std::path::Path::new(shell_file).exists()
            && let Ok(content) = std::fs::read_to_string(shell_file)
            && !content.contains("PYENV_ROOT")
        {
            let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(shell_file) else {
                continue;
            };

            use std::io::Write;
            let _ = writeln!(file, "\n# pyenv");
            for line in &pyenv_setup {
                let _ = writeln!(file, "{}", line);
            }

            println!("✅ Added pyenv to {}", shell_file);
        }
    }
}

fn setup_conda_environment() {
    let Some(home_dir) = dirs::home_dir() else {
        return;
    };

    let shell_files = [
        format!("{}/.bashrc", home_dir.display()),
        format!("{}/.zshrc", home_dir.display()),
    ];

    let conda_path = format!("{}/miniconda3/bin", home_dir.display());

    for shell_file in &shell_files {
        if std::path::Path::new(shell_file).exists()
            && let Ok(content) = std::fs::read_to_string(shell_file)
            && !content.contains("miniconda3")
        {
            let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(shell_file) else {
                continue;
            };

            use std::io::Write;
            let _ = writeln!(file, "\n# Miniconda");
            let _ = writeln!(file, "export PATH=\"{}:$PATH\"", conda_path);

            println!("✅ Added conda to {}", shell_file);
        }
    }
}
