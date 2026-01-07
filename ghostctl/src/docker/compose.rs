use crate::tui;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::fs;
use std::process::Command;

/// Compose command type - either 'docker compose' or 'docker-compose'
#[derive(Clone, Copy)]
enum ComposeCommand {
    /// Docker CLI plugin: `docker compose`
    DockerCompose,
    /// Standalone: `docker-compose`
    Standalone,
}

/// Detect which compose command is available
/// Prefers `docker compose` (CLI plugin) over `docker-compose` (standalone)
fn detect_compose_command() -> Option<ComposeCommand> {
    // First try docker compose (new CLI plugin)
    if Command::new("docker")
        .args(&["compose", "version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return Some(ComposeCommand::DockerCompose);
    }

    // Fall back to docker-compose (standalone)
    if Command::new("docker-compose")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return Some(ComposeCommand::Standalone);
    }

    None
}

/// Run a compose command with automatic detection
fn run_compose(
    args: &[&str],
    work_dir: Option<&std::path::Path>,
) -> std::io::Result<std::process::ExitStatus> {
    match detect_compose_command() {
        Some(ComposeCommand::DockerCompose) => {
            let mut docker_args = vec!["compose"];
            docker_args.extend_from_slice(args);
            let mut cmd = Command::new("docker");
            cmd.args(&docker_args);
            if let Some(dir) = work_dir {
                cmd.current_dir(dir);
            }
            return cmd.status();
        }
        Some(ComposeCommand::Standalone) => {
            let mut cmd = Command::new("docker-compose");
            cmd.args(args);
            if let Some(dir) = work_dir {
                cmd.current_dir(dir);
            }
            return cmd.status();
        }
        None => {
            tui::error("Neither 'docker compose' nor 'docker-compose' found");
            tui::info("Install Docker Compose:");
            tui::info("  Arch: sudo pacman -S docker-compose");
            tui::info("  Or use Docker Desktop which includes the compose plugin");
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Docker Compose not found",
            ));
        }
    };
}

/// Run a compose command and capture output
fn run_compose_output(
    args: &[&str],
    work_dir: Option<&std::path::Path>,
) -> std::io::Result<std::process::Output> {
    match detect_compose_command() {
        Some(ComposeCommand::DockerCompose) => {
            let mut docker_args = vec!["compose"];
            docker_args.extend_from_slice(args);
            let mut cmd = Command::new("docker");
            cmd.args(&docker_args);
            if let Some(dir) = work_dir {
                cmd.current_dir(dir);
            }
            cmd.output()
        }
        Some(ComposeCommand::Standalone) => {
            let mut cmd = Command::new("docker-compose");
            cmd.args(args);
            if let Some(dir) = work_dir {
                cmd.current_dir(dir);
            }
            cmd.output()
        }
        None => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Docker Compose not found",
        )),
    }
}

/// Get a human-readable name for the compose command in use
fn compose_command_name() -> &'static str {
    match detect_compose_command() {
        Some(ComposeCommand::DockerCompose) => "docker compose",
        Some(ComposeCommand::Standalone) => "docker-compose",
        None => "docker compose",
    }
}

pub fn compose_stack_manager() {
    println!("🐳 Docker Compose Stack Manager");
    println!("===============================");

    let options = [
        "📁 Browse Local Stacks",
        "🚀 Deploy New Stack",
        "📊 Stack Status Overview",
        "🔧 Manage Running Stacks",
        "📦 Stack Templates Library",
        "🔄 Update All Stacks",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Compose Stack Manager")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => browse_local_stacks(),
        1 => deploy_new_stack(),
        2 => stack_status_overview(),
        3 => manage_running_stacks(),
        4 => stack_templates_library(),
        5 => update_all_stacks(),
        _ => return,
    }
}

fn browse_local_stacks() {
    println!("📁 Local Docker Compose Stacks");
    println!("==============================");

    let stack_dirs = find_compose_stacks();

    if stack_dirs.is_empty() {
        println!("No Docker Compose stacks found.");
        println!("Searched in common directories: ./docker, /opt/docker, ~/docker");
        return;
    }

    let mut menu_items: Vec<String> = stack_dirs
        .iter()
        .map(|dir| format!("📁 {}", dir.display()))
        .collect();
    menu_items.push("⬅️  Back".to_string());

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select stack to manage")
        .items(&menu_items)
        .default(0)
        .interact()
        .unwrap();

    if choice < stack_dirs.len() {
        manage_stack(&stack_dirs[choice]);
    }
}

