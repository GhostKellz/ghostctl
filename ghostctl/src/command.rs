//! Command Runner Module
//!
//! Provides a trait-based abstraction for executing system commands,
//! enabling unit testing without actual system side effects.

use std::collections::HashMap;
use std::io;
use std::process::{Command, ExitStatus, Output, Stdio};
use std::sync::{Arc, Mutex};

/// Result of a command execution
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

impl CommandResult {
    pub fn from_output(output: &Output) -> Self {
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

    /// Create a successful result
    pub fn ok(stdout: impl Into<String>) -> Self {
        Self {
            success: true,
            stdout: stdout.into(),
            stderr: String::new(),
            exit_code: Some(0),
        }
    }

    /// Create a failure result
    pub fn err(stderr: impl Into<String>, code: i32) -> Self {
        Self {
            success: false,
            stdout: String::new(),
            stderr: stderr.into(),
            exit_code: Some(code),
        }
    }
}

/// Trait for command execution - mockable for testing
pub trait CommandRunner: Send + Sync {
    /// Execute a command and capture output
    fn run(&self, cmd: &str, args: &[&str]) -> io::Result<CommandResult>;

    /// Execute a command with sudo (or directly if root)
    fn run_sudo(&self, cmd: &str, args: &[&str]) -> io::Result<CommandResult>;

    /// Execute a command interactively (inherits stdio)
    fn run_interactive(&self, cmd: &str, args: &[&str]) -> io::Result<CommandResult>;

    /// Execute a command with sudo interactively
    fn run_sudo_interactive(&self, cmd: &str, args: &[&str]) -> io::Result<CommandResult>;

    /// Execute a shell command
    fn run_shell(&self, shell_cmd: &str) -> io::Result<CommandResult>;

    /// Execute a shell command with sudo
    fn run_sudo_shell(&self, shell_cmd: &str) -> io::Result<CommandResult>;

    /// Check if a command exists
    fn command_exists(&self, cmd: &str) -> bool;

    /// Check if running as root
    fn is_root(&self) -> bool;

    /// Check if sudo is available
    fn has_sudo(&self) -> bool;

    /// Read a file (with sudo fallback if needed)
    fn read_file(&self, path: &str) -> io::Result<String>;

    /// Write to a file (with sudo fallback if needed)
    fn write_file(&self, path: &str, content: &str) -> io::Result<()>;

    /// Check if a file exists
    fn file_exists(&self, path: &str) -> bool;
}

/// System command runner - executes real commands
pub struct SystemRunner {
    dry_run: bool,
}

impl SystemRunner {
    pub fn new() -> Self {
        Self { dry_run: false }
    }

    pub fn with_dry_run(dry_run: bool) -> Self {
        Self { dry_run }
    }

    fn is_root_impl(&self) -> bool {
        unsafe { libc::geteuid() == 0 }
    }

    fn has_sudo_impl(&self) -> bool {
        Command::new("which")
            .arg("sudo")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

impl Default for SystemRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRunner for SystemRunner {
    fn run(&self, cmd: &str, args: &[&str]) -> io::Result<CommandResult> {
        if self.dry_run {
            return Ok(CommandResult::ok(format!(
                "[DRY RUN] Would execute: {} {}",
                cmd,
                args.join(" ")
            )));
        }

        let output = Command::new(cmd).args(args).output()?;
        Ok(CommandResult::from_output(&output))
    }

    fn run_sudo(&self, cmd: &str, args: &[&str]) -> io::Result<CommandResult> {
        if self.dry_run {
            return Ok(CommandResult::ok(format!(
                "[DRY RUN] Would execute: sudo {} {}",
                cmd,
                args.join(" ")
            )));
        }

        let output = if self.is_root_impl() {
            Command::new(cmd).args(args).output()?
        } else {
            Command::new("sudo").arg(cmd).args(args).output()?
        };
        Ok(CommandResult::from_output(&output))
    }

    fn run_interactive(&self, cmd: &str, args: &[&str]) -> io::Result<CommandResult> {
        if self.dry_run {
            return Ok(CommandResult::ok(format!(
                "[DRY RUN] Would execute: {} {}",
                cmd,
                args.join(" ")
            )));
        }

        let status = Command::new(cmd)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;
        Ok(CommandResult::from_status(status))
    }

    fn run_sudo_interactive(&self, cmd: &str, args: &[&str]) -> io::Result<CommandResult> {
        if self.dry_run {
            return Ok(CommandResult::ok(format!(
                "[DRY RUN] Would execute: sudo {} {}",
                cmd,
                args.join(" ")
            )));
        }

        let status = if self.is_root_impl() {
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
        Ok(CommandResult::from_status(status))
    }

    fn run_shell(&self, shell_cmd: &str) -> io::Result<CommandResult> {
        if self.dry_run {
            return Ok(CommandResult::ok(format!(
                "[DRY RUN] Would execute: bash -c '{}'",
                shell_cmd
            )));
        }

        let output = Command::new("bash").arg("-c").arg(shell_cmd).output()?;
        Ok(CommandResult::from_output(&output))
    }

    fn run_sudo_shell(&self, shell_cmd: &str) -> io::Result<CommandResult> {
        if self.dry_run {
            return Ok(CommandResult::ok(format!(
                "[DRY RUN] Would execute: sudo bash -c '{}'",
                shell_cmd
            )));
        }

        let output = if self.is_root_impl() {
            Command::new("bash").arg("-c").arg(shell_cmd).output()?
        } else {
            Command::new("sudo")
                .args(["bash", "-c", shell_cmd])
                .output()?
        };
        Ok(CommandResult::from_output(&output))
    }

    fn command_exists(&self, cmd: &str) -> bool {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn is_root(&self) -> bool {
        self.is_root_impl()
    }

    fn has_sudo(&self) -> bool {
        self.has_sudo_impl()
    }

    fn read_file(&self, path: &str) -> io::Result<String> {
        // Try normal read first
        if let Ok(content) = std::fs::read_to_string(path) {
            return Ok(content);
        }

        // Fall back to sudo
        let result = self.run_sudo("cat", &[path])?;
        if result.success {
            Ok(result.stdout)
        } else {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                result.stderr,
            ))
        }
    }

    fn write_file(&self, path: &str, content: &str) -> io::Result<()> {
        if self.dry_run {
            return Ok(());
        }

        // Try normal write first
        if std::fs::write(path, content).is_ok() {
            return Ok(());
        }

        // Fall back to sudo with tee
        let mut child = if self.is_root_impl() {
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
                "Failed to write file",
            ))
        }
    }

