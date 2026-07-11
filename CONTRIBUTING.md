# Contributing to GhostCTL

GhostCTL is a systems administration CLI, so changes should be practical,
auditable, and safe to run on real machines.

## Development Setup

```bash
cd ghostctl
cargo fmt --check
cargo test
cargo clippy --release -- -D warnings
cargo audit
```

Keep changes focused. Prefer small fixes with clear verification over broad
refactors. If a command touches packages, credentials, networking, VMs, or host
state, document the behavior and make dry-run or confirmation behavior explicit
where appropriate.

## Pull Request Checklist

- Update docs when command behavior or configuration changes.
- Update `CHANGELOG.md` for user-visible changes.
- Add or update tests for parsing, validation, and safety-sensitive logic.
- Run format, tests, clippy, and audit before requesting review.
- Do not commit secrets, local config, generated scratch files, or machine-only
  state.

## Security

Report vulnerabilities privately using the repository security policy. Avoid
opening public issues for exploitable bugs until a fix is available.
