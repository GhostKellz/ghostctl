#!/bin/bash
# Rust Development Environment Setup
# Sets up a complete Rust development environment with tools and configurations

set -e

echo "🦀 Setting up Rust Development Environment..."

# Install rustup if not present
if ! command -v rustup &> /dev/null; then
    echo "📦 Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
else
    echo "✅ rustup already installed"
fi

# Update Rust
echo "🔄 Updating Rust toolchain..."
rustup update stable
rustup default stable

# Install essential components
echo "🔧 Installing Rust components..."
rustup component add clippy rustfmt rust-analyzer

# Install common cargo tools
echo "📦 Installing cargo tools..."
cargo install cargo-watch cargo-edit cargo-audit cargo-outdated

# Create .gitignore template for Rust projects
echo "📝 Creating Rust .gitignore template..."
cat > ~/rust-gitignore-template << 'EOF'
# Rust
/target/
**/*.rs.bk
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db
EOF

# Configure git for Rust development
echo "⚙️  Configuring git for Rust..."
git config --global init.defaultBranch main
git config --global core.autocrlf input

echo "✅ Rust development environment setup complete!"
echo "💡 Use 'cargo new project_name' to create a new project"
echo "🔧 Available tools: cargo-watch, cargo-edit, cargo-audit, cargo-outdated"