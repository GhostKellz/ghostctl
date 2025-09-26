# 🐳 Docker Management with GhostCTL

Complete Docker integration and container management guide.

## 🚀 Quick Start

### Installation & Setup
```bash
# Install Docker (via GhostCTL)
ghostctl -> Docker & DevOps -> Install Docker

# Or manual installation
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER
sudo systemctl enable --now docker
```

### Verify Installation
```bash
docker --version
docker-compose --version
docker run hello-world
```

## 📦 Container Management

### Basic Operations
```bash
# List containers
docker ps                    # Running containers
docker ps -a                # All containers
docker ps -q                # Container IDs only

# Container lifecycle
docker run [options] image   # Create and start
docker start container       # Start existing
docker stop container        # Stop gracefully
docker kill container        # Force stop
docker restart container     # Restart
docker rm container          # Remove container
```

### Interactive Containers
```bash
# Run interactive container
docker run -it ubuntu bash

# Enter running container
docker exec -it container bash
docker exec -it container sh

# Run single command
docker exec container ls -la
```

### Container Information
```bash
# Container details
docker inspect container     # Full details
docker logs container        # View logs
docker logs -f container     # Follow logs
docker stats container       # Resource usage
docker top container         # Running processes
```

## 🖼️ Image Management

### Image Operations
```bash
# List images
docker images               # Local images
docker images -a           # All images (including intermediate)

# Pull/Push images
docker pull image:tag      # Download image
docker push image:tag      # Upload image
docker search term         # Search Docker Hub

# Build images
docker build -t name:tag . # Build from Dockerfile
docker build -f Dockerfile.prod -t name:prod .

# Remove images
docker rmi image          # Remove image
docker image prune        # Remove unused images
docker system prune -a    # Remove all unused data
```

### Dockerfile Best Practices
```dockerfile
# Multi-stage build example
FROM rust:1.70 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ghostctl /usr/local/bin/
CMD ["ghostctl"]
```

## 🎼 Docker Compose

### Compose Files
```yaml
# docker-compose.yml
version: '3.8'

services:
  web:
    build: .
    ports:
      - "8080:80"
    environment:
      - NODE_ENV=production
    volumes:
      - ./data:/app/data
    depends_on:
      - db
    restart: unless-stopped

  db:
    image: postgres:15
    environment:
      POSTGRES_DB: myapp
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: unless-stopped

volumes:
  postgres_data:
```

### Compose Commands
```bash
# Service management
docker-compose up           # Start services
docker-compose up -d        # Start in background
docker-compose down         # Stop and remove
docker-compose stop         # Stop services
docker-compose start        # Start stopped services
docker-compose restart      # Restart services

# Service operations
docker-compose ps           # List services
docker-compose logs         # View logs
docker-compose logs -f web  # Follow service logs
docker-compose exec web bash  # Enter service container

# Build and update
docker-compose build        # Build services
docker-compose pull         # Pull image updates
docker-compose up --build   # Rebuild and start
```

## 🏠 Homelab Docker Stacks

### Media Server Stack
```yaml
# media-server/docker-compose.yml
version: '3.8'

services:
  plex:
    image: lscr.io/linuxserver/plex
    container_name: plex
    network_mode: host
    environment:
      - PUID=1000
      - PGID=1000
      - VERSION=docker
    volumes:
      - ./config:/config
      - /media/movies:/movies
      - /media/tv:/tv
    restart: unless-stopped

  jellyfin:
    image: jellyfin/jellyfin
    container_name: jellyfin
    ports:
      - "8096:8096"
    environment:
      - JELLYFIN_PublishedServerUrl=http://jellyfin.local
    volumes:
      - ./jellyfin-config:/config
      - ./jellyfin-cache:/cache
      - /media:/media:ro
    restart: unless-stopped

  radarr:
    image: lscr.io/linuxserver/radarr
    container_name: radarr
    ports:
      - "7878:7878"
    environment:
      - PUID=1000
      - PGID=1000
    volumes:
      - ./radarr-config:/config
      - /media/movies:/movies
      - /downloads:/downloads
    restart: unless-stopped
```

### Monitoring Stack
```yaml
# monitoring/docker-compose.yml
version: '3.8'

services:
  prometheus:
    image: prom/prometheus
    container_name: prometheus
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    restart: unless-stopped

  grafana:
    image: grafana/grafana
    container_name: grafana
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana-data:/var/lib/grafana
    restart: unless-stopped

  node-exporter:
    image: prom/node-exporter
    container_name: node-exporter
    ports:
      - "9100:9100"
    volumes:
      - /proc:/host/proc:ro
      - /sys:/host/sys:ro
      - /:/rootfs:ro
    command:
      - '--path.procfs=/host/proc'
      - '--path.sysfs=/host/sys'
      - '--collector.filesystem.mount-points-exclude=^/(sys|proc|dev|host|etc)($$|/)'
    restart: unless-stopped

volumes:
  prometheus-data:
  grafana-data:
```

