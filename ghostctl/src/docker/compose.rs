use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::fs;
use std::process::Command;

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
        _ => (),
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
        if path.exists() {
            if let Ok(entries) = fs::read_dir(path) {
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
        _ => (),
    }
}

fn show_stack_status(stack_path: &std::path::Path) {
    println!("📊 Stack Status: {}", stack_path.display());

    let status = Command::new("docker-compose")
        .args(["ps"])
        .current_dir(stack_path)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Status retrieved successfully"),
        _ => println!("❌ Failed to get stack status"),
    }
}

fn start_stack(stack_path: &std::path::Path) {
    println!("🚀 Starting stack: {}", stack_path.display());

    let status = Command::new("docker-compose")
        .args(["up", "-d"])
        .current_dir(stack_path)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Stack started successfully"),
        _ => println!("❌ Failed to start stack"),
    }
}

fn stop_stack(stack_path: &std::path::Path) {
    println!("🛑 Stopping stack: {}", stack_path.display());

    let confirm = Confirm::new()
        .with_prompt("Are you sure you want to stop this stack?")
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let status = Command::new("docker-compose")
            .args(["down"])
            .current_dir(stack_path)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Stack stopped successfully"),
            _ => println!("❌ Failed to stop stack"),
        }
    }
}

fn restart_stack(stack_path: &std::path::Path) {
    println!("🔄 Restarting stack: {}", stack_path.display());

    let status = Command::new("docker-compose")
        .args(["restart"])
        .current_dir(stack_path)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Stack restarted successfully"),
        _ => println!("❌ Failed to restart stack"),
    }
}

fn view_stack_logs(stack_path: &std::path::Path) {
    println!("📝 Viewing logs for: {}", stack_path.display());

    let log_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Log options")
        .items(&[
            "📜 All logs (last 50 lines)",
            "🔄 Follow logs (real-time)",
            "📦 Specific service logs",
        ])
        .default(0)
        .interact()
        .unwrap();

    match log_type {
        0 => {
            let _ = Command::new("docker-compose")
                .args(["logs", "--tail=50"])
                .current_dir(stack_path)
                .status();
        }
        1 => {
            println!("Press Ctrl+C to stop following logs");
            let _ = Command::new("docker-compose")
                .args(["logs", "-f"])
                .current_dir(stack_path)
                .status();
        }
        2 => {
            let service: String = Input::new()
                .with_prompt("Enter service name")
                .interact_text()
                .unwrap();

            let _ = Command::new("docker-compose")
                .args(["logs", "--tail=50", &service])
                .current_dir(stack_path)
                .status();
        }
        _ => (),
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
    println!("📥 Pulling images for: {}", stack_path.display());

    let status = Command::new("docker-compose")
        .args(["pull"])
        .current_dir(stack_path)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Images pulled successfully"),
        _ => println!("❌ Failed to pull images"),
    }
}

fn remove_stack(stack_path: &std::path::Path) {
    println!("🗑️  Remove stack: {}", stack_path.display());

    let confirm = Confirm::new()
        .with_prompt(
            "This will stop and remove all containers, networks, and volumes. Are you sure?",
        )
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        let status = Command::new("docker-compose")
            .args(["down", "-v", "--remove-orphans"])
            .current_dir(stack_path)
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Stack removed successfully"),
            _ => println!("❌ Failed to remove stack"),
        }
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
        _ => (),
    }
}

fn deploy_from_directory() {
    let directory: String = Input::new()
        .with_prompt("Enter path to directory containing docker-compose.yml")
        .interact_text()
        .unwrap();

    let path = std::path::Path::new(&directory);

    if !path.exists() {
        println!("❌ Directory does not exist: {}", directory);
        return;
    }

    let compose_file = if path.join("docker-compose.yml").exists() {
        "docker-compose.yml"
    } else if path.join("docker-compose.yaml").exists() {
        "docker-compose.yaml"
    } else {
        println!("❌ No docker-compose.yml found in directory");
        return;
    };

    println!("🚀 Deploying from: {}/{}", directory, compose_file);

    let status = Command::new("docker-compose")
        .args(["up", "-d"])
        .current_dir(path)
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Stack deployed successfully"),
        _ => println!("❌ Failed to deploy stack"),
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
    println!("📊 All Stacks Status Overview");
    println!("=============================");

    let stacks = find_compose_stacks();

    for stack in stacks {
        println!("\n📁 Stack: {}", stack.display());

        let output = Command::new("docker-compose")
            .args(["ps", "--format", "table"])
            .current_dir(&stack)
            .output();

        match output {
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
    let _output = Command::new("docker").args([
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
    println!("🔄 Update All Stacks");
    println!("====================");

    let stacks = find_compose_stacks();

    if stacks.is_empty() {
        println!("No stacks found to update");
        return;
    }

    let confirm = Confirm::new()
        .with_prompt(format!(
            "Update {} stacks (pull latest images)?",
            stacks.len()
        ))
        .default(false)
        .interact()
        .unwrap();

    if confirm {
        for stack in stacks {
            println!("\n🔄 Updating: {}", stack.display());

            let status = Command::new("docker-compose")
                .args(["pull"])
                .current_dir(&stack)
                .status();

            match status {
                Ok(s) if s.success() => println!("  ✅ Updated"),
                _ => println!("  ❌ Update failed"),
            }
        }

        println!("\n✅ All stacks updated!");
        println!("💡 Don't forget to restart stacks to use new images");
    }
}
