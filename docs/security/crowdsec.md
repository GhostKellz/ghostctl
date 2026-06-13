# CrowdSec & Threat Intel

GhostCTL surfaces CrowdSec threat intelligence and LAPI metrics, and provides
DNS resolver diagnostics for the host running the Local API.

## Quick Commands

```bash
ghostctl crowdsec feed         # Inspect the public threat feed
ghostctl crowdsec metrics      # Summarize CrowdSec LAPI Prometheus metrics
ghostctl crowdsec cli ...      # Passthrough to local cscli (LAPI host only)
ghostctl crowdsec dns          # Check DNS resolver reachability and DNSSEC
```

## Features

- Public threat-feed inspection
- CrowdSec LAPI Prometheus metrics summary (when configured)
- `cscli` passthrough on the LAPI host
- DNS resolver reachability and DNSSEC validation checks

## Configuration

Settings live under `[crowdsec]` in `config.toml`
(`~/.config/ghostctl/config.toml`): the threat-feed URL, an optional LAPI
metrics endpoint, and the primary DNS resolver. Run `ghostctl config show` to
see resolved values. The `cli` passthrough only works on a host running the
CrowdSec Local API.
