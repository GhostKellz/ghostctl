pub fn up() {
    println!("ghostctl :: Tailscale Up (custom config)");
    let status = std::process::Command::new("sudo")
        .args([
            "tailscale",
            "up",
            "--login-server",
            "https://ghost.cktechx.com",
            "--accept-routes",
            "--accept-dns=false",
            "--ssh",
            "--operator=chris",
        ])
        .status();
    match status {
        Ok(s) if s.success() => println!("Tailscale up with custom config!"),
        _ => println!("Failed to bring up Tailscale."),
    }
}

pub fn advertise(subnet: &str) {
    println!("ghostctl :: Tailscale Up (advertise subnet)");
    let status = std::process::Command::new("sudo")
        .args([
            "tailscale",
            "up",
            "--login-server",
            "https://ghost.cktechx.com",
            "--accept-routes",
            "--accept-dns=false",
            "--ssh",
            "--operator=chris",
            "--advertise-routes",
            subnet,
        ])
        .status();
    match status {
        Ok(s) if s.success() => println!("Tailscale up with subnet advertised!"),
        _ => println!("Failed to advertise subnet."),
    }
}

pub fn status() {
    let status = std::process::Command::new("tailscale")
        .arg("status")
        .status();
    match status {
        Ok(s) if s.success() => (),
        _ => println!("Failed to get Tailscale status."),
    }
}

pub fn down() {
    let status = std::process::Command::new("sudo")
        .args(["tailscale", "down"])
        .status();
    match status {
        Ok(s) if s.success() => println!("Tailscale brought down."),
        _ => println!("Failed to bring down Tailscale."),
    }
}

#[allow(dead_code)]
pub fn headscale_join(namespace: &str) {
    println!("Joining Headscale namespace: {}", namespace);
    // Wrap shell command to pull key or run pre-auth etc.
}
