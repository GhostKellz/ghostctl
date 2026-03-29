pub mod credential_backends;
pub mod credentials;
pub mod gpg;
pub mod ssh;
pub mod validation;

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

    let choice = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Security Management")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(choice)) => choice,
        Ok(None) => return, // User cancelled
        Err(e) => {
            eprintln!("Menu selection failed: {}", e);
            return;
        }
    };

    match choice {
        0 => ssh::ssh_management(),
        1 => gpg::gpg_key_management(),
        2 => credentials::credential_management(),
        _ => return,
    }
}
