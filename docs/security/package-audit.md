# Package Security Audit

Following the AUR supply-chain compromises (hundreds of backdoored packages) and
the malicious-PKGBUILD RCE cases, GhostCTL audits an Arch system three ways:
cross-referencing installed packages against the Arch Security Tracker,
heuristically scanning AUR/foreign PKGBUILDs *and their `.install` hooks* for the
remote-payload patterns those attacks rely on, and matching an external
indicator-of-compromise feed of suspect package names against the system.

## Quick Commands

```bash
ghostctl audit summary               # Installed / foreign / orphan counts
ghostctl audit cve                   # Installed packages vs. Arch Security Tracker
ghostctl audit aur                   # Scan installed AUR/foreign PKGBUILDs + .install hooks
ghostctl audit pkgbuild ./PKGBUILD   # Scan a local PKGBUILD (and its .install) file
ghostctl audit pkgbuild yay          # Fetch + scan an AUR package's PKGBUILD + .install
ghostctl audit ioc --feed FILE       # Match an IOC package-name feed (file or URL)
```

## CVE Checks

`audit cve` downloads the Arch Security Tracker advisory database and matches it
against `pacman -Q`. The installed version is compared to the fixed version with
`vercmp`, so only packages that are *actually still vulnerable* are reported,
sorted by severity, with the advisory (AVG) id and associated CVEs.

## PKGBUILD Scanning

The scanner flags suspicious patterns; it reports *suspicion, not proof* — a
human still reviews each finding.

| Severity | Rules |
|----------|-------|
| HIGH | download-pipe-to-shell, eval-command-substitution, base64-decode-exec, dev-tcp-socket, persistence-target, registry-install-js |
| MEDIUM | netcat-usage, hardcoded-ip, network-fetch, registry-install |
| LOW | sudo-in-build, chmod-executable |

`registry-install-js` flags pulling a *named* package from a JavaScript registry
during build/install (`npm install <pkg>`, `bun install <pkg>`, etc.) — the
vector used by the June 2026 `atomic-lockfile` / `js-digest` AUR campaign. A bare
`npm install` (installing declared dependencies) is not flagged. `registry-install`
covers the same pattern for `pip`/`cargo`/`go`/`gem`.

`audit aur` inventories foreign packages with `pacman -Qm`, fetches each PKGBUILD
*and its `.install` hook* (which runs as root via pacman) from the AUR, and scans
both. `audit pkgbuild <target>` scans a single local path (and its sibling
`.install`) or fetches a named AUR package.

## IOC Feed Matching

`audit ioc` cross-references an external feed of suspect package names against:

1. **Currently installed** foreign packages (`pacman -Qm`), reporting the install date.
2. **Historical** `pacman.log` events (install/upgrade/reinstall), so a package
   that was installed and later removed is still surfaced. Rotated and compressed
   logs (`.gz`, `.xz`, `.zst`, `.bz2`) are read automatically when the matching
   decompressor is available.

The feed is a plain list of package names (one per line, `#` comments allowed)
and is supplied by you — no campaign-specific data is baked into the binary, so
the detection logic does not go stale. Point it at a file or an http(s) URL:

```bash
ghostctl audit ioc --feed /etc/ghostctl/compromised.txt
ghostctl audit ioc --feed https://example.com/aur-iocs.txt
```

## Configuration

Settings live under `[audit]` in `config.toml` (`~/.config/ghostctl/config.toml`):
the Arch Security Tracker URL, the AUR base URL, the HTTP timeout, an optional
`ioc_feed` (path or URL), and the `pacman_log_glob` scanned by `audit ioc`. Run
`ghostctl config show` to see resolved values.

## Notes

- `cve`, `aur`, `ioc`, and `summary` require `pacman` (Arch-based systems).
- The CVE check and remote feeds/PKGBUILDs are network-bound.
- Heuristics are intentionally conservative; always read flagged files.
