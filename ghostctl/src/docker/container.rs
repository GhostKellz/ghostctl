use dialoguer::{Select, theme::ColorfulTheme};
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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Container Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    match choice {
        0 => list_containers(),
        1 => run_container(),
        2 => {
            println!("Enter container ID to stop:");
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_err() {
                return;
            }
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
    if let Err(e) = Command::new("docker").args(["ps", "-a"]).status() {
        println!("❌ Failed to list containers: {}", e);
    }
}

fn run_container() {
    use dialoguer::Input;

    let image: String = match Input::new().with_prompt("Image name").interact_text() {
        Ok(i) => i,
        Err(_) => return,
    };

    // Validate image name
    if let Err(e) = crate::docker::validate_image_name(&image) {
        println!("❌ Invalid image name: {}", e);
        return;
    }

    let name: String = match Input::new()
        .with_prompt("Container name (optional)")
        .allow_empty(true)
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    // Validate container name if provided
    if !name.is_empty()
        && let Err(e) = crate::docker::validate_container_name(&name)
    {
        println!("❌ Invalid container name: {}", e);
        return;
    }

    let mut args = vec!["run", "-d"];

    if !name.is_empty() {
        args.extend_from_slice(&["--name", &name]);
    }

    args.push(&image);

    println!("🚀 Running container from image: {}", image);
    let status = Command::new("docker").args(&args).status();

    match status {
        Ok(s) if s.success() => println!("✅ Container started successfully"),
        Ok(s) => println!("❌ Failed to start container (exit code: {:?})", s.code()),
        Err(e) => println!("❌ Failed to start container: {}", e),
    }
}

pub fn stop_container(id: String) {
    // Validate container ID/name
    if let Err(e) = crate::docker::validate_container_name(&id) {
        println!("❌ Invalid container ID/name: {}", e);
        return;
    }

    println!("🛑 Stopping Docker Container: {}", id);
    match std::process::Command::new("docker")
        .args(["stop", &id])
        .status()
    {
        Ok(s) if s.success() => println!("✅ Container stopped"),
        Ok(_) => println!("⚠️  Container stop returned non-zero exit"),
        Err(e) => println!("❌ Failed to stop container: {}", e),
    }
}

fn restart_container() {
    use dialoguer::Input;

    let container: String = match Input::new()
        .with_prompt("Container name or ID")
        .interact_text()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    // Validate container name/ID
    if let Err(e) = crate::docker::validate_container_name(&container) {
        println!("❌ Invalid container name/ID: {}", e);
        return;
    }

    println!("🔄 Restarting container: {}", container);
    let status = Command::new("docker")
        .args(["restart", &container])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Container restarted successfully"),
        Ok(s) => println!("❌ Failed to restart container (exit code: {:?})", s.code()),
        Err(e) => println!("❌ Failed to restart container: {}", e),
    }
}

fn remove_container() {
    use dialoguer::{Confirm, Input};

    let container: String = match Input::new()
        .with_prompt("Container name or ID")
        .interact_text()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    // Validate container name/ID
    if let Err(e) = crate::docker::validate_container_name(&container) {
        println!("❌ Invalid container name/ID: {}", e);
        return;
    }

    let force = Confirm::new()
        .with_prompt("Force remove (stop if running)?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    let mut args = vec!["rm"];
    if force {
        args.push("-f");
    }
    args.push(&container);

    println!("🗑️  Removing container: {}", container);
    let status = Command::new("docker").args(&args).status();

    match status {
        Ok(s) if s.success() => println!("✅ Container removed successfully"),
        Ok(s) => println!("❌ Failed to remove container (exit code: {:?})", s.code()),
        Err(e) => println!("❌ Failed to remove container: {}", e),
    }
}

fn container_stats() {
    println!("📊 Container Stats");
    if let Err(e) = Command::new("docker")
        .args(["stats", "--no-stream"])
        .status()
    {
        println!("❌ Failed to get container stats: {}", e);
    }
}

fn container_logs() {
    use dialoguer::{Confirm, Input};

    let container: String = match Input::new()
        .with_prompt("Container name or ID")
        .interact_text()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    // Validate container name/ID
    if let Err(e) = crate::docker::validate_container_name(&container) {
        println!("❌ Invalid container name/ID: {}", e);
        return;
    }

    let follow = Confirm::new()
        .with_prompt("Follow logs (real-time)?")
        .default(false)
        .interact_opt()
        .ok()
        .flatten()
        .unwrap_or(false);

    let mut args = vec!["logs"];
    if follow {
        args.push("-f");
    }
    args.extend_from_slice(&["--tail", "100"]);
    args.push(&container);

    println!("📜 Container logs for: {}", container);
    if let Err(e) = Command::new("docker").args(&args).status() {
        println!("❌ Failed to get container logs: {}", e);
    }
}

fn inspect_container() {
    use dialoguer::Input;

    let container: String = match Input::new()
        .with_prompt("Container name or ID")
        .interact_text()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    // Validate container name/ID
    if let Err(e) = crate::docker::validate_container_name(&container) {
        println!("❌ Invalid container name/ID: {}", e);
        return;
    }

    println!("🔍 Inspecting container: {}", container);
    if let Err(e) = Command::new("docker")
        .args(["inspect", &container])
        .status()
    {
        println!("❌ Failed to inspect container: {}", e);
    }
}
