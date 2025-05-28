use crate::{arch, dev, btrfs, nvidia, nvim, shell, systemd, restic, scripts};
use dialoguer::{theme::ColorfulTheme, Select};

pub fn show() {
    let opts = [
        "Fix Arch Issues",
        "Stage Dev Project (Rust/Go/Zig)",
        "Manage Btrfs Snapshots",
        "NVIDIA Tools",
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
        0 => arch::fix("pacman".into()),
        1 => dev::stage("rust".into()),
        2 => btrfs::run(),
        3 => nvidia::optimize(),
        4 => nvim::install(),
        5 => shell::setup(),
        6 => {
            systemd::handle("status".into());
            restic::setup();
        }
        7 => scripts::run_from_url("https://raw.githubusercontent.com/..."),
        _ => println!("Goodbye."),
    }
}
