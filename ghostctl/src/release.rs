use std::fs;

#[allow(dead_code)]
pub fn create_release_structure() {
    println!("🏗️  Creating GhostCTL Release Structure");

    let release_dir = "/data/projects/ghostctl/release";
    let install_dir = format!("{}/install", release_dir);
    let pkg_dir = format!("{}/pkg", release_dir);
    let debian_dir = format!("{}/debian", release_dir);

    // Create directories
    for dir in [release_dir, &install_dir, &pkg_dir, &debian_dir] {
        if let Err(e) = fs::create_dir_all(dir) {
            eprintln!("Failed to create directory {}: {}", dir, e);
            continue;
        }
        println!("📁 Created: {}", dir);
    }

    create_arch_pkgbuild(&pkg_dir);
    create_debian_package(&debian_dir);
    create_universal_installer(&install_dir);
    create_proxmox_installer(&install_dir);

    println!("✅ Release structure created successfully!");
}

fn create_arch_pkgbuild(pkg_dir: &str) {
    let version = env!("CARGO_PKG_VERSION");

    let pkgbuild_template = r#"# Maintainer: Christopher Kelley <ckelley@ghostkellz.sh>
# Contributor: GhostCTL Development Team

pkgname=ghostctl
pkgver=0.3.0
pkgrel=1
pkgdesc="Comprehensive system administration toolkit for Linux power users, homelabbers, and DevOps professionals"
arch=('x86_64' 'aarch64')
url="https://github.com/ghostkellz/ghostctl"
license=('MIT')
depends=('rust' 'git' 'curl' 'docker' 'systemd')
makedepends=('cargo' 'pkgconf')
optdepends=(
    'ansible: Infrastructure automation'
    'terraform: Infrastructure as code'
    'azure-cli: Azure cloud management'
    'aws-cli: AWS cloud management'
    'docker-compose: Container orchestration'
    'minio: Object storage server'
    'restic: Backup solution'
    'btrfs-progs: Btrfs filesystem tools'
    'snapper: Btrfs snapshot management'
    'tailscale: Mesh networking'
    'neovim: Advanced text editor'
    'zsh: Advanced shell'
    'tmux: Terminal multiplexer'
)
backup=('etc/ghostctl/config.toml')
source=("$pkgname-$pkgver.tar.gz::https://github.com/ghostkellz/ghostctl/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
    cd "$srcdir/$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$srcdir/$pkgname-$pkgver/ghostctl"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    cd "$srcdir/$pkgname-$pkgver/ghostctl"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    
    # Install binary
    install -Dm755 "ghostctl/target/release/ghostctl" "$pkgdir/usr/bin/ghostctl"
    
    # Install configuration
    install -Dm644 "ghostctl/config/default.toml" "$pkgdir/etc/ghostctl/config.toml"
    
    # Install systemd services
    install -Dm644 "install/systemd/ghostctl-backup.service" "$pkgdir/usr/lib/systemd/user/ghostctl-backup.service"
    install -Dm644 "install/systemd/ghostctl-backup.timer" "$pkgdir/usr/lib/systemd/user/ghostctl-backup.timer"
    
    # Install shell completions
    install -Dm644 "completions/ghostctl.bash" "$pkgdir/usr/share/bash-completion/completions/ghostctl"
    install -Dm644 "completions/_ghostctl" "$pkgdir/usr/share/zsh/site-functions/_ghostctl"
    install -Dm644 "completions/ghostctl.fish" "$pkgdir/usr/share/fish/vendor_completions.d/ghostctl.fish"
    
    # Install desktop entry
    install -Dm644 "install/ghostctl.desktop" "$pkgdir/usr/share/applications/ghostctl.desktop"
    
    # Install documentation
    install -Dm644 "README.md" "$pkgdir/usr/share/doc/ghostctl/README.md"
    install -Dm644 "docs.md" "$pkgdir/usr/share/doc/ghostctl/docs.md"
    install -Dm644 "commands.md" "$pkgdir/usr/share/doc/ghostctl/commands.md"
    
    # Install license
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/ghostctl/LICENSE"
    
    # Install example scripts
    install -dm755 "$pkgdir/usr/share/ghostctl/examples"
    cp -r "scripts/"* "$pkgdir/usr/share/ghostctl/examples/"
    
    # Install man page
    install -Dm644 "man/ghostctl.1" "$pkgdir/usr/share/man/man1/ghostctl.1"
}

post_install() {
    echo "🎉 GhostCTL installed successfully!"
    echo ""
    echo "🚀 Getting Started:"
    echo "  • Run: ghostctl menu"
    echo "  • Config: ghostctl config show"
    echo "  • Help: ghostctl --help"
    echo ""
    echo "📚 Documentation: /usr/share/doc/ghostctl/"
    echo "🔧 Examples: /usr/share/ghostctl/examples/"
    echo ""
    echo "⚡ Optional: Enable user systemd timer for automated backups"
    echo "  systemctl --user enable --now ghostctl-backup.timer"
}

post_upgrade() {
    echo "🔄 GhostCTL upgraded successfully!"
    echo "📝 Check changelog: https://github.com/ghostkellz/ghostctl/releases"
    echo "⚙️  Update config if needed: ghostctl config show"
}

pre_remove() {
    # Stop and disable timers
    systemctl --user disable --now ghostctl-backup.timer 2>/dev/null || true
}
"#;

    let pkgbuild_content = pkgbuild_template.replace("pkgver=0.3.0", &format!("pkgver={version}"));

    if let Err(e) = fs::write(format!("{}/PKGBUILD", pkg_dir), pkgbuild_content) {
        eprintln!("Failed to write PKGBUILD: {}", e);
        return;
    }

    // Create .SRCINFO
    let srcinfo_template = r#"pkgbase = ghostctl
	pkgdesc = Comprehensive system administration toolkit for Linux power users, homelabbers, and DevOps professionals
	pkgver = 0.4.0
	pkgrel = 1
	url = https://github.com/ghostkellz/ghostctl
	arch = x86_64
	arch = aarch64
	license = MIT
	makedepends = cargo
	makedepends = pkgconf
	depends = rust
	depends = git
	depends = curl
	depends = docker
	depends = systemd
	optdepends = ansible: Infrastructure automation
	optdepends = terraform: Infrastructure as code
	optdepends = azure-cli: Azure cloud management
	optdepends = aws-cli: AWS cloud management
	optdepends = docker-compose: Container orchestration
	optdepends = minio: Object storage server
	optdepends = restic: Backup solution
	optdepends = btrfs-progs: Btrfs filesystem tools
	optdepends = snapper: Btrfs snapshot management
	optdepends = tailscale: Mesh networking
	optdepends = neovim: Advanced text editor
	optdepends = zsh: Advanced shell
	optdepends = tmux: Terminal multiplexer
	backup = etc/ghostctl/config.toml
	source = ghostctl-1.0.0.tar.gz::https://github.com/ghostkellz/ghostctl/archive/v1.0.0.tar.gz
	sha256sums = SKIP

pkgname = ghostctl
"#;

    let srcinfo_content = srcinfo_template
        .replace("\tpkgver = 0.4.0", &format!("\tpkgver = {version}"))
        .replace(
            "\tsource = ghostctl-1.0.0.tar.gz::https://github.com/ghostkellz/ghostctl/archive/v1.0.0.tar.gz",
            &format!(
                "\tsource = ghostctl-{version}.tar.gz::https://github.com/ghostkellz/ghostctl/archive/v{version}.tar.gz"
            ),
        );

    if let Err(e) = fs::write(format!("{}/.SRCINFO", pkg_dir), srcinfo_content) {
        eprintln!("Failed to write .SRCINFO: {}", e);
        return;
    }

    println!("📦 Created Arch package files in: {}", pkg_dir);
}

