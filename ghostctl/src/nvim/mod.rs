pub mod setup;
pub mod diagnostics;
pub mod plugins;

pub fn install() {
    println!("ghostctl :: Neovim Setup");
    setup::install_lazyvim();
}

pub fn diag() {
    diagnostics::run_checks();
}

pub fn list_plugins() {
    plugins::list();
}