    fn file_exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }
}

/// Mock command runner for testing
#[derive(Default)]
pub struct MockRunner {
    /// Map of command+args -> result
    responses: Arc<Mutex<HashMap<String, CommandResult>>>,
    /// Record of all commands executed
    history: Arc<Mutex<Vec<String>>>,
    /// Default response for unmatched commands
    default_response: Arc<Mutex<Option<CommandResult>>>,
    /// Mock file system
    files: Arc<Mutex<HashMap<String, String>>>,
    /// Whether to simulate running as root
    is_root: bool,
}

impl MockRunner {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a mock that runs as root
    pub fn as_root() -> Self {
        Self {
            is_root: true,
            ..Self::default()
        }
    }

    /// Add a mock response for a command
    pub fn mock_command(&self, cmd: &str, args: &[&str], result: CommandResult) -> &Self {
        let key = format!("{} {}", cmd, args.join(" "));
        self.responses.lock().unwrap().insert(key, result);
        self
    }

    /// Add a mock response for a shell command
    pub fn mock_shell(&self, shell_cmd: &str, result: CommandResult) -> &Self {
        let key = format!("bash -c {}", shell_cmd);
        self.responses.lock().unwrap().insert(key, result);
        self
    }

    /// Set default response for unmatched commands
    pub fn set_default(&self, result: CommandResult) -> &Self {
        *self.default_response.lock().unwrap() = Some(result);
        self
    }

    /// Add a mock file
    pub fn mock_file(&self, path: &str, content: &str) -> &Self {
        self.files
            .lock()
            .unwrap()
            .insert(path.to_string(), content.to_string());
        self
    }

    /// Get command history
    pub fn get_history(&self) -> Vec<String> {
        self.history.lock().unwrap().clone()
    }

    /// Clear command history
    pub fn clear_history(&self) {
        self.history.lock().unwrap().clear();
    }

    /// Check if a command was executed
    pub fn was_called(&self, cmd: &str) -> bool {
        self.history.lock().unwrap().iter().any(|h| h.contains(cmd))
    }

    /// Get the number of times a command was called
    pub fn call_count(&self, cmd: &str) -> usize {
        self.history
            .lock()
            .unwrap()
            .iter()
            .filter(|h| h.contains(cmd))
            .count()
    }

    fn record_and_respond(&self, key: &str) -> io::Result<CommandResult> {
        self.history.lock().unwrap().push(key.to_string());

        let responses = self.responses.lock().unwrap();
        if let Some(result) = responses.get(key) {
            return Ok(result.clone());
        }

        // Try partial match
        for (pattern, result) in responses.iter() {
            if key.starts_with(pattern) || key.contains(pattern) {
                return Ok(result.clone());
            }
        }

        // Return default if set
        if let Some(default) = self.default_response.lock().unwrap().as_ref() {
            return Ok(default.clone());
        }

        // Default success if no mock configured
        Ok(CommandResult::ok(""))
    }
}

impl CommandRunner for MockRunner {
    fn run(&self, cmd: &str, args: &[&str]) -> io::Result<CommandResult> {
        let key = format!("{} {}", cmd, args.join(" "));
        self.record_and_respond(&key)
    }

