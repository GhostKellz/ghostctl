#!/bin/bash

# CONFIGURATION
BRIDGE_NAME="br0"
PHYS_IF="eno1"
BRIDGE_CON="bridge-$BRIDGE_NAME"
SLAVE_CON="slave-$PHYS_IF"
STATIC_IP="10.0.0.21/24"
GATEWAY="10.0.0.1"
DNS="10.0.0.2"
CLONED_MAC="10:7C:61:3D:E5:14"

echo "[!] Disabling original connection (ck21) if active"
if nmcli con show ck21 &>/dev/null; then
    nmcli con down ck21
    nmcli con modify ck21 connection.autoconnect no
    echo "  ↪ ck21 disabled (saved, not deleted)"
fi

echo "[+] Creating bridge: $BRIDGE_CON"
nmcli con add type bridge ifname "$BRIDGE_NAME" con-name "$BRIDGE_CON" stp no

echo "[+] Cloning MAC to bridge: $CLONED_MAC"
nmcli con modify "$BRIDGE_CON" ethernet.cloned-mac-address "$CLONED_MAC"

echo "[+] Assigning static IP to bridge"
nmcli con modify "$BRIDGE_CON" ipv4.addresses "$STATIC_IP"
nmcli con modify "$BRIDGE_CON" ipv4.gateway "$GATEWAY"
nmcli con modify "$BRIDGE_CON" ipv4.dns "$DNS"
nmcli con modify "$BRIDGE_CON" ipv4.method manual
nmcli con modify "$BRIDGE_CON" connection.autoconnect yes

echo "[+] Adding $PHYS_IF as bridge-slave: $SLAVE_CON"
nmcli con add type bridge-slave ifname "$PHYS_IF" master "$BRIDGE_NAME" con-name "$SLAVE_CON"
nmcli con modify "$SLAVE_CON" connection.autoconnect yes

echo "[+] Bringing up bridge + slave..."
nmcli con up "$SLAVE_CON"
nmcli con up "$BRIDGE_CON"

echo "[✓] Bridge $BRIDGE_NAME is now active on $STATIC_IP with MAC $CLONED_MAC"

