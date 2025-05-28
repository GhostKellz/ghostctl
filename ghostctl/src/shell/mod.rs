pub mod zsh;
pub mod terminals;

pub fn setup() {
    println!("ghostctl :: Shell Setup");
    zsh::install_zsh();
    terminals::setup_wezterm();
}
