#!/bin/bash
# Docker Container Health Monitor and Cleanup
# Monitors Docker containers and performs maintenance tasks

set -e

echo "🐳 Docker Homelab Health Check & Cleanup"

# Check if Docker is running
if ! systemctl is-active --quiet docker; then
    echo "❌ Docker is not running. Starting Docker..."
    sudo systemctl start docker
fi

echo "📊 Container Status Report:"
echo "=========================="

# Show running containers
echo "🟢 Running Containers:"
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"

# Show stopped containers
echo -e "\n🔴 Stopped Containers:"
docker ps -a --filter "status=exited" --format "table {{.Names}}\t{{.Status}}"

# Check container resource usage
echo -e "\n💾 Resource Usage:"
docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.MemPerc}}"

# Check for unhealthy containers
echo -e "\n🏥 Health Status:"
docker ps --filter "health=unhealthy" --format "table {{.Names}}\t{{.Status}}"

# Cleanup section
echo -e "\n🧹 Cleanup Options:"
read -p "Remove stopped containers? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🗑️  Removing stopped containers..."
    docker container prune -f
fi

read -p "Remove unused images? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🗑️  Removing unused images..."
    docker image prune -f
fi

read -p "Remove unused volumes? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🗑️  Removing unused volumes..."
    docker volume prune -f
fi

# System df
echo -e "\n📊 Docker System Usage:"
docker system df

echo "✅ Docker homelab maintenance complete!"