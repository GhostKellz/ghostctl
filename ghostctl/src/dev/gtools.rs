use dialoguer::{Select, theme::ColorfulTheme};
use std::process::Command;
use which::which;

#[allow(dead_code)]
pub fn install_ghost_tools_menu() {
    let repos = [
        "ghostkellz/ghostbrew",
        "ghostkellz/ghostscan",
        "ghostkellz/ghostforge",
    ];
    let binaries = ["ghostbrew", "ghostscan", "ghostforge"];
    loop {
        let status_labels: Vec<String> = binaries.iter().map(|bin| {
            if which(bin).is_ok() {
                format!("{} [INSTALLED]", bin)
            } else {
                format!("{} [not installed]", bin)
            }
        }).collect();
        let mut menu_items: Vec<String> = status_labels.clone();
        menu_items.push("Back".to_string());
        let idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Install Ghost Tools")
            .items(&menu_items)
            .default(0)
            .interact()
            .unwrap();
        if idx == menu_items.len() - 1 { break; }
        let repo = repos[idx];
        let bin = binaries[idx];
        let action_opts = ["Install/Update", "Uninstall", "Back"];
        let action = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{}: Choose action", bin))
            .items(&action_opts)
            .default(0)
            .interact()
            .unwrap();
        match action {
            0 => {
                println!("Cloning and building {}...", repo);
                let status = Command::new("bash")
                    .arg("-c")
                    .arg(format!(
                        "git clone https://github.com/{repo}.git && cd {dir} && cargo build --release && sudo install -Dm755 target/release/{bin} /usr/bin/{bin}",
                        repo = repo,
                        dir = bin,
                        bin = bin
                    ))
                    .status();
                match status {
                    Ok(s) if s.success() => println!("{} installed successfully!", bin),
                    _ => println!("Failed to install {}.", bin),
                }
            }
            1 => {
                println!("Uninstalling {}...", bin);
                let status = Command::new("sudo")
                    .args(["rm", "-f", &format!("/usr/bin/{}", bin)])
                    .status();
                match status {
                    Ok(s) if s.success() => println!("{} uninstalled.", bin),
                    _ => println!("Failed to uninstall {}.", bin),
                }
            }
            _ => (),
        }
    }
}
