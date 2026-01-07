// Clippy suppressions - reduced from 39 to 8
// These cover patterns used consistently throughout the codebase
#![allow(dead_code)] // Many utility functions used conditionally by features
#![allow(unused_variables)] // Some params needed for API consistency
#![allow(clippy::upper_case_acronyms)] // VFIO, PBS, PVE are proper acronyms
#![allow(clippy::enum_variant_names)] // Consistent naming preferred
#![allow(clippy::needless_return)] // Match arm returns are more readable
#![allow(clippy::needless_borrows_for_generic_args)] // Common pattern throughout
#![allow(clippy::duplicated_attributes)] // Allow for local overrides
#![allow(clippy::too_many_arguments)] // Some builder functions need many params

mod arch;
mod backup;
#[cfg(target_os = "linux")]
mod bluetooth;
mod btrfs;
mod cli;
mod cloud;
pub mod command;
mod config;
mod dev;
mod docker;
mod gaming;
mod http_client;
mod logging;
mod menu;
mod network;
mod networking;
mod nginx;
mod nix;
mod nvidia;
mod nvim;
mod plugins;
pub mod progress;
mod proxmox;
mod release;
mod restic;
mod restore;
mod scripts;
mod security;
mod shell;
mod storage;
mod sysctl;
mod systemd;
mod terminal;
mod tools;
pub mod tui;
mod utils;
mod wifi;

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
    SnapperCleanup,
    Status,
    Scrub { mountpoint: String },
    Balance { mountpoint: String },
    Usage { mountpoint: String },
    Quota { mountpoint: String },
    EmergencyCleanup,
    CleanupByAge { days: String },
    CleanupByRange { range: String },
    DiskSpace,
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
