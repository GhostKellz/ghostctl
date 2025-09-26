# 🏠 Proxmox Integration with GhostCTL

Complete guide for homelab automation and Proxmox VE integration.

## 🚀 Overview

GhostCTL provides comprehensive Proxmox Virtual Environment (PVE) integration for:

### 🆕 v1.0.0 Enterprise Features:
- **Template Management**: Complete lifecycle for LXC/VM templates with upload/download
- **Storage Migration**: VM/Container storage migration with live operations support
- **Backup Rotation & Pruning**: Advanced backup management with retention policies  
- **Firewall Automation**: Security rule management with gscan network scanning integration

### Core Features:
- Virtual machine management and automation
- Container (LXC) deployment and configuration
- Storage and network management
- Backup and disaster recovery
- Template creation and deployment
- Cluster management

## 📦 Proxmox VE Installation

### Install Proxmox VE
```bash
# Download Proxmox VE ISO
wget https://www.proxmox.com/downloads/proxmox-virtual-environment/iso

# Flash to USB (replace /dev/sdX with your USB device)
dd if=proxmox-ve_*.iso of=/dev/sdX bs=1M status=progress

# Boot from USB and follow installation wizard
```

### Post-Installation Setup
```bash
# Update system
apt update && apt full-upgrade

# Configure repositories (remove enterprise repo if not subscribed)
echo "deb http://download.proxmox.com/debian/pve bookworm pve-no-subscription" > /etc/apt/sources.list.d/pve-install-repo.list

# Disable subscription notice (optional)
sed -i "s/data.status !== 'Active'/false/g" /usr/share/javascript/proxmox-widget-toolkit/proxmoxlib.js
systemctl restart pveproxy
```

## 🖥️ Virtual Machine Management

### VM Creation Scripts
```bash
#!/bin/bash
# create-vm.sh - Automated VM creation

VM_ID=${1:-100}
VM_NAME=${2:-"vm-$VM_ID"}
VM_MEMORY=${3:-2048}
VM_CORES=${4:-2}
VM_DISK_SIZE=${5:-20G}
TEMPLATE_ID=${6:-9000}

echo "Creating VM: $VM_NAME (ID: $VM_ID)"

# Clone from template
qm clone $TEMPLATE_ID $VM_ID --name $VM_NAME --full

# Configure VM
qm set $VM_ID \
  --memory $VM_MEMORY \
  --cores $VM_CORES \
  --net0 virtio,bridge=vmbr0,firewall=1

# Resize disk if needed
qm resize $VM_ID scsi0 $VM_DISK_SIZE

# Start VM
qm start $VM_ID

echo "VM $VM_NAME created and started"
echo "Access via: ssh user@$(qm guest cmd $VM_ID network-get-interfaces | jq -r '.result[1]["ip-addresses"][0]["ip-address"]')"
```

### VM Template Creation
```bash
#!/bin/bash
# create-ubuntu-template.sh

TEMPLATE_ID=9000
TEMPLATE_NAME="ubuntu-22.04-template"
IMAGE_URL="https://cloud-images.ubuntu.com/jammy/current/jammy-server-cloudimg-amd64.img"

# Download cloud image
wget $IMAGE_URL -O ubuntu-22.04.img

# Create VM
qm create $TEMPLATE_ID --name $TEMPLATE_NAME --memory 2048 --cores 2 --net0 virtio,bridge=vmbr0

# Import disk
qm importdisk $TEMPLATE_ID ubuntu-22.04.img local-lvm

# Configure VM
qm set $TEMPLATE_ID \
  --scsihw virtio-scsi-pci \
  --scsi0 local-lvm:vm-$TEMPLATE_ID-disk-0 \
  --ide2 local-lvm:cloudinit \
  --boot c \
  --bootdisk scsi0 \
  --serial0 socket \
  --vga serial0 \
  --agent enabled=1

# Convert to template
qm template $TEMPLATE_ID

echo "Template $TEMPLATE_NAME created with ID $TEMPLATE_ID"
```

### VM Operations
```bash
# List VMs
qm list

# VM lifecycle
qm start 100              # Start VM
qm stop 100               # Stop VM
qm shutdown 100           # Graceful shutdown
qm reset 100              # Reset VM
qm suspend 100            # Suspend VM
qm resume 100             # Resume VM

# VM configuration
qm config 100             # Show configuration
qm set 100 --memory 4096  # Set memory
qm set 100 --cores 4      # Set CPU cores
qm monitor 100            # Enter monitor mode

# VM snapshots
qm snapshot 100 snap1     # Create snapshot
qm listsnapshot 100       # List snapshots
qm rollback 100 snap1     # Rollback to snapshot
qm delsnapshot 100 snap1  # Delete snapshot

# VM migration
qm migrate 100 node2      # Migrate to another node
qm move_disk 100 scsi0 local-lvm  # Move disk
```