fn create_debian_package(debian_dir: &str) {
    let version = env!("CARGO_PKG_VERSION");

    let control_template = r#"Package: ghostctl
Version: 0.3.0
Section: admin
Priority: optional
Architecture: amd64
Depends: curl, git, docker.io, systemd
Recommends: ansible, terraform, azure-cli, restic, btrfs-progs, snapper, neovim, zsh, tmux
Suggests: minio, tailscale
Maintainer: Christopher Kelley <ckelley@ghostkellz.sh>
Description: Comprehensive system administration toolkit
 GhostCTL is a modular CLI toolkit for Linux power users, sysadmins,
 and homelabbers. It provides interactive and scriptable management for:
 .
  * Btrfs snapshots and Snapper integration
  * Restic backups with automated scheduling
  * Docker container and registry management
  * Infrastructure as Code (Ansible, Terraform)
  * Multi-cloud management (Azure, AWS, GCP)
  * Neovim configuration and plugin management
  * Shell setup (ZSH, Oh My Zsh, Powerlevel10k)
  * Proxmox VE helper scripts
  * System diagnostics and maintenance
  * Plugin system with Lua and shell script support
 .
 Built in Rust for performance and reliability.
Homepage: https://github.com/ghostkellz/ghostctl
"#;

    let control_content =
        control_template.replace("Version: 0.3.0", &format!("Version: {version}"));

    if let Err(e) = fs::write(format!("{}/control", debian_dir), control_content) {
        eprintln!("Failed to write debian/control: {}", e);
        return;
    }

    let postinst_content = r#"#!/bin/bash
set -e

echo "🎉 GhostCTL installed successfully!"
echo ""
echo "🚀 Getting Started:"
echo "  • Run: ghostctl menu"
echo "  • Config: ghostctl config show"
echo "  • Help: ghostctl --help"
echo ""
echo "📚 Documentation: /usr/share/doc/ghostctl/"
echo "🔧 Examples: /usr/share/ghostctl/examples/"
echo ""
echo "⚡ Optional: Enable user systemd timer for automated backups"
echo "  systemctl --user enable --now ghostctl-backup.timer"

# Create config directory
mkdir -p /etc/ghostctl
if [ ! -f /etc/ghostctl/config.toml ]; then
    cp /usr/share/ghostctl/config/default.toml /etc/ghostctl/config.toml
fi

# Enable bash completion
if [ -f /usr/share/bash-completion/completions/ghostctl ]; then
    echo "✅ Bash completion installed"
fi

exit 0
"#;

    if let Err(e) = fs::write(format!("{}/postinst", debian_dir), postinst_content) {
        eprintln!("Failed to write debian/postinst: {}", e);
        return;
    }

    let prerm_content = r#"#!/bin/bash
set -e

# Stop and disable timers
systemctl --user disable --now ghostctl-backup.timer 2>/dev/null || true

exit 0
"#;

    if let Err(e) = fs::write(format!("{}/prerm", debian_dir), prerm_content) {
        eprintln!("Failed to write debian/prerm: {}", e);
        return;
    }

    println!("📦 Created Debian package files in: {}", debian_dir);
}

