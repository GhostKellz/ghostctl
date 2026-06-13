//! `ghostctl dev js` - JavaScript/TypeScript toolchain readiness checks.
//!
//! A `doctor` for Node/Bun/Deno projects: reports which runtimes and package
//! managers are installed (with versions), detects the project's package manager
//! from its lockfile (reusing the same detection as `audit node`), and points at
//! the matching `audit`/install next steps. Read-only — nothing is installed or
//! modified.

use anyhow::Result;
use std::path::Path;
use std::process::Command as ProcCommand;

use crate::audit::lockfile::detect_node_lockfile;
use crate::utils::is_plain_mode;

/// JS runtimes ghostctl knows how to report on.
const RUNTIMES: &[(&str, &str)] = &[
    ("node", "--version"),
    ("bun", "--version"),
    ("deno", "--version"),
];

/// Node-ecosystem package managers, in the order shown by the doctor.
const PACKAGE_MANAGERS: &[(&str, &str)] = &[
    ("npm", "--version"),
    ("pnpm", "--version"),
    ("yarn", "--version"),
    ("bun", "--version"),
];

pub fn doctor(dir: &Path) -> Result<()> {
    println!("JavaScript toolchain");
    println!("────────────────────");

    println!("Runtimes:");
    let mut any_runtime = false;
    for (bin, ver_arg) in RUNTIMES {
        match tool_version(bin, ver_arg) {
            Some(v) => {
                any_runtime = true;
                mark(true, &format!("{bin} {v}"));
            }
            None => mark(false, &format!("{bin} (not installed)")),
        }
    }
    if !any_runtime {
        println!(
            "    Install Node via `ghostctl dev` or your distro; Bun via `curl -fsSL https://bun.sh/install | bash`."
        );
    }

    println!("\nPackage managers:");
    for (bin, ver_arg) in PACKAGE_MANAGERS {
        match tool_version(bin, ver_arg) {
            Some(v) => mark(true, &format!("{bin} {v}")),
            None => mark(false, &format!("{bin} (not installed)")),
        }
    }

    println!("\nProject ({}):", dir.display());
    let pkg_json = dir.join("package.json");
    mark(pkg_json.is_file(), "package.json present");

    match detect_node_lockfile(dir) {
        Some((pm, path)) => {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("lockfile");
            mark(true, &format!("lockfile: {name} → {} project", pm.label()));
            println!("    Audit dependencies with `ghostctl audit node`.");
        }
        None => {
            mark(false, "lockfile (no bun/pnpm/yarn/npm lock found)");
            if pkg_json.is_file() {
                println!("    Run your package manager's install to generate a lockfile.");
            }
        }
    }

    Ok(())
}

/// Run `<bin> <ver_arg>` and return the cleaned version string, or `None` if the
/// binary is missing or fails. The first line is used and a leading `v` is
/// trimmed so `node` and `bun` print uniformly.
fn tool_version(bin: &str, ver_arg: &str) -> Option<String> {
    if which::which(bin).is_err() {
        return None;
    }
    let out = ProcCommand::new(bin).arg(ver_arg).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    Some(clean_version(bin, &text))
}

/// Normalize version output to a single short token: first non-empty line with a
/// leading `v` stripped (handles `v22.3.0`). `deno --version` prints
/// `deno 1.46.0 (...)`, so a redundant leading binary name is dropped to avoid
/// printing `deno deno 1.46.0`.
fn clean_version(bin: &str, raw: &str) -> String {
    let line = raw
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or("");
    // Drop a redundant leading binary name (`deno 2.8.2` -> `2.8.2`).
    let line = line.strip_prefix(bin).map_or(line, str::trim_start);
    // Drop a leading `v` (`v22.3.0` -> `22.3.0`).
    line.strip_prefix('v').unwrap_or(line).to_string()
}

fn mark(ok: bool, label: &str) {
    let icon = if is_plain_mode() {
        if ok { "[OK]" } else { "[--]" }
    } else if ok {
        "✓"
    } else {
        "✗"
    };
    println!("  {icon} {label}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_version_strips_v_prefix() {
        assert_eq!(clean_version("node", "v22.3.0\n"), "22.3.0");
    }

    #[test]
    fn test_clean_version_plain() {
        assert_eq!(clean_version("bun", "1.1.34\n"), "1.1.34");
    }

    #[test]
    fn test_clean_version_strips_redundant_bin_name() {
        assert_eq!(
            clean_version("deno", "deno 1.46.0 (release)\nv8 12.0\n"),
            "1.46.0 (release)"
        );
    }

    #[test]
    fn test_clean_version_skips_blank_lines() {
        assert_eq!(clean_version("npm", "\n\n  9.6.7  \n"), "9.6.7");
    }
}