## 📦 Container (LXC) Management

### Container Creation
```bash
#!/bin/bash
# create-lxc.sh - Create LXC container

CT_ID=${1:-200}
CT_NAME=${2:-"ct-$CT_ID"}
CT_TEMPLATE=${3:-"ubuntu-22.04-standard"}
CT_MEMORY=${4:-1024}
CT_CORES=${5:-1}
CT_DISK_SIZE=${6:-8}
CT_PASSWORD=${7:-"changeme"}

echo "Creating container: $CT_NAME (ID: $CT_ID)"

# Create container
pct create $CT_ID \
  local:vztmpl/$CT_TEMPLATE.tar.xz \
  --hostname $CT_NAME \
  --memory $CT_MEMORY \
  --cores $CT_CORES \
  --rootfs local-lvm:$CT_DISK_SIZE \
  --net0 name=eth0,bridge=vmbr0,ip=dhcp \
  --password $CT_PASSWORD \
  --unprivileged 1 \
  --features keyctl=1,nesting=1

# Start container
pct start $CT_ID

echo "Container $CT_NAME created and started"
echo "Access via: pct enter $CT_ID"
```

### Container Operations
```bash
# List containers
pct list

# Container lifecycle
pct start 200             # Start container
pct stop 200              # Stop container
pct shutdown 200          # Graceful shutdown
pct reboot 200            # Reboot container

# Container access
pct enter 200             # Enter container
pct exec 200 -- ls -la    # Execute command

# Container configuration
pct config 200            # Show configuration
pct set 200 --memory 2048 # Set memory
pct set 200 --cores 2     # Set CPU cores

# Container snapshots
pct snapshot 200 snap1    # Create snapshot
pct listsnapshot 200      # List snapshots
pct rollback 200 snap1    # Rollback to snapshot
pct delsnapshot 200 snap1 # Delete snapshot
```

### Docker in LXC
```bash
#!/bin/bash
# setup-docker-lxc.sh - Setup Docker in LXC container

CT_ID=$1

# Configure container for Docker
pct set $CT_ID \
  --features keyctl=1,nesting=1 \
  --unprivileged 0

# Restart container
pct stop $CT_ID
pct start $CT_ID

# Install Docker
pct exec $CT_ID -- bash -c "
  apt update
  apt install -y ca-certificates curl gnupg
  curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
  echo 'deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu jammy stable' > /etc/apt/sources.list.d/docker.list
  apt update
  apt install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
  systemctl enable --now docker
"

echo "Docker installed in container $CT_ID"
```

## 🗄️ Storage Management

### Storage Configuration
```bash
# List storage
pvesm status

# Add storage
pvesm add dir backup-storage --path /mnt/backup --content backup
pvesm add nfs nfs-storage --server 192.168.1.100 --export /volume1/proxmox

# Storage operations
pvesm list local           # List content
pvesm alloc local-lvm 100 vm-100-disk-0 32G  # Allocate disk
pvesm free local-lvm:vm-100-disk-0  # Free disk
```

### ZFS Storage Setup
```bash
#!/bin/bash
# setup-zfs-storage.sh

# Install ZFS
apt install zfsutils-linux

# Create ZFS pool
zpool create -o ashift=12 \
  -O compression=lz4 \
  -O atime=off \
  -O relatime=on \
  tank mirror /dev/sdb /dev/sdc

# Create ZFS dataset for VMs
zfs create tank/vmdata

# Add to Proxmox
pvesm add zfspool tank --pool tank/vmdata --content images,rootdir
```

## 🌐 Network Configuration

### Network Setup
```bash
# Edit network configuration
nano /etc/network/interfaces

# Example: Bridge with VLAN
auto lo
iface lo inet loopback

iface eno1 inet manual

auto vmbr0
iface vmbr0 inet static
    address 192.168.1.10/24
    gateway 192.168.1.1
    bridge-ports eno1
    bridge-stp off
    bridge-fd 0

# VLAN interface
auto vmbr0.100
iface vmbr0.100 inet static
    address 192.168.100.1/24
    vlan-raw-device vmbr0

# Apply network changes
ifreload -a
```

### SDN (Software Defined Networking)
```bash
# Enable SDN
echo "source /etc/network/interfaces.d/*" >> /etc/network/interfaces

# Create VNet
pvesh create /cluster/sdn/vnets --vnet vnet1 --zone zone1
pvesh create /cluster/sdn/subnets --subnet 192.168.100.0/24 --vnet vnet1

# Apply SDN configuration
pvesh set /cluster/sdn
```

## 💾 Backup & Disaster Recovery

