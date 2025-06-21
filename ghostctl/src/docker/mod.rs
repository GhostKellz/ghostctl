pub mod compose;
pub mod container;
pub mod devops;
pub mod registry;
pub mod security; // This will be our main docker management

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

pub fn docker_menu() {
    devops::docker_management();
}
