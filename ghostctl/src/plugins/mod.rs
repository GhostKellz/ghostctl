pub mod runner;
pub mod manager;
pub mod core;

pub fn list() {
    manager::list_plugins();
}

pub fn run(name: &str) {
    runner::execute(name);
}

pub fn install_from_url(url: &str) {
    manager::install_from_url(url);
}