### Development Environment
```yaml
# dev-env/docker-compose.yml
version: '3.8'

services:
  postgres:
    image: postgres:15
    container_name: dev-postgres
    ports:
      - "5432:5432"
    environment:
      POSTGRES_DB: development
      POSTGRES_USER: dev
      POSTGRES_PASSWORD: devpass
    volumes:
      - postgres-dev-data:/var/lib/postgresql/data
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    container_name: dev-redis
    ports:
      - "6379:6379"
    restart: unless-stopped

  mailhog:
    image: mailhog/mailhog
    container_name: dev-mailhog
    ports:
      - "1025:1025"  # SMTP
      - "8025:8025"  # Web UI
    restart: unless-stopped

  minio:
    image: minio/minio
    container_name: dev-minio
    ports:
      - "9000:9000"
      - "9001:9001"
    environment:
      MINIO_ROOT_USER: admin
      MINIO_ROOT_PASSWORD: password
    volumes:
      - minio-data:/data
    command: server /data --console-address ":9001"
    restart: unless-stopped

volumes:
  postgres-dev-data:
  minio-data:
```

## 🌐 Reverse Proxy with Traefik

### Traefik Configuration
```yaml
# traefik/docker-compose.yml
version: '3.8'

services:
  traefik:
    image: traefik:v3.0
    container_name: traefik
    command:
      - --api.dashboard=true
      - --providers.docker=true
      - --providers.docker.exposedbydefault=false
      - --entrypoints.web.address=:80
      - --entrypoints.websecure.address=:443
      - --certificatesresolvers.letsencrypt.acme.tlschallenge=true
      - --certificatesresolvers.letsencrypt.acme.email=your@email.com
      - --certificatesresolvers.letsencrypt.acme.storage=/letsencrypt/acme.json
    ports:
      - "80:80"
      - "443:443"
      - "8080:8080"  # Dashboard
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./letsencrypt:/letsencrypt
    labels:
      - traefik.enable=true
      - traefik.http.routers.dashboard.rule=Host(`traefik.local`)
      - traefik.http.routers.dashboard.tls=true
      - traefik.http.routers.dashboard.tls.certresolver=letsencrypt
    restart: unless-stopped

  whoami:
    image: traefik/whoami
    container_name: whoami
    labels:
      - traefik.enable=true
      - traefik.http.routers.whoami.rule=Host(`whoami.local`)
      - traefik.http.routers.whoami.tls=true
      - traefik.http.routers.whoami.tls.certresolver=letsencrypt
    restart: unless-stopped
```

## 🔧 Docker Networking

### Network Management
```bash
# List networks
docker network ls

# Create network
docker network create mynetwork
docker network create --driver bridge mybridge

# Connect container to network
docker network connect mynetwork container

# Inspect network
docker network inspect mynetwork

# Remove network
docker network rm mynetwork
```

### Network Types
```yaml
# Custom bridge network
version: '3.8'

services:
  app:
    image: nginx
    networks:
      - frontend
      - backend

  db:
    image: postgres
    networks:
      - backend

networks:
  frontend:
    driver: bridge
  backend:
    driver: bridge
    internal: true  # No external access
```

## 💾 Volume Management

### Volume Commands
```bash
# List volumes
docker volume ls

# Create volume
docker volume create myvolume

# Inspect volume
docker volume inspect myvolume

# Remove volume
docker volume rm myvolume

# Remove unused volumes
docker volume prune
```

### Volume Types
```yaml
# docker-compose.yml volume examples
version: '3.8'

services:
  app:
    image: nginx
    volumes:
      # Named volume
      - app-data:/var/lib/app
      
      # Host bind mount
      - ./config:/etc/nginx/conf.d
      
      # Anonymous volume
      - /var/log/nginx
      
      # Read-only mount
      - ./static:/usr/share/nginx/html:ro

volumes:
  app-data:
    driver: local
    driver_opts:
      type: none
      device: /host/path
      o: bind
```

## 🚀 Deployment Strategies

### Blue-Green Deployment
```bash
#!/bin/bash
# blue-green-deploy.sh

# Deploy new version (green)
docker-compose -f docker-compose.green.yml up -d

# Health check
while ! curl -f http://localhost:8081/health; do
  sleep 5
done

# Switch traffic (update load balancer)
docker-compose -f docker-compose.lb.yml up -d

# Stop old version (blue)
docker-compose -f docker-compose.blue.yml down
```

### Rolling Updates
```yaml
# docker-compose.yml with rolling update
version: '3.8'

services:
  web:
    image: nginx:latest
    deploy:
      replicas: 3
      update_config:
        parallelism: 1
        delay: 10s
        order: start-first
      restart_policy:
        condition: on-failure
        delay: 5s
        max_attempts: 3
```

## 🔐 Security Best Practices

