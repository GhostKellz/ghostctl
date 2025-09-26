# 🔍 GhostCTL Native Scanner

GhostCTL includes a powerful **native Rust scanner** that replaces the external `gscan` dependency with a high-performance, feature-rich implementation.

## ⚡ **Key Features**

- ✅ **Native Rust Implementation** - No external dependencies
- ✅ **Beautiful TUI Interface** - Real-time progress and results display
- ✅ **Multi-threaded Scanning** - Configurable parallelism for optimal performance
- ✅ **Service Detection** - Automatic service identification and banner grabbing
- ✅ **Multiple Scan Types** - TCP Connect, SYN, UDP, ICMP scanning techniques
- ✅ **Advanced Timing Control** - Six timing templates from Paranoid to Insane
- ✅ **Network Range Support** - CIDR notation, include/exclude ranges
- ✅ **Adaptive Rate Limiting** - Automatic adjustment based on network conditions

## 🎮 **TUI Interface**

The scanner features a comprehensive Terminal User Interface with four main views:

### 📊 **Overview Tab**
- Real-time scan progress gauge
- Target and port count information
- Open/closed port statistics
- Elapsed time and ETA

### 🔍 **Results Tab**
- Live display of discovered open ports
- Service identification with confidence levels
- Response times for each port
- Navigate results with arrow keys

### 📈 **Statistics Tab**
- Visual bar chart of port status distribution
- Detailed scan metrics and performance data
- Success rate and timing statistics

### ⚙️ **Settings Tab**
- Current scan configuration display
- Keyboard shortcuts and controls
- Real-time configuration overview

## 🚀 **Usage Examples**

### Basic Scanning
```bash
# Single host scan
ghostctl scan 192.168.1.1

# Network range scan
ghostctl scan 192.168.1.0/24

# Specific hostname
ghostctl scan example.com
```

### Advanced Options
```bash
# Custom port ranges
ghostctl scan 192.168.1.1 -p 80,443,8080
ghostctl scan 192.168.1.1 -p 1-1000

# Custom threading
ghostctl scan 192.168.1.1 --threads 50

# Service detection enabled
ghostctl scan 192.168.1.1 --service
```

## 🔧 **Scan Configuration**

### **Timing Templates**
| Template | Speed | Parallelism | Use Case |
|----------|-------|-------------|----------|
| Paranoid (T0) | Very Slow | 1 thread | Maximum stealth, IDS evasion |
| Sneaky (T1) | Slow | 5 threads | Good stealth with reasonable speed |
| Polite (T2) | Moderate | 10 threads | Network-friendly scanning |
| Normal (T3) | Fast | 50 threads | **Default** - Balanced performance |
| Aggressive (T4) | Very Fast | 100 threads | Fast networks, time-sensitive |
| Insane (T5) | Maximum | 300 threads | High-speed networks only |

### **Scan Techniques**
- **TCP Connect**: Full three-way handshake (default)
- **TCP SYN**: Half-open scanning (stealth mode)
- **TCP ACK**: Firewall detection and mapping
- **UDP Scan**: UDP port discovery
- **ICMP Scan**: Host discovery and ping scanning

## 🛠️ **Advanced Features**

### **Service Detection**
```rust
// Automatic service identification
- HTTP/HTTPS servers with version detection
- SSH, FTP, SMTP, POP3, IMAP protocols
- Database services (MySQL, PostgreSQL, Redis)
- Custom probe database for enhanced detection
```

### **Network Range Management**
```bash
# Include/exclude specific ranges
ghostctl scan 192.168.0.0/16 --exclude 192.168.1.0/24
ghostctl scan 10.0.0.0/8 --include 10.0.1.0/24,10.0.2.0/24
```

### **Adaptive Rate Limiting**
The scanner automatically adjusts scan speed based on:
- Network RTT (Round Trip Time)
- Success/failure rates
- Target responsiveness
- Congestion detection

## 🎯 **Performance Characteristics**

### **Benchmarks**
- **Single Host**: ~1000 ports in 30-60 seconds
- **Network Range**: 254 hosts x 1000 ports in 10-15 minutes
- **Memory Usage**: <50MB for large scans
- **CPU Efficiency**: Multi-core utilization with async I/O

### **Optimization Tips**
1. **Use appropriate timing templates** for your network
2. **Adjust thread count** based on system capabilities
3. **Limit port ranges** to essential services for faster scans
4. **Use CIDR notation** for efficient network scanning

## 🔌 **Integration Points**

### **Proxmox Firewall Integration**
The scanner integrates with Proxmox firewall automation:
```bash
# Scan and automatically configure firewall rules
ghostctl pve firewall --scan-and-configure
```

### **Network Module Integration**
```bash
# Available through network subcommands
ghostctl network scan <target>
ghostctl network security-scan
```

## 🐛 **Troubleshooting**

### **Common Issues**

**Port scanning blocked by firewall:**
```bash
# Try different timing template
ghostctl scan target --timing sneaky
```

**High resource usage:**
```bash
# Reduce thread count
ghostctl scan target --threads 10
```

**Slow scan performance:**
```bash
# Use aggressive timing for fast networks
ghostctl scan target --timing aggressive
```

### **Error Messages**
- `Connection refused`: Port is closed
- `Connection timeout`: Port is filtered/firewalled
- `Host unreachable`: Network connectivity issues
- `Permission denied`: May need elevated privileges for some scan types

## 🔮 **Future Enhancements**

- [ ] **OS Fingerprinting** - Advanced operating system detection
- [ ] **Vulnerability Scanning** - CVE database integration
- [ ] **Script Scanning** - NSE-like script engine
- [ ] **IPv6 Support** - Full IPv6 scanning capabilities
- [ ] **Raw Socket Support** - SYN/UDP scanning without external tools
- [ ] **Report Generation** - XML/JSON/HTML output formats

## 📚 **Related Documentation**

- [Network Features](PROXMOX.md#network-features) - Proxmox network integration
- [Command Reference](../reference/COMMANDS.md#scan-commands) - Complete command syntax
- [Architecture](../architecture/) - Scanner implementation details