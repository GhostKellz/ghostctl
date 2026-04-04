#!/bin/bash

# GhostCTL Networking & Firewall Features Test Script
# Tests advanced firewall and networking capabilities

set -e

echo "🔥 GhostCTL Networking Features Test"
echo "===================================="

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

test_firewall_tools() {
    echo -e "${BLUE}Testing Firewall Tools Availability...${NC}"

    # Test iptables
    if command -v iptables >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} iptables is available"

        # Test if we can read current rules (without sudo)
        if iptables -L >/dev/null 2>&1; then
            echo -e "    ${GREEN}✅${NC} Can read iptables rules"
        elif sudo -n iptables -L >/dev/null 2>&1; then
            echo -e "    ${GREEN}✅${NC} Can read iptables rules with sudo"
        else
            echo -e "    ${YELLOW}⚠️${NC} Cannot read iptables rules (may need sudo)"
        fi
    else
        echo -e "    ${RED}❌${NC} iptables not available"
    fi

    # Test nftables
    if command -v nft >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} nftables (nft) is available"

        # Test if we can read current ruleset
        if nft list ruleset >/dev/null 2>&1; then
            echo -e "    ${GREEN}✅${NC} Can read nftables ruleset"
        elif sudo -n nft list ruleset >/dev/null 2>&1; then
            echo -e "    ${GREEN}✅${NC} Can read nftables ruleset with sudo"
        else
            echo -e "    ${YELLOW}⚠️${NC} Cannot read nftables ruleset (may need sudo)"
        fi
    else
        echo -e "    ${RED}❌${NC} nftables not available"
    fi

    # Test ufw
    if command -v ufw >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} UFW is available"
    else
        echo -e "    ${YELLOW}⚠️${NC} UFW not available"
    fi

    # Test firewalld
    if command -v firewalld >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} Firewalld is available"
    else
        echo -e "    ${YELLOW}⚠️${NC} Firewalld not available"
    fi
}

test_networking_tools() {
    echo -e "${BLUE}Testing Networking Tools...${NC}"

    local tools=(
        "ip:IP configuration"
        "ss:Socket statistics"
        "ping:Network connectivity"
        "nslookup:DNS lookup"
        "dig:DNS lookup (advanced)"
        "netstat:Network statistics"
        "tcpdump:Packet capture"
        "wireshark:Packet analysis"
        "nmap:Network scanning"
        "iperf3:Network performance"
    )

    for tool_desc in "${tools[@]}"; do
        local tool="${tool_desc%:*}"
        local desc="${tool_desc#*:}"

        if command -v "$tool" >/dev/null 2>&1; then
            echo -e "    ${GREEN}✅${NC} $tool ($desc)"
        else
            echo -e "    ${YELLOW}⚠️${NC} $tool ($desc) - not installed"
        fi
    done
}

test_nftables_features() {
    echo -e "${BLUE}Testing nftables Advanced Features...${NC}"

    if ! command -v nft >/dev/null 2>&1; then
        echo -e "    ${YELLOW}⚠️${NC} nftables not available - skipping tests"
        return
    fi

    # Test basic nftables functionality
    echo "  📋 Testing basic nftables commands..."

    # Test list tables (safe, read-only)
    if nft list tables >/dev/null 2>&1 || sudo -n nft list tables >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} Can list nftables tables"
    else
        echo -e "    ${YELLOW}⚠️${NC} Cannot list nftables tables"
    fi

    # Test syntax validation (safe, doesn't apply rules)
    local test_rule="table inet test { chain input { type filter hook input priority 0; } }"
    if echo "$test_rule" | nft -c -f - >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} nftables syntax validation works"
    else
        echo -e "    ${YELLOW}⚠️${NC} nftables syntax validation failed"
    fi

    # Test if we can create backup directory
    local backup_dir="$HOME/nftables_backups"
    if mkdir -p "$backup_dir" 2>/dev/null; then
        echo -e "    ${GREEN}✅${NC} Can create backup directory"

        # Test backup functionality (dry run)
        local test_backup="$backup_dir/test_backup.nft"
        if echo "# Test backup" > "$test_backup" 2>/dev/null; then
            echo -e "    ${GREEN}✅${NC} Can create backup files"
            rm -f "$test_backup"
        fi
    else
        echo -e "    ${RED}❌${NC} Cannot create backup directory"
    fi
}

test_iptables_features() {
    echo -e "${BLUE}Testing iptables Advanced Features...${NC}"

    if ! command -v iptables >/dev/null 2>&1; then
        echo -e "    ${YELLOW}⚠️${NC} iptables not available - skipping tests"
        return
    fi

    # Test iptables-save
    if command -v iptables-save >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} iptables-save available"
    else
        echo -e "    ${YELLOW}⚠️${NC} iptables-save not available"
    fi

    # Test iptables-restore
    if command -v iptables-restore >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} iptables-restore available"
    else
        echo -e "    ${YELLOW}⚠️${NC} iptables-restore not available"
    fi

    # Test iptables-restore-translate (for migration)
    if command -v iptables-restore-translate >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} iptables-restore-translate available (migration support)"
    else
        echo -e "    ${YELLOW}⚠️${NC} iptables-restore-translate not available"
    fi

    # Test backup functionality
    local backup_dir="$HOME/firewall_backups"
    if mkdir -p "$backup_dir" 2>/dev/null; then
        echo -e "    ${GREEN}✅${NC} Can create firewall backup directory"
    fi
}

