//! IOC (indicator-of-compromise) feed matching for `audit ioc` (pure, testable).
//!
//! Supply-chain campaigns (e.g. the June 2026 AUR `atomic-lockfile`/`js-digest`
//! wave) publish lists of backdoored package names. This module cross-references
//! such a feed against what is *currently installed* and against the historical
//! `pacman.log`, so a package that was installed and later removed is still
//! surfaced. The feed itself is supplied externally - no campaign-specific data
//! is hardcoded here, so the logic does not go stale.

use std::collections::BTreeSet;

/// A historical install/upgrade/reinstall event for a flagged package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogHit {
    pub package: String,
    pub action: String,
    pub date: String,
}

/// Parse a feed into a set of package names: one per line, `#` comments and
/// blank lines ignored, surrounding whitespace trimmed.
pub fn parse_feed(text: &str) -> BTreeSet<String> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        // Tolerate "name version" or "name # note" lines: take the first field.
        .filter_map(|l| l.split_whitespace().next())
        .map(str::to_string)
        .collect()
}

/// Scan pacman log content for install/upgrade/reinstall events of any flagged
/// package. Recognises lines like:
///   `[2026-06-10T12:00:00+0000] [ALPM] installed foo (1.0-1)`
pub fn scan_pacman_log(text: &str, flagged: &BTreeSet<String>) -> Vec<LogHit> {
    let mut hits = Vec::new();
    for line in text.lines() {
        let Some(date) = line
            .strip_prefix('[')
            .and_then(|r| r.split(['T', ']']).next())
        else {
            continue;
        };
        let Some(idx) = line.find("[ALPM] ") else {
            continue;
        };
        let mut fields = line[idx + "[ALPM] ".len()..].split_whitespace();
        let Some(action) = fields.next() else {
            continue;
        };
        if !matches!(action, "installed" | "upgraded" | "reinstalled") {
            continue;
        }
        let Some(pkg) = fields.next() else {
            continue;
        };
        if flagged.contains(pkg) {
            hits.push(LogHit {
                package: pkg.to_string(),
                action: action.to_string(),
                date: date.to_string(),
            });
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_feed_skips_comments_and_blanks() {
        let feed = "# campaign feed\natomic-lockfile\n\n  js-digest  \nfoo 1.2.3\n";
        let set = parse_feed(feed);
        assert!(set.contains("atomic-lockfile"));
        assert!(set.contains("js-digest"));
        assert!(set.contains("foo"));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn scan_log_matches_flagged_install_events() {
        let flagged: BTreeSet<String> =
            ["atomic-lockfile".to_string(), "js-digest".to_string()].into();
        let log = "\
[2026-06-10T12:00:00+0000] [ALPM] installed atomic-lockfile (1.4.2-1)
[2026-06-10T12:01:00+0000] [ALPM] installed firefox (126.0-1)
[2026-06-11T09:00:00+0000] [ALPM] upgraded js-digest (1.0-1 -> 1.1-1)
[2026-06-12T09:00:00+0000] [ALPM] removed atomic-lockfile (1.4.2-1)
";
        let hits = scan_pacman_log(log, &flagged);
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].package, "atomic-lockfile");
        assert_eq!(hits[0].action, "installed");
        assert_eq!(hits[0].date, "2026-06-10");
        assert_eq!(hits[1].package, "js-digest");
        assert_eq!(hits[1].action, "upgraded");
    }

    #[test]
    fn scan_log_ignores_unflagged_and_removals() {
        let flagged: BTreeSet<String> = ["evil".to_string()].into();
        let log = "\
[2026-06-10T12:00:00+0000] [ALPM] installed good (1.0-1)
[2026-06-10T12:00:00+0000] [ALPM] removed evil (1.0-1)
";
        assert!(scan_pacman_log(log, &flagged).is_empty());
    }
}
