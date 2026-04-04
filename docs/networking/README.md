# Networking

GhostCTL provides comprehensive networking capabilities including firewall management, network scanning, DNS tools, and utility functions.

## Documentation

- [Firewall](firewall.md) - nftables and UFW management
- [Scanner](scanner.md) - Network security scanning
- [DNS](dns.md) - DNS lookup and DNSSEC verification
- [Netcat](netcat.md) - File transfer, chat, and port checking

## Quick Start

```bash
ghostctl network menu             # Network management menu
```

## Menu Structure

```
Network Management
├── DNS Lookup
│   ├── DNS Lookup
│   └── DNSSEC Check
├── Network Scanning
│   ├── Target Scan
│   └── Interactive Scan
├── Netcat Utilities
│   ├── Send a file
│   ├── Receive a file
│   ├── Chat session
│   └── Check port connectivity
└── Mesh Networking
```

## Features

### Firewall Management
- nftables enterprise features (multi-family, flow offloading)
- UFW integration for simplified management
- Rate limiting and DDoS protection
- Connection tracking optimization

### Network Scanning
- Port scanning
- Service detection
- CIDR and range support
- Interactive scan mode

### DNS Tools
- Domain resolution
- DNSSEC verification
- Record type queries

### Netcat Utilities
- File transfer (send/receive)
- Chat sessions
- Port connectivity testing

### Virtualization Networking
- libvirt/KVM network interfaces
- Bridge network configuration
- VM network diagnostics
- SR-IOV support

## Related Documentation

- [Proxmox Networking](../proxmox/README.md)
- [Virtualization](../virtualization/README.md)
