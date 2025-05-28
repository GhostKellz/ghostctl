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
        0 => crate::commands::arch::fix("pacman".into()),
        1 => crate::commands::dev::stage("rust".into()),
        2 => crate::commands::btrfs::run(),
        3 => crate::commands::nvidia::optimize(),
        4 => crate::commands::nvim::install(),
        5 => crate::commands::shell::setup(),
        6 => {
            crate::commands::systemd::handle("status".into());
            crate::commands::restic::setup();
        }
        7 => crate::scripts::run_from_url("https://raw.githubusercontent.com/..."),
        _ => println!("Goodbye."),
    }
}
