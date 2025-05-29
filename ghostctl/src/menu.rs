use crate::{arch, dev, btrfs, nvidia, nvim, shell, terminal, systemd, restic, scripts};
use dialoguer::{theme::ColorfulTheme, Select};

pub fn show() {
    let opts = [
        "Fix Arch Issues (Pacman, PKGBUILD, Optimize)",
        "Stage Dev Project (Rust/Go/Zig)",
        "Manage Btrfs Snapshots",
        "NVIDIA Tools (Clean, Fix, Diagnostics)",
        "Neovim Configurator",
        "Shell Setup (ZSH, Oh My Zsh, Powerlevel10k, tmux)",
        "Terminal Setup (Ghostty, WezTerm)",
        "Diagnostics/Status",
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
                "Fix Pacman/Keyring", "Clean Orphans", "Performance Tuning", "Back"
            ];
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Arch Maintenance")
                .items(&arch_opts)
                .default(0)
                .interact()
                .unwrap() {
                0 => arch::archfix::fix(),
                1 => arch::archfix::orphans(),
                2 => arch::perf::tune(),
                _ => (),
            }
        },
        1 => dev::stage("rust".into()),
        2 => {
            let btrfs_opts = [
                "List Snapshots", "Create Snapshot", "Delete Snapshot", "Restore Snapshot", "Deploy Snapper Base Configs", "Edit Snapper Config", "List Snapper Configs", "Back"
            ];
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Btrfs Snapshot Manager")
                .items(&btrfs_opts)
                .default(0)
                .interact()
                .unwrap() {
                0 => btrfs::snapshot::list_snapshots(),
                1 => {
                    use dialoguer::Input;
                    let name: String = Input::new().with_prompt("Snapshot name").interact_text().unwrap();
                    let subvol: String = Input::new().with_prompt("Subvolume (e.g. /)").default("/".into()).interact_text().unwrap();
                    btrfs::snapshot::create_snapshot(&subvol, &name)
                },
                2 => {
                    use dialoguer::Input;
                    let name: String = Input::new().with_prompt("Snapshot name to delete").interact_text().unwrap();
                    btrfs::snapshot::delete_snapshot(&name)
                },
                3 => {
                    use dialoguer::Input;
                    let name: String = Input::new().with_prompt("Snapshot name to restore").interact_text().unwrap();
                    let target: String = Input::new().with_prompt("Restore target (mountpoint or subvolume)").interact_text().unwrap();
                    btrfs::snapshot::restore_snapshot(&name, &target)
                },
                4 => btrfs::snapshot::snapper_setup(),
                5 => {
                    use dialoguer::Input;
                    let config: String = Input::new().with_prompt("Snapper config to edit").default("root".into()).interact_text().unwrap();
                    btrfs::snapshot::snapper_edit(&config)
                },
                6 => btrfs::snapshot::snapper_list(),
                _ => (),
            }
        },
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
        4 => {
            let nvim_opts = [
                "Install Neovim Distro", "Diagnostics", "List Plugins", "Update Plugins", "Back"
            ];
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Neovim Setup")
                .items(&nvim_opts)
                .default(0)
                .interact()
                .unwrap() {
                0 => nvim::install(),
                1 => nvim::diagnostics(),
                2 => nvim::list_plugins(),
                3 => nvim::update_plugins(),
                _ => (),
            }
        },
        5 => {
            let shell_opts = [
                "Install ZSH", "Install Oh My Zsh", "Install Powerlevel10k", "Set Default ZSH", "Install tmux", "Back"
            ];
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Shell Setup")
                .items(&shell_opts)
                .default(0)
                .interact()
                .unwrap() {
                0 => shell::install_zsh(),
                1 => shell::install_ohmyzsh(),
                2 => shell::install_powerlevel10k(),
                3 => shell::set_default_zsh(),
                4 => shell::install_tmux(),
                _ => (),
            }
        },
        6 => {
            let term_opts = [
                "Setup Ghostty", "Setup WezTerm", "Back"
            ];
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Terminal Setup")
                .items(&term_opts)
                .default(0)
                .interact()
                .unwrap() {
                0 => terminal::setup_ghostty(),
                1 => terminal::setup_wezterm(),
                _ => (),
            }
        },
        7 => {
            // Diagnostics/Status
            use std::process::Command;
            let tools = [
                ("nvim", "Neovim"),
                ("zsh", "ZSH"),
                ("ghostty", "Ghostty"),
                ("wezterm", "WezTerm"),
                ("tmux", "tmux"),
            ];
            println!("\nDiagnostics/Status:");
            for (bin, name) in tools.iter() {
                let found = Command::new("which").arg(bin).output().map(|o| o.status.success()).unwrap_or(false);
                if found {
                    println!("[OK]   {} is installed", name);
                } else {
                    println!("[MISS] {} is NOT installed", name);
                }
            }
        },
        8 => {
            systemd::handle("status".into());
            restic::setup();
        },
        9 => scripts::run_from_url("https://raw.githubusercontent.com/..."),
        _ => println!("Goodbye."),
    }
}
