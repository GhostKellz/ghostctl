//! Thin wrapper around the Hermes agent CLI.
//!
//! ghostctl does not reimplement Hermes; it shells out to the installed `hermes`
//! binary so the agent's own UX (TUI, setup wizard, etc.) is preserved. We only
//! provide a convenient passthrough and a presence check.

use anyhow::{Result, bail};
use std::process::Command;

/// Run `hermes <args...>`, inheriting stdio so interactive flows work.
pub fn run(bin: &str, args: &[String]) -> Result<()> {
    if which::which(bin).is_err() {
        bail!(
            "hermes CLI '{}' not found in PATH. Install it or set [ai].hermes_bin in config.",
            bin
        );
    }

    let status = Command::new(bin)
        .args(args)
        .status()
        .map_err(|e| anyhow::anyhow!("failed to launch {bin}: {e}"))?;

    if !status.success() {
        if let Some(code) = status.code() {
            std::process::exit(code);
        }
        bail!("{bin} terminated by signal");
    }
    Ok(())
}
