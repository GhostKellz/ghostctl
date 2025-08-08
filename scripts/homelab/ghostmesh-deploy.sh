#!/bin/bash

# GhostMesh PVE Container Deployment Script
# Deploys GhostMesh coordination server in Ubuntu 24.04 LTS or Debian 12 container
# Compatible with Proxmox VE 8.x

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEFAULT_VMID=200
DEFAULT_HOSTNAME="ghostmesh-server"
DEFAULT_MEMORY=2048
DEFAULT_DISK_SIZE=20
DEFAULT_CORES=2
DEFAULT_PASSWORD=""
DEFAULT_SSH_KEY=""
DEFAULT_STORAGE="local-lvm"
DEFAULT_BRIDGE="vmbr0"
DEFAULT_OS="ubuntu"  # ubuntu or debian

# Banner
print_banner() {
    echo -e "${PURPLE}"
    echo "  ██████╗ ██╗  ██╗ ██████╗ ███████╗████████╗███╗   ███╗███████╗███████╗██╗  ██╗"
    echo " ██╔════╝ ██║  ██║██╔═══██╗██╔════╝╚══██╔══╝████╗ ████║██╔════╝██╔════╝██║  ██║"
    echo " ██║  ███╗███████║██║   ██║███████╗   ██║   ██╔████╔██║█████╗  ███████╗███████║"
    echo " ██║   ██║██╔══██║██║   ██║╚════██║   ██║   ██║╚██╔╝██║██╔══╝  ╚════██║██╔══██║"
    echo " ╚██████╔╝██║  ██║╚██████╔╝███████║   ██║   ██║ ╚═╝ ██║███████╗███████║██║  ██║"
    echo "  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ╚═╝     ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝"
    echo ""
    echo -e "${CYAN}                   Proxmox VE Container Deployment Script${NC}"
    echo -e "${BLUE}                        v0.2.0 - Production Ready${NC}"
    echo ""
}

# Logging functions
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Check if running on Proxmox VE
check_proxmox() {
    if ! command -v pct &> /dev/null; then
        log_error "This script must be run on a Proxmox VE host"
        exit 1
    fi
    
    if ! systemctl is-active --quiet pve-cluster 2>/dev/null; then
        log_warning "PVE cluster service not running, continuing anyway..."
    fi
    
    log_success "Detected Proxmox VE environment"
}

# Interactive configuration
configure_deployment() {
    echo -e "${CYAN}=== GhostMesh Container Configuration ===${NC}"
    echo ""
    
    read -p "Container ID [$DEFAULT_VMID]: " VMID
    VMID=${VMID:-$DEFAULT_VMID}
    
    read -p "Hostname [$DEFAULT_HOSTNAME]: " HOSTNAME
    HOSTNAME=${HOSTNAME:-$DEFAULT_HOSTNAME}
    
    echo ""
    echo "OS Template Options:"
    echo "  1) Ubuntu 24.04 LTS (Recommended)"
    echo "  2) Debian 12"
    read -p "Select OS [1]: " OS_CHOICE
    case ${OS_CHOICE:-1} in
        1) OS_TYPE="ubuntu" ;;
        2) OS_TYPE="debian" ;;
        *) OS_TYPE="ubuntu" ;;
    esac
    
    read -p "Memory (MB) [$DEFAULT_MEMORY]: " MEMORY
    MEMORY=${MEMORY:-$DEFAULT_MEMORY}
    
    read -p "Disk size (GB) [$DEFAULT_DISK_SIZE]: " DISK_SIZE
    DISK_SIZE=${DISK_SIZE:-$DEFAULT_DISK_SIZE}
    
    read -p "CPU cores [$DEFAULT_CORES]: " CORES
    CORES=${CORES:-$DEFAULT_CORES}
    
    read -p "Storage pool [$DEFAULT_STORAGE]: " STORAGE
    STORAGE=${STORAGE:-$DEFAULT_STORAGE}
    
    read -p "Network bridge [$DEFAULT_BRIDGE]: " BRIDGE
    BRIDGE=${BRIDGE:-$DEFAULT_BRIDGE}
    
    echo ""
    read -s -p "Root password (leave empty for SSH key only): " PASSWORD
    echo ""
    
    if [[ -z "$PASSWORD" ]]; then
        read -p "SSH public key file path (optional): " SSH_KEY_PATH
        if [[ -n "$SSH_KEY_PATH" && -f "$SSH_KEY_PATH" ]]; then
            SSH_KEY=$(cat "$SSH_KEY_PATH")
        fi
    fi
    
    echo ""
    echo -e "${CYAN}=== Configuration Summary ===${NC}"
    echo "Container ID: $VMID"
    echo "Hostname: $HOSTNAME"
    echo "OS: $OS_TYPE"
    echo "Memory: ${MEMORY}MB"
    echo "Disk: ${DISK_SIZE}GB"
    echo "CPU Cores: $CORES"
    echo "Storage: $STORAGE"
    echo "Network: $BRIDGE"
    echo ""
    
    read -p "Proceed with deployment? [y/N]: " CONFIRM
    if [[ ! "$CONFIRM" =~ ^[Yy]$ ]]; then
        log_info "Deployment cancelled"
        exit 0
    fi
}

