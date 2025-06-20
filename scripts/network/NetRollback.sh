#!/bin/bash

BACKUP_DIR="$1"

if [[ -z "$BACKUP_DIR" || ! -d "$BACKUP_DIR" ]]; then
    echo "Usage: $0 /path/to/network-backup-dir"
    exit 1
fi

echo "[!] Deleting all NetworkManager connections (except tailscale0 and docker0)..."
for CON in $(nmcli -g NAME connection show); do
    if [[ "$CON" == "tailscale0" || "$CON" == "docker0" ]]; then
        echo "  ↪ Skipping $CON"
        continue
    fi
    echo "  ↪ Deleting $CON"
    nmcli connection delete "$CON"
done

echo "[!] Explicitly deleting bridge-br0 and slave-eno1 if they exist..."
for NAME in bridge-br0 slave-eno1; do
    if nmcli con show "$NAME" &>/dev/null; then
        echo "  ↪ Deleting $NAME"
        nmcli connection delete "$NAME"
    fi
done

echo "[+] Restoring connections from $BACKUP_DIR"
for FILE in "$BACKUP_DIR"/*.nmconnection; do
    BASENAME=$(basename "$FILE")
    echo "  ↪ Importing $BASENAME"
    IMPORTED=0
    for TYPE in ethernet bridge bridge-slave generic; do
        if nmcli connection import type "$TYPE" file "$FILE" &>/dev/null; then
            echo "    ✓ Imported as $TYPE"
            IMPORTED=1
            break
        fi
    done
    if [[ "$IMPORTED" -eq 0 ]]; then
        echo "    ✗ Failed to import $BASENAME"
    fi
done

# Re-enable and bring up ck21 if it was part of the backup
if nmcli con show ck21 &>/dev/null; then
    echo "[+] Re-enabling autoconnect and bringing up 'ck21'"
    nmcli con modify ck21 connection.autoconnect yes
    nmcli con up ck21
fi

echo "[✓] Rollback complete. Run 'nmcli connection show' to confirm restored connections."

