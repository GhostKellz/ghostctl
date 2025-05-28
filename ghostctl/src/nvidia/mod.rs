pub mod dkms;
pub mod wayland;

pub fn optimize() {
    println!("ghostctl :: NVIDIA Optimizer");

    dkms::rebuild();
    wayland::configure();
}

pub fn clean() {
    println!("ghostctl :: NVIDIA Clean DKMS/Modules");
    println!("- Removing old DKMS modules");
    println!("- Cleaning up NVIDIA driver artifacts");
}

pub fn fix() {
    println!("ghostctl :: NVIDIA Fix/Rebuild DKMS/Initramfs");
    println!("- Rebuilding DKMS modules");
    println!("- Regenerating initramfs");
}