### Backup Configuration
```bash
#!/bin/bash
# setup-backup.sh - Configure automated backups

# Create backup storage
pvesm add dir backup-local --path /var/lib/vz/dump --content backup

# Create backup job
pvesh create /cluster/backup --id backup-daily --schedule "mon,tue,wed,thu,fri,sat,sun 02:00" --storage backup-local --all 1 --compress zstd --mode snapshot

# Manual backup
vzdump 100 --storage backup-local --compress zstd --mode snapshot
```

### Restore Operations
```bash
# List backups
pct list-backup backup-local

# Restore VM
qmrestore backup-local:backup/vzdump-qemu-100-2023_12_01-02_00_00.vma.zst 101 --storage local-lvm

# Restore container
pct restore 201 backup-local:backup/vzdump-lxc-200-2023_12_01-02_00_00.tar.zst --storage local-lvm
```

### Proxmox Backup Server (PBS)
```bash
# Install PBS (on separate machine)
wget https://enterprise.proxmox.com/debian/proxmox-backup-server_*.deb
dpkg -i proxmox-backup-server_*.deb
apt install -f

# Add PBS storage to PVE
pvesm add pbs pbs-storage --server pbs.local --username admin@pbs --password secret --datastore backup

# Create backup job with PBS
pvesh create /cluster/backup --id pbs-daily --schedule "daily 03:00" --storage pbs-storage --all 1 --mode snapshot
```

## 🔧 Automation Scripts

### Bulk VM Deployment
```bash
#!/bin/bash
# deploy-vm-farm.sh - Deploy multiple VMs

TEMPLATE_ID=9000
BASE_VM_ID=100
COUNT=5
VM_PREFIX="web-server"

for i in $(seq 1 $COUNT); do
    VM_ID=$((BASE_VM_ID + i))
    VM_NAME="${VM_PREFIX}-${i}"
    
    echo "Creating $VM_NAME (ID: $VM_ID)"
    
    # Clone template
    qm clone $TEMPLATE_ID $VM_ID --name $VM_NAME --full
    
    # Configure VM
    qm set $VM_ID \
      --memory 2048 \
      --cores 2 \
      --net0 virtio,bridge=vmbr0
    
    # Start VM
    qm start $VM_ID
    
    echo "Waiting for VM to start..."
    sleep 30
done

echo "Deployed $COUNT VMs"
```

### Container Orchestration
```bash
#!/bin/bash
# deploy-container-stack.sh - Deploy application stack

# Database container
pct create 301 local:vztmpl/ubuntu-22.04-standard.tar.xz \
  --hostname postgres-db \
  --memory 2048 \
  --cores 2 \
  --rootfs local-lvm:20 \
  --net0 name=eth0,bridge=vmbr0,ip=192.168.1.201/24,gw=192.168.1.1 \
  --password changeme \
  --unprivileged 1

# Web server container
pct create 302 local:vztmpl/ubuntu-22.04-standard.tar.xz \
  --hostname web-server \
  --memory 1024 \
  --cores 1 \
  --rootfs local-lvm:10 \
  --net0 name=eth0,bridge=vmbr0,ip=192.168.1.202/24,gw=192.168.1.1 \
  --password changeme \
  --unprivileged 1

# Start containers
pct start 301
pct start 302

# Configure containers
sleep 30

# Setup PostgreSQL
pct exec 301 -- bash -c "
  apt update && apt install -y postgresql postgresql-contrib
  systemctl enable --now postgresql
  sudo -u postgres createdb myapp
"

# Setup Nginx
pct exec 302 -- bash -c "
  apt update && apt install -y nginx
  systemctl enable --now nginx
"

echo "Application stack deployed"
```

## 📊 Monitoring & Management

### Resource Monitoring
```bash
#!/bin/bash
# monitor-resources.sh

echo "=== Proxmox Resource Usage ==="
echo

echo "Node Status:"
pvesh get /nodes/$(hostname)/status

echo
echo "VM Status:"
qm list

echo
echo "Container Status:"
pct list

echo
echo "Storage Usage:"
pvesm status

echo
echo "Memory Usage:"
free -h

echo
echo "CPU Usage:"
top -bn1 | grep "Cpu(s)"

echo
echo "Disk Usage:"
df -h
```

### Automated Health Checks
```bash
#!/bin/bash
# health-check.sh

# Check PVE services
systemctl is-active pvedaemon pveproxy pvestatd pvescheduler

# Check cluster status (if clustered)
pvecm status

# Check VM/CT status
echo "Checking VMs..."
for vm in $(qm list | awk 'NR>1 {print $1}'); do
    status=$(qm status $vm | awk '{print $2}')
    echo "VM $vm: $status"
    if [ "$status" != "running" ]; then
        echo "WARNING: VM $vm is not running"
    fi
done

echo "Checking Containers..."
for ct in $(pct list | awk 'NR>1 {print $1}'); do
    status=$(pct status $ct | awk '{print $2}')
    echo "CT $ct: $status"
    if [ "$status" != "running" ]; then
        echo "WARNING: Container $ct is not running"
    fi
done
```

