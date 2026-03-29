/// Validate a systemd service/timer name.
/// Valid names contain alphanumeric characters, hyphens, underscores, at signs, and dots.
/// Names cannot be empty or start with a hyphen.
fn validate_service_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Service name cannot be empty".to_string());
    }

    if name.starts_with('-') {
        return Err("Service name cannot start with a hyphen".to_string());
    }

    // systemd service names can contain: alphanumeric, hyphen, underscore, at sign, dot
    // They may also have a colon for template instances (e.g., getty@tty1.service)
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '@' || c == '.' || c == ':')
    {
        return Err(
            "Service name contains invalid characters. Use only alphanumeric, -, _, @, ., or :"
                .to_string(),
        );
    }

    // Check for path traversal attempts
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        return Err("Service name cannot contain path components".to_string());
    }

    Ok(())
}

#[allow(dead_code)]
pub fn manage_service(action: &str) {
    match action {
        "enable" => enable(),
        "disable" => disable(),
        "status" => status(),
        "create" => create(),
        "list" => list_services(),
        _ => println!("Unknown systemd action. Use enable, disable, status, list, or create."),
    }
}

#[allow(dead_code)]
pub fn handle(action: String) {
    match action.as_str() {
        "enable" => super::systemd::enable(),
        "disable" => super::systemd::disable(),
        "status" => super::systemd::status(),
        "create" => super::systemd::create(),
        "list" => super::systemd::list_services(),
        _ => println!("Unknown systemd action. Use enable, disable, status, list, or create."),
    }
}

#[allow(dead_code)]
fn list_services() {
    println!("Listing systemd services and timers...");

    let service_status = std::process::Command::new("systemctl")
        .args(["list-units", "--type=service"])
        .status();

    match service_status {
        Ok(s) if s.success() => {}
        Ok(s) => {
            eprintln!(
                "Failed to list services (exit code: {})",
                s.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            eprintln!("Failed to execute systemctl: {}", e);
        }
    }

    println!();

    let timer_status = std::process::Command::new("systemctl")
        .args(["list-timers", "--all"])
        .status();

    match timer_status {
        Ok(s) if s.success() => {}
        Ok(s) => {
            eprintln!(
                "Failed to list timers (exit code: {})",
                s.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            eprintln!("Failed to execute systemctl: {}", e);
        }
    }
}

pub fn enable() {
    let name: String = match dialoguer::Input::new()
        .with_prompt("Service/Timer to enable")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => {
            println!("Input cancelled.");
            return;
        }
    };

    let name = name.trim();
    if let Err(e) = validate_service_name(name) {
        eprintln!("Invalid service name: {}", e);
        return;
    }

    let status = std::process::Command::new("sudo")
        .args(["systemctl", "enable", "--now", name])
        .status();

    match status {
        Ok(s) if s.success() => println!("{} enabled and started.", name),
        Ok(s) => {
            eprintln!(
                "Failed to enable {} (exit code: {})",
                name,
                s.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            eprintln!("Failed to execute systemctl: {}", e);
        }
    }
}

pub fn disable() {
    let name: String = match dialoguer::Input::new()
        .with_prompt("Service/Timer to disable")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => {
            println!("Input cancelled.");
            return;
        }
    };

    let name = name.trim();
    if let Err(e) = validate_service_name(name) {
        eprintln!("Invalid service name: {}", e);
        return;
    }

    let status = std::process::Command::new("sudo")
        .args(["systemctl", "disable", "--now", name])
        .status();

    match status {
        Ok(s) if s.success() => println!("{} disabled and stopped.", name),
        Ok(s) => {
            eprintln!(
                "Failed to disable {} (exit code: {})",
                name,
                s.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            eprintln!("Failed to execute systemctl: {}", e);
        }
    }
}

pub fn status() {
    let name: String = match dialoguer::Input::new()
        .with_prompt("Service/Timer to check status")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => {
            println!("Input cancelled.");
            return;
        }
    };

    let name = name.trim();
    if let Err(e) = validate_service_name(name) {
        eprintln!("Invalid service name: {}", e);
        return;
    }

    let status = std::process::Command::new("systemctl")
        .args(["status", name])
        .status();

    match status {
        Ok(s) if s.success() => {}
        // systemctl status returns exit code 3 for stopped services, which is expected
        Ok(s) if s.code() == Some(3) => {}
        Ok(s) => {
            eprintln!(
                "Failed to get status for {} (exit code: {})",
                name,
                s.code().unwrap_or(-1)
            );
        }
        Err(e) => {
            eprintln!("Failed to execute systemctl: {}", e);
        }
    }
}

pub fn create() {
    println!("ghostctl :: Create new systemd service/timer (not yet implemented)");
    // TODO: Implement interactive creation of service/timer units
}
