# Container Security

## Security Scanning

GhostCTL integrates with Trivy for container vulnerability scanning.

### Features
- Image vulnerability scanning
- Running container analysis
- Security score calculation
- Root user detection
- Security options validation

## Security Checks

The security module checks:
- Running as root (risky)
- Privileged mode
- Host network mode
- Mounted sensitive paths
- Security options (seccomp, AppArmor)
- Read-only root filesystem
- Resource limits

## Security Score

Containers receive a security score:
- **A**: Excellent - follows best practices
- **B**: Good - minor improvements possible
- **C**: Fair - some security concerns
- **D**: Poor - significant issues
- **F**: Critical - immediate attention needed

## Best Practices

### Don't run as root
```yaml
services:
  app:
    user: "1000:1000"
```

### Use read-only filesystem
```yaml
services:
  app:
    read_only: true
    tmpfs:
      - /tmp
```

### Set resource limits
```yaml
services:
  app:
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
```

### Use security options
```yaml
services:
  app:
    security_opt:
      - no-new-privileges:true
```
