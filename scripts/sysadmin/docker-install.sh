#!/bin/bash
# Docker & Docker Compose v2 Installation Script
# Supports: Ubuntu/Debian, Arch Linux, and RHEL/Fedora

set -e

echo "🐳 Docker & Docker Compose v2 Installation"
echo "==========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Detect OS
detect_os() {
    if [[ -f /etc/arch-release ]]; then
        OS="arch"
        DISTRO="Arch Linux"
    elif [[ -f /etc/debian_version ]]; then
        OS="debian"
        if [[ -f /etc/lsb-release ]] && grep -q "Ubuntu" /etc/lsb-release; then
            DISTRO="Ubuntu"
        else
            DISTRO="Debian"
        fi
    elif [[ -f /etc/redhat-release ]]; then
        OS="redhat"
        if grep -q "Fedora" /etc/redhat-release; then
            DISTRO="Fedora"
        else
            DISTRO="RHEL/CentOS"
        fi
    else
        print_error "Unsupported OS detected"
        exit 1
    fi
    
    print_status "Detected OS: $DISTRO"
}

# Check if Docker is already installed
check_docker_installation() {
    if command -v docker &> /dev/null; then
        DOCKER_VERSION=$(docker --version | cut -d' ' -f3 | cut -d',' -f1)
        print_warning "Docker is already installed (version: $DOCKER_VERSION)"
        
        read -p "Do you want to continue and reinstall? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_status "Installation cancelled"
            exit 0
        fi
    fi
}

# Install Docker on Arch Linux
install_docker_arch() {
    print_status "Installing Docker on Arch Linux..."
    
    # Update package database
    sudo pacman -Sy
    
    # Install Docker
    sudo pacman -S --noconfirm docker docker-compose
    
    # Enable and start Docker service
    sudo systemctl enable --now docker
    
    print_success "Docker installed on Arch Linux"
}

# Install Docker on Ubuntu/Debian
install_docker_debian() {
    print_status "Installing Docker on $DISTRO..."
    
    # Update package index
    sudo apt-get update
    
    # Install prerequisites
    sudo apt-get install -y \
        apt-transport-https \
        ca-certificates \
        curl \
        gnupg \
        lsb-release
    
    # Add Docker's official GPG key
    curl -fsSL https://download.docker.com/linux/$([[ "$DISTRO" == "Ubuntu" ]] && echo "ubuntu" || echo "debian")/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
    
    # Set up the stable repository
    echo \
        "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/$([[ "$DISTRO" == "Ubuntu" ]] && echo "ubuntu" || echo "debian") \
        $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
    
    # Update package index
    sudo apt-get update
    
    # Install Docker Engine, containerd, and Docker Compose
    sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
    
    # Enable and start Docker service
    sudo systemctl enable --now docker
    
    print_success "Docker installed on $DISTRO"
}

# Install Docker on RHEL/Fedora
install_docker_redhat() {
    print_status "Installing Docker on $DISTRO..."
    
    if [[ "$DISTRO" == "Fedora" ]]; then
        # Install Docker on Fedora
        sudo dnf -y install dnf-plugins-core
        sudo dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo
        sudo dnf install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
    else
        # Install Docker on RHEL/CentOS
        sudo yum install -y yum-utils
        sudo yum-config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo
        sudo yum install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
    fi
    
    # Enable and start Docker service
    sudo systemctl enable --now docker
    
    print_success "Docker installed on $DISTRO"
}

# Configure Docker for current user
configure_docker_user() {
    print_status "Configuring Docker for current user..."
    
    # Add current user to docker group
    sudo usermod -aG docker $USER
    
    print_success "User $USER added to docker group"
    print_warning "You need to log out and back in for group changes to take effect"
    print_status "Or run: newgrp docker"
}