fn create_universal_installer(install_dir: &str) {
    let installer_content = r#"#!/bin/bash
# GhostCTL Universal Installer
# Supports: Arch Linux, Ubuntu/Debian, Fedora/RHEL, CentOS, openSUSE
# Auto-detects system and installs appropriate dependencies

set -e

REPO="ghostkellz/ghostctl"
INSTALL_DIR="/usr/local/bin"
TEMP_DIR="/tmp/ghostctl-install"
VERSION="latest"
CONFIG_DIR="/etc/ghostctl"
USER_CONFIG_DIR="$HOME/.config/ghostctl"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${CYAN}"
    echo "██████  ██   ██  ██████  ███████ ████████  ██████ ████████ ██      "
    echo "██      ██   ██ ██    ██ ██         ██    ██         ██    ██      "
    echo "██████  ███████ ██    ██ ███████    ██    ██         ██    ██      "
    echo "██      ██   ██ ██    ██      ██    ██    ██         ██    ██      "
    echo "██████  ██   ██  ██████  ███████    ██     ██████    ██    ███████ "
    echo -e "${NC}"
    echo -e "${PURPLE}🎉 GhostCTL - Professional System Administration Toolkit${NC}"
    echo -e "${BLUE}🔗 https://github.com/$REPO${NC}"
    echo
}

