use std::fs;

pub fn list_plugins() {
    let plugin_dir = dirs::config_dir().unwrap().join("ghostctl/plugins");
    println!("ghostctl :: Available Plugins");
    if let Ok(entries) = fs::read_dir(&plugin_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "lua" {
                    if let Some(name) = path.file_stem() {
                        println!("- {}", name.to_string_lossy());
                    }
                }
            }
        }
    } else {
        println!("No plugins directory found at {:?}", plugin_dir);
    }
}

pub fn install_from_url(url: &str) {
    println!("Installing plugin from URL: {}", url);
    // TODO: Download and save to ~/.config/ghostctl/plugins/
}

