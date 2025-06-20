#!/bin/bash
# Network Connectivity and Performance Diagnostics
# Comprehensive network troubleshooting and performance testing

set -e

echo "🌐 Network Diagnostics Suite"
echo "============================"

# Basic connectivity tests
echo "🔍 Basic Connectivity Tests:"
echo "----------------------------"

# Test DNS resolution
echo -n "DNS Resolution (8.8.8.8): "
if nslookup google.com 8.8.8.8 &>/dev/null; then
    echo "✅ OK"
else
    echo "❌ FAILED"
fi

# Test internet connectivity
echo -n "Internet Connectivity: "
if ping -c 1 8.8.8.8 &>/dev/null; then
    echo "✅ OK"
else
    echo "❌ FAILED"
fi

# Test local gateway
GATEWAY=$(ip route | grep default | awk '{print $3}' | head -1)
echo -n "Gateway ($GATEWAY): "
if ping -c 1 $GATEWAY &>/dev/null; then
    echo "✅ OK"
else
    echo "❌ FAILED"
fi

# Network interface information
echo -e "\n🔧 Network Interface Information:"
echo "--------------------------------"
ip addr show | grep -E '^[0-9]+: |inet '

# Active connections
echo -e "\n🔗 Active Network Connections:"
echo "-----------------------------"
ss -tuln | head -10

# DNS servers
echo -e "\n🌐 DNS Configuration:"
echo "-------------------"
cat /etc/resolv.conf

# Network performance test
echo -e "\n⚡ Speed Test (to 8.8.8.8):"
echo "--------------------------"
echo "Ping test (10 packets):"
ping -c 10 8.8.8.8 | tail -1

# Port connectivity test
echo -e "\n🚪 Common Port Tests:"
echo "-------------------"
ports=("80" "443" "53" "22")
for port in "${ports[@]}"; do
    echo -n "Port $port: "
    if timeout 3 bash -c "echo >/dev/tcp/8.8.8.8/$port" 2>/dev/null; then
        echo "✅ Open"
    else
        echo "❌ Closed/Filtered"
    fi
done

# Firewall status
echo -e "\n🛡️  Firewall Status:"
echo "------------------"
if command -v ufw &> /dev/null; then
    sudo ufw status
elif command -v firewalld &> /dev/null; then
    sudo firewall-cmd --state
    sudo firewall-cmd --list-all
else
    echo "iptables rules:"
    sudo iptables -L INPUT -n | head -5
fi

echo -e "\n✅ Network diagnostics complete!"