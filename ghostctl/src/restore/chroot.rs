pub fn enter() {
    println!("Setting up chroot environment...");

    println!("- Mounting /mnt");
    println!("- Binding /dev, /proc, /sys, /run...");
    println!("arch-chroot /mnt");

    // Future: run via duct or shell
}