detect_system() {
    if [[ -f /etc/arch-release ]]; then
        DISTRO="arch"
        PACKAGE_MANAGER="pacman"
    elif [[ -f /etc/debian_version ]]; then
        DISTRO="debian"
        PACKAGE_MANAGER="apt"
    elif [[ -f /etc/redhat-release ]]; then
        if grep -q "Fedora" /etc/redhat-release; then
            DISTRO="fedora"
            PACKAGE_MANAGER="dnf"
        else
            DISTRO="rhel"
            PACKAGE_MANAGER="yum"
        fi
    elif [[ -f /etc/SuSE-release ]] || [[ -f /etc/SUSE-brand ]]; then
        DISTRO="suse"
        PACKAGE_MANAGER="zypper"
    else
        DISTRO="unknown"
        PACKAGE_MANAGER="unknown"
    fi
    
    echo -e "${BLUE}🔍 Detected System: $DISTRO${NC}"
    echo -e "${BLUE}📦 Package Manager: $PACKAGE_MANAGER${NC}"
}

install_dependencies() {
    echo -e "${YELLOW}📦 Installing dependencies...${NC}"
    
    case $DISTRO in
        "arch")
            sudo pacman -S --needed --noconfirm \
                rust cargo git curl docker systemd \
                base-devel || {
                echo -e "${RED}❌ Failed to install dependencies${NC}"
                exit 1
            }
            ;;
        "debian")
            sudo apt update
            sudo apt install -y \
                curl git build-essential pkg-config libssl-dev \
                docker.io systemd || {
                echo -e "${RED}❌ Failed to install dependencies${NC}"
                exit 1
            }
            
            # Install Rust
            if ! command -v cargo &> /dev/null; then
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                source ~/.cargo/env
            fi
            ;;
        "fedora")
            sudo dnf install -y \
                rust cargo git curl docker systemd-devel \
                gcc pkg-config openssl-devel || {
                echo -e "${RED}❌ Failed to install dependencies${NC}"
                exit 1
            }
            ;;
        "rhel")
            sudo yum install -y \
                git curl docker systemd-devel \
                gcc pkg-config openssl-devel || {
                echo -e "${RED}❌ Failed to install dependencies${NC}"
                exit 1
            }
            
            # Install Rust
            if ! command -v cargo &> /dev/null; then
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                source ~/.cargo/env
            fi
            ;;
        "suse")
            sudo zypper install -y \
                rust cargo git curl docker systemd-devel \
                gcc pkg-config libopenssl-devel || {
                echo -e "${RED}❌ Failed to install dependencies${NC}"
                exit 1
            }
            ;;
        *)
            echo -e "${RED}❌ Unsupported distribution${NC}"
            echo -e "${YELLOW}💡 Manual installation required${NC}"
            echo "Please install: rust, cargo, git, curl, docker"
            exit 1
            ;;
    esac
}

try_package_install() {
    case $DISTRO in
        "arch")
            if command -v yay &> /dev/null; then
                echo -e "${YELLOW}📦 Trying AUR installation...${NC}"
                if yay -S ghostctl --noconfirm; then
                    return 0
                fi
            fi
            ;;
        "debian")
            echo -e "${YELLOW}📦 Checking for .deb package...${NC}"
            LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep -o '"tag_name": "[^"]*' | cut -d'"' -f4)
            DEB_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/ghostctl_${LATEST_RELEASE#v}_amd64.deb"
            
            if curl -f -L "$DEB_URL" -o "/tmp/ghostctl.deb" 2>/dev/null; then
                sudo dpkg -i /tmp/ghostctl.deb
                sudo apt-get install -f  # Fix dependencies if needed
                return 0
            fi
            ;;
    esac
    return 1
}

