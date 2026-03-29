pub mod go;
pub mod gtools;
pub mod python;
pub mod rust;
pub mod zig;

use dialoguer::{Select, theme::ColorfulTheme};

pub fn development_menu() {
    println!("Development Environment");
    println!("===========================");

    let options = [
        "Rust Development",
        "Zig Development",
        "Go Development",
        "Python Development",
        "Ghost Tools (Reaper, Oxygen, Zion)",
        "Package Managers & Tools",
        "IDE & Editor Setup",
        "Back",
    ];

    let Ok(choice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Development Tools")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match choice {
        0 => crate::dev::rust::rust_development(),
        1 => crate::dev::zig::zig_development_menu(),
        2 => crate::dev::go::go_development_menu(),
        3 => crate::dev::python::python_development_menu(),
        4 => crate::dev::gtools::ghost_ecosystem_menu(),
        5 => package_managers_menu(),
        6 => ide_setup_menu(),
        _ => return,
    }
}

pub use gtools::check_tool_status;
pub use gtools::ghost_ecosystem_menu;
pub use gtools::install_all_ghost_tools;
pub use gtools::install_oxygen;
pub use gtools::install_reaper;
pub use gtools::install_zion;

pub fn rust_development_menu() {
    println!("🦀 Rust Development");
    println!("===================");

    println!("💡 Rust development tools:");
    println!("  • rustup: Rust toolchain installer");
    println!("  • cargo: Package manager and build tool");
    println!("  • rust-analyzer: Language server");
    println!("  • clippy: Linter");
    println!("  • rustfmt: Code formatter");
}

#[allow(dead_code)]
fn python_development() {
    println!("🐍 Python Development - Coming Soon!");
    println!("====================================");
    println!("💡 This feature will be added in a future update");
}

#[allow(dead_code)]
fn go_development() {
    println!("🐹 Go Development - Coming Soon!");
    println!("=================================");
    println!("💡 This feature will be added in a future update");
}

#[allow(dead_code)]
fn nodejs_development() {
    println!("🟦 Node.js Development - Coming Soon!");
    println!("=====================================");
    println!("💡 This feature will be added in a future update");
}

fn package_managers_menu() {
    println!("📦 Package Managers & Tools");
    println!("===========================");

    println!("💡 Available package managers:");
    println!("  • Cargo (Rust)");
    println!("  • pip (Python)");
    println!("  • go mod (Go)");
    println!("  • npm/yarn (Node.js)");
    println!("  • reaper (AUR helper)");
}

fn ide_setup_menu() {
    println!("🔧 IDE & Editor Setup");
    println!("=====================");

    println!("💡 Supported editors:");
    println!("  • Neovim with LazyVim");
    println!("  • VS Code");
    println!("  • Vim");
    println!("  • Emacs");
}
