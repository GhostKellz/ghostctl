use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::process::Command;

pub fn go_development_menu() {
    println!("🐹 Go Development Environment");
    println!("=============================");

    let options = [
        "📦 Install Go Compiler",
        "🛠️  Go Project Management",
        "📦 Package Management",
        "🔧 Development Tools",
        "🧪 Testing & Benchmarking",
        "📚 Learning Resources",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Go Development")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_go_compiler(),
        1 => go_project_management(),
        2 => go_package_management(),
        3 => go_development_tools(),
        4 => go_testing_benchmarking(),
        5 => go_learning_resources(),
        _ => return,
    }
}

fn install_go_compiler() {
    println!("📦 Installing Go Compiler");
    println!("==========================");

    if Command::new("which").arg("go").status().is_ok() {
        println!("✅ Go is already installed");
        show_go_version();
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
        0 => install_go_package_manager(),
        1 => install_go_official(),
        2 => install_go_from_source(),
        _ => return,
    }
}

fn install_go_package_manager() {
    // Try different package managers with reaper priority
    if Command::new("which").arg("reap").status().is_ok() {
        println!("📦 Installing Go with reaper...");
        let _ = Command::new("reap").arg("go").status();
    } else if Command::new("which").arg("pacman").status().is_ok() {
        println!("📦 Installing Go with pacman...");
        let _ = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", "go"])
            .status();
    } else if Command::new("which").arg("apt").status().is_ok() {
        println!("📦 Installing Go with apt...");
        let _ = Command::new("sudo").args(&["apt", "update"]).status();
        let _ = Command::new("sudo")
            .args(&["apt", "install", "-y", "golang-go"])
            .status();
    } else if Command::new("which").arg("dnf").status().is_ok() {
        println!("📦 Installing Go with dnf...");
        let _ = Command::new("sudo")
            .args(&["dnf", "install", "-y", "go"])
            .status();
    }

    if Command::new("which").arg("go").status().is_ok() {
        println!("✅ Go installed successfully");
        setup_go_environment();
        show_go_version();
    } else {
        println!("❌ Package manager installation failed. Try official download.");
    }
}

fn install_go_official() {
    println!("🌐 Installing Go from Official Downloads");
    println!("========================================");

    println!("💡 Visit https://golang.org/dl/ for the latest version");

    let confirm = Confirm::new()
        .with_prompt("Download and install latest Go?")
        .default(true)
        .interact()
        .unwrap();

    if confirm {
        // Remove existing Go installation
        let _ = Command::new("sudo")
            .args(&["rm", "-rf", "/usr/local/go"])
            .status();

        // Download latest Go (this would need to be updated with actual latest version)
        println!("📥 Downloading Go...");
        let download_url = "https://golang.org/dl/go1.21.5.linux-amd64.tar.gz";

        let _ = Command::new("curl")
            .args(&["-L", download_url, "-o", "/tmp/go.tar.gz"])
            .status();

        // Extract
        let _ = Command::new("sudo")
            .args(&["tar", "-C", "/usr/local", "-xzf", "/tmp/go.tar.gz"])
            .status();

        // Setup environment
        setup_go_environment();

        // Cleanup
        let _ = std::fs::remove_file("/tmp/go.tar.gz");

        println!("✅ Go installed to /usr/local/go");
    }
}

fn install_go_from_source() {
    println!("🔨 Building Go from Source");
    println!("===========================");

    println!("⚠️  Building Go from source requires an existing Go installation (Go 1.17+)");

    let confirm = Confirm::new()
        .with_prompt("Continue with source build?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        println!("📥 Cloning Go repository...");
        let _ = Command::new("git")
            .args(&["clone", "https://go.googlesource.com/go", "/tmp/go-source"])
            .status();

        println!("🔨 Building... (this will take a while)");
        let _ = Command::new("./all.bash")
            .current_dir("/tmp/go-source/src")
            .status();

        println!("💡 For detailed build instructions, see: https://golang.org/doc/install/source");
    }
}

