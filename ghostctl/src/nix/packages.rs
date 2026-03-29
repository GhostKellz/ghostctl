use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::process::Command;

pub fn package_management() {
    println!("📦 NixOS Package Management");
    println!("===========================");

    let options = [
        "🔍 Search packages",
        "📦 Install package",
        "🗑️  Remove package",
        "📋 List installed packages",
        "🔄 Update packages",
        "📊 Package information",
        "⬅️  Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Package Management")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => search_packages(),
        1 => install_package(),
        2 => remove_package(),
        3 => list_packages(),
        4 => update_packages(),
        5 => package_info(),
        _ => return,
    }
}

fn search_packages() {
    let Ok(query) = Input::<String>::new()
        .with_prompt("Search term")
        .interact_text()
    else {
        return;
    };

    println!("🔍 Searching for: {}", query);
    let _ = Command::new("nix")
        .args(&["search", "nixpkgs", &query])
        .status();
}

fn install_package() {
    let Ok(package) = Input::<String>::new()
        .with_prompt("Package name")
        .interact_text()
    else {
        return;
    };

    println!("📦 Installing: {}", package);
    let _ = Command::new("nix-env")
        .args(&["-iA", &format!("nixpkgs.{}", package)])
        .status();
}

fn remove_package() {
    let Ok(package) = Input::<String>::new()
        .with_prompt("Package name to remove")
        .interact_text()
    else {
        return;
    };

    println!("🗑️  Removing: {}", package);
    let _ = Command::new("nix-env").args(&["-e", &package]).status();
}

fn list_packages() {
    println!("📋 Installed packages:");
    let _ = Command::new("nix-env").args(&["-q"]).status();
}

fn update_packages() {
    println!("🔄 Updating packages...");
    let _ = Command::new("nix-env").args(&["-u"]).status();
}

fn package_info() {
    let Ok(package) = Input::<String>::new()
        .with_prompt("Package name")
        .interact_text()
    else {
        return;
    };

    println!("📊 Package information for: {}", package);
    let _ = Command::new("nix")
        .args(&["show-derivation", &format!("nixpkgs#{}", package)])
        .status();
}
