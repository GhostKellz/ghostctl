use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command;

pub fn container_management() {
    println!("📦 Docker Container Management");
    println!("==============================");

    let options = [
        "📋 List containers",
        "🚀 Run container",
        "🛑 Stop container",
        "🔄 Restart container",
        "🗑️  Remove container",
        "📊 Container stats",
        "📜 Container logs",
        "🔍 Inspect container",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Container Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => list_containers(),
        1 => run_container(),
        2 => {
            println!("Enter container ID to stop:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let id = input.trim().to_string();
            if !id.is_empty() {
                stop_container(id);
            }
        }
        3 => restart_container(),
        4 => remove_container(),
        5 => container_stats(),
        6 => container_logs(),
        7 => inspect_container(),
        _ => return,
    }
}

pub fn list_containers() {
    println!("📋 Docker Containers");
    let _ = Command::new("docker").args(&["ps", "-a"]).status();
}

fn run_container() {
    use dialoguer::Input;

    let image: String = Input::new()
        .with_prompt("Image name")
        .interact_text()
        .unwrap();

    let name: String = Input::new()
        .with_prompt("Container name (optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let mut args = vec!["run", "-d"];

    if !name.is_empty() {
        args.extend_from_slice(&["--name", &name]);
    }

    args.push(&image);

    println!("🚀 Running container from image: {}", image);
    let status = Command::new("docker").args(&args).status();

    match status {
        Ok(s) if s.success() => println!("✅ Container started successfully"),
        _ => println!("❌ Failed to start container"),
    }
}

pub fn stop_container(id: String) {
    println!("🛑 Stopping Docker Container: {}", id);
    let _ = std::process::Command::new("docker")
        .args(&["stop", &id])
        .status();
}

fn restart_container() {
    use dialoguer::Input;

    let container: String = Input::new()
        .with_prompt("Container name or ID")
        .interact_text()
        .unwrap();

    println!("🔄 Restarting container: {}", container);
    let status = Command::new("docker")
        .args(&["restart", &container])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Container restarted successfully"),
        _ => println!("❌ Failed to restart container"),
    }
}

fn remove_container() {
    use dialoguer::{Confirm, Input};

    let container: String = Input::new()
        .with_prompt("Container name or ID")
        .interact_text()
        .unwrap();

    let force = Confirm::new()
        .with_prompt("Force remove (stop if running)?")
        .default(false)
        .interact()
        .unwrap();

    let mut args = vec!["rm"];
    if force {
        args.push("-f");
    }
    args.push(&container);

    println!("🗑️  Removing container: {}", container);
    let status = Command::new("docker").args(&args).status();

    match status {
        Ok(s) if s.success() => println!("✅ Container removed successfully"),
        _ => println!("❌ Failed to remove container"),
    }
}

fn container_stats() {
    println!("📊 Container Stats");
    let _ = Command::new("docker")
        .args(&["stats", "--no-stream"])
        .status();
}

fn container_logs() {
    use dialoguer::{Confirm, Input};

    let container: String = Input::new()
        .with_prompt("Container name or ID")
        .interact_text()
        .unwrap();

    let follow = Confirm::new()
        .with_prompt("Follow logs (real-time)?")
        .default(false)
        .interact()
        .unwrap();

    let mut args = vec!["logs"];
    if follow {
        args.push("-f");
    }
    args.extend_from_slice(&["--tail", "100"]);
    args.push(&container);

    println!("📜 Container logs for: {}", container);
    let _ = Command::new("docker").args(&args).status();
}

fn inspect_container() {
    use dialoguer::Input;

    let container: String = Input::new()
        .with_prompt("Container name or ID")
        .interact_text()
        .unwrap();

    println!("🔍 Inspecting container: {}", container);
    let _ = Command::new("docker")
        .args(&["inspect", &container])
        .status();
}
