# NVIDIA Container Runtime

GPU support for Docker and Podman containers.

## Installation

The nvidia-container-toolkit enables GPU access in containers.

### Arch Linux
```bash
# Via AUR
yay -S nvidia-container-toolkit

# Or through ghostctl
ghostctl nvidia  # Select container runtime option
```

### Debian/Ubuntu
```bash
# Add NVIDIA repo
distribution=$(. /etc/os-release;echo $ID$VERSION_ID)
curl -s -L https://nvidia.github.io/nvidia-docker/gpgkey | sudo apt-key add -
curl -s -L https://nvidia.github.io/nvidia-docker/$distribution/nvidia-docker.list | \
  sudo tee /etc/apt/sources.list.d/nvidia-docker.list

sudo apt update
sudo apt install nvidia-container-toolkit
```

## Docker Configuration

```bash
# Configure Docker runtime
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker
```

## Usage

```bash
# Run container with GPU access
docker run --gpus all nvidia/cuda:12.0-base nvidia-smi

# Specific GPU
docker run --gpus '"device=0"' nvidia/cuda:12.0-base nvidia-smi
```

## Docker Compose

```yaml
services:
  gpu-app:
    image: nvidia/cuda:12.0-base
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all
              capabilities: [gpu]
```

## Verification

```bash
# Check runtime is configured
docker info | grep -i nvidia

# Test GPU access
docker run --rm --gpus all nvidia/cuda:12.0-base nvidia-smi
```
