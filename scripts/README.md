# GhostCTL Scripts

This directory contains curated scripts organized by category for use with GhostCTL.

## Directory Structure

- `development/` - Development environment setup and tools
- `homelab/` - Homelab management and automation scripts  
- `network/` - Network configuration and diagnostics
- `system/` - System maintenance and optimization
- `sysadmin/` - Professional/work scripts (user mgmt, compliance, enterprise)
- `etc/` - Miscellaneous and experimental scripts

## Usage

Scripts can be accessed via:
- `ghostctl menu` → "Plugin & Script Management"
- `ghostctl script <url>` - Run any script from URL

## Adding Scripts

1. Place scripts in the appropriate category directory
2. Use descriptive filenames (e.g., `setup-docker-dev.sh`)
3. Include shebang line for proper interpreter detection
4. Add comments explaining what the script does

## Script Types Supported

- Shell scripts (`.sh`) - Most common
- Python scripts (`.py`) - For complex automation
- Lua scripts (`.lua`) - For lightweight scripting

Scripts are automatically discovered and added to the TUI menu.