use std::process::Command;

pub fn execute(name: &str) {
    let plugin_path = dirs::config_dir()
        .unwrap()
        .join("ghostctl/plugins")
        .join(name);

    if plugin_path.extension().unwrap_or_default() == "sh" {
        println!("Running shell plugin: {}", name);
        let _ = Command::new("bash")
            .arg(plugin_path)
            .status();
    } else if plugin_path.extension().unwrap_or_default() == "lua" {
        println!("Running Lua plugin: {}", name);
        let _ = Command::new("lua")
            .arg(plugin_path)
            .status();
    } else {
        println!("Unknown plugin type.");
    }
}
