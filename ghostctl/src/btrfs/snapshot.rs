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
    if Confirm::new().with_prompt(&format!("Delete snapshot '{}'?", name)).default(false).interact().unwrap() {
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
    if Confirm::new().with_prompt(&format!("This will overwrite '{}'. Continue?", target)).default(false).interact().unwrap() {
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

pub fn snapper_config(subvolume: &str, config: &str) {
    println!("Configuring Snapper for subvolume '{}' as config '{}'...", subvolume, config);
    let status = std::process::Command::new("sudo")
        .args(["snapper", "-c", config, "create-config", subvolume])
        .status();
    match status {
        Ok(s) if s.success() => println!("Snapper config '{}' created for '{}'.", config, subvolume),
        _ => println!("Failed to configure Snapper."),
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
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    let path = format!("/etc/snapper/configs/{}", config);
    let status = std::process::Command::new(editor)
        .arg(&path)
        .status();
    match status {
        Ok(s) if s.success() => println!("Edited Snapper config: {}", path),
        _ => println!("Failed to edit Snapper config: {}", path),
    }
}

pub fn snapper_list() {
    println!("Available Snapper configs:");
    let output = std::process::Command::new("ls")
        .arg("/etc/snapper/configs/")
        .output();
    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout)),
        Err(_) => println!("Failed to list Snapper configs."),
    }
}

