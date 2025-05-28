use std::fs;

pub fn list_plugins() {
    let plugin_dir = dirs::config_dir()
        .unwrap()
        .join("ghostctl/plugins");

    println!("ghostctl :: Available Plugins");
    if let Ok(entries) = fs::read_dir(&plugin_dir) {
        for entry in entries.flatten() {
            println!("- {}", entry.file_name().to_string_lossy());
        }
    } else {
        println!("No plugins found.");
    }
}

pub fn install_from_url(url: &str) {
    println!("Installing plugin from: {}", url);
    let filename = url.split('/').last().unwrap_or("plugin.sh");
    let path = dirs::config_dir().unwrap().join("ghostctl/plugins").join(filename);

    if let Ok(content) = reqwest::blocking::get(url).and_then(|r| r.text()) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, content).unwrap();
        println!("Saved to: {}", path.display());
    } else {
        println!("Failed to download plugin");
    }
}

