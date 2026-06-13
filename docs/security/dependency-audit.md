# Dependency Vulnerability Audit

`ghostctl audit cargo|node|deps` scans a project's *locked* dependency graph
against the [OSV.dev](https://osv.dev) vulnerability database. Lockfiles are
parsed natively — no package-manager binary is invoked — so the audit works
offline up to the single OSV query, and ghostctl adds no new dependencies of its
own (it stays clean under `cargo audit`).

## Quick Commands

```bash
ghostctl audit cargo            # Audit Cargo.lock in the current directory (crates.io)
ghostctl audit node             # Audit the Node lockfile (bun/pnpm/yarn/npm)
ghostctl audit deps             # Auto-detect cargo + node lockfiles and audit both
ghostctl audit cargo ./path     # Audit a project elsewhere
ghostctl audit node --json      # Machine-readable output (exit 1 on High/Critical)
```

The optional positional argument is the project directory (default: the current
directory). ghostctl walks up from there to find the lockfile, so running it from
a subdirectory works.

## Supported Lockfiles

| Lockfile | Ecosystem | Notes |
|----------|-----------|-------|
| `Cargo.lock` | crates.io | TOML `[[package]]` entries; RustSec advisories surface via OSV |
| `package-lock.json` | npm | lockfileVersion 1, 2, and 3 |
| `yarn.lock` | npm | classic and berry formats |
| `pnpm-lock.yaml` | npm | v5 (`/name/ver`) and v6+ (`/name@ver`) key forms |
| `bun.lock` | npm | JSONC text format |
| `npm-shrinkwrap.json` | npm | same parser as `package-lock.json` |

For `audit node`, detection prefers `bun.lock`, then `pnpm-lock.yaml`, then
`yarn.lock`, then `package-lock.json` — mirroring what a developer expects when
several are present.

## How It Works

1. The lockfile is parsed into a flat set of `{ecosystem, name, version}`
   records (duplicates removed).
2. ghostctl POSTs the batch to OSV's `querybatch` endpoint, then fetches the
   details for each advisory id.
3. Each finding is rendered with its severity (derived from the CVSS v3.1 vector
   when the advisory provides one), the first fixed version, and the advisory
   URL, sorted most-severe first.

## Output and Exit Codes

The default output is a human-readable report. `--json` emits the findings as a
JSON array for CI pipelines.

In both modes, the command exits non-zero when any **High** or **Critical**
finding is present and the run is non-interactive (or `--json` is set), so it can
gate a build. An interactive terminal run is informational and returns success.

## Examples

```bash
# Fail a CI job on a high-severity transitive dependency
ghostctl audit deps --json > audit.json || echo "vulnerabilities found"

# Audit a Bun project in another checkout
ghostctl audit node ~/src/webapp
```

## See Also

- [CI/CD Workflow Audit](ci-workflow-audit.md) — `ghostctl audit ci`
- [Package Security Audit](package-audit.md) — Arch/AUR package auditing
- [JavaScript toolchain](../development/javascript.md) — `ghostctl dev js doctor`
