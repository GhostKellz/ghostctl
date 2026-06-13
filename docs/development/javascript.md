# JavaScript / TypeScript Toolchain

`ghostctl dev js doctor` reports the state of a JavaScript/TypeScript development
environment: which runtimes and package managers are installed (with versions),
and which package manager a project uses based on its lockfile. It is read-only —
nothing is installed or modified.

## Quick Commands

```bash
ghostctl dev js doctor          # Inspect the current directory
ghostctl dev js doctor ./path   # Inspect a project elsewhere
```

## What It Checks

**Runtimes** — `node`, `bun`, `deno`. Each is reported with its version, or
marked missing.

**Package managers** — `npm`, `pnpm`, `yarn`, `bun`. Same per-tool version
reporting.

**Project** — whether `package.json` is present, and which lockfile (and
therefore package manager) the project uses. Lockfile detection reuses the same
logic as `audit node`, preferring `bun.lock`, then `pnpm-lock.yaml`, then
`yarn.lock`, then `package-lock.json`.

## Example

```text
JavaScript toolchain
────────────────────
Runtimes:
  ✓ node 24.10.0
  ✓ bun 1.3.11
  ✓ deno 2.8.2 (stable, release, x86_64-unknown-linux-gnu)

Package managers:
  ✓ npm 11.6.2
  ✓ pnpm 10.18.1
  ✓ yarn 1.22.22
  ✓ bun 1.3.11

Project (.):
  ✓ package.json present
  ✓ lockfile: bun.lock → bun project
    Audit dependencies with `ghostctl audit node`.
```

When a project lockfile is detected, the doctor points at
[`ghostctl audit node`](../security/dependency-audit.md) to scan its dependencies
for known vulnerabilities.

## See Also

- [Dependency Vulnerability Audit](../security/dependency-audit.md) — `ghostctl audit node`
