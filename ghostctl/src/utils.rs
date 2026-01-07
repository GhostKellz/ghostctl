use std::io;
use std::process::{Command, ExitStatus, Output, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};

// Global state for headless mode
static HEADLESS_MODE: AtomicBool = AtomicBool::new(false);
static DRY_RUN_MODE: AtomicBool = AtomicBool::new(false);
static PLAIN_MODE: AtomicBool = AtomicBool::new(false);

/// Enable headless (non-interactive) mode
pub fn set_headless_mode(enabled: bool) {
    HEADLESS_MODE.store(enabled, Ordering::SeqCst);
}

/// Check if running in headless mode
pub fn is_headless() -> bool {
    HEADLESS_MODE.load(Ordering::SeqCst)
        || std::env::var("GHOSTCTL_HEADLESS").is_ok()
        || std::env::var("CI").is_ok()
}

/// Enable dry-run mode (no actual changes)
pub fn set_dry_run_mode(enabled: bool) {
    DRY_RUN_MODE.store(enabled, Ordering::SeqCst);
}

/// Check if running in dry-run mode
pub fn is_dry_run() -> bool {
    DRY_RUN_MODE.load(Ordering::SeqCst) || std::env::var("GHOSTCTL_DRY_RUN").is_ok()
}

/// Enable plain output mode (no emojis/colors)
pub fn set_plain_mode(enabled: bool) {
    PLAIN_MODE.store(enabled, Ordering::SeqCst);
}

/// Check if running in plain output mode
pub fn is_plain_mode() -> bool {
    PLAIN_MODE.load(Ordering::SeqCst)
        || std::env::var("GHOSTCTL_PLAIN").is_ok()
        || std::env::var("NO_COLOR").is_ok()
}

// ============================================================================
// Sudo Helper - Centralized Privilege Escalation
// ============================================================================

/// Result of a privileged command execution
#[derive(Debug)]
pub struct SudoResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

impl SudoResult {
    pub fn from_output(output: Output) -> Self {
        Self {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
        }
    }

    pub fn from_status(status: ExitStatus) -> Self {
        Self {
            success: status.success(),
            stdout: String::new(),
            stderr: String::new(),
            exit_code: status.code(),
        }
    }
}

/// Check if we're running as root
pub fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

/// Check if sudo is available
pub fn has_sudo() -> bool {
    Command::new("which")
        .arg("sudo")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Check if user has cached sudo credentials (won't prompt)
pub fn has_sudo_cached() -> bool {
    Command::new("sudo")
        .args(["-n", "true"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Ensure we can run privileged commands
/// Returns true if we're root or have sudo access
pub fn ensure_privileges() -> bool {
    if is_root() {
        return true;
    }

    if !has_sudo() {
        eprintln!("❌ sudo is not installed and not running as root");
        return false;
    }

    // Try to get sudo credentials if not cached
    if !has_sudo_cached() {
        println!("🔐 This operation requires elevated privileges.");
        let status = Command::new("sudo")
            .arg("-v")
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if !status {
            eprintln!("❌ Failed to obtain sudo credentials");
            return false;
        }
    }

    true
}

/// Run a command with sudo (or directly if already root)
pub fn sudo_run(cmd: &str, args: &[&str]) -> io::Result<SudoResult> {
    if is_dry_run() {
        println!("[DRY RUN] Would execute: sudo {} {}", cmd, args.join(" "));
        return Ok(SudoResult {
            success: true,
            stdout: String::new(),
            stderr: String::new(),
            exit_code: Some(0),
        });
    }

    let output = if is_root() {
        Command::new(cmd).args(args).output()?
    } else {
        Command::new("sudo").arg(cmd).args(args).output()?
    };

    Ok(SudoResult::from_output(output))
}

/// Run a command with sudo and show output in real-time
pub fn sudo_run_interactive(cmd: &str, args: &[&str]) -> io::Result<SudoResult> {
    if is_dry_run() {
        println!("[DRY RUN] Would execute: sudo {} {}", cmd, args.join(" "));
        return Ok(SudoResult {
            success: true,
            stdout: String::new(),
            stderr: String::new(),
            exit_code: Some(0),
        });
    }

    let status = if is_root() {
        Command::new(cmd)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
    } else {
        Command::new("sudo")
            .arg(cmd)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
    };

    Ok(SudoResult::from_status(status))
}

/// Run a shell command with sudo
pub fn sudo_shell(shell_cmd: &str) -> io::Result<SudoResult> {
    if is_dry_run() {
        println!("[DRY RUN] Would execute: sudo bash -c '{}'", shell_cmd);
        return Ok(SudoResult {
            success: true,
            stdout: String::new(),
            stderr: String::new(),
            exit_code: Some(0),
        });
    }

    let output = if is_root() {
        Command::new("bash").arg("-c").arg(shell_cmd).output()?
    } else {
        Command::new("sudo")
            .args(["bash", "-c", shell_cmd])
            .output()?
    };

    Ok(SudoResult::from_output(output))
}

/// Check if a file exists (with sudo if needed for permission)
pub fn sudo_file_exists(path: &str) -> bool {
    if std::path::Path::new(path).exists() {
        return true;
    }

    // Try with sudo for permission-restricted paths
    sudo_run("test", &["-e", path])
        .map(|r| r.success)
        .unwrap_or(false)
}

/// Read a file that may require root permissions
pub fn sudo_read_file(path: &str) -> io::Result<String> {
    // Try normal read first
    if let Ok(content) = std::fs::read_to_string(path) {
        return Ok(content);
    }

    // Fall back to sudo
    let result = sudo_run("cat", &[path])?;
    if result.success {
        Ok(result.stdout)
    } else {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            result.stderr,
        ))
    }
}

/// Write to a file that may require root permissions
pub fn sudo_write_file(path: &str, content: &str) -> io::Result<()> {
    if is_dry_run() {
        println!("[DRY RUN] Would write to: {}", path);
        return Ok(());
    }

    // Try normal write first
    if std::fs::write(path, content).is_ok() {
        return Ok(());
    }

    // Fall back to sudo with tee
    let mut child = if is_root() {
        Command::new("tee")
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?
    } else {
        Command::new("sudo")
            .args(["tee", path])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?
    };

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin.write_all(content.as_bytes())?;
    }

    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Failed to write file with sudo",
        ))
    }
}

