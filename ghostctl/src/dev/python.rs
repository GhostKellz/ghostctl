use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::process::Command;

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Python Development")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let method = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Installation method")
        .items(&install_methods)
        .default(0)
        .interact()
        .unwrap();

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
    if Command::new("which").arg("reap").status().is_ok() {
        println!("📦 Installing Python with reaper...");
        let _ = Command::new("reap").arg("python").status();
        let _ = Command::new("reap").arg("python-pip").status();
    } else if Command::new("which").arg("pacman").status().is_ok() {
        println!("📦 Installing Python with pacman...");
        let _ = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", "python", "python-pip"])
            .status();
    } else if Command::new("which").arg("apt").status().is_ok() {
        println!("📦 Installing Python with apt...");
        let _ = Command::new("sudo").args(&["apt", "update"]).status();
        let _ = Command::new("sudo")
            .args(&[
                "apt",
                "install",
                "-y",
                "python3",
                "python3-pip",
                "python3-venv",
            ])
            .status();
    } else if Command::new("which").arg("dnf").status().is_ok() {
        println!("📦 Installing Python with dnf...");
        let _ = Command::new("sudo")
            .args(&["dnf", "install", "-y", "python3", "python3-pip"])
            .status();
    }

    if Command::new("which").arg("python3").status().is_ok() {
        println!("✅ Python installed successfully");
        show_python_version();
        setup_python_environment();
    } else {
        println!("❌ Package manager installation failed");
    }
}

