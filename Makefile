# ghostctl Makefile

.PHONY: all build install test fmt clippy audit dev dev-cycle release clean completions package-arch help

CARGO_DIR = ghostctl

# Default target
all: build

# Build targets
build:
	@echo "Building ghostctl (release)..."
	cd $(CARGO_DIR) && cargo build --release

dev:
	@echo "Building ghostctl (debug)..."
	cd $(CARGO_DIR) && cargo build

# Installation
install: build
	@echo "Installing ghostctl to ~/.local/bin..."
	mkdir -p ~/.local/bin
	install -m755 $(CARGO_DIR)/target/release/ghostctl ~/.local/bin/ghostctl
	@echo "ghostctl installed to ~/.local/bin/ghostctl"

# Quality checks
test:
	cd $(CARGO_DIR) && cargo test

fmt:
	cd $(CARGO_DIR) && cargo fmt

fmt-check:
	cd $(CARGO_DIR) && cargo fmt --check

clippy:
	cd $(CARGO_DIR) && cargo clippy --release -- -D warnings

audit:
	cd $(CARGO_DIR) && cargo audit

# Development workflow
dev-cycle: fmt clippy test dev
	@echo "Development cycle complete"

release: clean fmt-check clippy test build
	@echo "Release build ready"

# Shell completions
completions: build
	@mkdir -p completions
	$(CARGO_DIR)/target/release/ghostctl completion bash > completions/ghostctl.bash
	$(CARGO_DIR)/target/release/ghostctl completion zsh > completions/_ghostctl
	$(CARGO_DIR)/target/release/ghostctl completion fish > completions/ghostctl.fish
	@echo "Completions generated in completions/"

# Packaging
package-arch:
	cd packaging/arch && makepkg -si

# Cleanup
clean:
	cd $(CARGO_DIR) && cargo clean

# Help
help:
	@echo "ghostctl Makefile"
	@echo "================="
	@echo ""
	@echo "Build:"
	@echo "  build         Build release binary"
	@echo "  dev           Build debug binary"
	@echo "  clean         Clean build artifacts"
	@echo ""
	@echo "Install:"
	@echo "  install       Install to ~/.local/bin"
	@echo ""
	@echo "Quality:"
	@echo "  test          Run tests"
	@echo "  fmt           Format code"
	@echo "  fmt-check     Check formatting"
	@echo "  clippy        Run linter (zero-warning policy)"
	@echo "  audit         Run cargo audit"
	@echo "  dev-cycle     fmt + clippy + test + dev build"
	@echo ""
	@echo "Release:"
	@echo "  release       Full release build (clean + checks + build)"
	@echo "  completions   Generate shell completions"
	@echo "  package-arch  Build Arch package"
