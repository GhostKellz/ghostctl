use std::process::{Command, Output};
use std::io;

pub fn run_command(command: &str, args: &[&str]) -> io::Result<Output> {
    Command::new(command)
        .args(args)
        .output()
}

pub fn run_command_with_status(command: &str, args: &[&str]) -> io::Result<bool> {
    let status = Command::new(command)
        .args(args)
        .status()?;
    Ok(status.success())
}

pub fn run_bash_command(command: &str) -> io::Result<Output> {
    Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
}

pub fn check_command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn get_system_info() -> SystemInfo {
    let sysinfo = sysinfo::System::new_all();
    
    SystemInfo {
        total_memory: sysinfo.total_memory(),
        available_memory: sysinfo.available_memory(),
        cpu_count: num_cpus::get(),
        hostname: gethostname::gethostname().to_string_lossy().to_string(),
        kernel_version: sysinfo::System::kernel_version().unwrap_or_else(|| "unknown".to_string()),
        os_version: sysinfo::System::os_version().unwrap_or_else(|| "unknown".to_string()),
    }
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub total_memory: u64,
    pub available_memory: u64,
    pub cpu_count: usize,
    pub hostname: String,
    pub kernel_version: String,
    pub os_version: String,
}