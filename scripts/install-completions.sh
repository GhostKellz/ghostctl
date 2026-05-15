#!/bin/bash
# Shell completion installation script for ghostctl

set -e

COMPLETION_DIR=""
SHELL_TYPE=""

# Detect shell
if [[ -n "$ZSH_VERSION" ]]; then
    SHELL_TYPE="zsh"
    COMPLETION_DIR="/usr/share/zsh/site-functions"
elif [[ -n "$BASH_VERSION" ]]; then
    SHELL_TYPE="bash"
    COMPLETION_DIR="/usr/share/bash-completion/completions"
elif [[ -n "$FISH_VERSION" ]]; then
    SHELL_TYPE="fish"
    COMPLETION_DIR="/usr/share/fish/vendor_completions.d"
else
    echo "Unsupported shell. Please specify shell type:"
    echo "Usage: $0 [bash|zsh|fish]"
    exit 1
fi

# Allow override via command line
if [[ $# -gt 0 ]]; then
    SHELL_TYPE="$1"
    case "$SHELL_TYPE" in
        bash)
            COMPLETION_DIR="/usr/share/bash-completion/completions"
            ;;
        zsh)
            COMPLETION_DIR="/usr/share/zsh/site-functions"
            ;;
        fish)
            COMPLETION_DIR="/usr/share/fish/vendor_completions.d"
            ;;
        *)
            echo "Unsupported shell: $SHELL_TYPE"
            echo "Supported shells: bash, zsh, fish"
            exit 1
            ;;
    esac
fi

echo "Installing $SHELL_TYPE completions for ghostctl..."

# Create completion directory if it doesn't exist
sudo mkdir -p "$COMPLETION_DIR"

# Generate and install completion script
if command -v ghostctl >/dev/null 2>&1; then
    case "$SHELL_TYPE" in
        bash)
            ghostctl completion bash | sudo tee "$COMPLETION_DIR/ghostctl" > /dev/null
            ;;
        zsh)
            ghostctl completion zsh | sudo tee "$COMPLETION_DIR/_ghostctl" > /dev/null
            ;;
        fish)
            ghostctl completion fish | sudo tee "$COMPLETION_DIR/ghostctl.fish" > /dev/null
            ;;
    esac

    echo "Completions installed to $COMPLETION_DIR"
    echo "Please restart your shell or run 'source ~/.${SHELL_TYPE}rc' to enable completions"
else
    echo "Error: ghostctl not found in PATH. Please install ghostctl first."
    exit 1
fi