build_from_source() {
    echo -e "${YELLOW}🔧 Building GhostCTL from source...${NC}"
    
    # Clean temp directory
    rm -rf "$TEMP_DIR"
    mkdir -p "$TEMP_DIR"
    cd "$TEMP_DIR"
    
    # Get latest release
    if [[ $VERSION == "latest" ]]; then
        VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep -o '"tag_name": "[^"]*' | cut -d'"' -f4)
    fi
    
    echo -e "${BLUE}📥 Downloading GhostCTL $VERSION...${NC}"
    
    # Try release tarball first
    if curl -f -L "https://github.com/$REPO/archive/$VERSION.tar.gz" -o "ghostctl-$VERSION.tar.gz"; then
        tar -xzf "ghostctl-$VERSION.tar.gz"
        cd "ghostctl-${VERSION#v}"
    else
        # Fallback to git clone
        echo -e "${YELLOW}📥 Cloning repository...${NC}"
        git clone "https://github.com/$REPO.git" .
        if [[ $VERSION != "latest" ]]; then
            git checkout "$VERSION"
        fi
    fi
    
    # Build
    echo -e "${BLUE}🔨 Building (this may take 5-10 minutes)...${NC}"
    cd ghostctl
    
    # Use system Rust or installed Rust
    if [[ -f ~/.cargo/env ]]; then
        source ~/.cargo/env
    fi
    
    cargo build --release || {
        echo -e "${RED}❌ Build failed${NC}"
        exit 1
    }
    
    # Install binary
    echo -e "${BLUE}📦 Installing...${NC}"
    sudo install -Dm755 target/release/ghostctl "$INSTALL_DIR/ghostctl"
    
    # Install shell completions
    sudo mkdir -p /usr/share/bash-completion/completions
    sudo mkdir -p /usr/share/zsh/site-functions
    sudo mkdir -p /usr/share/fish/vendor_completions.d
    
    if [[ -d ../completions ]]; then
        sudo cp ../completions/ghostctl.bash /usr/share/bash-completion/completions/ghostctl 2>/dev/null || true
        sudo cp ../completions/_ghostctl /usr/share/zsh/site-functions/_ghostctl 2>/dev/null || true
        sudo cp ../completions/ghostctl.fish /usr/share/fish/vendor_completions.d/ghostctl.fish 2>/dev/null || true
    fi
    
    # Install documentation
    sudo mkdir -p /usr/share/doc/ghostctl
    sudo cp ../README.md /usr/share/doc/ghostctl/ 2>/dev/null || true
    sudo cp ../docs.md /usr/share/doc/ghostctl/ 2>/dev/null || true
    sudo cp ../commands.md /usr/share/doc/ghostctl/ 2>/dev/null || true
}

setup_configuration() {
    echo -e "${BLUE}⚙️  Setting up configuration...${NC}"
    
    # System config
    sudo mkdir -p "$CONFIG_DIR"
    if [[ ! -f "$CONFIG_DIR/config.toml" ]]; then
        sudo tee "$CONFIG_DIR/config.toml" > /dev/null <<EOF
[general]
github_user = "ghostkellz"
default_editor = "nano"
log_level = "info"
auto_update_check = true

[backup]
default_paths = ["/home", "/etc"]
exclude_patterns = ["*.tmp", "*.cache"]
retention_daily = 7
retention_weekly = 4
retention_monthly = 12

[scripts]
local_scripts_dir = "~/.config/ghostctl/scripts"
auto_discover = true
trusted_sources = ["https://github.com/ghostkellz/ghostctl"]

[ghost_tools]
auto_install_deps = true
preferred_build_jobs = 4
install_location = "/usr/bin"

[ui]
theme = "default"
show_tips = true
confirmation_prompts = true
EOF
    fi
    
    # User config
    mkdir -p "$USER_CONFIG_DIR"/{scripts,plugins}
    mkdir -p ~/.local/share/ghostctl
    
    # Create example local script
    if [[ ! -f "$USER_CONFIG_DIR/scripts/example.sh" ]]; then
        cat > "$USER_CONFIG_DIR/scripts/example.sh" <<'EOF'
#!/bin/bash
# Example local script
echo "🎉 This is an example local script!"
echo "📁 Located in: ~/.config/ghostctl/scripts/"
echo "✏️  Edit this file to create your own scripts"
EOF
        chmod +x "$USER_CONFIG_DIR/scripts/example.sh"
    fi
}

