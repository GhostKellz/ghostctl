use std::fs;
use std::path::PathBuf;

pub fn list_plugins() {
    let plugins_dir = PathBuf::from("./plugins");
    println!("ghostctl :: List Plugins");
    if let Ok(entries) = fs::read_dir(&plugins_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                println!("- {}", path.display());
            }
        }
    } else {
        println!("No plugins directory found or unable to read.");
    }
}

pub fn install_from_url(url: &str) {
    use std::process::Command;
    use std::fs::create_dir_all;
    use std::io::Write;
    let plugins_dir = PathBuf::from("./plugins");
    if let Err(e) = create_dir_all(&plugins_dir) {
        println!("Failed to create plugins directory: {}", e);
        return;
    }
    let filename = url.split('/').last().unwrap_or("plugin");
    let dest_path = plugins_dir.join(filename);
    println!("Downloading {} to {}", url, dest_path.display());
    let output = Command::new("curl")
        .args(["-fsSL", url])
        .output();
    match output {
        Ok(out) if out.status.success() => {
            if let Ok(mut file) = fs::File::create(&dest_path) {
                if file.write_all(&out.stdout).is_ok() {
                    println!("Plugin installed at {}", dest_path.display());
                } else {
                    println!("Failed to write plugin file.");
                }
            } else {
                println!("Failed to create plugin file.");
            }
        },
        _ => println!("Failed to download plugin from URL."),
    }
}

