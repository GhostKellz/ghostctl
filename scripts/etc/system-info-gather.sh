#!/bin/bash
# Quick System Information Gatherer
# Collects useful system information for troubleshooting or documentation

set -e

echo "📋 Quick System Information Report"
echo "================================="

# System identification
echo "🖥️  System Information:"
echo "---------------------"
echo "Hostname: $(hostname)"
echo "OS: $(cat /etc/os-release | grep PRETTY_NAME | cut -d'"' -f2)"
echo "Kernel: $(uname -r)"
echo "Architecture: $(uname -m)"
echo "Uptime: $(uptime | awk -F'up ' '{print $2}' | awk -F',' '{print $1}')"

# Hardware info
echo -e "\n🔧 Hardware:"
echo "-----------"
echo "CPU: $(lscpu | grep 'Model name' | cut -d':' -f2 | xargs)"
echo "CPU Cores: $(nproc)"
echo "RAM: $(free -h | grep Mem | awk '{print $2}')"

# Storage info
echo -e "\n💾 Storage:"
echo "----------"
lsblk -f | grep -E '^[a-z]' | head -5

# Network info
echo -e "\n🌐 Network:"
echo "----------"
ip -4 addr show | grep -E 'inet.*scope global' | awk '{print $2}' | head -3

# Quick status checks
echo -e "\n⚡ Quick Status:"
echo "--------------"
echo "Load: $(cat /proc/loadavg | awk '{print $1, $2, $3}')"
echo "Memory: $(free | grep Mem | awk '{printf "%.1f%%", $3/$2 * 100.0}')"

# Largest directories
echo -e "\n📁 Largest Directories in /:"
echo "---------------------------"
sudo du -h --max-depth=1 / 2>/dev/null | sort -hr | head -5

# Running services
echo -e "\n🔄 Active Services (top 5):"
echo "-------------------------"
systemctl list-units --state=active --type=service --no-pager | head -7 | tail -5

# Recent logs
echo -e "\n📝 Recent Important Logs:"
echo "-----------------------"
journalctl -p err --since "1 hour ago" --no-pager | tail -3 | head -3 || echo "No recent errors"

timestamp=$(date '+%Y-%m-%d_%H-%M-%S')
report_file="/tmp/system-info-$timestamp.txt"

echo -e "\n💾 Save full report to file? (y/N)"
read -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Generate detailed report
    {
        echo "System Information Report - $(date)"
        echo "=================================="
        echo
        hostnamectl 2>/dev/null || uname -a
        echo
        echo "Memory:"
        free -h
        echo
        echo "Disk Usage:"
        df -h
        echo
        echo "Network Interfaces:"
        ip addr show
        echo
        echo "Active Services:"
        systemctl list-units --state=active --type=service --no-pager
    } > "$report_file"
    
    echo "✅ Full report saved to: $report_file"
fi

echo -e "\n✅ System information gathering complete!"