#!/bin/bash

BACKUP_DIR="$HOME/network-backup-$(date +%F-%H%M%S)"
mkdir -p "$BACKUP_DIR"

echo "[+] Backing up active NetworkManager connection configs to $BACKUP_DIR"

for CON in $(nmcli -g NAME connection show); do
    echo "  ↪ Exporting $CON"
    nmcli connection export "$CON" > "$BACKUP_DIR/$CON.nmconnection" 2>/dev/null || \
        echo "    ⚠ Failed to export $CON"
done

echo "[✓] Backup complete. Files stored in: $BACKUP_DIR"