fn find_compose_stacks() -> Vec<std::path::PathBuf> {
    let mut stacks = Vec::new();

    let search_dirs = ["./docker", "/opt/docker", "~/docker", "./"];

    for dir in &search_dirs {
        let path = std::path::Path::new(dir);
        if path.exists()
            && let Ok(entries) = fs::read_dir(path)
        {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    // Check if directory contains docker-compose.yml
                    let compose_file = entry_path.join("docker-compose.yml");
                    let compose_file_alt = entry_path.join("docker-compose.yaml");

                    if compose_file.exists() || compose_file_alt.exists() {
                        stacks.push(entry_path);
                    }
                }
            }
        }
    }

    stacks
}

fn manage_stack(stack_path: &std::path::Path) {
    println!("🐳 Managing Stack: {}", stack_path.display());

    let options = [
        "📊 Show Status",
        "🚀 Start Stack",
        "🛑 Stop Stack",
        "🔄 Restart Stack",
        "📝 View Logs",
        "📋 Show Configuration",
        "📥 Pull Images",
        "🗑️  Remove Stack",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Stack Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => show_stack_status(stack_path),
        1 => start_stack(stack_path),
        2 => stop_stack(stack_path),
        3 => restart_stack(stack_path),
        4 => view_stack_logs(stack_path),
        5 => show_stack_config(stack_path),
        6 => pull_stack_images(stack_path),
        7 => remove_stack(stack_path),
        _ => return,
    }
}

fn show_stack_status(stack_path: &std::path::Path) {
    tui::status("📊", &format!("Stack Status: {}", stack_path.display()));

    match run_compose(&["ps"], Some(stack_path)) {
        Ok(s) if s.success() => tui::success("Status retrieved successfully"),
        _ => tui::error("Failed to get stack status"),
    }
}

fn start_stack(stack_path: &std::path::Path) {
    tui::status("🚀", &format!("Starting stack: {}", stack_path.display()));

    match run_compose(&["up", "-d"], Some(stack_path)) {
        Ok(s) if s.success() => tui::success("Stack started successfully"),
        _ => tui::error("Failed to start stack"),
    }
}

fn stop_stack(stack_path: &std::path::Path) {
    tui::status("🛑", &format!("Stopping stack: {}", stack_path.display()));

    if !tui::confirm("Are you sure you want to stop this stack?", false) {
        return;
    }

    match run_compose(&["down"], Some(stack_path)) {
        Ok(s) if s.success() => tui::success("Stack stopped successfully"),
        _ => tui::error("Failed to stop stack"),
    }
}

fn restart_stack(stack_path: &std::path::Path) {
    tui::status("🔄", &format!("Restarting stack: {}", stack_path.display()));

    match run_compose(&["restart"], Some(stack_path)) {
        Ok(s) if s.success() => tui::success("Stack restarted successfully"),
        _ => tui::error("Failed to restart stack"),
    }
}

fn view_stack_logs(stack_path: &std::path::Path) {
    tui::status("📝", &format!("Viewing logs for: {}", stack_path.display()));

    let log_options = [
        "📜 All logs (last 50 lines)",
        "🔄 Follow logs (real-time)",
        "📦 Specific service logs",
    ];

    let log_type = match tui::select_with_back("Log options", &log_options, 0) {
        Some(t) => t,
        None => return,
    };

    match log_type {
        0 => {
            let _ = run_compose(&["logs", "--tail=50"], Some(stack_path));
        }
        1 => {
            tui::info("Press Ctrl+C to stop following logs");
            let _ = run_compose(&["logs", "-f"], Some(stack_path));
        }
        2 => {
            let service = match tui::input("Enter service name", None) {
                Some(s) if !s.is_empty() => s,
                _ => return,
            };

            let _ = run_compose(&["logs", "--tail=50", &service], Some(stack_path));
        }
        _ => {}
    }
}

fn show_stack_config(stack_path: &std::path::Path) {
    println!("📋 Stack Configuration: {}", stack_path.display());

    let compose_file = if stack_path.join("docker-compose.yml").exists() {
        stack_path.join("docker-compose.yml")
    } else {
        stack_path.join("docker-compose.yaml")
    };

    if let Ok(content) = fs::read_to_string(&compose_file) {
        println!("📄 {}", compose_file.display());
        println!("{}", content);
    } else {
        println!("❌ Could not read compose file");
    }
}

fn pull_stack_images(stack_path: &std::path::Path) {
    tui::status(
        "📥",
        &format!("Pulling images for: {}", stack_path.display()),
    );

    match run_compose(&["pull"], Some(stack_path)) {
        Ok(s) if s.success() => tui::success("Images pulled successfully"),
        _ => tui::error("Failed to pull images"),
    }
}

