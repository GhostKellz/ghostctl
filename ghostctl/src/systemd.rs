use dialoguer::Input;

pub fn handle(action: String) {
    use dialoguer::Input;
    match action.as_str() {
        "enable" => super::systemd::enable(),
        "disable" => super::systemd::disable(),
        "status" => super::systemd::status(),
        "create" => super::systemd::create(),
        _ => println!("Unknown systemd action. Use enable, disable, status, or create."),
    }
}

fn list_services() {
    println!("Listing systemd services and timers...");
    let _ = std::process::Command::new("systemctl").arg("list-units").arg("--type=service").status();
    let _ = std::process::Command::new("systemctl").arg("list-timers").status();
}

pub fn enable() {
    let name: String = Input::new().with_prompt("Service/Timer to enable").interact_text().unwrap();
    let status = std::process::Command::new("sudo")
        .args(["systemctl", "enable", "--now", &name])
        .status();
    match status {
        Ok(s) if s.success() => println!("{} enabled and started.", name),
        _ => println!("Failed to enable {}.", name),
    }
}

pub fn disable() {
    let name: String = Input::new().with_prompt("Service/Timer to disable").interact_text().unwrap();
    let status = std::process::Command::new("sudo")
        .args(["systemctl", "disable", "--now", &name])
        .status();
    match status {
        Ok(s) if s.success() => println!("{} disabled and stopped.", name),
        _ => println!("Failed to disable {}.", name),
    }
}

pub fn status() {
    let name: String = Input::new().with_prompt("Service/Timer to check status").interact_text().unwrap();
    let status = std::process::Command::new("systemctl")
        .args(["status", &name])
        .status();
    match status {
        Ok(s) if s.success() => (),
        _ => println!("Failed to get status for {}.", name),
    }
}

pub fn create() {
    println!("ghostctl :: Create new systemd service/timer (not yet implemented)");
    // TODO: Implement interactive creation of service/timer units
}