pub fn handle(action: String) {
    match action.as_str() {
        "list" => list_services(),
        "enable" => enable_service(),
        "disable" => disable_service(),
        "status" => status_service(),
        "create-timer" => create_timer(),
        _ => println!("Unknown systemd action: {}", action),
    }
}

fn list_services() {
    println!("Listing systemd services and timers...");
    let _ = std::process::Command::new("systemctl").arg("list-units").arg("--type=service").status();
    let _ = std::process::Command::new("systemctl").arg("list-timers").status();
}

fn enable_service() {
    println!("Enabling a systemd service/timer (stub, prompt for name in future)");
}

fn disable_service() {
    println!("Disabling a systemd service/timer (stub, prompt for name in future)");
}

fn status_service() {
    println!("Checking status of a systemd service/timer (stub, prompt for name in future)");
}

fn create_timer() {
    println!("Creating a systemd timer (stub, integrate with restic/backup setup)");
}