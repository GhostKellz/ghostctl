# Netcat Utilities

## Overview

GhostCTL provides netcat-based utilities for file transfer, chat, and port connectivity testing.

## Access

```bash
ghostctl network menu
# Select: Netcat Utilities
```

## File Transfer

### Send a File
```bash
# Through menu:
ghostctl network menu > Netcat Utilities > Send a file
```

Prompts for:
- File to send
- Target host
- Target port

Receiver should be listening:
```bash
nc -l -p PORT > received_file
```

### Receive a File
```bash
# Through menu:
ghostctl network menu > Netcat Utilities > Receive a file
```

Prompts for:
- File to save as
- Port to listen on

Sender then connects:
```bash
nc HOST PORT < file_to_send
```

## Chat Session

Start a simple text chat between two systems:

```bash
# Through menu:
ghostctl network menu > Netcat Utilities > Chat session
```

Options:
- **Server mode**: Leave host empty, specify port to listen
- **Client mode**: Enter host and port to connect

## Port Connectivity

Check if a port is open on a remote host:

```bash
# Through menu:
ghostctl network menu > Netcat Utilities > Check port connectivity
```

Prompts for:
- Host to check
- Port to check

## Command-Line Examples

### File Transfer
```bash
# Receiver (start first)
nc -l -p 12345 > received_file.txt

# Sender
nc target-host 12345 < file_to_send.txt
```

### Port Check
```bash
# Check if port is open
nc -zv host 80

# Check port range
nc -zv host 20-25
```

### Simple Chat
```bash
# Server
nc -l -p 12345

# Client
nc server-host 12345
```

### Banner Grabbing
```bash
# Grab service banner
echo "" | nc -v host 22
```

## Security Considerations

- Netcat transfers are unencrypted
- Use SSH or other encrypted channels for sensitive data
- Be cautious with listening ports on public networks
- Consider firewall rules when opening ports
