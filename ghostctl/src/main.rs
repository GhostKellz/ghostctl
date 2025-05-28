mod menu;
mod commands;
mod scripts;
mod arch;
mod btrfs;
mod nvim;
mod nvidia;
mod shell;
mod systemd;
mod dev;
mod backup;
mod restore;
mod network;
mod plugins;
mod restic;
mod terminal;

// ghostctl - Arch sysadmin CLI & TUI toolkit
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ghostctl", version, about = "Arch sysadmin CLI & TUI toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Launch the interactive TUI menu
    Menu,

    /// Fix Arch-specific components (e.g., pacman, pkgbuild)
    Fix { target: String },

    /// Stage a development environment (Rust, Go, Zig)
    Stage { project: String },

    /// Manage Btrfs snapshots and restores
    Btrfs,

    /// Run NVIDIA driver-related fixes
    Nvidia,

    /// Setup Neovim distributions (LazyVim, AstroNvim, Kickstart)
    Nvim,

    /// Setup ZSH shell, Starship, and terminals
    Shell,

    /// Run a plugin or script from a URL
    Script { url: String },

    /// Setup Restic-based backups
    Restic,

    /// Manage systemd services and timers
    Systemd { action: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Menu) => menu::show(),
        Some(Command::Fix { target }) => arch::fix(target),
        Some(Command::Stage { project }) => dev::stage(project),
        Some(Command::Btrfs) => btrfs::run(),
        Some(Command::Nvidia) => nvidia::optimize(),
        Some(Command::Nvim) => nvim::install(),
        Some(Command::Shell) => shell::setup(),
        Some(Command::Script { url }) => scripts::run_from_url(&url),
        Some(Command::Restic) => restic::setup(),
        Some(Command::Systemd { action }) => systemd::handle(action),
        None => {
            println!(
                r#"
ghostctl :: Arch sysadmin CLI & TUI toolkit

Available Commands:
  ghostctl menu                    Launch interactive menu
  ghostctl fix <target>           Fix arch tools (pacman, pkgbuild, optimize)
  ghostctl stage <lang>           Stage Rust, Go, or Zig projects
  ghostctl btrfs                  Manage Btrfs snapshots
  ghostctl nvidia                 Fix/patch NVIDIA systems
  ghostctl nvim                   Setup Neovim (LazyVim, Kickstart, Astro)
  ghostctl shell                  ZSH, Powerlevel10k, Ghostty, Kitty, WezTerm
  ghostctl script <url>          Run remote script
  ghostctl restic                 Setup Restic backups
  ghostctl systemd <action>      Manage systemd services or timers

Tip: Use `ghostctl menu` for a guided interactive setup.
"#
            );
        }
    }
}