fn setup_go_environment() {
    println!("⚙️  Setting up Go environment...");

    let shell_files = [
        format!("{}/.bashrc", dirs::home_dir().unwrap().display()),
        format!("{}/.zshrc", dirs::home_dir().unwrap().display()),
    ];

    let go_env = vec![
        "export PATH=$PATH:/usr/local/go/bin",
        "export GOPATH=$HOME/go",
        "export GOBIN=$GOPATH/bin",
        "export PATH=$PATH:$GOBIN",
    ];

    for shell_file in &shell_files {
        if std::path::Path::new(shell_file).exists()
            && let Ok(content) = std::fs::read_to_string(shell_file)
                && !content.contains("GOPATH") {
                    let mut file = std::fs::OpenOptions::new()
                        .append(true)
                        .open(shell_file)
                        .unwrap();

                    use std::io::Write;
                    writeln!(file, "\n# Go environment").unwrap();
                    for env_var in &go_env {
                        writeln!(file, "{}", env_var).unwrap();
                    }

                    println!("✅ Added Go environment to {}", shell_file);
                }
    }

    // Create GOPATH directory
    let gopath = format!("{}/go", dirs::home_dir().unwrap().display());
    std::fs::create_dir_all(format!("{}/src", gopath)).unwrap();
    std::fs::create_dir_all(format!("{}/bin", gopath)).unwrap();
    std::fs::create_dir_all(format!("{}/pkg", gopath)).unwrap();
}

fn go_project_management() {
    println!("🛠️  Go Project Management");
    println!("=========================");

    let options = [
        "🆕 Create New Go Module",
        "🔧 Initialize Go Module",
        "📦 Add Dependencies",
        "🏗️  Build Project",
        "🚀 Run Project",
        "🧹 Clean Project",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => create_go_module(),
        1 => init_go_module(),
        2 => add_go_dependencies(),
        3 => build_go_project(),
        4 => run_go_project(),
        5 => clean_go_project(),
        _ => return,
    }
}

fn create_go_module() {
    let module_name: String = Input::new()
        .with_prompt("Module name (e.g., github.com/user/project)")
        .interact_text()
        .unwrap();

    let project_name = module_name.split('/').next_back().unwrap_or("go-project");

    println!("📁 Creating project directory...");
    std::fs::create_dir_all(project_name).unwrap();

    let _ = Command::new("go")
        .args(&["mod", "init", &module_name])
        .current_dir(project_name)
        .status();

    // Create main.go
    let main_go = format!(
        r#"package main

import "fmt"

func main() {{
    fmt.Println("Hello, {}!")
}}
"#,
        project_name
    );

    std::fs::write(format!("{}/main.go", project_name), main_go).unwrap();

    println!("✅ Go module '{}' created", project_name);
}

fn init_go_module() {
    let module_name: String = Input::new()
        .with_prompt("Module name")
        .interact_text()
        .unwrap();

    let _ = Command::new("go")
        .args(&["mod", "init", &module_name])
        .status();

    println!("✅ Go module initialized");
}

fn add_go_dependencies() {
    println!("📦 Adding Go Dependencies");
    println!("=========================");

    if !std::path::Path::new("go.mod").exists() {
        println!("❌ No go.mod found. Initialize a Go module first.");
        return;
    }

    let package: String = Input::new()
        .with_prompt("Package to add (e.g., github.com/gin-gonic/gin)")
        .interact_text()
        .unwrap();

    let _ = Command::new("go").args(&["get", &package]).status();

    println!("✅ Added dependency: {}", package);
}

fn build_go_project() {
    println!("🏗️  Building Go Project");
    println!("=======================");

    let _ = Command::new("go").args(&["build"]).status();
    println!("✅ Build completed");
}

fn run_go_project() {
    println!("🚀 Running Go Project");
    println!("=====================");

    let _ = Command::new("go").args(&["run", "."]).status();
}

fn clean_go_project() {
    println!("🧹 Cleaning Go Project");
    println!("======================");

    let _ = Command::new("go").args(&["clean"]).status();
    let _ = Command::new("go").args(&["mod", "tidy"]).status();

    println!("✅ Project cleaned");
}

fn go_package_management() {
    println!("📦 Go Package Management");
    println!("========================");

    let options = [
        "📋 List Dependencies",
        "🔄 Update Dependencies",
        "🧹 Tidy Modules",
        "📊 Dependency Graph",
        "🔍 Search Packages",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Package Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => list_go_dependencies(),
        1 => update_go_dependencies(),
        2 => tidy_go_modules(),
        3 => show_dependency_graph(),
        4 => search_go_packages(),
        _ => return,
    }
}

fn list_go_dependencies() {
    println!("📋 Go Dependencies");
    println!("==================");

    let _ = Command::new("go").args(&["list", "-m", "all"]).status();
}

fn update_go_dependencies() {
    println!("🔄 Updating Go Dependencies");
    println!("===========================");

    let _ = Command::new("go").args(&["get", "-u", "./..."]).status();
    println!("✅ Dependencies updated");
}

fn tidy_go_modules() {
    println!("🧹 Tidying Go Modules");
    println!("=====================");

    let _ = Command::new("go").args(&["mod", "tidy"]).status();
    println!("✅ Modules tidied");
}

fn show_dependency_graph() {
    println!("📊 Dependency Graph");
    println!("==================");

    let _ = Command::new("go").args(&["mod", "graph"]).status();
}

