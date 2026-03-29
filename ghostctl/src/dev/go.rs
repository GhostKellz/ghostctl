use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

/// Validate Go module/package name
fn validate_go_module_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("Module name cannot be empty");
    }
    if name.len() > 255 {
        return Err("Module name too long");
    }
    // Go module names: alphanumeric, dots, slashes, hyphens, underscores
    // Must start with a domain-like path (github.com/user/repo)
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '.' || c == '/' || c == '-' || c == '_')
    {
        return Err("Module name contains invalid characters");
    }
    if name.contains("..") || name.starts_with('/') || name.ends_with('/') {
        return Err("Invalid module name format");
    }
    Ok(())
}

/// Validate Go package import path
fn validate_go_package(pkg: &str) -> Result<(), &'static str> {
    if pkg.is_empty() {
        return Err("Package name cannot be empty");
    }
    if pkg.len() > 500 {
        return Err("Package path too long");
    }
    // Go packages: similar to module names, may include @version
    if !pkg
        .chars()
        .all(|c| c.is_alphanumeric() || c == '.' || c == '/' || c == '-' || c == '_' || c == '@')
    {
        return Err("Package contains invalid characters");
    }
    Ok(())
}

/// Validate simple project/file name
fn validate_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("Name cannot be empty");
    }
    if name.len() > 100 {
        return Err("Name too long");
    }
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        return Err("Name contains invalid characters");
    }
    if name.starts_with('.') || name.contains("..") {
        return Err("Invalid name format");
    }
    Ok(())
}

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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Go Development")
        .items(&options)
        .default(0)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

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

    let method = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Installation method")
        .items(&install_methods)
        .default(0)
        .interact()
    {
        Ok(m) => m,
        Err(_) => return,
    };

    match method {
        0 => install_go_package_manager(),
        1 => install_go_official(),
        2 => install_go_from_source(),
        _ => return,
    }
}

fn install_go_package_manager() {
    // Try different package managers with reaper priority
    let install_result = if Command::new("which")
        .arg("reap")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Installing Go with reaper...");
        Command::new("reap").arg("go").status()
    } else if Command::new("which")
        .arg("pacman")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Installing Go with pacman...");
        Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "go"])
            .status()
    } else if Command::new("which")
        .arg("apt")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Installing Go with apt...");
        if let Err(e) = Command::new("sudo").args(["apt", "update"]).status() {
            eprintln!("Warning: apt update failed: {}", e);
        }
        Command::new("sudo")
            .args(["apt", "install", "-y", "golang-go"])
            .status()
    } else if Command::new("which")
        .arg("dnf")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Installing Go with dnf...");
        Command::new("sudo")
            .args(["dnf", "install", "-y", "go"])
            .status()
    } else {
        eprintln!("No supported package manager found");
        return;
    };

    match install_result {
        Ok(status) if status.success() => {
            println!("Go installation command completed");
        }
        Ok(status) => {
            eprintln!("Package manager exited with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run package manager: {}", e);
        }
    }

    if Command::new("which")
        .arg("go")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        println!("Go installed successfully");
        setup_go_environment();
        show_go_version();
    } else {
        eprintln!("Package manager installation failed. Try official download.");
    }
}

fn install_go_official() {
    println!("Installing Go from Official Downloads");
    println!("========================================");

    println!("Visit https://golang.org/dl/ for the latest version");

    let confirm = match Confirm::new()
        .with_prompt("Download and install latest Go?")
        .default(true)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    if confirm {
        // Remove existing Go installation
        match Command::new("sudo")
            .args(["rm", "-rf", "/usr/local/go"])
            .status()
        {
            Ok(status) if !status.success() => {
                eprintln!("Warning: Failed to remove existing Go installation");
            }
            Err(e) => {
                eprintln!("Warning: Could not remove existing Go: {}", e);
            }
            _ => {}
        }

        // Download latest Go - using hardcoded trusted URL
        println!("Downloading Go...");
        let download_url = "https://golang.org/dl/go1.21.5.linux-amd64.tar.gz";

        match Command::new("curl")
            .args([
                "-L",
                "-f",
                "--proto",
                "=https",
                download_url,
                "-o",
                "/tmp/go.tar.gz",
            ])
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
                eprintln!("Failed to download Go: {}", e);
                return;
            }
        }

        // Extract
        match Command::new("sudo")
            .args(["tar", "-C", "/usr/local", "-xzf", "/tmp/go.tar.gz"])
            .status()
        {
            Ok(status) if status.success() => {
                println!("Extraction completed");
            }
            Ok(_) => {
                eprintln!("Failed to extract Go tarball");
                return;
            }
            Err(e) => {
                eprintln!("Failed to run tar: {}", e);
                return;
            }
        }

        // Setup environment
        setup_go_environment();

        // Cleanup
        if let Err(e) = std::fs::remove_file("/tmp/go.tar.gz") {
            eprintln!("Warning: Failed to cleanup download: {}", e);
        }

        println!("Go installed to /usr/local/go");
    }
}

