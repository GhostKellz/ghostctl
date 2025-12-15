// Allow common patterns for cleaner CI builds
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(for_loops_over_fallibles)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::needless_return)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::useless_vec)]
#![allow(clippy::option_map_or_none)]
#![allow(clippy::map_entry)]
#![allow(clippy::single_match)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::unit_arg)]
#![allow(clippy::double_ended_iterator_last)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::println_empty_string)]
#![allow(clippy::manual_flatten)]
#![allow(clippy::or_fun_call)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::len_zero)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::useless_format)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::to_string_in_format_args)]
#![allow(clippy::bind_instead_of_map)]
#![allow(clippy::cmp_owned)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::option_map_unit_fn)]
#![allow(clippy::unnecessary_map_or)]
#![allow(clippy::unnecessary_to_owned)]
#![allow(clippy::format_in_format_args)]
#![allow(clippy::trim_split_whitespace)]
#![allow(clippy::collapsible_str_replace)]
#![allow(clippy::duplicated_attributes)]
#![allow(clippy::unwrap_or_default)]

mod arch;
mod backup;
mod btrfs;
mod cli;
mod cloud;
mod config;
mod dev;
mod docker;
mod gaming;
mod logging;
mod menu;
mod network;
mod networking;
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
mod storage;
mod systemd;
mod terminal;
mod tools;
mod utils;

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
