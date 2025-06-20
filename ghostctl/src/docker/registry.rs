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
        _ => return,
    }
}

fn search_images() {
    let search_term: String = Input::new()
        .with_prompt("Search term")
        .interact_text()
        .unwrap();

    println!("🔍 Searching for: {}", search_term);
    let _ = Command::new("docker")
        .args(&["search", &search_term])
        .status();
}

fn pull_image() {
    println!("📥 Pull Docker Image - TODO: Implement");
}

fn push_image() {
    println!("📤 Push Docker Image - TODO: Implement");
}

fn list_images() {
    println!("📋 Local Docker Images");
    let _ = Command::new("docker").args(&["images"]).status();
}

fn remove_image() {
    println!("🗑️  Remove Docker Image - TODO: Implement");
}

fn tag_image() {
    println!("🏷️  Tag Docker Image - TODO: Implement");
}

fn image_history() {
    println!("📊 Image History - TODO: Implement");
}
