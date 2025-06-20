#!/bin/bash
# Docker & Docker Compose v2 Installation Script
# Supports: Arch Linux, Ubuntu/Debian, RHEL/Fedora/CentOS

set -e

echo "🐳 Docker & Docker Compose v2 Installation"
echo "=========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect OS
detect_os() {
    if [[ -f /etc/arch-release ]]; then
        OS="arch"
    elif [[ -f /etc/debian_version ]]; then
        OS="debian"
    elif [[ -f /etc/redhat-release ]]; then
        OS="redhat"
    else
        echo -e "${RED}❌ Unsupported OS${NC}"
        exit 1
    fi
    echo -e "${BLUE}ℹ️  Detected OS: $OS${NC}"
}

install_docker_arch() {
    echo -e "${YELLOW}📦 Installing Docker on Arch Linux...${NC}"
    
    # Update system
    sudo pacman -Syu --noconfirm
    
    # Install Docker
    sudo pacman -S --noconfirm docker docker-compose
    
    # Enable and start Docker service
    sudo systemctl enable --now docker
    
    # Add user to docker group
    sudo usermod -aG docker $USER
    
    echo -e "${GREEN}✅ Docker installed on Arch Linux${NC}"
}

install_docker_debian() {
    echo -e "${YELLOW}📦 Installing Docker on Ubuntu/Debian...${NC}"
    
    # Update package index
    sudo apt update
    
    # Install required packages
    sudo apt install -y apt-transport-https ca-certificates curl gnupg lsb-release
    
    # Add Docker's official GPG key
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
    
    # Set up the stable repository
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
    
    # Update package index again
    sudo apt update
    
    # Install Docker Engine
    sudo apt install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
    
    # Enable and start Docker service
    sudo systemctl enable --now docker
    
    # Add user to docker group
    sudo usermod -aG docker $USER
    
    echo -e "${GREEN}✅ Docker installed on Ubuntu/Debian${NC}"
}

install_docker_redhat() {
    echo -e "${YELLOW}📦 Installing Docker on RHEL/Fedora/CentOS...${NC}"
    
    # Remove old versions
    sudo dnf remove -y docker docker-client docker-client-latest docker-common docker-latest docker-latest-logrotate docker-logrotate docker-engine
    
    # Install required packages
    sudo dnf install -y dnf-plugins-core
    
    # Add Docker repository
    sudo dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo
    
    # Install Docker Engine
    sudo dnf install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
    
    # Enable and start Docker service
    sudo systemctl enable --now docker
    
    # Add user to docker group
    sudo usermod -aG docker $USER
    
    echo -e "${GREEN}✅ Docker installed on RHEL/Fedora${NC}"
}

install_docker_compose_v2() {
    echo -e "${YELLOW}📦 Installing Docker Compose v2...${NC}"
    
    # Docker Compose v2 is included with Docker Desktop and as a plugin
    # For manual installation on systems without the plugin:
    if ! docker compose version &>/dev/null; then
        echo "Installing Docker Compose v2 manually..."
        
        # Get latest version
        COMPOSE_VERSION=$(curl -s https://api.github.com/repos/docker/compose/releases/latest | grep -Po '"tag_name": "\K[^"]*')
        
        # Download and install
        sudo curl -L "https://github.com/docker/compose/releases/download/${COMPOSE_VERSION}/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
        sudo chmod +x /usr/local/bin/docker-compose
        
        # Create symlink for v2 command
        sudo ln -sf /usr/local/bin/docker-compose /usr/local/bin/docker-compose-v2
    fi
    
    echo -e "${GREEN}✅ Docker Compose v2 ready${NC}"
}

configure_docker() {
    echo -e "${YELLOW}⚙️  Configuring Docker...${NC}"
    
    # Create docker group if it doesn't exist
    sudo groupadd -f docker
    
    # Add current user to docker group
    sudo usermod -aG docker $USER
    
    # Configure Docker daemon
    sudo tee /etc/docker/daemon.json > /dev/null <<EOF
{
    "log-driver": "json-file",
    "log-opts": {
        "max-size": "10m",
        "max-file": "3"
    },
    "storage-driver": "overlay2"
}
EOF
    
    # Restart Docker to apply configuration
    sudo systemctl restart docker
    
    echo -e "${GREEN}✅ Docker configured${NC}"
}

verify_installation() {
    echo -e "${YELLOW}🔍 Verifying installation...${NC}"
    
    # Test Docker
    echo "Testing Docker:"
    if docker --version; then
        echo -e "${GREEN}✅ Docker is working${NC}"
    else
        echo -e "${RED}❌ Docker installation failed${NC}"
        exit 1
    fi
    
    # Test Docker Compose
    echo "Testing Docker Compose:"
    if docker compose version; then
        echo -e "${GREEN}✅ Docker Compose v2 is working${NC}"
    elif docker-compose --version; then
        echo -e "${GREEN}✅ Docker Compose (legacy) is working${NC}"
    else
        echo -e "${RED}❌ Docker Compose installation failed${NC}"
    fi
    
    # Test Docker daemon
    echo "Testing Docker daemon:"
    if docker run --rm hello-world; then
        echo -e "${GREEN}✅ Docker daemon is working${NC}"
    else
        echo -e "${RED}❌ Docker daemon test failed${NC}"
        echo -e "${YELLOW}💡 You may need to log out and back in for group changes to take effect${NC}"
    fi
}

post_install_info() {
    echo -e "${BLUE}📋 Post-Installation Information${NC}"
    echo "================================"
    echo ""
    echo -e "${GREEN}🎉 Docker installation complete!${NC}"
    echo ""
    echo "📚 Next steps:"
    echo "  • Log out and back in (or run: newgrp docker)"
    echo "  • Test: docker run hello-world"
    echo "  • Manage: systemctl status docker"
    echo ""
    echo "🔧 Docker Compose commands:"
    echo "  • New syntax: docker compose up"
    echo "  • Legacy: docker-compose up"
    echo ""
    echo "📖 Documentation:"
    echo "  • Docker: https://docs.docker.com/"
    echo "  • Compose: https://docs.docker.com/compose/"
    echo ""
    echo "🛠️  Useful Docker commands:"
    echo "  • docker ps                    # List running containers"
    echo "  • docker images                # List images"
    echo "  • docker system prune         # Clean up"
    echo "  • docker compose up -d        # Run compose in background"
}

main() {
    detect_os
    
    case $OS in
        "arch")
            install_docker_arch
            ;;
        "debian")
            install_docker_debian
            ;;
        "redhat")
            install_docker_redhat
            ;;
        *)
            echo -e "${RED}❌ Unsupported OS: $OS${NC}"
            exit 1
            ;;
    esac
    
    install_docker_compose_v2
    configure_docker
    verify_installation
    post_install_info
}

# Check if running as root
if [[ $EUID -eq 0 ]]; then
    echo -e "${RED}❌ This script should not be run as root${NC}"
    echo "Please run as a regular user with sudo privileges"
    exit 1
fi

# Check if user has sudo access
if ! sudo -n true 2>/dev/null; then
    echo -e "${YELLOW}🔐 This script requires sudo access${NC}"
    echo "Please ensure you have sudo privileges"
fi

main "$@"