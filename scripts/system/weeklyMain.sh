#!/bin/bash
# Weekly System Maintenance Script
# Runs system cleanup, Btrfs maintenance, dev env checkups, etc.

set -euo pipefail
echo "🛠️ Starting weekly maintenance: $(date)"

# 1. Update Mirrorlist
echo "🔄 Updating mirrors..."
reflector --country 'US' --age 12 --protocol https --sort rate --save /etc/pacman.d/mirrorlist

# 2. Update System (Pacman + AUR)
echo "📦 Updating system packages..."
pacman -Syu --noconfirm

echo "📦 Updating AUR packages..."
if command -v paru &>/dev/null; then
    paru -Syu --noconfirm
elif command -v yay &>/dev/null; then
    yay -Syu --noconfirm
fi

# 3. Remove Orphans
echo "🧹 Removing orphaned packages..."
pacman -Qtdq | xargs -r sudo pacman -Rns --

# 4. Pacman Cache Cleanup
echo "🧽 Cleaning pacman cache..."
paccache -rk2
pacman -Sc --noconfirm

# 5. Journal Cleanup
echo "🧾 Cleaning journal logs..."
journalctl --vacuum-time=7d

# 6. Btrfs Maintenance
echo "🧬 Running Btrfs scrub..."
btrfs scrub start -Bd /

echo "📊 Running Btrfs balance (75% usage)..."
btrfs balance start -dusage=75 -musage=75 /

# 7. DKMS (NVIDIA / Custom Kernel)
echo "🔧 Checking DKMS modules..."
dkms autoinstall

# 8. Font Cache Rebuild
echo "🔤 Rebuilding font cache..."
fc-cache -rv

# 9. Optional: Trim SSD (if not using autotrim)
echo "✂️ Running fstrim..."
fstrim -av

# 10. Check Failed Services
echo "🚨 Checking failed services..."
systemctl --failed

# 11. Check for large trash files
echo "🗑️ Checking for trash..."
trash-empty 7 &>/dev/null || echo "Install 'trash-cli' to auto-clear trash"

# 12. Verify Gaming/Dev Binaries
echo "🎮 Verifying GPU, Vulkan, dev tools..."
nvidia-smi
vulkaninfo | grep -i deviceName || echo "vulkaninfo missing"
for tool in rustc go zig python3 node; do
    command -v $tool >/dev/null && echo "$tool: $($tool --version)" || echo "$tool not found"
done

echo "✅ Weekly maintenance complete: $(date)"