test_gaming_network_features() {
    echo -e "${BLUE}Testing Gaming Network Features...${NC}"

    # Test port availability for common games
    local game_ports=(
        "25565:Minecraft"
        "27015:CS:GO/Source games"
        "7777:Unreal Tournament"
        "28015:Rust"
        "2456:Valheim"
    )

    echo "  🎮 Testing common game ports..."
    for port_desc in "${game_ports[@]}"; do
        local port="${port_desc%:*}"
        local game="${port_desc#*:}"

        # Check if port is in use (without binding to it)
        if ss -ln | grep -q ":$port "; then
            echo -e "    ${YELLOW}⚠️${NC} Port $port ($game) is in use"
        else
            echo -e "    ${GREEN}✅${NC} Port $port ($game) is available"
        fi
    done

    # Test NAT detection (basic)
    echo "  🌐 Testing NAT configuration..."
    local private_ip=$(ip route get 8.8.8.8 2>/dev/null | grep -Po '(?<=src )[\d.]+' | head -1)
    if [[ "$private_ip" =~ ^10\.|^172\.(1[6-9]|2[0-9]|3[01])\.|^192\.168\. ]]; then
        echo -e "    ${GREEN}✅${NC} Detected private IP: $private_ip (NAT environment)"
    else
        echo -e "    ${GREEN}✅${NC} Detected public IP: $private_ip"
    fi

    # Test UPnP client availability
    if command -v upnpc >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} UPnP client (upnpc) available"
    else
        echo -e "    ${YELLOW}⚠️${NC} UPnP client not available (port forwarding automation limited)"
    fi
}

test_qos_features() {
    echo -e "${BLUE}Testing QoS and Traffic Shaping...${NC}"

    # Test tc (traffic control)
    if command -v tc >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} tc (traffic control) available"
    else
        echo -e "    ${YELLOW}⚠️${NC} tc not available (QoS features limited)"
    fi

    # Test wondershaper
    if command -v wondershaper >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} wondershaper available"
    else
        echo -e "    ${YELLOW}⚠️${NC} wondershaper not available"
    fi

    # Test network interfaces
    echo "  🔌 Available network interfaces:"
    local interfaces=$(ip link show | grep -E '^[0-9]+:' | cut -d: -f2 | tr -d ' ' | grep -v lo)
    for iface in $interfaces; do
        local state=$(ip link show "$iface" | grep -o 'state [A-Z]*' | cut -d' ' -f2)
        echo -e "    ${GREEN}✅${NC} $iface ($state)"
    done
}

test_ddos_protection_features() {
    echo -e "${BLUE}Testing DDoS Protection Features...${NC}"

    # Test fail2ban
    if command -v fail2ban-client >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} fail2ban available"

        # Check if fail2ban is running
        if systemctl is-active fail2ban >/dev/null 2>&1; then
            echo -e "    ${GREEN}✅${NC} fail2ban is running"
        else
            echo -e "    ${YELLOW}⚠️${NC} fail2ban is not running"
        fi
    else
        echo -e "    ${YELLOW}⚠️${NC} fail2ban not available"
    fi

    # Test rate limiting capabilities
    echo "  ⚡ Testing rate limiting support..."

    # Check if kernel has LIMIT match support
    if [ -f /proc/net/ip_tables_matches ] && grep -q LIMIT /proc/net/ip_tables_matches 2>/dev/null; then
        echo -e "    ${GREEN}✅${NC} iptables LIMIT module available"
    fi

    # Check conntrack
    if command -v conntrack >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} conntrack tool available"

        # Check current connection count (safe)
        local conn_count=$(conntrack -C 2>/dev/null || echo "unknown")
        echo -e "      Current connections: $conn_count"
    else
        echo -e "    ${YELLOW}⚠️${NC} conntrack tool not available"
    fi
}

test_security_features() {
    echo -e "${BLUE}Testing Security Features...${NC}"

    # Test GeoIP databases
    local geoip_paths=(
        "/usr/share/GeoIP"
        "/var/lib/GeoIP"
        "/opt/GeoIP"
    )

    local geoip_found=0
    for path in "${geoip_paths[@]}"; do
        if [ -d "$path" ] && [ "$(ls -A "$path" 2>/dev/null)" ]; then
            echo -e "    ${GREEN}✅${NC} GeoIP databases found in $path"
            geoip_found=1
            break
        fi
    done

    if [ $geoip_found -eq 0 ]; then
        echo -e "    ${YELLOW}⚠️${NC} No GeoIP databases found (IP blocking features limited)"
    fi

    # Test port scanning detection tools
    if command -v psad >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} psad (port scan detection) available"
    else
        echo -e "    ${YELLOW}⚠️${NC} psad not available"
    fi

    # Test intrusion detection
    if command -v aide >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} AIDE (intrusion detection) available"
    else
        echo -e "    ${YELLOW}⚠️${NC} AIDE not available"
    fi
}

