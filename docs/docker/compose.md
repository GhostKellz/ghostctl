# Docker Compose

## Stack Management

GhostCTL provides compose stack management for multi-container applications.

### Features
- Discover compose files in directories
- Start/stop/restart stacks
- View stack status
- View service logs
- Validate compose files

## Compose File Locations

GhostCTL searches for:
- `docker-compose.yml`
- `docker-compose.yaml`
- `compose.yml`
- `compose.yaml`

## Common Operations

### Start Stack
```bash
docker compose up -d
```

### Stop Stack
```bash
docker compose down
```

### View Logs
```bash
docker compose logs -f
docker compose logs -f service_name
```

### Rebuild
```bash
docker compose up -d --build
```

## Homelab Stacks

```bash
ghostctl docker homelab           # Homelab stack templates
```

Pre-configured stacks for:
- Media servers (Plex, Jellyfin)
- Reverse proxies (Traefik, Nginx Proxy Manager)
- Monitoring (Grafana, Prometheus)
- Home automation (Home Assistant)
- Development tools
