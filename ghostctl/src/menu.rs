use crate::{arch, dev, btrfs, nvidia, nvim, shell, systemd, restic, scripts};
use dialoguer::{theme::ColorfulTheme, Select};

pub fn show() {
    let opts = [
        "Fix Arch Issues (Pacman, PKGBUILD, Optimize)",
        "Stage Dev Project (Rust/Go/Zig)",
        "Manage Btrfs Snapshots",
        "NVIDIA Tools (Clean, Fix, Diagnostics)",
        "Neovim Configurator",
        "Shell Setup (Ghostty/ZSH)",
        "Systemd + Restic",
        "Run Remote Script",
        "Exit",
    ];

    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("ghostctl :: Menu")
        .items(&opts)
        .default(0)
        .interact()
        .unwrap()
    {
        0 => {
            let arch_opts = [
                "Fix Pacman", "Fix PKGBUILD/Packages", "Optimize System", "Back"
            ];
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Arch Maintenance")
                .items(&arch_opts)
                .default(0)
                .interact()
                .unwrap() {
                0 => arch::archfix::fix_pacman(),
                1 => arch::pkgfix::fix_pkgbuild(),
                2 => arch::perf::tune(),
                _ => (),
            }
        },
        1 => dev::stage("rust".into()),
        2 => btrfs::run(),
        3 => {
            let nvidia_opts = [
                "Clean DKMS/Modules", "Fix/Rebuild DKMS/Initramfs", "Back"
            ];
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("NVIDIA Tools")
                .items(&nvidia_opts)
                .default(0)
                .interact()
                .unwrap() {
                0 => nvidia::clean(),
                1 => nvidia::fix(),
                _ => (),
            }
        },
        4 => nvim::install(),
        5 => shell::setup(),
        6 => {
            systemd::handle("status".into());
            restic::setup();
        },
        7 => scripts::run_from_url("https://raw.githubusercontent.com/..."),
        _ => println!("Goodbye."),
    }
}