### Image Security
```dockerfile
# Use official base images
FROM node:18-alpine

# Create non-root user
RUN addgroup -g 1001 -S nodejs
RUN adduser -S nodejs -u 1001

# Set working directory
WORKDIR /app

# Copy and install dependencies first (better caching)
COPY package*.json ./
RUN npm ci --only=production

# Copy application code
COPY --chown=nodejs:nodejs . .

# Switch to non-root user
USER nodejs

# Expose port
EXPOSE 3000

# Use specific command
CMD ["node", "server.js"]
```

### Runtime Security
```yaml
# docker-compose.yml security settings
version: '3.8'

services:
  app:
    image: myapp:latest
    security_opt:
      - no-new-privileges:true
    read_only: true
    tmpfs:
      - /tmp
    cap_drop:
      - ALL
    cap_add:
      - NET_BIND_SERVICE
    user: "1001:1001"
    restart: unless-stopped
```

### Secrets Management
```yaml
# Using Docker secrets
version: '3.8'

services:
  app:
    image: myapp:latest
    secrets:
      - db_password
      - api_key
    environment:
      - DB_PASSWORD_FILE=/run/secrets/db_password
      - API_KEY_FILE=/run/secrets/api_key

secrets:
  db_password:
    file: ./secrets/db_password.txt
  api_key:
    external: true
```

## 📊 Monitoring & Logging

### Container Monitoring
```bash
# Resource usage
docker stats                    # All containers
docker stats container         # Specific container

# System resource usage
docker system df               # Disk usage
docker system events          # System events

# Container health
docker inspect --format='{{.State.Health.Status}}' container
```

### Centralized Logging
```yaml
# ELK Stack for logging
version: '3.8'

services:
  elasticsearch:
    image: docker.elastic.co/elasticsearch/elasticsearch:8.0.0
    environment:
      - discovery.type=single-node
      - xpack.security.enabled=false
    ports:
      - "9200:9200"
    volumes:
      - elasticsearch-data:/usr/share/elasticsearch/data

  logstash:
    image: docker.elastic.co/logstash/logstash:8.0.0
    ports:
      - "5000:5000"
    volumes:
      - ./logstash.conf:/usr/share/logstash/pipeline/logstash.conf
    depends_on:
      - elasticsearch

  kibana:
    image: docker.elastic.co/kibana/kibana:8.0.0
    ports:
      - "5601:5601"
    environment:
      - ELASTICSEARCH_HOSTS=http://elasticsearch:9200
    depends_on:
      - elasticsearch

volumes:
  elasticsearch-data:
```

## 🔄 Backup & Recovery

### Container Backup
```bash
# Backup container data
docker run --rm -v container_volume:/data -v $(pwd):/backup alpine \
  tar czf /backup/backup.tar.gz -C /data .

# Restore container data
docker run --rm -v container_volume:/data -v $(pwd):/backup alpine \
  tar xzf /backup/backup.tar.gz -C /data

# Export/Import containers
docker export container > container.tar
docker import container.tar myimage:latest
```

### Database Backup
```bash
# PostgreSQL backup
docker exec postgres pg_dump -U user database > backup.sql

# MySQL backup
docker exec mysql mysqldump -u user -p database > backup.sql

# Automated backup script
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
docker exec postgres pg_dump -U user database | gzip > "backup_${DATE}.sql.gz"
find ./backups -name "backup_*.sql.gz" -mtime +7 -delete
```

## 🛠️ Development with Docker

### Development Dockerfile
```dockerfile
# Dockerfile.dev
FROM node:18

WORKDIR /app

# Install dependencies
COPY package*.json ./
RUN npm install

# Copy source code
COPY . .

# Install development tools
RUN npm install -g nodemon

# Expose port
EXPOSE 3000

# Development command
CMD ["npm", "run", "dev"]
```

### Development Compose
```yaml
# docker-compose.dev.yml
version: '3.8'

services:
  app:
    build:
      context: .
      dockerfile: Dockerfile.dev
    volumes:
      - .:/app
      - /app/node_modules
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=development
    depends_on:
      - db

  db:
    image: postgres:15
    environment:
      POSTGRES_DB: myapp_dev
      POSTGRES_USER: dev
      POSTGRES_PASSWORD: devpass
    ports:
      - "5432:5432"
    volumes:
      - postgres-dev:/var/lib/postgresql/data

volumes:
  postgres-dev:
```

## 🚨 Troubleshooting

### Common Issues
```bash
# Container won't start
docker logs container           # Check logs
docker inspect container       # Check configuration

# Permission issues
docker exec -it container ls -la  # Check file permissions
docker exec -it --user root container bash  # Enter as root

# Network issues
docker network ls              # List networks
docker exec container ping other-container  # Test connectivity

# Resource issues
docker stats                   # Check resource usage
docker system df              # Check disk usage
docker system prune           # Clean up resources
```

### Performance Optimization
```bash
# Optimize images
docker images --filter "dangling=true" -q | xargs docker rmi
docker system prune -a

# Monitor performance
docker stats --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}"

# Limit resources
docker run --memory="256m" --cpus="1.0" image
```

---

For more Docker-related automation, see [Proxmox Integration](PROXMOX.md) for container deployment in virtualized environments.