pub mod snapshot;

pub use snapshot::restore_snapshot;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum SnapshotAction {
    List,
    Create {
        name: String,
        subvolume: Option<String>,
    },
    Delete {
        name: String,
    },
    Restore {
        name: String,
        target: String,
    },
    SnapperSetup,
}

#[allow(dead_code)]
pub fn handle(action: SnapshotAction) {
    match action {
        SnapshotAction::List => snapshot::list_snapshots(),
        SnapshotAction::Create { name, subvolume } => {
            snapshot::create_snapshot(subvolume.as_deref().unwrap_or("/"), &name)
        }
        SnapshotAction::Delete { name } => snapshot::delete_snapshot(&name),
        SnapshotAction::Restore { name, target } => snapshot::restore_snapshot(&name, &target),
        SnapshotAction::SnapperSetup => snapshot::snapper_setup(),
    }
}

pub fn handle_none() {
    println!("No btrfs subcommand provided. Use 'ghostctl snapshot --help' for options.");
}
