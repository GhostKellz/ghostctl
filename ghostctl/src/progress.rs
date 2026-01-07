//! Progress Indicators Module
//!
//! Provides spinners, progress bars, and multi-progress displays for long operations.

use crate::utils::is_plain_mode;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

/// Spinner for indeterminate progress
pub struct Spinner {
    bar: ProgressBar,
}

impl Spinner {
    /// Create a new spinner with a message
    pub fn new(message: &str) -> Self {
        let bar = ProgressBar::new_spinner();

        if is_plain_mode() {
            bar.set_style(
                ProgressStyle::default_spinner()
                    .template("{msg} [{elapsed}]")
                    .unwrap(),
            );
        } else {
            bar.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                    .template("{spinner:.cyan} {msg} [{elapsed_precise}]")
                    .unwrap(),
            );
        }

        bar.set_message(message.to_string());
        bar.enable_steady_tick(Duration::from_millis(100));

        Self { bar }
    }

    /// Update the spinner message
    pub fn set_message(&self, message: &str) {
        self.bar.set_message(message.to_string());
    }

    /// Finish with success
    pub fn finish_with_success(&self, message: &str) {
        if is_plain_mode() {
            self.bar.finish_with_message(format!("[OK] {}", message));
        } else {
            self.bar.finish_with_message(format!("✅ {}", message));
        }
    }

    /// Finish with error
    pub fn finish_with_error(&self, message: &str) {
        if is_plain_mode() {
            self.bar.finish_with_message(format!("[ERROR] {}", message));
        } else {
            self.bar.finish_with_message(format!("❌ {}", message));
        }
    }

    /// Finish and clear
    pub fn finish(&self) {
        self.bar.finish_and_clear();
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if !self.bar.is_finished() {
            self.bar.finish_and_clear();
        }
    }
}

/// Progress bar for determinate progress
pub struct Progress {
    bar: ProgressBar,
}

impl Progress {
    /// Create a new progress bar
    pub fn new(total: u64, message: &str) -> Self {
        let bar = ProgressBar::new(total);

        if is_plain_mode() {
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{msg} [{bar:40}] {pos}/{len} ({percent}%)")
                    .unwrap()
                    .progress_chars("=>-"),
            );
        } else {
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{msg}\n{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) [{elapsed_precise}]")
                    .unwrap()
                    .progress_chars("█▓▒░"),
            );
        }

        bar.set_message(message.to_string());

        Self { bar }
    }

    /// Increment progress by 1
    pub fn inc(&self) {
        self.bar.inc(1);
    }

    /// Increment progress by n
    pub fn inc_by(&self, n: u64) {
        self.bar.inc(n);
    }

    /// Set current position
    pub fn set_position(&self, pos: u64) {
        self.bar.set_position(pos);
    }

    /// Update message
    pub fn set_message(&self, message: &str) {
        self.bar.set_message(message.to_string());
    }

    /// Finish with success
    pub fn finish_with_success(&self, message: &str) {
        if is_plain_mode() {
            self.bar.finish_with_message(format!("[OK] {}", message));
        } else {
            self.bar.finish_with_message(format!("✅ {}", message));
        }
    }

    /// Finish with error
    pub fn finish_with_error(&self, message: &str) {
        if is_plain_mode() {
            self.bar.finish_with_message(format!("[ERROR] {}", message));
        } else {
            self.bar.finish_with_message(format!("❌ {}", message));
        }
    }

    /// Finish and clear
    pub fn finish(&self) {
        self.bar.finish_and_clear();
    }
}

impl Drop for Progress {
    fn drop(&mut self) {
        if !self.bar.is_finished() {
            self.bar.finish_and_clear();
        }
    }
}

/// Multi-progress container for parallel operations
pub struct MultiProgressContainer {
    multi: MultiProgress,
}

impl MultiProgressContainer {
    pub fn new() -> Self {
        Self {
            multi: MultiProgress::new(),
        }
    }

    /// Add a new progress bar
    pub fn add_progress(&self, total: u64, message: &str) -> Progress {
        let bar = self.multi.add(ProgressBar::new(total));

        if is_plain_mode() {
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{msg} [{bar:30}] {pos}/{len}")
                    .unwrap()
                    .progress_chars("=>-"),
            );
        } else {
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{msg}\n  [{bar:30.cyan/blue}] {pos}/{len}")
                    .unwrap()
                    .progress_chars("█▓▒░"),
            );
        }

        bar.set_message(message.to_string());

        Progress { bar }
    }

    /// Add a spinner
    pub fn add_spinner(&self, message: &str) -> Spinner {
        let bar = self.multi.add(ProgressBar::new_spinner());

        if is_plain_mode() {
            bar.set_style(ProgressStyle::default_spinner().template("{msg}").unwrap());
        } else {
            bar.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                    .template("{spinner:.cyan} {msg}")
                    .unwrap(),
            );
        }

        bar.set_message(message.to_string());
        bar.enable_steady_tick(Duration::from_millis(100));

        Spinner { bar }
    }
}

impl Default for MultiProgressContainer {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to run a task with a spinner
pub fn with_spinner<T, F>(message: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let spinner = Spinner::new(message);
    let result = f();
    spinner.finish();
    result
}

/// Helper function to run a task with a spinner and success/error message
pub fn with_spinner_result<T, E, F>(message: &str, f: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E>,
    E: std::fmt::Display,
{
    let spinner = Spinner::new(message);
    match f() {
        Ok(result) => {
            spinner.finish_with_success("Done");
            Ok(result)
        }
        Err(e) => {
            spinner.finish_with_error(&format!("Failed: {}", e));
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_creation() {
        let spinner = Spinner::new("Testing...");
        spinner.set_message("Updated message");
        spinner.finish();
    }

    #[test]
    fn test_progress_creation() {
        let progress = Progress::new(100, "Processing...");
        progress.inc();
        progress.inc_by(10);
        progress.set_position(50);
        progress.finish();
    }
}
