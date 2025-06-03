pub fn install_rust() {
    println!("Installing Rust toolchain with rustup...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
        .status();
    match status {
        Ok(s) if s.success() => println!("Rust installed successfully."),
        _ => println!("Failed to install Rust."),
    }
}

pub fn install_zig() {
    println!("Installing Zig (via pacman or download)...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -S --noconfirm zig || (curl -LO https://ziglang.org/download/latest/zig-linux-x86_64.tar.xz && tar -xf zig-*.tar.xz && sudo mv zig-* /opt/zig)")
        .status();
    match status {
        Ok(s) if s.success() => println!("Zig installed successfully."),
        _ => println!("Failed to install Zig."),
    }
}

pub fn install_python() {
    println!("Installing Python (via pacman)...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -S --noconfirm python python-pip")
        .status();
    match status {
        Ok(s) if s.success() => println!("Python installed successfully."),
        _ => println!("Failed to install Python."),
    }
}

#[allow(dead_code)]
pub fn setup_python_venv(venv_name: &str) {
    println!("Setting up Python venv: {}", venv_name);
    let status = std::process::Command::new("python")
        .args(["-m", "venv", venv_name])
        .status();
    match status {
        Ok(s) if s.success() => println!("Virtual environment '{}' created.", venv_name),
        _ => println!("Failed to create venv."),
    }
}

pub fn install_go() {
    println!("Installing Go (via pacman)...");
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("sudo pacman -S --noconfirm go")
        .status();
    match status {
        Ok(s) if s.success() => println!("Go installed successfully."),
        _ => println!("Failed to install Go."),
    }
}

pub fn stage(project: String) {
    match project.as_str() {
        "rust" => install_rust(),
        "zig" => install_zig(),
        "python" => install_python(),
        "go" => install_go(),
        _ => println!(
            "Unknown project type: {}. Supported: rust, zig, python, go",
            project
        ),
    }
}

pub mod gtools;
