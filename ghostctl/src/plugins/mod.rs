pub mod core;
pub mod manager;
pub mod runner;

pub use core::run_core_script;
pub use manager::{install_from_url, list_plugins};
pub use runner::run_lua_plugin;