# Download OS template
download_template() {
    log_info "Checking for OS template..."
    
    case $OS_TYPE in
        ubuntu)
            TEMPLATE="ubuntu-24.04-standard_24.04-2_amd64.tar.zst"
            TEMPLATE_URL="http://download.proxmox.com/images/aplinfo.dat"
            ;;
        debian)
            TEMPLATE="debian-12-standard_12.7-1_amd64.tar.zst"
            TEMPLATE_URL="http://download.proxmox.com/images/aplinfo.dat"
            ;;
    esac
    
    # Check if template exists
    if ! pveam list local | grep -q "$TEMPLATE"; then
        log_info "Downloading $OS_TYPE template..."
        pveam update
        pveam download local "$TEMPLATE" || {
            log_error "Failed to download template"
            exit 1
        }
    fi
    
    log_success "Template ready: $TEMPLATE"
}

# Create container
create_container() {
    log_info "Creating GhostMesh container..."
    
    # Build pct create command
    PCT_CMD="pct create $VMID local:vztmpl/$TEMPLATE"
    PCT_CMD+=" --hostname $HOSTNAME"
    PCT_CMD+=" --memory $MEMORY"
    PCT_CMD+=" --cores $CORES"
    PCT_CMD+=" --rootfs $STORAGE:${DISK_SIZE}"
    PCT_CMD+=" --net0 name=eth0,bridge=$BRIDGE,ip=dhcp"
    PCT_CMD+=" --unprivileged 1"
    PCT_CMD+=" --features nesting=1"
    PCT_CMD+=" --start 1"
    
    if [[ -n "$PASSWORD" ]]; then
        PCT_CMD+=" --password"
    fi
    
    if [[ -n "$SSH_KEY" ]]; then
        PCT_CMD+=" --ssh-public-keys <(echo '$SSH_KEY')"
    fi
    
    # Execute container creation
    if [[ -n "$PASSWORD" ]]; then
        echo "$PASSWORD" | $PCT_CMD
    else
        eval $PCT_CMD
    fi
    
    log_success "Container $VMID created successfully"
    
    # Wait for container to start
    log_info "Waiting for container to start..."
    sleep 10
    
    # Wait for network
    log_info "Waiting for network connectivity..."
    for i in {1..30}; do
        if pct exec $VMID -- ping -c 1 8.8.8.8 &>/dev/null; then
            break
        fi
        sleep 2
    done
}

# Install dependencies in container
install_dependencies() {
    log_info "Installing system dependencies..."
    
    case $OS_TYPE in
        ubuntu)
            pct exec $VMID -- apt-get update
            pct exec $VMID -- apt-get install -y \
                curl wget git build-essential \
                wireguard-tools iptables \
                systemd-resolved resolvconf \
                htop nano sudo ufw
            ;;
        debian)
            pct exec $VMID -- apt-get update
            pct exec $VMID -- apt-get install -y \
                curl wget git build-essential \
                wireguard-tools iptables \
                systemd-resolved resolvconf \
                htop nano sudo ufw
            ;;
    esac
    
    log_success "System dependencies installed"
}

