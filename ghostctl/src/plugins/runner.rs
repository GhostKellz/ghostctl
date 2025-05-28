use std::process::Command;
use mlua::{Lua, Result};
use std::fs;
use std::path::PathBuf;
use dialoguer::{theme::ColorfulTheme, Select};

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
        run_lua_plugin(name);
    } else {
        println!("Unknown plugin type.");
    }
}

pub fn run_lua_plugin(name: &str) {
    let plugin_dir = dirs::config_dir().unwrap().join("ghostctl/plugins");
    let plugin_path = plugin_dir.join(format!("{}.lua", name));
    if !plugin_path.exists() {
        println!("Plugin not found: {:?}", plugin_path);
        return;
    }
    let code = match fs::read_to_string(&plugin_path) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to read plugin: {}", e);
            return;
        }
    };
    let lua = Lua::new();
    // Expose a minimal Rust API to Lua
    let globals = lua.globals();
    globals.set("run_command", lua.create_function(|_, cmd: String| {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output();
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                println!("{}", stdout);
            },
            Err(e) => println!("Failed to run command: {}", e),
        }
        Ok(())
    }).unwrap()).unwrap();
    match lua.load(&code).exec() {
        Ok(_) => println!("Plugin '{}' executed successfully.", name),
        Err(e) => println!("Error running plugin '{}': {}", name, e),
    }
}

pub fn run_user_script_menu() {
    let scripts_dir = dirs::config_dir().unwrap().join("ghostctl/scripts");
    let mut scripts = Vec::new();
    if let Ok(entries) = fs::read_dir(&scripts_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    scripts.push(name.to_string_lossy().to_string());
                }
            }
        }
    }
    if scripts.is_empty() {
        println!("No user scripts found in {:?}", scripts_dir);
        return;
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a script to run")
        .items(&scripts)
        .default(0)
        .interact()
        .unwrap();
    let script = &scripts[selection];
    let script_path = scripts_dir.join(script);
    if script.ends_with(".sh") {
        println!("Running shell script: {}", script);
        let _ = Command::new("bash").arg(&script_path).status();
    } else if script.ends_with(".lua") {
        println!("Running Lua script: {}", script);
        run_lua_script(&script_path);
    } else {
        println!("Unknown script type: {}", script);
    }
}

pub fn run_lua_script(path: &PathBuf) {
    let code = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to read script: {}", e);
            return;
        }
    };
    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("run_command", lua.create_function(|_, cmd: String| {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output();
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                println!("{}", stdout);
            },
            Err(e) => println!("Failed to run command: {}", e),
        }
        Ok(())
    }).unwrap()).unwrap();
    match lua.load(&code).exec() {
        Ok(_) => println!("Lua script executed successfully."),
        Err(e) => println!("Error running Lua script: {}", e),
    }
}