    fn run_sudo(&self, cmd: &str, args: &[&str]) -> io::Result<CommandResult> {
        let key = if self.is_root {
            format!("{} {}", cmd, args.join(" "))
        } else {
            format!("sudo {} {}", cmd, args.join(" "))
        };
        self.record_and_respond(&key)
    }

    fn run_interactive(&self, cmd: &str, args: &[&str]) -> io::Result<CommandResult> {
        let key = format!("{} {}", cmd, args.join(" "));
        self.record_and_respond(&key)
    }

    fn run_sudo_interactive(&self, cmd: &str, args: &[&str]) -> io::Result<CommandResult> {
        let key = if self.is_root {
            format!("{} {}", cmd, args.join(" "))
        } else {
            format!("sudo {} {}", cmd, args.join(" "))
        };
        self.record_and_respond(&key)
    }

    fn run_shell(&self, shell_cmd: &str) -> io::Result<CommandResult> {
        let key = format!("bash -c {}", shell_cmd);
        self.record_and_respond(&key)
    }

    fn run_sudo_shell(&self, shell_cmd: &str) -> io::Result<CommandResult> {
        let key = if self.is_root {
            format!("bash -c {}", shell_cmd)
        } else {
            format!("sudo bash -c {}", shell_cmd)
        };
        self.record_and_respond(&key)
    }

    fn command_exists(&self, cmd: &str) -> bool {
        // Check if we have a mock response for this command
        let responses = self.responses.lock().unwrap();
        responses.keys().any(|k| k.starts_with(cmd))
    }

    fn is_root(&self) -> bool {
        self.is_root
    }

    fn has_sudo(&self) -> bool {
        true // Mock always has sudo
    }

    fn read_file(&self, path: &str) -> io::Result<String> {
        self.files
            .lock()
            .unwrap()
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found in mock"))
    }

    fn write_file(&self, path: &str, content: &str) -> io::Result<()> {
        self.files
            .lock()
            .unwrap()
            .insert(path.to_string(), content.to_string());
        Ok(())
    }

    fn file_exists(&self, path: &str) -> bool {
        self.files.lock().unwrap().contains_key(path)
    }
}

/// Global runner instance - can be swapped for testing
static RUNNER: std::sync::OnceLock<Arc<dyn CommandRunner>> = std::sync::OnceLock::new();

/// Get the global command runner
pub fn runner() -> Arc<dyn CommandRunner> {
    RUNNER.get_or_init(|| Arc::new(SystemRunner::new())).clone()
}

/// Set a custom runner (for testing)
/// Note: This can only be called once per process
#[cfg(test)]
pub fn set_runner(r: Arc<dyn CommandRunner>) {
    let _ = RUNNER.set(r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_runner_basic() {
        let mock = MockRunner::new();
        mock.mock_command("echo", &["hello"], CommandResult::ok("hello"));

        let result = mock.run("echo", &["hello"]).unwrap();
        assert!(result.success);
        assert_eq!(result.stdout, "hello");
    }

    #[test]
    fn test_mock_runner_history() {
        let mock = MockRunner::new();

        let _ = mock.run("cmd1", &[]).unwrap();
        let _ = mock.run("cmd2", &["arg1"]).unwrap();

        assert!(mock.was_called("cmd1"));
        assert!(mock.was_called("cmd2"));
        assert!(!mock.was_called("cmd3"));
    }

    #[test]
    fn test_mock_runner_files() {
        let mock = MockRunner::new();
        mock.mock_file("/etc/test", "content");

        assert!(mock.file_exists("/etc/test"));
        assert!(!mock.file_exists("/etc/other"));

        let content = mock.read_file("/etc/test").unwrap();
        assert_eq!(content, "content");
    }

    #[test]
    fn test_mock_runner_sudo() {
        let mock = MockRunner::new();
        mock.mock_command("pacman", &["-Syu"], CommandResult::ok("updated"));

        let result = mock.run_sudo("pacman", &["-Syu"]).unwrap();
        assert!(result.success);
        assert!(mock.was_called("sudo pacman"));
    }

    #[test]
    fn test_mock_runner_as_root() {
        let mock = MockRunner::as_root();
        mock.mock_command("pacman", &["-Syu"], CommandResult::ok("updated"));

        let result = mock.run_sudo("pacman", &["-Syu"]).unwrap();
        assert!(result.success);
        // As root, should not prepend sudo
        assert!(mock.was_called("pacman -Syu"));
        assert!(!mock.was_called("sudo"));
    }

    #[test]
    fn test_command_result_helpers() {
        let ok = CommandResult::ok("success");
        assert!(ok.success);
        assert_eq!(ok.exit_code, Some(0));

        let err = CommandResult::err("failed", 1);
        assert!(!err.success);
        assert_eq!(err.exit_code, Some(1));
    }
}
