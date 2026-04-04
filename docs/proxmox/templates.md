# Template Management

## Features

- Download VM/CT templates
- Create templates from VMs/CTs
- Template lifecycle management
- Cloud-init integration

## Template Operations

```bash
ghostctl pve menu  # Select template management
```

### Available Operations
- Download container templates
- List available templates
- Create template from VM
- Create template from container
- Delete templates
- Clone from template

## Container Templates

### Download Templates
Common templates:
- Debian
- Ubuntu
- Alpine
- CentOS/Rocky

### Create from Container
1. Configure container as desired
2. Convert to template
3. Deploy new containers from template

## VM Templates

### Cloud-Init
Templates support cloud-init for:
- SSH key injection
- User creation
- Network configuration
- Package installation

### Create VM Template
1. Install and configure VM
2. Install cloud-init (Linux) or cloudbase-init (Windows)
3. Convert to template

## Best Practices

- Keep templates updated
- Use cloud-init for customization
- Document template configurations
- Version template names