fn remove_stack(stack_path: &std::path::Path) {
    tui::status("🗑️", &format!("Remove stack: {}", stack_path.display()));

    if !tui::confirm_dangerous(
        "This will stop and remove all containers, networks, and volumes. Are you sure?",
    ) {
        return;
    }

    match run_compose(&["down", "-v", "--remove-orphans"], Some(stack_path)) {
        Ok(s) if s.success() => tui::success("Stack removed successfully"),
        _ => tui::error("Failed to remove stack"),
    }
}

fn deploy_new_stack() {
    println!("🚀 Deploy New Stack");
    println!("===================");

    let deployment_options = [
        "📁 From local directory",
        "🌐 From URL (docker-compose.yml)",
        "📋 From template",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Deployment source")
        .items(&deployment_options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => deploy_from_directory(),
        1 => deploy_from_url(),
        2 => deploy_from_template(),
        _ => return,
    }
}

fn deploy_from_directory() {
    let directory = match tui::input(
        "Enter path to directory containing docker-compose.yml",
        None,
    ) {
        Some(d) if !d.is_empty() => d,
        _ => return,
    };

    let path = std::path::Path::new(&directory);

    if !path.exists() {
        tui::error(&format!("Directory does not exist: {}", directory));
        return;
    }

    let compose_file = if path.join("docker-compose.yml").exists() {
        "docker-compose.yml"
    } else if path.join("docker-compose.yaml").exists() {
        "docker-compose.yaml"
    } else {
        tui::error("No docker-compose.yml found in directory");
        return;
    };

    tui::status(
        "🚀",
        &format!("Deploying from: {}/{}", directory, compose_file),
    );

    match run_compose(&["up", "-d"], Some(path)) {
        Ok(s) if s.success() => tui::success("Stack deployed successfully"),
        _ => tui::error("Failed to deploy stack"),
    }
}

fn deploy_from_url() {
    let url: String = Input::new()
        .with_prompt("Enter URL to docker-compose.yml")
        .interact_text()
        .unwrap();

    println!("📥 Downloading compose file from: {}", url);

    // This would need reqwest or curl implementation
    println!("⚠️  URL deployment not yet implemented");
    println!("Use: curl -o docker-compose.yml {}", url);
}

fn deploy_from_template() {
    println!("📋 Template deployment not yet implemented");
    println!("This will integrate with the stack templates library");
}

fn stack_status_overview() {
    tui::header("All Stacks Status Overview");

    let stacks = find_compose_stacks();

    for stack in stacks {
        println!("\n📁 Stack: {}", stack.display());

        match run_compose_output(&["ps", "--format", "table"], Some(&stack)) {
            Ok(out) if out.status.success() => {
                let output_str = String::from_utf8_lossy(&out.stdout);
                if output_str.trim().is_empty() {
                    println!("  ⭕ No containers running");
                } else {
                    println!("  ✅ Running");
                }
            }
            _ => println!("  ❌ Error checking status"),
        }
    }
}

fn manage_running_stacks() {
    println!("🔧 Manage Running Stacks");
    println!("========================");

    // Get all running compose projects
    let _output = Command::new("docker").args(&[
        "ps",
        "--filter",
        "label=com.docker.compose.project",
        "--format",
        "{{.Label \"com.docker.compose.project\"}}",
    ]);

    println!("This feature needs enhancement to properly list compose projects");
}

fn stack_templates_library() {
    println!("📦 Stack Templates Library");
    println!("==========================");

    let templates = [
        "🌐 Nginx + PHP-FPM",
        "📊 Prometheus + Grafana",
        "💾 PostgreSQL + pgAdmin",
        "🔍 Elasticsearch + Kibana",
        "📝 WordPress + MySQL",
        "🚀 Node.js + MongoDB",
        "📡 Redis + Redis Commander",
        "🐰 RabbitMQ + Management",
    ];

    let mut menu_items: Vec<String> = templates.iter().map(|t| t.to_string()).collect();
    menu_items.push("⬅️  Back".to_string());

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select template")
        .items(&menu_items)
        .default(0)
        .interact()
        .unwrap();

    if choice < templates.len() {
        println!("📋 Template: {}", templates[choice]);
        println!("⚠️  Template deployment not yet implemented");
    }
}

fn update_all_stacks() {
    tui::header("Update All Stacks");

    let stacks = find_compose_stacks();

    if stacks.is_empty() {
        tui::info("No stacks found to update");
        return;
    }

    if !tui::confirm(
        &format!("Update {} stacks (pull latest images)?", stacks.len()),
        false,
    ) {
        return;
    }

    for stack in stacks {
        println!("\n🔄 Updating: {}", stack.display());

        match run_compose(&["pull"], Some(&stack)) {
            Ok(s) if s.success() => println!("  ✅ Updated"),
            _ => println!("  ❌ Update failed"),
        }
    }

    tui::success("All stacks updated!");
    tui::info("Don't forget to restart stacks to use new images");
}
