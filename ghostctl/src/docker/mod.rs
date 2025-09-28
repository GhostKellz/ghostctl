pub mod cleanup;
pub mod compose;
pub mod container;
pub mod devops;
pub mod registry;
pub mod security;

use dialoguer::{theme::ColorfulTheme, Select};

pub fn docker_menu() {
    loop {
        let options = vec![
            "Docker Management & DevOps",
            "Container Operations",
            "Compose Stack Manager",
            "Registry Management",
            "Docker Cleanup Tools",
            "Security & Scanning",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🐳 Docker Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => devops::docker_management(),
            1 => container::container_management(),
            2 => compose::compose_stack_manager(),
            3 => registry::registry_management(),
            4 => cleanup::cleanup_menu(),
            5 => security::container_security(),
            _ => break,
        }
    }
}

pub fn install_docker() {
    println!("🐳 Installing Docker");
    println!("===================");
    devops::docker_management();
}

pub fn homelab_stacks_menu() {
    println!("🏠 Homelab Docker Stacks");
    println!("========================");
    compose::compose_stack_manager();
}
