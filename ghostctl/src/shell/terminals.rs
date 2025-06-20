#[allow(dead_code)]
pub fn setup_wezterm() {
    println!("Installing/configuring WezTerm...");
    println!("mkdir -p ~/.config/wezterm");
    println!("cp ghostctl/resources/wezterm.lua ~/.config/wezterm/");

    // Later: detect + install from GitHub releases or pacman
}

#[allow(dead_code)]
pub fn setup_ghostty() {
    println!("Setting up Ghostty (WIP)");
    println!("https://github.com/mitchellh/ghostty");

    // Future: clone, build, configure theme
}

pub fn setup_starship() {
    println!("🚀 Setting up Starship prompt");
    // Implementation for starship setup
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("curl -sS https://starship.rs/install.sh | sh")
        .status();
    match status {
        Ok(s) if s.success() => println!("✅ Starship installed successfully"),
        _ => println!("❌ Failed to install Starship"),
    }
}
