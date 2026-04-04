# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.9.x   | :white_check_mark: |
| < 0.9   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in GhostCTL, please report it responsibly:

1. **Do not** open a public GitHub issue for security vulnerabilities
2. Email security concerns to: `ckelley@ghostkellz.sh`
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

You can expect:
- Acknowledgment within 48 hours
- Status update within 7 days
- Credit in the security advisory (if desired)

## Security Practices

### Dependency Auditing

We regularly audit dependencies for known vulnerabilities:

```bash
# Rust dependencies
cargo audit

# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update
```

### CI/CD Security

- All PRs run `cargo audit` before merge
- Dependencies are pinned to specific versions in `Cargo.lock`
- Release builds use `--locked` to ensure reproducible builds

### Code Security

- **No unsafe code** without explicit justification and review
- Input validation on all external data
- Proper error handling (no panic in library code)
- Secrets never logged or printed to stdout

### Privilege Handling

GhostCTL may require elevated privileges for certain operations:

| Operation | Privilege Required | Reason |
|-----------|-------------------|--------|
| System updates | sudo | Package manager access |
| Network config | sudo | Interface configuration |
| VFIO setup | sudo | Kernel module loading |
| Docker management | docker group | Docker socket access |
| VM management | libvirt group | libvirt socket access |

**Best practices:**
- Run with minimal required privileges
- Use `--dry-run` to preview changes before applying
- Audit commands before execution in headless mode

### File Permissions

- Config files: `600` (owner read/write only)
- Log files: `640` (owner read/write, group read)
- Scripts: `755` (executable, world readable)

### Network Security

- HTTPS enforced for all remote connections
- Certificate validation enabled by default
- No telemetry or data collection

## Known Security Considerations

### Shell Command Execution

GhostCTL executes shell commands for system management. Risks are mitigated by:
- No user input directly interpolated into commands
- Commands are constructed programmatically
- `--dry-run` mode available for inspection

### Configuration Files

Configuration may contain sensitive data:
- API keys for cloud providers
- Backup repository passwords
- SSH key paths

**Recommendations:**
- Store configs with restricted permissions (`chmod 600`)
- Use environment variables for secrets when possible
- Never commit secrets to version control

## Dependency Security

### Auditing Dependencies

```bash
# Install cargo-audit
cargo install cargo-audit

# Run audit
cargo audit

# Generate report
cargo audit --json > audit-report.json
```

### Key Dependencies

| Crate | Purpose | Security Notes |
|-------|---------|----------------|
| `clap` | CLI parsing | Well-maintained, no unsafe |
| `tokio` | Async runtime | Extensive security review |
| `reqwest` | HTTP client | TLS via rustls/native-tls |
| `serde` | Serialization | No unsafe in safe mode |
| `anyhow` | Error handling | Minimal, no unsafe |

### Updating Dependencies

```bash
# Check for updates
cargo outdated

# Update all dependencies
cargo update

# Update specific dependency
cargo update -p dependency_name

# Verify build after update
cargo build --release
cargo test
```

## Security Checklist for Contributors

Before submitting a PR:

- [ ] No hardcoded secrets or credentials
- [ ] No unsafe code without justification
- [ ] Input validation on external data
- [ ] Error messages don't leak sensitive info
- [ ] `cargo audit` passes
- [ ] `cargo clippy` passes without security warnings
- [ ] Tests cover security-sensitive code paths

## Vulnerability Disclosure Timeline

1. **Day 0**: Vulnerability reported
2. **Day 1-2**: Acknowledgment sent
3. **Day 3-7**: Initial assessment and triage
4. **Day 8-30**: Fix developed and tested
5. **Day 31-45**: Coordinated disclosure (if applicable)
6. **Day 45+**: Public disclosure with fix available

## Security Updates

Security updates are released as patch versions (e.g., 0.9.1, 0.9.2) and announced via:
- GitHub Security Advisories
- Release notes
- Project README

## Contact

- Security issues: `ckelley@ghostkellz.sh`
- General inquiries: GitHub Issues