fn install_pyenv() {
    println!("🐍 Installing pyenv (Python Version Manager)");
    println!("=============================================");

    if Command::new("which").arg("pyenv").status().is_ok() {
        println!("✅ pyenv is already installed");
        return;
    }

    let confirm = Confirm::new()
        .with_prompt("Install pyenv for Python version management?")
        .default(true)
        .interact()
        .unwrap();

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

    let confirm = Confirm::new()
        .with_prompt("Continue with source build?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        // Install build dependencies first
        if Command::new("which").arg("pacman").status().is_ok() {
            let _ = Command::new("sudo")
                .args(&[
                    "pacman",
                    "-S",
                    "--noconfirm",
                    "base-devel",
                    "openssl",
                    "zlib",
                    "bzip2",
                ])
                .status();
        }

        println!("📥 Downloading Python source...");
        let _ = Command::new("wget")
            .args(&["https://www.python.org/ftp/python/3.11.0/Python-3.11.0.tgz"])
            .status();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Virtual Environment")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
    let env_name: String = Input::new()
        .with_prompt("Environment name")
        .interact_text()
        .unwrap();

    let tools = ["venv (built-in)", "virtualenv", "conda"];
    let tool = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Environment tool")
        .items(&tools)
        .default(0)
        .interact()
        .unwrap();

    match tool {
        0 => {
            let _ = Command::new("python3")
                .args(&["-m", "venv", &env_name])
                .status();
            println!("✅ Virtual environment '{}' created with venv", env_name);
            println!("💡 Activate with: source {}/bin/activate", env_name);
        }
        1 => {
            let _ = Command::new("virtualenv").arg(&env_name).status();
            println!(
                "✅ Virtual environment '{}' created with virtualenv",
                env_name
            );
        }
        2 => {
            let _ = Command::new("conda")
                .args(&["create", "-n", &env_name, "python"])
                .status();
            println!("✅ Conda environment '{}' created", env_name);
            println!("💡 Activate with: conda activate {}", env_name);
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
        let _ = Command::new("conda").args(&["env", "list"]).status();
    }

    // List pyenv environments
    if Command::new("which").arg("pyenv").status().is_ok() {
        println!("\n🐍 Pyenv versions:");
        let _ = Command::new("pyenv").args(&["versions"]).status();
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
    let env_name: String = Input::new()
        .with_prompt("Environment name to remove")
        .interact_text()
        .unwrap();

    let tools = ["venv/virtualenv (delete directory)", "conda"];
    let tool = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Environment type")
        .items(&tools)
        .default(0)
        .interact()
        .unwrap();

    let confirm = Confirm::new()
        .with_prompt(&format!("Remove environment '{}'?", env_name))
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        match tool {
            0 => {
                let _ = std::fs::remove_dir_all(&env_name);
                println!("✅ Removed directory: {}", env_name);
            }
            1 => {
                let _ = Command::new("conda")
                    .args(&["env", "remove", "-n", &env_name])
                    .status();
                println!("✅ Removed conda environment: {}", env_name);
            }
            _ => return,
        }
    }
}

fn install_virtualenv() {
    println!("📦 Installing virtualenv");
    println!("========================");

    let _ = Command::new("pip3")
        .args(&["install", "--user", "virtualenv"])
        .status();

    println!("✅ virtualenv installed");
}

fn install_conda() {
    println!("🐍 Installing Miniconda");
    println!("=======================");

    if Command::new("which").arg("conda").status().is_ok() {
        println!("✅ Conda is already installed");
        return;
    }

    let confirm = Confirm::new()
        .with_prompt("Download and install Miniconda?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        println!("📥 Downloading Miniconda...");
        let _ = Command::new("wget")
            .args(&["https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh"])
            .status();

        println!("🔧 Installing Miniconda...");
        let _ = Command::new("bash")
            .args(&["Miniconda3-latest-Linux-x86_64.sh", "-b"])
            .status();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Package Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let _ = Command::new("pip3").args(&["list"]).status();
}

fn search_packages() {
    let query: String = Input::new()
        .with_prompt("Search query")
        .interact_text()
        .unwrap();

    println!("🔍 Searching for: {}", query);
    println!("💡 Visit https://pypi.org/search/?q={}", query);
}

fn install_package() {
    let package: String = Input::new()
        .with_prompt("Package name")
        .interact_text()
        .unwrap();

    let options = [
        "📦 Install normally",
        "👤 Install for user only",
        "🛠️  Install in development mode",
    ];
    let install_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Installation type")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match install_type {
        0 => {
            let _ = Command::new("pip3").args(&["install", &package]).status();
        }
        1 => {
            let _ = Command::new("pip3")
                .args(&["install", "--user", &package])
                .status();
        }
        2 => {
            let _ = Command::new("pip3")
                .args(&["install", "-e", &package])
                .status();
        }
        _ => return,
    }

    println!("✅ Package '{}' installed", package);
}

fn uninstall_package() {
    let package: String = Input::new()
        .with_prompt("Package name to uninstall")
        .interact_text()
        .unwrap();

    let _ = Command::new("pip3").args(&["uninstall", &package]).status();
}

fn update_packages() {
    println!("🔄 Updating Python Packages");
    println!("===========================");

    // Update pip first
    let _ = Command::new("pip3")
        .args(&["install", "--upgrade", "pip"])
        .status();

    // List outdated packages
    println!("📋 Checking for outdated packages...");
    let _ = Command::new("pip3").args(&["list", "--outdated"]).status();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Requirements")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let _ = Command::new("pip3")
                .args(&["freeze"])
                .output()
                .and_then(|output| {
                    std::fs::write("requirements.txt", output.stdout)?;
                    Ok(())
                });
            println!("✅ requirements.txt generated");
        }
        1 => {
            let _ = Command::new("pip3")
                .args(&["install", "-r", "requirements.txt"])
                .status();
        }
        2 => {
            println!("💡 Use pip-tools: pip install pip-tools");
            println!("💡 Then: pip-compile requirements.in");
        }
        3 => {
            println!("💡 Install pipdeptree: pip install pipdeptree");
            let _ = Command::new("pipdeptree").status();
        }
        _ => return,
    }
}

