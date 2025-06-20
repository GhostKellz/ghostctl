#!/bin/bash
# System Health Check and Maintenance
# Comprehensive system health monitoring and cleanup

set -e

echo "⚕️  System Health Check & Maintenance"
echo "===================================="

# System uptime and load
echo "⏰ System Status:"
echo "----------------"
echo "Uptime: $(uptime)"
echo "Load Average: $(cat /proc/loadavg)"

# Memory usage
echo -e "\n💾 Memory Usage:"
echo "---------------"
free -h

# Disk usage
echo -e "\n💿 Disk Usage:"
echo "-------------"
df -h | grep -E '^/dev/'

# Check for filesystems over 80% full
echo -e "\n⚠️  Disk Space Warnings:"
df -h | awk 'NR>1 && $5+0 > 80 {print "WARNING: " $6 " is " $5 " full"}'

# Check system services
echo -e "\n🔧 Failed Services:"
echo "------------------"
systemctl list-units --failed --no-pager

# Check journal disk usage
echo -e "\n📝 Journal Disk Usage:"
echo "---------------------"
journalctl --disk-usage

# Temperature check (if available)
if command -v sensors &> /dev/null; then
    echo -e "\n🌡️  Temperature:"
    echo "--------------"
    sensors | grep -E '(Core|temp|Package)'
fi

# Check for package updates
echo -e "\n📦 Package Updates:"
echo "------------------"
if command -v apt &> /dev/null; then
    apt list --upgradable 2>/dev/null | wc -l | xargs echo "Available updates:"
elif command -v pacman &> /dev/null; then
    checkupdates 2>/dev/null | wc -l | xargs echo "Available updates:"
elif command -v dnf &> /dev/null; then
    dnf check-update -q | wc -l | xargs echo "Available updates:"
fi

# Cleanup options
echo -e "\n🧹 Maintenance Options:"
echo "---------------------"

read -p "Clean package cache? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if command -v apt &> /dev/null; then
        sudo apt autoremove -y && sudo apt autoclean
    elif command -v pacman &> /dev/null; then
        sudo pacman -Sc --noconfirm
    elif command -v dnf &> /dev/null; then
        sudo dnf clean all
    fi
    echo "✅ Package cache cleaned"
fi

read -p "Clean journal logs older than 7 days? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    sudo journalctl --vacuum-time=7d
    echo "✅ Old journal logs cleaned"
fi

read -p "Clean temporary files? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    sudo find /tmp -type f -atime +7 -delete 2>/dev/null || true
    sudo find /var/tmp -type f -atime +7 -delete 2>/dev/null || true
    echo "✅ Temporary files cleaned"
fi

echo -e "\n✅ System maintenance complete!"
echo "💡 Consider rebooting if updates were installed"