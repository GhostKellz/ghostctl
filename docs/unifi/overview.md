# UniFi

`ghostctl unifi` manages a self-hosted **UniFi OS Server** controller: read-only
status and inventory, remote adoption of factory devices, and cross-vendor L2
diagnostics. It targets the current UOS Server (HTTPS on `:11443`, `X-API-KEY`
auth) — **not** the legacy self-hosted Network Application, which is end-of-life.

The status and inventory checks are read-only. `adopt` changes device state; it
honors the global `--dry-run` (print intent, run nothing) and `--yes` flags, and
otherwise prompts before touching anything. The API key is never logged.

## Quick Commands

```bash
ghostctl unifi status                     # Verify controller connectivity and API-key auth
ghostctl unifi devices                    # List adopted devices and their state
ghostctl unifi devices --pending          # Only devices pending adoption
ghostctl unifi devices --offline          # Only devices that are not online
ghostctl unifi adopt --subnet 192.168.1.0/24   # Remotely adopt factory devices (see adoption.md)
ghostctl unifi doctor                     # Diagnose STP, adoption, firmware issues (see doctor.md)
```

## Configuration

Settings live under `[unifi]` in `~/.config/ghostctl/config.toml`:

```toml
[unifi]
controller_url = "https://unifi.cktechx.com:11443"  # UOS Server base (default: https://127.0.0.1:11443)
site = "default"                                     # Site name (default: default)
verify_tls = false                                   # Self-signed cert → false (default: false)
inform_port = 8080                                   # Device inform port (default: 8080)
# inform_host = "69.169.98.98"                        # Override inform host (default: controller host)
timeout_secs = 15

# Remote adoption (see adoption.md)
adopt_ssh_users = ["ui", "ubnt"]                     # SSH users to try, in order
# adopt_ssh_key = "~/.ssh/id_ed25519"                 # Key auth (preferred over password)

# CrowdSec whitelist (see crowdsec.md)
exempt_cidrs = ["100.64.0.0/10"]                     # Tailscale CGNAT + your mgmt subnets

# api_key = "..."                                     # Optional; env vars take precedence
```

### API Key Resolution

The key is resolved in order; the first non-empty value wins:

1. `UNIFI_API_KEY`
2. `GHOSTCTL_UNIFI_API_KEY`
3. `[unifi].api_key` in the config file

Keeping the key in the environment means it never has to be written to disk. Mint
one in the Network app under **Settings → Integrations → API Key**.
`ghostctl unifi status` reports a clear error on a 401/403 so a bad or unscoped
key is obvious.

### TLS

A self-hosted UOS Server ships a self-signed cert, so `verify_tls` defaults to
`false`. Set it to `true` once the controller presents a trusted certificate
(e.g. behind a reverse proxy with a real cert).

## Subcommands

### `status`

Queries the integration `/sites` endpoint to confirm the controller is reachable
and the API key authenticates. Prints the controller URL, auth result, and the
sites it can see (marking the configured one).

### `devices`

Lists devices for the configured site with name, model, adoption state, IP, and
MAC. Filter with `--pending` (only devices awaiting adoption) or `--offline`
(only devices that are not online). Use `--pending` after running `adopt` or a
DHCP/DNS discovery change to confirm devices showed up.

## See Also

- [adoption.md](adoption.md) — remote adoption and controller discovery (DHCP
  Option 43, DNS, `set-inform`).
- [doctor.md](doctor.md) — STP, adoption, and firmware diagnostics for
  Fortinet + UniFi shops.
- [crowdsec.md](crowdsec.md) — generate a CrowdSec whitelist so legit device
  check-ins and admin traffic are never banned.
