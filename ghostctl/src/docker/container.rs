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
        },
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
    println!("🚀 Run Docker Container - TODO: Implement");
}

pub fn stop_container(id: String) {
    println!("🛑 Stopping Docker Container: {}", id);
    let _ = std::process::Command::new("docker")
        .args(&["stop", &id])
        .status();
}

fn restart_container() {
    println!("🔄 Restart Docker Container - TODO: Implement");
}

fn remove_container() {
    println!("🗑️  Remove Docker Container - TODO: Implement");
}

fn container_stats() {
    println!("📊 Container Stats");
    let _ = Command::new("docker")
        .args(&["stats", "--no-stream"])
        .status();
}

fn container_logs() {
    println!("📜 Container Logs - TODO: Implement");
}

fn inspect_container() {
    println!("🔍 Inspect Container - TODO: Implement");
}