create_desktop_entry() {
    echo -e "${BLUE}🖥️  Creating desktop entry...${NC}"
    sudo tee /usr/share/applications/ghostctl.desktop > /dev/null <<EOF
[Desktop Entry]
Name=GhostCTL
Comment=System Administration Toolkit
Exec=ghostctl menu
Icon=utilities-terminal
Terminal=true
Type=Application
Categories=System;Settings;Network;
Keywords=system;admin;terminal;devops;docker;ansible;terraform;
StartupNotify=false
EOF
}

setup_systemd_services() {
    echo -e "${BLUE}⚙️  Setting up systemd services...${NC}"
    
    # User backup service
    mkdir -p ~/.config/systemd/user
    
    cat > ~/.config/systemd/user/ghostctl-backup.service <<EOF
[Unit]
Description=GhostCTL Automated Backup
After=network-online.target

[Service]
Type=oneshot
ExecStart=$INSTALL_DIR/ghostctl backup run
Environment=HOME=%h
WorkingDirectory=%h

[Install]
WantedBy=default.target
EOF

    cat > ~/.config/systemd/user/ghostctl-backup.timer <<EOF
[Unit]
Description=GhostCTL Backup Timer
Requires=ghostctl-backup.service

[Timer]
OnCalendar=daily
Persistent=true
RandomizedDelaySec=3600

[Install]
WantedBy=timers.target
EOF

    systemctl --user daemon-reload
    
    echo -e "${GREEN}✅ Systemd services created${NC}"
    echo -e "${YELLOW}💡 Enable with: systemctl --user enable --now ghostctl-backup.timer${NC}"
}

install_optional_tools() {
    echo -e "${BLUE}🔧 Installing recommended tools...${NC}"
    
    read -p "Install optional tools (ansible, terraform, azure-cli)? [Y/n]: " -r
    if [[ ! $REPLY =~ ^[Nn]$ ]]; then
        case $DISTRO in
            "arch")
                sudo pacman -S --needed --noconfirm ansible terraform 2>/dev/null || true
                if command -v yay &> /dev/null; then
                    yay -S --noconfirm azure-cli 2>/dev/null || true
                fi
                ;;
            "debian")
                # Install Ansible
                sudo apt install -y ansible 2>/dev/null || true
                
                # Install Terraform
                curl -fsSL https://apt.releases.hashicorp.com/gpg | sudo apt-key add -
                sudo apt-add-repository "deb [arch=amd64] https://apt.releases.hashicorp.com $(lsb_release -cs) main" 2>/dev/null || true
                sudo apt update && sudo apt install -y terraform 2>/dev/null || true
                
                # Install Azure CLI
                curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash 2>/dev/null || true
                ;;
            *)
                echo -e "${YELLOW}💡 Manual installation required for optional tools${NC}"
                ;;
        esac
    fi
}

cleanup() {
    echo -e "${BLUE}🧹 Cleaning up...${NC}"
    rm -rf "$TEMP_DIR"
}

