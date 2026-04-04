# Container Management

## Commands

```bash
ghostctl docker menu              # Docker management menu
ghostctl docker status            # Docker status
```

## Container Operations

Through the interactive menu:
- List running containers
- List all containers
- Stop containers
- Remove containers
- View container logs
- Execute into containers

## Cleanup

```bash
# Through menu
ghostctl docker menu  # Select cleanup option
```

Cleanup options:
- Remove stopped containers
- Remove unused images
- Remove unused volumes
- Remove unused networks
- Prune everything (all unused resources)
- Remove containers by age

## System Cleanup

Full system prune removes:
- All stopped containers
- All networks not used by containers
- All dangling images
- All dangling build cache

```bash
# Manual equivalent
docker system prune -a --volumes
```