fn pip_configuration() {
    println!("🔧 pip Configuration");
    println!("====================");

    println!("📋 Current pip configuration:");
    let _ = Command::new("pip3").args(&["config", "list"]).status();

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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Development Tools")
        .items(&tools)
        .default(0)
        .interact()
        .unwrap();

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

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select language servers to install")
        .items(&servers)
        .interact()
        .unwrap();

    for &index in &selected {
        match index {
            0 => {
                let _ = Command::new("pip3")
                    .args(&["install", "python-lsp-server"])
                    .status();
                println!("✅ pylsp installed");
            }
            1 => {
                let _ = Command::new("pip3").args(&["install", "pyright"]).status();
                println!("✅ pyright installed");
            }
            2 => {
                let _ = Command::new("pip3")
                    .args(&["install", "jedi-language-server"])
                    .status();
                println!("✅ jedi-language-server installed");
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

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select formatters to install")
        .items(&formatters)
        .interact()
        .unwrap();

    for &index in &selected {
        match index {
            0 => {
                let _ = Command::new("pip3").args(&["install", "black"]).status();
                println!("✅ black installed");
            }
            1 => {
                let _ = Command::new("pip3").args(&["install", "autopep8"]).status();
                println!("✅ autopep8 installed");
            }
            2 => {
                let _ = Command::new("pip3").args(&["install", "isort"]).status();
                println!("✅ isort installed");
            }
            3 => {
                let _ = Command::new("pip3").args(&["install", "yapf"]).status();
                println!("✅ yapf installed");
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

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select linters to install")
        .items(&linters)
        .interact()
        .unwrap();

    for &index in &selected {
        match index {
            0 => {
                let _ = Command::new("pip3").args(&["install", "flake8"]).status();
                println!("✅ flake8 installed");
            }
            1 => {
                let _ = Command::new("pip3").args(&["install", "pylint"]).status();
                println!("✅ pylint installed");
            }
            2 => {
                let _ = Command::new("pip3").args(&["install", "mypy"]).status();
                println!("✅ mypy installed");
            }
            3 => {
                let _ = Command::new("pip3").args(&["install", "bandit"]).status();
                println!("✅ bandit installed");
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Jupyter tools")
        .items(&tools)
        .default(3)
        .interact()
        .unwrap();

    match choice {
        0 => {
            let _ = Command::new("pip3").args(&["install", "ipython"]).status();
            println!("✅ IPython installed");
        }
        1 => {
            let _ = Command::new("pip3").args(&["install", "notebook"]).status();
            println!("✅ Jupyter Notebook installed");
        }
        2 => {
            let _ = Command::new("pip3")
                .args(&["install", "jupyterlab"])
                .status();
            println!("✅ JupyterLab installed");
        }
        3 => {
            let _ = Command::new("pip3")
                .args(&["install", "ipython", "jupyterlab"])
                .status();
            println!("✅ IPython and JupyterLab installed");
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

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select utilities to install")
        .items(&utilities)
        .interact()
        .unwrap();

    for &index in &selected {
        match index {
            0 => {
                let _ = Command::new("pip3")
                    .args(&["install", "cookiecutter"])
                    .status();
                println!("✅ cookiecutter installed");
            }
            1 => {
                let _ = Command::new("pip3")
                    .args(&["install", "pre-commit"])
                    .status();
                println!("✅ pre-commit installed");
            }
            2 => {
                let _ = Command::new("pip3").args(&["install", "tox"]).status();
                println!("✅ tox installed");
            }
            3 => {
                let _ = Command::new("pip3").args(&["install", "poetry"]).status();
                println!("✅ poetry installed");
            }
            4 => {
                let _ = Command::new("pip3").args(&["install", "pipenv"]).status();
                println!("✅ pipenv installed");
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

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Testing & Quality")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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

    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select testing frameworks")
        .items(&frameworks)
        .interact()
        .unwrap();

    for &index in &selected {
        match index {
            0 => {
                let _ = Command::new("pip3").args(&["install", "pytest"]).status();
                println!("✅ pytest installed");
            }
            1 => {
                println!("✅ unittest is built-in with Python");
            }
            2 => {
                let _ = Command::new("pip3").args(&["install", "nose2"]).status();
                println!("✅ nose2 installed");
            }
            3 => {
                let _ = Command::new("pip3")
                    .args(&["install", "hypothesis"])
                    .status();
                println!("✅ hypothesis installed");
            }
            _ => {}
        }
    }
}

fn install_coverage_tools() {
    let _ = Command::new("pip3")
        .args(&["install", "coverage", "pytest-cov"])
        .status();
    println!("✅ Coverage tools installed");
}

fn install_quality_tools() {
    let tools = ["flake8", "pylint", "mypy", "bandit", "safety", "pydocstyle"];

    for tool in &tools {
        let _ = Command::new("pip3").args(&["install", tool]).status();
    }

    println!("✅ Quality tools installed");
}

fn run_tests() {
    println!("🧪 Running Python Tests");
    println!("=======================");

    if Command::new("which").arg("pytest").status().is_ok() {
        let _ = Command::new("pytest").status();
    } else {
        let _ = Command::new("python3")
            .args(&["-m", "unittest", "discover"])
            .status();
    }
}

fn generate_coverage_report() {
    println!("📋 Generating Coverage Report");
    println!("=============================");

    let _ = Command::new("coverage")
        .args(&["run", "-m", "pytest"])
        .status();
    let _ = Command::new("coverage").args(&["report"]).status();
    let _ = Command::new("coverage").args(&["html"]).status();

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
        let _ = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", "python-pip"])
            .status();
    } else if Command::new("which").arg("apt").status().is_ok() {
        let _ = Command::new("sudo")
            .args(&["apt", "install", "-y", "python3-pip"])
            .status();
    } else {
        println!("💡 Install pip manually: python3 -m ensurepip");
    }
}

fn setup_python_environment() {
    println!("⚙️  Setting up Python environment...");

    // Upgrade pip
    let _ = Command::new("pip3")
        .args(&["install", "--upgrade", "pip"])
        .status();

    // Install essential packages
    let essential_packages = ["wheel", "setuptools", "virtualenv"];
    for package in &essential_packages {
        let _ = Command::new("pip3")
            .args(&["install", "--user", package])
            .status();
    }

    println!("✅ Python environment setup completed");
}

fn setup_pyenv_environment() {
    let shell_files = [
        format!("{}/.bashrc", dirs::home_dir().unwrap().display()),
        format!("{}/.zshrc", dirs::home_dir().unwrap().display()),
    ];

    let pyenv_setup = vec![
        "export PYENV_ROOT=\"$HOME/.pyenv\"",
        "command -v pyenv >/dev/null || export PATH=\"$PYENV_ROOT/bin:$PATH\"",
        "eval \"$(pyenv init -)\"",
    ];

    for shell_file in &shell_files {
        if std::path::Path::new(shell_file).exists()
            && let Ok(content) = std::fs::read_to_string(shell_file)
                && !content.contains("PYENV_ROOT") {
                    let mut file = std::fs::OpenOptions::new()
                        .append(true)
                        .open(shell_file)
                        .unwrap();

                    use std::io::Write;
                    writeln!(file, "\n# pyenv").unwrap();
                    for line in &pyenv_setup {
                        writeln!(file, "{}", line).unwrap();
                    }

                    println!("✅ Added pyenv to {}", shell_file);
                }
    }
}

fn setup_conda_environment() {
    let shell_files = [
        format!("{}/.bashrc", dirs::home_dir().unwrap().display()),
        format!("{}/.zshrc", dirs::home_dir().unwrap().display()),
    ];

    let conda_path = format!("{}/miniconda3/bin", dirs::home_dir().unwrap().display());

    for shell_file in &shell_files {
        if std::path::Path::new(shell_file).exists()
            && let Ok(content) = std::fs::read_to_string(shell_file)
                && !content.contains("miniconda3") {
                    let mut file = std::fs::OpenOptions::new()
                        .append(true)
                        .open(shell_file)
                        .unwrap();

                    use std::io::Write;
                    writeln!(file, "\n# Miniconda").unwrap();
                    writeln!(file, "export PATH=\"{}:$PATH\"", conda_path).unwrap();

                    println!("✅ Added conda to {}", shell_file);
                }
    }
}