main() {
    print_header
    
    # Check if already installed
    if command -v ghostctl &> /dev/null; then
        echo -e "${GREEN}✅ GhostCTL is already installed!${NC}"
        echo -e "${BLUE}🔄 Current version: $(ghostctl --version 2>/dev/null || echo 'unknown')${NC}"
        echo
        read -p "🤔 Reinstall/update? [y/N]: " -r
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 0
        fi
    fi
    
    # Check for root privileges for system installation
    if [[ $EUID -eq 0 ]]; then
        echo -e "${RED}❌ Don't run as root! This installer will use sudo when needed.${NC}"
        exit 1
    fi
    
    detect_system
    install_dependencies
    
    # Try package installation first
    if try_package_install; then
        echo -e "${GREEN}✅ Package installation successful!${NC}"
    else
        build_from_source
    fi
    
    setup_configuration
    create_desktop_entry
    setup_systemd_services
    install_optional_tools
    cleanup
    
    echo
    echo -e "${GREEN}🎉 GhostCTL installation complete!${NC}"
    echo
    echo -e "${CYAN}🚀 Getting Started:${NC}"
    echo -e "${YELLOW}  • Interactive Menu: ${GREEN}ghostctl menu${NC}"
    echo -e "${YELLOW}  • View Config: ${GREEN}ghostctl config show${NC}"
    echo -e "${YELLOW}  • Help: ${GREEN}ghostctl --help${NC}"
    echo -e "${YELLOW}  • Documentation: ${GREEN}/usr/share/doc/ghostctl/${NC}"
    echo
    echo -e "${CYAN}🔧 Optional Setup:${NC}"
    echo -e "${YELLOW}  • Enable daily backups: ${GREEN}systemctl --user enable --now ghostctl-backup.timer${NC}"
    echo -e "${YELLOW}  • Shell integration: ${GREEN}ghostctl shell${NC}"
    echo -e "${YELLOW}  • Docker setup: ${GREEN}ghostctl devops docker${NC}"
    echo
    echo -e "${BLUE}📚 Resources:${NC}"
    echo -e "${YELLOW}  • GitHub: ${GREEN}https://github.com/$REPO${NC}"
    echo -e "${YELLOW}  • Issues: ${GREEN}https://github.com/$REPO/issues${NC}"
    echo
    
    # Verify installation
    if command -v ghostctl &> /dev/null; then
        echo -e "${GREEN}✅ Installation verified!${NC}"
        ghostctl --version
    else
        echo -e "${RED}❌ Installation verification failed${NC}"
        echo -e "${YELLOW}💡 Try logging out and back in, or run: source ~/.bashrc${NC}"
        exit 1
    fi
}

# Handle command line arguments
case "${1:-}" in
    --help|-h)
        echo "GhostCTL Universal Installer"
        echo
        echo "Usage: $0 [options]"
        echo
        echo "Options:"
        echo "  --help, -h     Show this help"
        echo "  --version VER  Install specific version"
        echo "  --no-optional  Skip optional tools"
        echo
        exit 0
        ;;
    --version)
        VERSION="$2"
        ;;
    --no-optional)
        SKIP_OPTIONAL=1
        ;;
esac

main "$@"
"#;

    let install_path = format!("{}/install.sh", install_dir);
    if let Err(e) = fs::write(&install_path, installer_content) {
        eprintln!("Failed to write install.sh: {}", e);
        return;
    }

    // Make executable
    use std::os::unix::fs::PermissionsExt;
    if let Ok(metadata) = fs::metadata(&install_path) {
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        if let Err(e) = fs::set_permissions(&install_path, perms) {
            eprintln!("Failed to set permissions on install.sh: {}", e);
        }
    }

    println!("🚀 Created universal installer: {}/install.sh", install_dir);
}

