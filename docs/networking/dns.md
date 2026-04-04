# DNS Tools

## Overview

GhostCTL provides DNS lookup and DNSSEC verification tools.

## Access

```bash
ghostctl network menu
# Select: DNS Lookup
```

## DNS Lookup

Resolve domain names using `dig`:

```bash
# Through menu:
ghostctl network menu > DNS Lookup > DNS Lookup
# Enter domain name
```

Shows:
- A records (IPv4)
- AAAA records (IPv6)
- MX records
- NS records
- Response time

## DNSSEC Check

Verify DNSSEC configuration for a domain:

```bash
# Through menu:
ghostctl network menu > DNS Lookup > DNSSEC Check
# Enter domain name
```

Shows:
- DNSKEY records
- RRSIG records
- DS records
- Validation status

## Command-Line Usage

For direct DNS queries:

```bash
# Basic lookup
dig example.com

# Specific record types
dig example.com MX
dig example.com AAAA
dig example.com NS

# DNSSEC verification
dig +dnssec +multi example.com

# Trace resolution
dig +trace example.com

# Use specific DNS server
dig @8.8.8.8 example.com
```

## Common DNS Servers

| Provider | Primary | Secondary |
|----------|---------|-----------|
| Google | 8.8.8.8 | 8.8.4.4 |
| Cloudflare | 1.1.1.1 | 1.0.0.1 |
| Quad9 | 9.9.9.9 | 149.112.112.112 |
| OpenDNS | 208.67.222.222 | 208.67.220.220 |

## Troubleshooting

### DNS Resolution Issues
1. Check local DNS configuration: `cat /etc/resolv.conf`
2. Test with external DNS: `dig @8.8.8.8 example.com`
3. Check for NXDOMAIN responses
4. Verify domain registration

### DNSSEC Issues
1. Check if domain has DNSSEC enabled
2. Verify DS records at registrar
3. Check DNSKEY validity
4. Test with DNSSEC validation: `dig +dnssec +cd example.com`