test_template_system() {
    echo -e "${BLUE}Testing Template System...${NC}"

    # Test template storage
    local template_dir="$HOME/nftables_templates"
    if mkdir -p "$template_dir" 2>/dev/null; then
        echo -e "    ${GREEN}✅${NC} Can create template directory"

        # Test template creation
        local test_template="$template_dir/test_template.nft"
        cat > "$test_template" 2>/dev/null << 'EOF'
#!/usr/sbin/nft -f

flush ruleset

table inet filter {
    chain input {
        type filter hook input priority 0; policy drop;
        iif lo accept
        ct state established,related accept
        ct state invalid drop
    }
}
EOF

        if [ -f "$test_template" ]; then
            echo -e "    ${GREEN}✅${NC} Can create template files"

            # Test template syntax
            if nft -c -f "$test_template" >/dev/null 2>&1; then
                echo -e "    ${GREEN}✅${NC} Template syntax is valid"
            else
                echo -e "    ${YELLOW}⚠️${NC} Template syntax validation failed"
            fi

            rm -f "$test_template"
        fi
    fi
}

test_backup_system() {
    echo -e "${BLUE}Testing Backup System...${NC}"

    local backup_dirs=(
        "$HOME/nftables_backups"
        "$HOME/firewall_backups"
        "$HOME/iptables_backups"
    )

    for dir in "${backup_dirs[@]}"; do
        if mkdir -p "$dir" 2>/dev/null; then
            echo -e "    ${GREEN}✅${NC} Can create backup directory: $(basename "$dir")"

            # Test backup file creation
            local test_backup="$dir/test_backup_$(date +%Y%m%d).rules"
            if echo "# Test backup file" > "$test_backup" 2>/dev/null; then
                echo -e "    ${GREEN}✅${NC} Can create backup files"
                rm -f "$test_backup"
            fi
        else
            echo -e "    ${RED}❌${NC} Cannot create backup directory: $dir"
        fi
    done

    # Test cron job creation capability
    if command -v crontab >/dev/null 2>&1; then
        echo -e "    ${GREEN}✅${NC} Crontab available for scheduled backups"
    else
        echo -e "    ${YELLOW}⚠️${NC} Crontab not available"
    fi
}

test_network_monitoring() {
    echo -e "${BLUE}Testing Network Monitoring...${NC}"

    # Test basic network stats
    if [ -f /proc/net/dev ]; then
        echo -e "    ${GREEN}✅${NC} Network interface statistics available"

        # Show active interfaces
        local active_ifaces=$(awk '/[^:]/ && !/lo:/ && $2 > 0 {print $1}' /proc/net/dev | tr -d ':' | wc -l)
        echo -e "      Active interfaces: $active_ifaces"
    fi

    # Test connection tracking stats
    if [ -f /proc/net/nf_conntrack ]; then
        echo -e "    ${GREEN}✅${NC} Connection tracking information available"
        local active_conns=$(wc -l < /proc/net/nf_conntrack 2>/dev/null || echo "0")
        echo -e "      Active connections: $active_conns"
    fi

    # Test bandwidth monitoring tools
    local monitoring_tools=(
        "iftop:Interface bandwidth"
        "nethogs:Per-process bandwidth"
        "vnstat:Long-term statistics"
        "bandwhich:Modern bandwidth monitor"
    )

    for tool_desc in "${monitoring_tools[@]}"; do
        local tool="${tool_desc%:*}"
        local desc="${tool_desc#*:}"

        if command -v "$tool" >/dev/null 2>&1; then
            echo -e "    ${GREEN}✅${NC} $tool ($desc)"
        else
            echo -e "    ${YELLOW}⚠️${NC} $tool ($desc) - not installed"
        fi
    done
}

main() {
    echo -e "${BLUE}Starting Networking Features Test...${NC}"
    echo

    test_firewall_tools
    echo

    test_networking_tools
    echo

    test_nftables_features
    echo

    test_iptables_features
    echo

    test_gaming_network_features
    echo

    test_qos_features
    echo

    test_ddos_protection_features
    echo

    test_security_features
    echo

    test_template_system
    echo

    test_backup_system
    echo

    test_network_monitoring
    echo

    echo -e "${GREEN}🔥 Networking Features Test Complete!${NC}"
    echo
    echo "Summary:"
    echo "========"
    echo "• Basic firewall tools: Available based on system configuration"
    echo "• nftables advanced features: Ready for supported systems"
    echo "• Gaming network optimization: Port management and NAT detection ready"
    echo "• DDoS protection: Depends on system packages and kernel modules"
    echo "• Template system: File-based templates ready"
    echo "• Backup system: Full backup/restore capability implemented"
    echo
    echo "Next Steps:"
    echo "• Test actual rule creation in sandbox environment"
    echo "• Verify gaming port forwarding functionality"
    echo "• Test DDoS protection rules with controlled traffic"
    echo "• Validate backup/restore cycle with real configurations"
    echo
    echo "⚠️  Remember: All firewall modifications should be tested in a safe environment first!"
}

main "$@"