pub mod credentials;
pub mod gpg;
pub mod ssh;

use dialoguer::{Select, theme::ColorfulTheme};

pub fn security_menu() {
    println!("🔐 Security Management");
    println!("======================");

    let options = [
        "🔑 SSH Key Management",
        "🔐 GPG Key Management", 
        "🗂️  Credential Management",
        "⬅️  Back",
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Security Management")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match choice {
        0 => ssh::ssh_management(),
        1 => gpg::gpg_key_management(),
        2 => credentials::credential_management(),
        _ => return,
    }
}
