pub mod local;
pub mod network;
pub mod s3_simple;
pub use s3_simple as s3;

use dialoguer::{theme::ColorfulTheme, Select};

pub fn storage_menu() {
    loop {
        let options = vec![
            "S3/MinIO Storage Management",
            "Local Storage Tools",
            "Network Storage (NFS/CIFS)",
            "Back",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("💾 Storage Management")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => s3::s3_menu(),
            1 => local::local_storage_menu(),
            2 => network::network_storage_menu(),
            _ => break,
        }
    }
}
