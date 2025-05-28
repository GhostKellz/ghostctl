pub mod dkms;
pub mod wayland;

pub fn optimize() {
    println!("ghostctl :: NVIDIA Optimizer");

    dkms::rebuild();
    wayland::configure();
}