fn search_go_packages() {
    let query: String = Input::new()
        .with_prompt("Search query")
        .interact_text()
        .unwrap();

    println!("🔍 Search results for: {}", query);
    println!("💡 Visit https://pkg.go.dev/search?q={}", query);
}

fn go_development_tools() {
    println!("🔧 Go Development Tools");
    println!("=======================");

    let tools = [
        "📝 Install gopls (Language Server)",
        "🎨 Install gofmt & goimports",
        "🔍 Install golint & vet",
        "🧪 Install testing tools",
        "📊 Install profiling tools",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Development Tools")
        .items(&tools)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => install_gopls(),
        1 => install_go_formatters(),
        2 => install_go_linters(),
        3 => install_go_testing_tools(),
        4 => install_go_profiling_tools(),
        _ => return,
    }
}

fn install_gopls() {
    println!("📝 Installing gopls (Go Language Server)");
    println!("=========================================");

    let _ = Command::new("go")
        .args(&["install", "golang.org/x/tools/gopls@latest"])
        .status();

    println!("✅ gopls installed");
}

fn install_go_formatters() {
    println!("🎨 Installing Go formatters");
    println!("===========================");

    let _ = Command::new("go")
        .args(&["install", "golang.org/x/tools/cmd/goimports@latest"])
        .status();

    println!("✅ goimports installed (gofmt is included with Go)");
}

fn install_go_linters() {
    println!("🔍 Installing Go linters");
    println!("========================");

    let _ = Command::new("go")
        .args(&["install", "golang.org/x/lint/golint@latest"])
        .status();

    println!("✅ golint installed (go vet is included with Go)");
}

fn install_go_testing_tools() {
    println!("🧪 Installing Go testing tools");
    println!("==============================");

    let tools = [
        "github.com/stretchr/testify@latest",
        "github.com/onsi/ginkgo/v2/ginkgo@latest",
        "github.com/onsi/gomega@latest",
    ];

    for tool in &tools {
        let _ = Command::new("go").args(&["install", tool]).status();
    }

    println!("✅ Testing tools installed");
}

fn install_go_profiling_tools() {
    println!("📊 Installing Go profiling tools");
    println!("================================");

    let _ = Command::new("go")
        .args(&["install", "github.com/google/pprof@latest"])
        .status();

    println!("✅ pprof installed");
}

fn go_testing_benchmarking() {
    println!("🧪 Go Testing & Benchmarking");
    println!("============================");

    let options = [
        "🧪 Run Tests",
        "📊 Run Benchmarks",
        "📋 Test Coverage",
        "🎯 Race Detection",
        "📝 Generate Test Files",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Testing & Benchmarking")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => run_go_tests(),
        1 => run_go_benchmarks(),
        2 => go_test_coverage(),
        3 => go_race_detection(),
        4 => generate_test_files(),
        _ => return,
    }
}

fn run_go_tests() {
    println!("🧪 Running Go Tests");
    println!("===================");

    let _ = Command::new("go").args(&["test", "./..."]).status();
}

fn run_go_benchmarks() {
    println!("📊 Running Go Benchmarks");
    println!("========================");

    let _ = Command::new("go")
        .args(&["test", "-bench=.", "./..."])
        .status();
}

fn go_test_coverage() {
    println!("📋 Go Test Coverage");
    println!("===================");

    let _ = Command::new("go")
        .args(&["test", "-cover", "./..."])
        .status();
}

fn go_race_detection() {
    println!("🎯 Go Race Detection");
    println!("===================");

    let _ = Command::new("go")
        .args(&["test", "-race", "./..."])
        .status();
}

fn generate_test_files() {
    println!("📝 Generating Test Files");
    println!("========================");

    let file: String = Input::new()
        .with_prompt("Go file to generate tests for")
        .interact_text()
        .unwrap();

    // This would require a tool like gotests
    println!("💡 Install gotests: go install github.com/cweill/gotests/gotests@latest");
    println!("💡 Then run: gotests -w {}", file);
}

fn go_learning_resources() {
    println!("📚 Go Learning Resources");
    println!("========================");

    println!("🌐 Official Documentation: https://golang.org/doc/");
    println!("📖 Go Tour: https://tour.golang.org/");
    println!("💡 Go by Example: https://gobyexample.com/");
    println!("📚 Effective Go: https://golang.org/doc/effective_go");
    println!("👥 Community: https://golang.org/help/");
    println!("📺 GopherCon: https://www.gophercon.com/");
}

fn show_go_version() {
    if let Ok(output) = Command::new("go").arg("version").output() {
        let version = String::from_utf8_lossy(&output.stdout);
        println!("📋 Go version: {}", version.trim());
    }
}
