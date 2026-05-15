# Support Bundles

GhostCTL support bundles collect local diagnostics that are useful when filing issues or debugging package, system, virtualization, networking, storage, and toolchain problems.

## Commands

```bash
ghostctl support paths
ghostctl support doctor
ghostctl support logs
ghostctl support bundle --redact-paths
ghostctl support bundle --redact-paths --gzip
ghostctl support bundle --redact-paths --tarball
```

By default, support files are written under the XDG state directory:

```text
~/.local/state/ghostctl/support/
~/.local/state/ghostctl/logs/history.log
```

If the system does not expose an XDG state directory, GhostCTL falls back to a local data or temporary state path.

## Bundle Contents

Current bundles include:

- GhostCTL version and creation timestamp
- state, support, and log paths
- basic OS and kernel information
- availability of common external tools used by GhostCTL modules
- recent GhostCTL activity log lines
- JSON metadata sidecar for issue triage and future automation

`ghostctl support doctor` is a quick readiness check for state path creation, write access, log paths, and visible external tools. It does not perform privileged or destructive checks.

Default bundle names include a timestamp to avoid overwriting previous diagnostics. Plain text output writes a `.txt` report plus a `.json` metadata sidecar. Gzip output writes compressed text plus a metadata sidecar. Tarball output writes a `.tar.gz` archive containing both `ghostctl-support.txt` and `ghostctl-support.json`.

## Redaction

Use `--redact-paths` before sharing a bundle publicly:

```bash
ghostctl support bundle --redact-paths
```

Redaction replaces the current home directory with `~` and masks common local identifiers including usernames, hostnames, IPv4 addresses, MAC addresses, PCI IDs, and simple serial/UUID-style key-value fields. Review bundles before attaching them to public issues, especially when command output may include service names or environment-specific values.

## Custom Output

```bash
ghostctl support bundle --redact-paths --output /tmp/ghostctl-support.txt
```

The command writes the text bundle plus a `.json` metadata sidecar next to it.

## Compressed Output

```bash
ghostctl support bundle --redact-paths --gzip
ghostctl support bundle --redact-paths --tarball
```

Use `--gzip` for compressed text output. Use `--tarball` when attaching a single archive is easier for issue reports.

## Issue Reports

For bugs, include:

- GhostCTL version
- install method
- distribution and kernel version
- exact command or menu path used
- relevant error output
- support bundle and metadata sidecar when possible