fn install_go_from_source() {
    println!("Building Go from Source");
    println!("===========================");

    println!("Warning: Building Go from source requires an existing Go installation (Go 1.17+)");

    let confirm = match Confirm::new()
        .with_prompt("Continue with source build?")
        .default(false)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    if confirm {
        // Clean up any previous attempt
        if std::path::Path::new("/tmp/go-source").exists() {
            if let Err(e) = std::fs::remove_dir_all("/tmp/go-source") {
                eprintln!("Warning: Failed to clean previous build directory: {}", e);
            }
        }

        println!("Cloning Go repository...");
        match Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "https://go.googlesource.com/go",
                "/tmp/go-source",
            ])
            .status()
        {
            Ok(status) if status.success() => {
                println!("Clone completed");
            }
            Ok(_) => {
                eprintln!("Failed to clone Go repository");
                return;
            }
            Err(e) => {
                eprintln!("Failed to run git: {}", e);
                return;
            }
        }

        println!("Building... (this will take a while)");
        // Use bash to run the build script in the correct directory
        match Command::new("bash")
            .arg("all.bash")
            .current_dir("/tmp/go-source/src")
            .status()
        {
            Ok(status) if status.success() => {
                println!("Build completed successfully");
            }
            Ok(status) => {
                eprintln!("Build exited with code: {:?}", status.code());
            }
            Err(e) => {
                eprintln!("Failed to run build: {}", e);
            }
        }

        println!("For detailed build instructions, see: https://golang.org/doc/install/source");
    }
}

fn setup_go_environment() {
    println!("⚙️  Setting up Go environment...");

    let home = match dirs::home_dir() {
        Some(h) => h,
        None => {
            eprintln!("Could not determine home directory");
            return;
        }
    };
    let shell_files = [
        format!("{}/.bashrc", home.display()),
        format!("{}/.zshrc", home.display()),
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
            && !content.contains("GOPATH")
        {
            if let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(shell_file) {
                use std::io::Write;
                let _ = writeln!(file, "\n# Go environment");
                for env_var in &go_env {
                    let _ = writeln!(file, "{}", env_var);
                }

                println!("✅ Added Go environment to {}", shell_file);
            }
        }
    }

    // Create GOPATH directory
    let gopath = format!("{}/go", home.display());
    let _ = std::fs::create_dir_all(format!("{}/src", gopath));
    let _ = std::fs::create_dir_all(format!("{}/bin", gopath));
    let _ = std::fs::create_dir_all(format!("{}/pkg", gopath));
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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project Management")
        .items(&options)
        .default(0)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

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
    let module_name: String = match Input::new()
        .with_prompt("Module name (e.g., github.com/user/project)")
        .interact_text()
    {
        Ok(m) => m,
        Err(_) => return,
    };

    // Validate module name
    if let Err(e) = validate_go_module_name(&module_name) {
        eprintln!("Invalid module name: {}", e);
        return;
    }

    let project_name = module_name.split('/').next_back().unwrap_or("go-project");

    // Validate extracted project name
    if let Err(e) = validate_name(project_name) {
        eprintln!("Invalid project name: {}", e);
        return;
    }

    println!("Creating project directory...");
    if let Err(e) = std::fs::create_dir_all(project_name) {
        eprintln!("Failed to create directory: {}", e);
        return;
    }

    match Command::new("go")
        .args(["mod", "init", &module_name])
        .current_dir(project_name)
        .status()
    {
        Ok(status) if status.success() => {
            println!("Go module initialized");
        }
        Ok(status) => {
            eprintln!("go mod init failed with code: {:?}", status.code());
            return;
        }
        Err(e) => {
            eprintln!("Failed to run go mod init: {}", e);
            return;
        }
    }

    // Create main.go with safe project name (already validated)
    let main_go = format!(
        r#"package main

import "fmt"

func main() {{
    fmt.Println("Hello, {}!")
}}
"#,
        project_name
    );

    if let Err(e) = std::fs::write(format!("{}/main.go", project_name), main_go) {
        eprintln!("Failed to write main.go: {}", e);
        return;
    }

    println!("Go module '{}' created", project_name);
}

