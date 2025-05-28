pub mod core;
pub mod manager;
pub mod runner;

pub use core::run_core_script;
pub use manager::{list_plugins, install_from_url};
pub use runner::run_lua_plugin;
