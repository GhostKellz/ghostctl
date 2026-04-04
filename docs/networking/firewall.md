# Firewall Management

GhostCTL provides enterprise-grade firewall management with nftables and UFW support.

## nftables

Modern Linux firewall framework replacing iptables.

### Table Families

| Family | Description |
|--------|-------------|
| inet | IPv4 and IPv6 combined |
| ip | IPv4 only |
| ip6 | IPv6 only |
| bridge | Bridge traffic |
| arp | ARP traffic |
| netdev | Network device level |

### Chain Hooks

- `prerouting` - Before routing decisions
- `input` - Traffic destined for local system
- `forward` - Traffic being routed through
- `output` - Locally generated traffic
- `postrouting` - After routing decisions
- `ingress/egress` - Network device level (netdev)

### Basic Commands

```bash
# List all rules
sudo nft list ruleset

# List specific table
sudo nft list table inet filter

# Flush all rules
sudo nft flush ruleset

# Load from file
sudo nft -f /etc/nftables.conf
```

### Example Configuration

```nft
table inet filter {
    chain input {
        type filter hook input priority 0; policy drop;

        # Allow established connections
        ct state established,related accept

        # Allow loopback
        iif lo accept

        # Allow SSH
        tcp dport 22 accept

        # Allow HTTP/HTTPS
        tcp dport { 80, 443 } accept
    }

    chain forward {
        type filter hook forward priority 0; policy drop;
    }

    chain output {
        type filter hook output priority 0; policy accept;
    }
}
```

### Flow Offloading

Hardware-accelerated packet processing:

```nft
table inet filter {
    flowtable fastpath {
        hook ingress priority 0
        devices = { eth0, eth1 }
    }

    chain forward {
        tcp flags syn / fin,syn,rst,ack ct state established flow add @fastpath
    }
}
```

### Rate Limiting

DDoS protection rules:

```nft
table inet filter {
    chain input {
        tcp dport 22 ct state new limit rate 5/minute accept
        tcp dport 80 ct state new limit rate 100/second accept
    }
}
```

## UFW (Uncomplicated Firewall)

Frontend for iptables, easier for basic setups.

### Basic Commands

```bash
# Enable/disable
sudo ufw enable
sudo ufw disable

# Status
sudo ufw status verbose

# Allow/deny
sudo ufw allow 22/tcp
sudo ufw deny 23

# Delete rule
sudo ufw delete allow 22/tcp
```

### Application Profiles

```bash
sudo ufw app list
sudo ufw allow 'Nginx Full'
sudo ufw allow 'OpenSSH'
```

### Network-Based Rules

```bash
# From specific network
sudo ufw allow from 192.168.1.0/24 to any port 22

# Rate limiting
sudo ufw limit 22/tcp

# Logging
sudo ufw logging on
```

## Connection Tracking

Optimize for high-load scenarios:

```bash
echo 1048576 > /proc/sys/net/netfilter/nf_conntrack_max
echo 300 > /proc/sys/net/netfilter/nf_conntrack_tcp_timeout_time_wait
```

## Best Practices

- Start with deny-all policy
- Allow only necessary services
- Use connection tracking for stateful filtering
- Enable logging for security monitoring
- Regularly audit firewall rules
- Test rules before production deployment

## Related

- [Networking overview](README.md)
- [Scanner](scanner.md) - Port scanning