fn init_go_module() {
    let module_name: String = match Input::new().with_prompt("Module name").interact_text() {
        Ok(m) => m,
        Err(_) => return,
    };

    // Validate module name
    if let Err(e) = validate_go_module_name(&module_name) {
        eprintln!("Invalid module name: {}", e);
        return;
    }

    match Command::new("go")
        .args(["mod", "init", &module_name])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Go module initialized");
        }
        Ok(status) => {
            eprintln!("go mod init failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run go mod init: {}", e);
        }
    }
}

fn add_go_dependencies() {
    println!("Adding Go Dependencies");
    println!("=========================");

    if !std::path::Path::new("go.mod").exists() {
        eprintln!("No go.mod found. Initialize a Go module first.");
        return;
    }

    let package: String = match Input::new()
        .with_prompt("Package to add (e.g., github.com/gin-gonic/gin)")
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    // Validate package path
    if let Err(e) = validate_go_package(&package) {
        eprintln!("Invalid package path: {}", e);
        return;
    }

    match Command::new("go").args(["get", &package]).status() {
        Ok(status) if status.success() => {
            println!("Added dependency: {}", package);
        }
        Ok(status) => {
            eprintln!("go get failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run go get: {}", e);
        }
    }
}

fn build_go_project() {
    println!("Building Go Project");
    println!("=======================");

    match Command::new("go").args(["build"]).status() {
        Ok(status) if status.success() => {
            println!("Build completed");
        }
        Ok(status) => {
            eprintln!("Build failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run go build: {}", e);
        }
    }
}

fn run_go_project() {
    println!("Running Go Project");
    println!("=====================");

    match Command::new("go").args(["run", "."]).status() {
        Ok(status) if !status.success() => {
            eprintln!("Program exited with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run go run: {}", e);
        }
        _ => {}
    }
}

fn clean_go_project() {
    println!("Cleaning Go Project");
    println!("======================");

    if let Err(e) = Command::new("go").args(["clean"]).status() {
        eprintln!("Warning: go clean failed: {}", e);
    }

    match Command::new("go").args(["mod", "tidy"]).status() {
        Ok(status) if status.success() => {
            println!("Project cleaned");
        }
        Ok(status) => {
            eprintln!("go mod tidy failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run go mod tidy: {}", e);
        }
    }
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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Package Management")
        .items(&options)
        .default(0)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

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
    println!("Go Dependencies");
    println!("==================");

    if let Err(e) = Command::new("go").args(["list", "-m", "all"]).status() {
        eprintln!("Failed to list dependencies: {}", e);
    }
}

fn update_go_dependencies() {
    println!("Updating Go Dependencies");
    println!("===========================");

    match Command::new("go").args(["get", "-u", "./..."]).status() {
        Ok(status) if status.success() => {
            println!("Dependencies updated");
        }
        Ok(status) => {
            eprintln!("Update failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to update dependencies: {}", e);
        }
    }
}

fn tidy_go_modules() {
    println!("Tidying Go Modules");
    println!("=====================");

    match Command::new("go").args(["mod", "tidy"]).status() {
        Ok(status) if status.success() => {
            println!("Modules tidied");
        }
        Ok(status) => {
            eprintln!("go mod tidy failed with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to tidy modules: {}", e);
        }
    }
}

fn show_dependency_graph() {
    println!("Dependency Graph");
    println!("==================");

    if let Err(e) = Command::new("go").args(["mod", "graph"]).status() {
        eprintln!("Failed to show dependency graph: {}", e);
    }
}

fn search_go_packages() {
    let query: String = match Input::new().with_prompt("Search query").interact_text() {
        Ok(q) => q,
        Err(_) => return,
    };

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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Development Tools")
        .items(&tools)
        .default(0)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    match choice {
        0 => install_gopls(),
        1 => install_go_formatters(),
        2 => install_go_linters(),
        3 => install_go_testing_tools(),
        4 => install_go_profiling_tools(),
        _ => return,
    }
}

