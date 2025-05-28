use std::process::Command;

pub fn tailscale_up() {
    println!("Running: tailscale up");
    let _ = Command::new("tailscale")
        .arg("up")
        .status()
        .expect("Failed to run tailscale up");
}

pub fn tailscale_status() {
    let _ = Command::new("tailscale")
        .arg("status")
        .status();
}

pub fn headscale_join(namespace: &str) {
    println!("Joining Headscale namespace: {}", namespace);
    // Wrap shell command to pull key or run pre-auth etc.
}