# Install Zig 0.15.0-dev
install_zig() {
    log_info "Installing Zig 0.15.0-dev..."
    
    pct exec $VMID -- bash -c "
        cd /opt
        wget -q https://ziglang.org/builds/zig-linux-x86_64-0.15.0-dev.2145+6db0ba36c.tar.xz
        tar -xf zig-linux-x86_64-0.15.0-dev.2145+6db0ba36c.tar.xz
        ln -sf /opt/zig-linux-x86_64-0.15.0-dev.2145+6db0ba36c/zig /usr/local/bin/zig
        rm zig-linux-x86_64-0.15.0-dev.2145+6db0ba36c.tar.xz
    "
    
    # Verify installation
    if pct exec $VMID -- zig version | grep -q "0.15.0-dev"; then
        log_success "Zig installed successfully"
    else
        log_error "Zig installation failed"
        exit 1
    fi
}

# Deploy GhostMesh
deploy_ghostmesh() {
    log_info "Deploying GhostMesh..."
    
    # Copy source code to container
    log_info "Copying GhostMesh source code..."
    pct exec $VMID -- mkdir -p /opt/ghostmesh
    
    # Create temporary tar to transfer files
    cd "$SCRIPT_DIR"
    tar --exclude='.git' --exclude='zig-cache' --exclude='zig-out' \
        -czf /tmp/ghostmesh-source.tar.gz .
    
    # Copy and extract in container
    pct push $VMID /tmp/ghostmesh-source.tar.gz /tmp/ghostmesh-source.tar.gz
    pct exec $VMID -- tar -xzf /tmp/ghostmesh-source.tar.gz -C /opt/ghostmesh
    pct exec $VMID -- rm /tmp/ghostmesh-source.tar.gz
    rm /tmp/ghostmesh-source.tar.gz
    
    # Build GhostMesh
    log_info "Building GhostMesh coordination server..."
    pct exec $VMID -- bash -c "
        cd /opt/ghostmesh
        zig build -Doptimize=ReleaseFast
    "
    
    # Install binaries
    pct exec $VMID -- bash -c "
        cd /opt/ghostmesh
        cp zig-out/bin/ghostmesh /usr/local/bin/
        chmod +x /usr/local/bin/ghostmesh
    "
    
    log_success "GhostMesh built and installed"
}

# Configure GhostMesh
configure_ghostmesh() {
    log_info "Configuring GhostMesh..."
    
    # Create directories
    pct exec $VMID -- mkdir -p /etc/ghostmesh /var/lib/ghostmesh /var/log/ghostmesh
    
    # Copy default configuration
    pct exec $VMID -- cp /opt/ghostmesh/ghostmesh.toml /etc/ghostmesh/
    
    # Create ghostmesh user
    pct exec $VMID -- useradd --system --home /var/lib/ghostmesh --shell /bin/false ghostmesh
    pct exec $VMID -- chown -R ghostmesh:ghostmesh /var/lib/ghostmesh /var/log/ghostmesh
    
    # Set up systemd service
    pct exec $VMID -- bash -c "cat > /etc/systemd/system/ghostmesh.service << 'EOF'
[Unit]
Description=GhostMesh Coordination Server
Documentation=https://github.com/ghostkellz/ghostmesh
After=network-online.target
Wants=network-online.target

[Service]
Type=exec
User=ghostmesh
Group=ghostmesh
ExecStart=/usr/local/bin/ghostmesh
WorkingDirectory=/var/lib/ghostmesh
Restart=always
RestartSec=5
LimitNOFILE=65536

# Security settings
NoNewPrivileges=yes
PrivateTmp=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/var/lib/ghostmesh /var/log/ghostmesh /etc/ghostmesh

# Network capabilities for VPN
AmbientCapabilities=CAP_NET_ADMIN CAP_NET_RAW
CapabilityBoundingSet=CAP_NET_ADMIN CAP_NET_RAW

[Install]
WantedBy=multi-user.target
EOF"
    
    # Enable and start service
    pct exec $VMID -- systemctl daemon-reload
    pct exec $VMID -- systemctl enable ghostmesh
    
    log_success "GhostMesh service configured"
}