// Trusted Go tool install paths - these are well-known, safe package paths
const GO_TOOL_GOPLS: &str = "golang.org/x/tools/gopls@latest";
const GO_TOOL_GOIMPORTS: &str = "golang.org/x/tools/cmd/goimports@latest";
const GO_TOOL_GOLINT: &str = "golang.org/x/lint/golint@latest";
const GO_TOOL_PPROF: &str = "github.com/google/pprof@latest";

fn install_go_tool(tool_path: &str, tool_name: &str) -> bool {
    match Command::new("go").args(["install", tool_path]).status() {
        Ok(status) if status.success() => {
            println!("{} installed successfully", tool_name);
            true
        }
        Ok(status) => {
            eprintln!(
                "Failed to install {} (exit code: {:?})",
                tool_name,
                status.code()
            );
            false
        }
        Err(e) => {
            eprintln!("Failed to run go install for {}: {}", tool_name, e);
            false
        }
    }
}

fn install_gopls() {
    println!("Installing gopls (Go Language Server)");
    println!("=========================================");

    install_go_tool(GO_TOOL_GOPLS, "gopls");
}

fn install_go_formatters() {
    println!("Installing Go formatters");
    println!("===========================");

    if install_go_tool(GO_TOOL_GOIMPORTS, "goimports") {
        println!("Note: gofmt is included with Go");
    }
}

fn install_go_linters() {
    println!("Installing Go linters");
    println!("========================");

    if install_go_tool(GO_TOOL_GOLINT, "golint") {
        println!("Note: go vet is included with Go");
    }
}

fn install_go_testing_tools() {
    println!("Installing Go testing tools");
    println!("==============================");

    // Trusted testing tool packages
    let tools = [
        ("github.com/stretchr/testify@latest", "testify"),
        ("github.com/onsi/ginkgo/v2/ginkgo@latest", "ginkgo"),
        ("github.com/onsi/gomega@latest", "gomega"),
    ];

    let mut success_count = 0;
    for (path, name) in &tools {
        if install_go_tool(path, name) {
            success_count += 1;
        }
    }

    println!("Installed {}/{} testing tools", success_count, tools.len());
}

fn install_go_profiling_tools() {
    println!("Installing Go profiling tools");
    println!("================================");

    install_go_tool(GO_TOOL_PPROF, "pprof");
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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Testing & Benchmarking")
        .items(&options)
        .default(0)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return,
    };

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
    println!("Running Go Tests");
    println!("===================");

    match Command::new("go").args(["test", "./..."]).status() {
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

fn run_go_benchmarks() {
    println!("Running Go Benchmarks");
    println!("========================");

    match Command::new("go")
        .args(["test", "-bench=.", "./..."])
        .status()
    {
        Ok(status) if !status.success() => {
            eprintln!("Benchmarks exited with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run benchmarks: {}", e);
        }
        _ => {}
    }
}

fn go_test_coverage() {
    println!("Go Test Coverage");
    println!("===================");

    match Command::new("go")
        .args(["test", "-cover", "./..."])
        .status()
    {
        Ok(status) if !status.success() => {
            eprintln!("Coverage tests exited with code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run coverage: {}", e);
        }
        _ => {}
    }
}

fn go_race_detection() {
    println!("Go Race Detection");
    println!("===================");

    match Command::new("go").args(["test", "-race", "./..."]).status() {
        Ok(status) if status.success() => {
            println!("No race conditions detected");
        }
        Ok(status) => {
            eprintln!(
                "Race detection found issues or tests failed (code: {:?})",
                status.code()
            );
        }
        Err(e) => {
            eprintln!("Failed to run race detection: {}", e);
        }
    }
}

fn generate_test_files() {
    println!("Generating Test Files");
    println!("========================");

    let file: String = match Input::new()
        .with_prompt("Go file to generate tests for")
        .interact_text()
    {
        Ok(f) => f,
        Err(_) => return,
    };

    // Validate file name
    if let Err(e) = validate_name(&file) {
        eprintln!("Invalid file name: {}", e);
        return;
    }

    // Check file exists and has .go extension
    if !file.ends_with(".go") {
        eprintln!("File must have .go extension");
        return;
    }

    if !std::path::Path::new(&file).exists() {
        eprintln!("File does not exist: {}", file);
        return;
    }

    // Provide instructions - gotests requires separate installation
    println!("Install gotests: go install github.com/cweill/gotests/gotests@latest");
    println!("Then run: gotests -w {}", file);
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