fn create_proxmox_installer(install_dir: &str) {
    let proxmox_installer = r#"#!/bin/bash
# GhostCTL Proxmox VE Installer
# Optimized for Proxmox VE 7.x and 8.x

set -e

echo "🏥 GhostCTL Proxmox VE Installation"
echo "================================="

# Check if running on Proxmox
if [[ ! -f /etc/pve/local/pve-ssl.pem ]]; then
    echo "❌ This doesn't appear to be a Proxmox VE system"
    echo "💡 Use the universal installer instead"
    exit 1
fi

echo "✅ Proxmox VE detected"

# Install dependencies
echo "📦 Installing dependencies..."
apt update
apt install -y curl git build-essential pkg-config libssl-dev

# Install Rust
if ! command -v cargo &> /dev/null; then
    echo "🦀 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
fi

# Download and build GhostCTL
TEMP_DIR="/tmp/ghostctl-proxmox-install"
rm -rf "$TEMP_DIR"
mkdir -p "$TEMP_DIR"
cd "$TEMP_DIR"

echo "📥 Downloading GhostCTL..."
git clone https://github.com/ghostkellz/ghostctl.git .

echo "🔨 Building..."
cd ghostctl
cargo build --release

echo "📦 Installing..."
install -Dm755 target/release/ghostctl /usr/local/bin/ghostctl

# Proxmox-specific setup
echo "🏥 Setting up Proxmox integration..."

# Create Proxmox-specific config
mkdir -p /etc/ghostctl
cat > /etc/ghostctl/proxmox.toml <<EOF
[proxmox]
enabled = true
api_endpoint = "https://localhost:8006"
node_name = "$(hostname)"
backup_storage = "local"
iso_storage = "local"

[containers]
default_template = "debian-12"
default_bridge = "vmbr0"
default_storage = "local-lvm"

[vms]
default_iso_storage = "local"
default_disk_storage = "local-lvm"
default_bridge = "vmbr0"
EOF

# Create systemd service for Proxmox monitoring
cat > /etc/systemd/system/ghostctl-proxmox.service <<EOF
[Unit]
Description=GhostCTL Proxmox Monitor
After=pve-cluster.service

[Service]
Type=oneshot
ExecStart=/usr/local/bin/ghostctl proxmox health-check
User=root

[Install]
WantedBy=multi-user.target
EOF

cat > /etc/systemd/system/ghostctl-proxmox.timer <<EOF
[Unit]
Description=GhostCTL Proxmox Monitor Timer
Requires=ghostctl-proxmox.service

[Timer]
OnCalendar=hourly
Persistent=true

[Install]
WantedBy=timers.target
EOF

systemctl daemon-reload
systemctl enable ghostctl-proxmox.timer

# Install Proxmox helper scripts locally
mkdir -p /opt/ghostctl/proxmox-scripts
cd /opt/ghostctl/proxmox-scripts

# Download community scripts index
curl -s https://raw.githubusercontent.com/community-scripts/ProxmoxVE/main/README.md > scripts-index.md

echo "✅ Proxmox VE installation complete!"
echo ""
echo "🚀 Getting Started:"
echo "  • Proxmox Menu: ghostctl proxmox menu"
echo "  • Health Check: ghostctl proxmox health-check"
echo "  • System Monitor: systemctl status ghostctl-proxmox.timer"
echo ""
echo "🔧 Proxmox-specific features:"
echo "  • Container management"
echo "  • VM deployment"
echo "  • Storage monitoring"
echo "  • Backup automation"
echo "  • Network configuration"

# Cleanup
rm -rf "$TEMP_DIR"
"#;

    let proxmox_path = format!("{}/install-proxmox.sh", install_dir);
    if let Err(e) = fs::write(&proxmox_path, proxmox_installer) {
        eprintln!("Failed to write install-proxmox.sh: {}", e);
        return;
    }

    // Make executable
    use std::os::unix::fs::PermissionsExt;
    if let Ok(metadata) = fs::metadata(&proxmox_path) {
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        if let Err(e) = fs::set_permissions(&proxmox_path, perms) {
            eprintln!("Failed to set permissions on install-proxmox.sh: {}", e);
        }
    }

    println!(
        "🏥 Created Proxmox installer: {}/install-proxmox.sh",
        install_dir
    );
}
