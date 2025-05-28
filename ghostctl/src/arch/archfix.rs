pub fn fix_pacman() {
    println!("ghostctl :: Arch Pacman Fix");

    // Later: run commands using `duct` or `std::process::Command`
    println!("- sudo pacman -Syyu");
    println!("- sudo pacman -S archlinux-keyring");
    println!("- Optionally refresh mirrorlist with reflector");
}