/// Append to a file that may require root permissions
pub fn sudo_append_file(path: &str, content: &str) -> io::Result<()> {
    if is_dry_run() {
        println!("[DRY RUN] Would append to: {}", path);
        return Ok(());
    }

    let shell_cmd = format!(
        "echo '{}' | tee -a {}",
        content.replace('\'', "'\\''"),
        path
    );
    let result = sudo_shell(&shell_cmd)?;

    if result.success {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            result.stderr,
        ))
    }
}

/// Create a directory with sudo if needed
pub fn sudo_mkdir(path: &str) -> io::Result<()> {
    if is_dry_run() {
        println!("[DRY RUN] Would create directory: {}", path);
        return Ok(());
    }

    // Try normal mkdir first
    if std::fs::create_dir_all(path).is_ok() {
        return Ok(());
    }

    // Fall back to sudo
    let result = sudo_run("mkdir", &["-p", path])?;
    if result.success {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            result.stderr,
        ))
    }
}

/// Remove a file with sudo if needed
pub fn sudo_rm(path: &str) -> io::Result<()> {
    if is_dry_run() {
        println!("[DRY RUN] Would remove: {}", path);
        return Ok(());
    }

    // Try normal remove first
    if std::fs::remove_file(path).is_ok() {
        return Ok(());
    }

    // Fall back to sudo
    let result = sudo_run("rm", &["-f", path])?;
    if result.success {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            result.stderr,
        ))
    }
}

/// Remove a directory recursively with sudo if needed
pub fn sudo_rm_rf(path: &str) -> io::Result<()> {
    if is_dry_run() {
        println!("[DRY RUN] Would remove directory: {}", path);
        return Ok(());
    }

    // Try normal remove first
    if std::fs::remove_dir_all(path).is_ok() {
        return Ok(());
    }

    // Fall back to sudo
    let result = sudo_run("rm", &["-rf", path])?;
    if result.success {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            result.stderr,
        ))
    }
}

/// Copy a file with sudo if needed
pub fn sudo_cp(src: &str, dst: &str) -> io::Result<()> {
    if is_dry_run() {
        println!("[DRY RUN] Would copy: {} -> {}", src, dst);
        return Ok(());
    }

    // Try normal copy first
    if std::fs::copy(src, dst).is_ok() {
        return Ok(());
    }

    // Fall back to sudo
    let result = sudo_run("cp", &["-a", src, dst])?;
    if result.success {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            result.stderr,
        ))
    }
}

/// Run systemctl command with sudo
pub fn sudo_systemctl(action: &str, service: &str) -> io::Result<SudoResult> {
    sudo_run("systemctl", &[action, service])
}

/// Run pacman command with sudo
pub fn sudo_pacman(args: &[&str]) -> io::Result<SudoResult> {
    sudo_run_interactive("pacman", args)
}

// ============================================================================
// Original functions
// ============================================================================

pub fn run_command(command: &str, args: &[&str]) -> io::Result<Output> {
    Command::new(command).args(args).output()
}

pub fn run_command_with_status(command: &str, args: &[&str]) -> io::Result<bool> {
    let status = Command::new(command).args(args).status()?;
    Ok(status.success())
}

pub fn run_bash_command(command: &str) -> io::Result<Output> {
    Command::new("bash").arg("-c").arg(command).output()
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
