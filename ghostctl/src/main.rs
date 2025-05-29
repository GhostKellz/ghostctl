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

    /// Fix Arch-specific components (e.g., pacman, pkgbuild, optimize)
    Fix { target: String },

    /// Stage a development environment (Rust, Go, Zig)
    Stage { project: String },

    /// Manage Btrfs snapshots and restores
    Btrfs {
        #[command(subcommand)]
        action: Option<BtrfsAction>,
    },

    /// Run NVIDIA driver-related fixes
    Nvidia {
        #[command(subcommand)]
        action: Option<NvidiaAction>,
    },

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

    /// Manage Restic backups and restores
    Backup {
        #[command(subcommand)]
        action: Option<BackupAction>,
    },
}

#[derive(Subcommand)]
enum BtrfsAction {
    List,
    Create { name: String, subvolume: Option<String> },
    Delete { name: String },
    Restore { name: String, target: String },
    SnapperSetup,
    SnapperEdit { config: String },
    SnapperList,
}

#[derive(Subcommand)]
enum NvidiaAction {
    Clean,
    Fix,
    Diagnostics,
    Install,
    Open,
    Openbeta,
    Info,
    Status,
    Optimize,
}

#[derive(Subcommand)]
enum BackupAction {
    Run,
    Schedule,
    Verify,
    Cleanup,
    Restore,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Menu) => menu::show(),
        Some(Command::Fix { target }) => arch::fix(target),
        Some(Command::Stage { project }) => dev::stage(project),
        Some(Command::Btrfs { action }) => match action {
            Some(BtrfsAction::List) => btrfs::snapshot::list_snapshots(),
            Some(BtrfsAction::Create { name, subvolume }) => btrfs::snapshot::create_snapshot(subvolume.as_deref().unwrap_or("/"), &name),
            Some(BtrfsAction::Delete { name }) => btrfs::snapshot::delete_snapshot(&name),
            Some(BtrfsAction::Restore { name, target }) => btrfs::snapshot::restore_snapshot(&name, &target),
            Some(BtrfsAction::SnapperSetup) => btrfs::snapshot::snapper_setup(),
            Some(BtrfsAction::SnapperEdit { config }) => btrfs::snapshot::snapper_edit(&config),
            Some(BtrfsAction::SnapperList) => btrfs::snapshot::snapper_list(),
            None => btrfs::run(),
        },
        Some(Command::Nvidia { action }) => match action {
            Some(NvidiaAction::Clean) => nvidia::clean(),
            Some(NvidiaAction::Fix) => nvidia::fix(),
            Some(NvidiaAction::Diagnostics) => nvidia::diagnostics(),
            Some(NvidiaAction::Install) => nvidia::install_proprietary(),
            Some(NvidiaAction::Open) => nvidia::install_open(),
            Some(NvidiaAction::Openbeta) => nvidia::install_open_beta(),
            Some(NvidiaAction::Info) => nvidia::info(),
            Some(NvidiaAction::Status) => nvidia::status(),
            Some(NvidiaAction::Optimize) => nvidia::optimize(),
            None => nvidia::optimize(),
        },
        Some(Command::Nvim) => nvim::install(),
        Some(Command::Shell) => shell::setup(),
        Some(Command::Script { url }) => scripts::run_from_url(&url),
        Some(Command::Restic) => restic::setup(),
        Some(Command::Systemd { action }) => systemd::handle(action),
        Some(Command::Backup { action }) => match action {
            Some(BackupAction::Run) => backup::schedule::run(),
            Some(BackupAction::Schedule) => backup::schedule::schedule(),
            Some(BackupAction::Verify) => backup::verify::verify(),
            Some(BackupAction::Cleanup) => backup::cleanup::run(),
            Some(BackupAction::Restore) => backup::restore::run(),
            None => backup::menu(),
        },
        None => {
            println!(
                r#"
ghostctl :: Arch sysadmin CLI & TUI toolkit

Available Commands:
  ghostctl menu                    Launch interactive menu
  ghostctl fix <target>           Fix arch tools (pacman, pkgbuild, optimize)
  ghostctl stage <lang>           Stage Rust, Go, or Zig projects
  ghostctl btrfs <subcommand>     Manage Btrfs snapshots (list, create, delete, restore, snapper_setup, snapper_edit, snapper_list)
  ghostctl nvidia <subcommand>    NVIDIA tools (clean, fix, diagnostics, install, open, openbeta, info, status, optimize)
  ghostctl nvim                   Setup Neovim (LazyVim, Kickstart, Astro)
  ghostctl shell                  ZSH, Powerlevel10k, Ghostty, Kitty, WezTerm
  ghostctl script <url>           Run remote script
  ghostctl restic                 Setup Restic backups
  ghostctl systemd <action>       Manage systemd services or timers
  ghostctl backup <subcommand>    Manage Restic backups (run, schedule, verify, cleanup, restore)

Tip: Use `ghostctl menu` for a guided interactive setup.
"#
            );
        }
    }
}
