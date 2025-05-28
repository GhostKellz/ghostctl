pub fn install_zsh() {
    println!("- Installing Zsh + setting as default shell...");
    println!("sudo pacman -S zsh");
    println!("chsh -s /bin/zsh");

    println!("- Installing Oh My Zsh...");
    println!("sh -c \"$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)\"");

    println!("- Installing Powerlevel10k...");
    println!("git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ~/.oh-my-zsh/custom/themes/powerlevel10k");

    println!("- Installing Starship...");
    println!("curl -sS https://starship.rs/install.sh | sh");
}
