# Monitoring & Observability

GhostCTL talks to a Prometheus / Loki / Alertmanager / Grafana stack so you can
check service health, query logs, and inspect alerts without leaving the
terminal.

## Quick Commands

```bash
ghostctl monitor health                          # Probe all configured services
ghostctl mon health                              # Short alias for `monitor`
ghostctl monitor targets                         # Prometheus scrape targets + health
ghostctl monitor alerts                          # Active Alertmanager alerts
ghostctl monitor logs '{source_type="fortigate"}'   # Query Loki (LogQL)
ghostctl monitor logs '{job="nginx"}' --limit 200    # Cap returned lines
ghostctl monitor tail '{job="nginx"}'            # Follow new log lines
ghostctl monitor query up                        # Run a PromQL query
ghostctl monitor reload prometheus               # Hot-reload a service config
ghostctl monitor datasources                     # Grafana datasource health
```

## Features

- Liveness probes for Prometheus, Loki, Alertmanager, and Grafana
- Prometheus scrape-target health overview
- Alertmanager alert listing
- Loki log queries and live tailing via LogQL
- Ad-hoc PromQL queries
- Config hot-reload (no service restart)
- Grafana datasource health checks (requires a Grafana token)

## Configuration

Endpoints live under `[monitor]` in `config.toml`
(`~/.config/ghostctl/config.toml`). Run `ghostctl config show` to see the
resolved Prometheus, Loki, Alertmanager, and Grafana URLs. A `grafana_token`
is needed for the `datasources` check.
