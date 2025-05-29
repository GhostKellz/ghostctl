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
