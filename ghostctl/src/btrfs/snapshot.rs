pub fn create_snapshot(subvolume: &str, name: &str) {
    println!("Creating snapshot: {}", name);
    let target = format!("/@snapshots/{}", name);
    let status = std::process::Command::new("sudo")
        .args(["btrfs", "subvolume", "snapshot", subvolume, &target])
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapshot '{}' created.", name),
        _ => println!("Failed to create snapshot."),
    }
}

pub fn list_snapshots() {
    println!("Listing Btrfs snapshots:");
    let output = std::process::Command::new("sudo")
        .args(["btrfs", "subvolume", "list", "/@snapshots"])
        .output();
    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout)),
        Err(_) => println!("Failed to list snapshots."),
    }
}

pub fn delete_snapshot(name: &str) {
    use dialoguer::Confirm;
    let target = format!("/@snapshots/{}", name);
    if Confirm::new().with_prompt(format!("Delete snapshot '{}'?", name)).default(false).interact().unwrap() {
        let status = std::process::Command::new("sudo")
            .args(["btrfs", "subvolume", "delete", &target])
            .status();
        match status {
            Ok(s) if s.success() => println!("Snapshot '{}' deleted.", name),
            _ => println!("Failed to delete snapshot."),
        }
    } else {
        println!("Aborted deletion.");
    }
}

pub fn restore_snapshot(name: &str, target: &str) {
    use dialoguer::Confirm;
    println!("Restoring snapshot '{}' to '{}'...", name, target);
    if Confirm::new().with_prompt(format!("This will overwrite '{}'. Continue?", target)).default(false).interact().unwrap() {
        let source = format!("/@snapshots/{}", name);
        let status = std::process::Command::new("sudo")
            .args(["btrfs", "subvolume", "snapshot", &source, target])
            .status();
        match status {
            Ok(s) if s.success() => println!("Snapshot '{}' restored to '{}'.", name, target),
            _ => println!("Failed to restore snapshot."),
        }
    } else {
        println!("Aborted restore.");
    }
}

pub fn snapper_setup() {
    println!("Deploying Snapper base configs for root and home...");
    // Install snapper if not present
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -S --noconfirm snapper")
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapper installed or already present."),
        _ => println!("Failed to install snapper."),
    }
    // Create root config
    let status = std::process::Command::new("sudo")
        .args(["snapper", "-c", "root", "create-config", "/"])
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapper config 'root' created for '/'."),
        _ => println!("Failed to create Snapper config for root."),
    }
    // Create home config
    let status = std::process::Command::new("sudo")
        .args(["snapper", "-c", "home", "create-config", "/home"])
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapper config 'home' created for '/home'."),
        _ => println!("Failed to create Snapper config for home."),
    }
    println!("You may want to edit /etc/snapper/configs/root and /etc/snapper/configs/home for retention and cleanup settings.");
}

pub fn snapper_edit(config: &str) {
    use std::process::Command;
    let config_path = format!("/etc/snapper/configs/{}", config);
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    let status = Command::new(&editor)
        .arg(&config_path)
        .status();
    match status {
        Ok(s) if s.success() => println!("Edited Snapper config: {}", config_path),
        _ => println!("Failed to edit Snapper config: {}", config_path),
    }
}

pub fn snapper_list() {
    use std::fs;
    let configs_dir = "/etc/snapper/configs";
    match fs::read_dir(configs_dir) {
        Ok(entries) => {
            println!("Available Snapper configs:");
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    println!("- {}", name);
                }
            }
        },
        Err(_) => println!("No Snapper configs found in {}", configs_dir),
    }
}

pub fn scrub(mountpoint: &str) {
    println!("Starting btrfs scrub on {}...", mountpoint);
    let status = std::process::Command::new("sudo")
        .args(["btrfs", "scrub", "start", mountpoint])
        .status();
    match status {
        Ok(s) if s.success() => println!("Scrub started on {}.", mountpoint),
        _ => println!("Failed to start scrub on {}.", mountpoint),
    }
}

pub fn balance(mountpoint: &str) {
    println!("Starting btrfs balance on {}...", mountpoint);
    let status = std::process::Command::new("sudo")
        .args(["btrfs", "balance", "start", mountpoint])
        .status();
    match status {
        Ok(s) if s.success() => println!("Balance started on {}.", mountpoint),
        _ => println!("Failed to start balance on {}.", mountpoint),
    }
}

pub fn snapper_menu() {
    use dialoguer::{theme::ColorfulTheme, Select, Input};
    let opts = [
        "Deploy Base Config",
        "Edit Config",
        "List Configs",
        "Back",
    ];
    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Snapper Menu")
        .items(&opts)
        .default(0)
        .interact()
        .unwrap() {
        0 => snapper_setup(),
        1 => {
            let config: String = Input::new().with_prompt("Config to edit").default("root".into()).interact_text().unwrap();
            snapper_edit(&config)
        },
        2 => snapper_list(),
        _ => (),
    }
}

