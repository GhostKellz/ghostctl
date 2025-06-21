mod arch;
mod backup;
mod btrfs;
mod cli;
mod cloud;
mod config;
mod dev;
mod docker;
mod logging;
mod menu;
mod network;
mod nginx;
mod nix;
mod nvidia;
mod nvim;
mod plugins;
mod proxmox;
mod release;
mod restic;
mod restore;
mod scripts;
mod security;
mod shell;
mod systemd;
mod terminal;
mod tools;

use cli::{build_cli, handle_cli_args};

// Define action enums for CLI subcommands
#[derive(Debug)]
pub enum BtrfsAction {
    List,
    Create { name: String, subvolume: String },
    Delete { name: String },
    Restore { name: String, target: String },
    SnapperSetup,
    SnapperEdit { config: String },
    SnapperList,
    Status,
    Scrub { mountpoint: String },
    Balance { mountpoint: String },
    Usage { mountpoint: String },
    Quota { mountpoint: String },
}

#[derive(Debug)]
pub enum NixosAction {
    Rebuild,
    Update,
    Rollback,
    GarbageCollect,
    Generations,
}

fn main() {
    logging::GhostLogger::init();

    let matches = build_cli().get_matches();
    handle_cli_args(&matches);
}