# Install Docker Compose v2 (if not already installed)
install_docker_compose_v2() {
    if docker compose version &> /dev/null; then
        COMPOSE_VERSION=$(docker compose version --short)
        print_success "Docker Compose v2 is already available (version: $COMPOSE_VERSION)"
        return
    fi
    
    print_status "Installing Docker Compose v2..."
    
    # Get latest version
    COMPOSE_VERSION=$(curl -s https://api.github.com/repos/docker/compose/releases/latest | grep -oP '"tag_name": "\K(.*)(?=")')
    
    # Download and install
    sudo curl -L "https://github.com/docker/compose/releases/download/${COMPOSE_VERSION}/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    sudo chmod +x /usr/local/bin/docker-compose
    
    # Create symlink for v2 command
    sudo ln -sf /usr/local/bin/docker-compose /usr/local/bin/docker-compose-v2
    
    print_success "Docker Compose v2 installed (version: $COMPOSE_VERSION)"
}

# Verify installation
verify_installation() {
    print_status "Verifying Docker installation..."
    
    # Check Docker version
    if docker --version &> /dev/null; then
        DOCKER_VERSION=$(docker --version)
        print_success "Docker: $DOCKER_VERSION"
    else
        print_error "Docker installation failed"
        exit 1
    fi
    
    # Check Docker Compose
    if docker compose version &> /dev/null; then
        COMPOSE_VERSION=$(docker compose version)
        print_success "Docker Compose: $COMPOSE_VERSION"
    elif docker-compose --version &> /dev/null; then
        COMPOSE_VERSION=$(docker-compose --version)
        print_success "Docker Compose (standalone): $COMPOSE_VERSION"
    else
        print_warning "Docker Compose not found"
    fi
    
    # Test Docker with hello-world (optional)
    read -p "Run Docker hello-world test? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Running Docker hello-world test..."
        if sudo docker run --rm hello-world; then
            print_success "Docker is working correctly!"
        else
            print_error "Docker test failed"
        fi
    fi
}

# Install useful Docker tools
install_docker_tools() {
    read -p "Install additional Docker tools? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Installing additional Docker tools..."
        
        case $OS in
            "arch")
                sudo pacman -S --noconfirm docker-scan docker-buildx
                ;;
            "debian")
                # Tools are included with docker-ce
                print_status "Docker tools included with installation"
                ;;
            "redhat")
                # Tools are included with docker-ce
                print_status "Docker tools included with installation"
                ;;
        esac
        
        # Install ctop (container monitoring)
        if ! command -v ctop &> /dev/null; then
            print_status "Installing ctop..."
            sudo wget https://github.com/bcicen/ctop/releases/download/v0.7.7/ctop-0.7.7-linux-amd64 -O /usr/local/bin/ctop
            sudo chmod +x /usr/local/bin/ctop
            print_success "ctop installed"
        fi
        
        # Install dive (image analysis)
        if ! command -v dive &> /dev/null; then
            print_status "Installing dive..."
            DIVE_VERSION=$(curl -s https://api.github.com/repos/wagoodman/dive/releases/latest | grep -oP '"tag_name": "\K(.*)(?=")')
            wget -O /tmp/dive.tar.gz "https://github.com/wagoodman/dive/releases/download/${DIVE_VERSION}/dive_${DIVE_VERSION#v}_linux_amd64.tar.gz"
            tar -xzf /tmp/dive.tar.gz -C /tmp
            sudo mv /tmp/dive /usr/local/bin/
            sudo chmod +x /usr/local/bin/dive
            rm /tmp/dive.tar.gz
            print_success "dive installed"
        fi
    fi
}

# Security hardening recommendations
show_security_recommendations() {
    print_status "Security Recommendations:"
    echo "🔒 Enable Docker content trust: export DOCKER_CONTENT_TRUST=1"
    echo "🔒 Use non-root containers when possible"
    echo "🔒 Regularly update Docker and images"
    echo "🔒 Use Docker secrets for sensitive data"
    echo "🔒 Enable Docker daemon logging"
    echo "🔒 Consider using Docker rootless mode"
    echo ""
    echo "📚 Docker security guide: https://docs.docker.com/engine/security/"
}

# Main installation flow
main() {
    echo "🐳 Starting Docker installation process..."
    echo ""
    
    detect_os
    check_docker_installation
    
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
            print_error "Unsupported OS: $OS"
            exit 1
            ;;
    esac
    
    configure_docker_user
    install_docker_compose_v2
    verify_installation
    install_docker_tools
    show_security_recommendations
    
    print_success "🎉 Docker installation completed successfully!"
    print_status "Next steps:"
    echo "  1. Log out and back in (or run 'newgrp docker')"
    echo "  2. Test: docker run hello-world"
    echo "  3. Create your first container: docker run -it ubuntu bash"
    echo "  4. Learn Docker Compose: docker compose --help"
}

# Run main function
main "$@"