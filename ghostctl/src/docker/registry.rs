use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn registry_management() {
    println!("🗄️  Docker Registry Management");
    println!("==============================");

    let options = [
        "🔍 Search images",
        "📥 Pull image",
        "📤 Push image",
        "📋 List local images",
        "🗑️  Remove image",
        "🏷️  Tag image",
        "📊 Image history",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => search_images(),
        1 => pull_image(),
        2 => push_image(),
        3 => list_images(),
        4 => remove_image(),
        5 => tag_image(),
        6 => image_history(),
        _ => (),
    }
}

fn search_images() {
    let search_term: String = Input::new()
        .with_prompt("Search term")
        .interact_text()
        .unwrap();

    println!("🔍 Searching for: {}", search_term);
    let _ = Command::new("docker")
        .args(["search", &search_term])
        .status();
}

fn pull_image() {
    let image: String = Input::new()
        .with_prompt("Image name (e.g., nginx:latest)")
        .interact_text()
        .unwrap();

    println!("📥 Pulling image: {}", image);
    let status = Command::new("docker")
        .args(["pull", &image])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Successfully pulled {}", image),
        _ => println!("❌ Failed to pull {}", image),
    }
}

fn push_image() {
    let image: String = Input::new()
        .with_prompt("Image name to push (e.g., myregistry.com/myimage:tag)")
        .interact_text()
        .unwrap();

    println!("📤 Pushing image: {}", image);
    let status = Command::new("docker")
        .args(["push", &image])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Successfully pushed {}", image),
        _ => println!("❌ Failed to push {}", image),
    }
}

fn list_images() {
    println!("📋 Local Docker Images");
    let _ = Command::new("docker").args(["images"]).status();
}

fn remove_image() {
    let image: String = Input::new()
        .with_prompt("Image name or ID to remove")
        .interact_text()
        .unwrap();

    println!("🗑️  Removing image: {}", image);
    let status = Command::new("docker")
        .args(["rmi", &image])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Successfully removed {}", image),
        _ => println!("❌ Failed to remove {}", image),
    }
}

fn tag_image() {
    let source: String = Input::new()
        .with_prompt("Source image name")
        .interact_text()
        .unwrap();

    let target: String = Input::new()
        .with_prompt("Target tag name")
        .interact_text()
        .unwrap();

    println!("🏷️  Tagging {} as {}", source, target);
    let status = Command::new("docker")
        .args(["tag", &source, &target])
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Successfully tagged {} as {}", source, target),
        _ => println!("❌ Failed to tag image"),
    }
}

fn image_history() {
    let image: String = Input::new()
        .with_prompt("Image name")
        .interact_text()
        .unwrap();

    println!("📊 Image History for: {}", image);
    let _ = Command::new("docker")
        .args(["history", &image])
        .status();
}