## 🏗️ Infrastructure as Code

### Terraform Integration
```hcl
# main.tf - Proxmox VM with Terraform

terraform {
  required_providers {
    proxmox = {
      source = "telmate/proxmox"
      version = "2.9.14"
    }
  }
}

provider "proxmox" {
  pm_api_url = "https://proxmox.local:8006/api2/json"
  pm_user = "terraform@pve"
  pm_password = "terraform_password"
  pm_tls_insecure = true
}

resource "proxmox_vm_qemu" "web_server" {
  count = 3
  name = "web-server-${count.index + 1}"
  target_node = "proxmox"
  clone = "ubuntu-22.04-template"
  
  memory = 2048
  cores = 2
  
  disk {
    size = "20G"
    type = "scsi"
    storage = "local-lvm"
  }
  
  network {
    model = "virtio"
    bridge = "vmbr0"
  }
  
  lifecycle {
    ignore_changes = [
      network,
    ]
  }
}
```

### Ansible Playbooks
```yaml
# proxmox-setup.yml
---
- name: Configure Proxmox VMs
  hosts: proxmox_vms
  become: yes
  
  tasks:
    - name: Update system
      apt:
        update_cache: yes
        upgrade: dist
        
    - name: Install Docker
      shell: |
        curl -fsSL https://get.docker.com | sh
        usermod -aG docker ubuntu
        
    - name: Install monitoring agent
      apt:
        name: prometheus-node-exporter
        state: present
        
    - name: Configure firewall
      ufw:
        rule: allow
        port: "{{ item }}"
      loop:
        - ssh
        - "80"
        - "443"
        - "9100"
```

## 🔐 Security & Access Control

### User Management
```bash
# Create user
pveum user add admin@pve --password changeme --firstname Admin --lastname User

# Create group
pveum group add admins --comment "Administrator Group"

# Add user to group
pveum user modify admin@pve --groups admins

# Set permissions
pveum aclmod / --users admin@pve --roles Administrator
pveum aclmod /vms --groups admins --roles PVEVMAdmin
```

### API Access
```bash
# Create API token
pveum user token add admin@pve monitoring --privsep=0

# Use API
curl -k -H "Authorization: PVEAPIToken=admin@pve!monitoring=secret-token" \
  https://proxmox.local:8006/api2/json/version
```

### SSL Certificates
```bash
# Install Let's Encrypt certificate
apt install certbot

# Get certificate
certbot certonly --standalone -d proxmox.local

# Install certificate
cp /etc/letsencrypt/live/proxmox.local/fullchain.pem /etc/pve/local/pve-ssl.pem
cp /etc/letsencrypt/live/proxmox.local/privkey.pem /etc/pve/local/pve-ssl.key

# Restart proxy
systemctl restart pveproxy
```

## 🔄 Cluster Management

### Cluster Setup
```bash
# Initialize cluster on first node
pvecm create my-cluster

# Add nodes to cluster (run on additional nodes)
pvecm add node1.local

# Check cluster status
pvecm status
pvecm nodes
```

### Cluster Operations
```bash
# Migrate VM
qm migrate 100 node2

# Live migration
qm migrate 100 node2 --online

# Cluster resource management
ha-manager add vm:100
ha-manager set vm:100 --state started --group default
```

## 📱 Mobile Management

### Proxmox Mobile App Setup
```bash
# Install Proxmox Mobile API endpoint
echo "PVEMobileAPI: 1" >> /etc/pve/datacenter.cfg

# Create mobile-specific user
pveum user add mobile@pve --password mobile123
pveum aclmod / --users mobile@pve --roles PVEVMUser
```

## 🚨 Troubleshooting

### Common Issues
```bash
# Check logs
journalctl -u pvedaemon
journalctl -u pveproxy
tail -f /var/log/pve/cluster.log

# Fix broken cluster
systemctl stop pve-cluster
pmxcfs -l

# Reset cluster (emergency)
rm /etc/pve/cluster.conf
systemctl restart pve-cluster

# Storage issues
lvs                       # List logical volumes
vgs                       # List volume groups
pvs                       # List physical volumes

# Network troubleshooting
ip a                      # Show interfaces
bridge vlan show          # Show VLAN configuration
iptables -L               # Show firewall rules
```

### Performance Optimization
```bash
# Kernel optimization
echo 'vm.swappiness = 10' >> /etc/sysctl.conf
echo 'vm.vfs_cache_pressure = 50' >> /etc/sysctl.conf

# I/O scheduler
echo 'deadline' > /sys/block/sda/queue/scheduler

# Disable transparent hugepages
echo never > /sys/kernel/mm/transparent_hugepage/enabled
```

---

For container deployment strategies, see [Docker Guide](DOCKER.md). For backup integration, see [Backup with Restic](RESTIC.md).