# Configure firewall
configure_firewall() {
    log_info "Configuring firewall..."
    
    pct exec $VMID -- bash -c "
        # Enable UFW
        ufw --force enable
        
        # Allow SSH
        ufw allow 22/tcp
        
        # Allow GhostMesh ports
        ufw allow 443/tcp comment 'GhostMesh QUIC'
        ufw allow 443/udp comment 'GhostMesh QUIC'
        ufw allow 41641/tcp comment 'GhostMesh Control'
        ufw allow 41641/udp comment 'GhostMesh Control'
        ufw allow 41642/tcp comment 'GhostMesh QUIC Alt'
        ufw allow 41642/udp comment 'GhostMesh QUIC Alt'
        
        # Allow WireGuard
        ufw allow 51820/udp comment 'WireGuard'
        
        # Show status
        ufw status numbered
    "
    
    log_success "Firewall configured"
}

# Start GhostMesh
start_ghostmesh() {
    log_info "Starting GhostMesh coordination server..."
    
    pct exec $VMID -- systemctl start ghostmesh
    
    # Wait for startup
    sleep 5
    
    # Check status
    if pct exec $VMID -- systemctl is-active --quiet ghostmesh; then
        log_success "GhostMesh started successfully"
    else
        log_error "Failed to start GhostMesh"
        log_info "Check logs with: pct exec $VMID -- journalctl -u ghostmesh -f"
        exit 1
    fi
}

# Get container IP
get_container_info() {
    log_info "Retrieving container information..."
    
    CONTAINER_IP=$(pct exec $VMID -- hostname -I | awk '{print $1}')
    
    echo ""
    echo -e "${GREEN}=== GhostMesh Deployment Complete ===${NC}"
    echo ""
    echo -e "${CYAN}Container Details:${NC}"
    echo "  ID: $VMID"
    echo "  Hostname: $HOSTNAME"
    echo "  IP Address: $CONTAINER_IP"
    echo "  OS: $OS_TYPE"
    echo ""
    echo -e "${CYAN}GhostMesh Service:${NC}"
    echo "  Status: $(pct exec $VMID -- systemctl is-active ghostmesh)"
    echo "  Config: /etc/ghostmesh/ghostmesh.toml"
    echo "  Logs: journalctl -u ghostmesh -f"
    echo ""
    echo -e "${CYAN}Access Information:${NC}"
    echo "  SSH: ssh root@$CONTAINER_IP"
    echo "  QUIC Server: $CONTAINER_IP:443"
    echo "  Control Port: $CONTAINER_IP:41641"
    echo ""
    echo -e "${CYAN}Management Commands:${NC}"
    echo "  Start:   pct exec $VMID -- systemctl start ghostmesh"
    echo "  Stop:    pct exec $VMID -- systemctl stop ghostmesh" 
    echo "  Status:  pct exec $VMID -- systemctl status ghostmesh"
    echo "  Logs:    pct exec $VMID -- journalctl -u ghostmesh -f"
    echo "  Config:  pct exec $VMID -- nano /etc/ghostmesh/ghostmesh.toml"
    echo ""
    echo -e "${YELLOW}Next Steps:${NC}"
    echo "  1. Review configuration: pct exec $VMID -- nano /etc/ghostmesh/ghostmesh.toml"
    echo "  2. Customize mesh subnet and DNS settings"
    echo "  3. Deploy GhostLink clients to connect to this coordinator"
    echo "  4. Monitor logs for client connections"
    echo ""
    echo -e "${PURPLE}🚀 GhostMesh v0.2.0 is ready for production use!${NC}"
}

# Cleanup on failure
cleanup() {
    if [[ $? -ne 0 ]] && [[ -n "${VMID:-}" ]]; then
        log_warning "Deployment failed, cleaning up..."
        pct stop $VMID 2>/dev/null || true
        pct destroy $VMID 2>/dev/null || true
    fi
}
trap cleanup EXIT

# Main deployment function
main() {
    print_banner
    
    # Pre-flight checks
    check_proxmox
    
    # Configuration
    configure_deployment
    
    # Deployment steps
    download_template
    create_container
    install_dependencies
    install_zig
    deploy_ghostmesh
    configure_ghostmesh
    configure_firewall
    start_ghostmesh
    get_container_info
    
    log_success "GhostMesh deployment completed successfully!"
}

# Check if script is being sourced or executed
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi