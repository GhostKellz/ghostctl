//! Validation utilities for Proxmox module
//!
//! Provides input validation for VM IDs, storage names, node names, and other
//! Proxmox-related identifiers to prevent injection attacks and ensure data integrity.

use super::errors::ProxmoxError;
use std::process::Command;

/// Validates a VM ID (VMID) - must be a positive integer between 100-999999999
pub fn validate_vmid(vmid: &str) -> Result<u32, ProxmoxError> {
    let vmid_trimmed = vmid.trim();

    if vmid_trimmed.is_empty() {
        return Err(ProxmoxError::VmOperationError {
            vmid: 0,
            reason: "VMID cannot be empty".to_string(),
        });
    }

    match vmid_trimmed.parse::<u32>() {
        Ok(id) if (100..=999_999_999).contains(&id) => Ok(id),
        Ok(id) => Err(ProxmoxError::VmOperationError {
            vmid: id,
            reason: "VMID must be between 100 and 999999999".to_string(),
        }),
        Err(_) => Err(ProxmoxError::VmOperationError {
            vmid: 0,
            reason: format!("Invalid VMID format: '{}'", vmid_trimmed),
        }),
    }
}

/// Validates a container ID (CTID) - must be a positive integer between 100-999999999
pub fn validate_ctid(ctid: &str) -> Result<u32, ProxmoxError> {
    let ctid_trimmed = ctid.trim();

    if ctid_trimmed.is_empty() {
        return Err(ProxmoxError::ContainerOperationError {
            ctid: 0,
            reason: "CTID cannot be empty".to_string(),
        });
    }

    match ctid_trimmed.parse::<u32>() {
        Ok(id) if (100..=999_999_999).contains(&id) => Ok(id),
        Ok(id) => Err(ProxmoxError::ContainerOperationError {
            ctid: id,
            reason: "CTID must be between 100 and 999999999".to_string(),
        }),
        Err(_) => Err(ProxmoxError::ContainerOperationError {
            ctid: 0,
            reason: format!("Invalid CTID format: '{}'", ctid_trimmed),
        }),
    }
}

/// Validates a storage name - alphanumeric with hyphens and underscores, 1-40 chars
pub fn validate_storage_name(name: &str) -> Result<&str, ProxmoxError> {
    let name_trimmed = name.trim();

    if name_trimmed.is_empty() {
        return Err(ProxmoxError::StorageError {
            storage: String::new(),
            reason: "Storage name cannot be empty".to_string(),
        });
    }

    if name_trimmed.len() > 40 {
        return Err(ProxmoxError::StorageError {
            storage: name_trimmed.to_string(),
            reason: "Storage name must be 40 characters or less".to_string(),
        });
    }

    // Must start with a letter, then alphanumeric, hyphens, or underscores
    let mut chars = name_trimmed.chars();
    if let Some(first) = chars.next()
        && !first.is_ascii_alphabetic()
    {
        return Err(ProxmoxError::StorageError {
            storage: name_trimmed.to_string(),
            reason: "Storage name must start with a letter".to_string(),
        });
    }

    for c in name_trimmed.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
            return Err(ProxmoxError::StorageError {
                storage: name_trimmed.to_string(),
                reason: format!(
                    "Storage name contains invalid character: '{}'. Only alphanumeric, hyphens, and underscores allowed",
                    c
                ),
            });
        }
    }

    Ok(name_trimmed)
}

/// Validates a node name - alphanumeric with hyphens, 1-63 chars (hostname format)
pub fn validate_node_name(name: &str) -> Result<&str, ProxmoxError> {
    let name_trimmed = name.trim();

    if name_trimmed.is_empty() {
        return Err(ProxmoxError::NodeError {
            node: String::new(),
            reason: "Node name cannot be empty".to_string(),
        });
    }

    if name_trimmed.len() > 63 {
        return Err(ProxmoxError::NodeError {
            node: name_trimmed.to_string(),
            reason: "Node name must be 63 characters or less".to_string(),
        });
    }

    // Hostname rules: alphanumeric and hyphens, no leading/trailing hyphens
    if name_trimmed.starts_with('-') || name_trimmed.ends_with('-') {
        return Err(ProxmoxError::NodeError {
            node: name_trimmed.to_string(),
            reason: "Node name cannot start or end with a hyphen".to_string(),
        });
    }

    for c in name_trimmed.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' {
            return Err(ProxmoxError::NodeError {
                node: name_trimmed.to_string(),
                reason: format!(
                    "Node name contains invalid character: '{}'. Only alphanumeric and hyphens allowed",
                    c
                ),
            });
        }
    }

    Ok(name_trimmed)
}

/// Validates an IP address or CIDR notation
pub fn validate_ip_or_cidr(input: &str) -> Result<&str, ProxmoxError> {
    let input_trimmed = input.trim();

    // Allow "any" as a special case
    if input_trimmed.eq_ignore_ascii_case("any") {
        return Ok(input_trimmed);
    }

    // Check for CIDR notation
    if let Some((ip_part, cidr_part)) = input_trimmed.split_once('/') {
        // Validate CIDR prefix
        match cidr_part.parse::<u8>() {
            Ok(prefix) if prefix <= 128 => {}
            _ => {
                return Err(ProxmoxError::FirewallError(format!(
                    "Invalid CIDR prefix: '{}'",
                    cidr_part
                )));
            }
        }
        validate_ip_address(ip_part)?;
    } else {
        validate_ip_address(input_trimmed)?;
    }

    Ok(input_trimmed)
}

/// Validates an IP address (IPv4 or IPv6)
fn validate_ip_address(ip: &str) -> Result<(), ProxmoxError> {
    // Try parsing as IPv4
    if ip.contains('.') {
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() != 4 {
            return Err(ProxmoxError::FirewallError(format!(
                "Invalid IPv4 address: '{}'",
                ip
            )));
        }
        for part in parts {
            match part.parse::<u8>() {
                Ok(_) => {}
                Err(_) => {
                    return Err(ProxmoxError::FirewallError(format!(
                        "Invalid IPv4 octet: '{}'",
                        part
                    )));
                }
            }
        }
        return Ok(());
    }

    // Try parsing as IPv6 (basic validation)
    if ip.contains(':') {
        // Basic check for valid IPv6 characters
        for c in ip.chars() {
            if !c.is_ascii_hexdigit() && c != ':' {
                return Err(ProxmoxError::FirewallError(format!(
                    "Invalid IPv6 character: '{}'",
                    c
                )));
            }
        }
        return Ok(());
    }

    Err(ProxmoxError::FirewallError(format!(
        "Invalid IP address format: '{}'",
        ip
    )))
}

/// Validates a port number or port range
pub fn validate_port(port: &str) -> Result<&str, ProxmoxError> {
    let port_trimmed = port.trim();

    if port_trimmed.is_empty() {
        return Ok(port_trimmed); // Empty is allowed (means all ports)
    }

    // Handle port range (e.g., "80-443")
    if let Some((start, end)) = port_trimmed.split_once('-') {
        validate_single_port(start)?;
        validate_single_port(end)?;
        return Ok(port_trimmed);
    }

    // Handle comma-separated ports (e.g., "80,443,8080")
    for port_part in port_trimmed.split(',') {
        validate_single_port(port_part.trim())?;
    }

    Ok(port_trimmed)
}

/// Validates a single port number (1-65535)
fn validate_single_port(port: &str) -> Result<u16, ProxmoxError> {
    match port.parse::<u16>() {
        Ok(p) if p >= 1 => Ok(p),
        _ => Err(ProxmoxError::FirewallError(format!(
            "Invalid port number: '{}'. Must be 1-65535",
            port
        ))),
    }
}

/// Validates a datastore name for PBS
pub fn validate_datastore_name(name: &str) -> Result<&str, ProxmoxError> {
    let name_trimmed = name.trim();

    if name_trimmed.is_empty() {
        return Err(ProxmoxError::PbsError(
            "Datastore name cannot be empty".to_string(),
        ));
    }

    if name_trimmed.len() > 64 {
        return Err(ProxmoxError::PbsError(
            "Datastore name must be 64 characters or less".to_string(),
        ));
    }

    // Same rules as storage names
    for c in name_trimmed.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
            return Err(ProxmoxError::PbsError(format!(
                "Datastore name contains invalid character: '{}'",
                c
            )));
        }
    }

    Ok(name_trimmed)
}

/// Validates a path (basic sanitization to prevent shell injection)
pub fn validate_path(path: &str) -> Result<&str, ProxmoxError> {
    let path_trimmed = path.trim();

    if path_trimmed.is_empty() {
        return Err(ProxmoxError::ConfigError {
            path: String::new(),
            reason: "Path cannot be empty".to_string(),
        });
    }

    // Check for dangerous characters that could be used for shell injection
    let dangerous_chars = [
        '`', '$', '(', ')', '|', ';', '&', '<', '>', '\n', '\r', '\0',
    ];

    for c in dangerous_chars {
        if path_trimmed.contains(c) {
            return Err(ProxmoxError::ConfigError {
                path: path_trimmed.to_string(),
                reason: format!("Path contains invalid character: '{}'", c),
            });
        }
    }

    Ok(path_trimmed)
}

/// Runs a command with proper error handling and returns the output
pub fn run_command_with_output(
    program: &str,
    args: &[&str],
) -> Result<std::process::Output, ProxmoxError> {
    Command::new(program)
        .args(args)
        .output()
        .map_err(|e| ProxmoxError::CommandError(format!("Failed to execute {}: {}", program, e)))
}

/// Runs a command and returns success/failure status with error feedback
pub fn run_command_checked(program: &str, args: &[&str]) -> Result<(), ProxmoxError> {
    let output = run_command_with_output(program, args)?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(ProxmoxError::CommandError(format!(
            "{} failed: {}",
            program,
            stderr.trim()
        )))
    }
}

/// Runs a command, prints stdout/stderr, and returns status
pub fn run_command_interactive(program: &str, args: &[&str]) -> Result<bool, ProxmoxError> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|e| ProxmoxError::CommandError(format!("Failed to execute {}: {}", program, e)))?;

    Ok(status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_vmid() {
        assert!(validate_vmid("100").is_ok());
        assert!(validate_vmid("999").is_ok());
        assert!(validate_vmid("999999999").is_ok());

        assert!(validate_vmid("99").is_err()); // Too low
        assert!(validate_vmid("0").is_err());
        assert!(validate_vmid("-1").is_err());
        assert!(validate_vmid("abc").is_err());
        assert!(validate_vmid("").is_err());
    }

    #[test]
    fn test_validate_storage_name() {
        assert!(validate_storage_name("local").is_ok());
        assert!(validate_storage_name("local-lvm").is_ok());
        assert!(validate_storage_name("my_storage_1").is_ok());

        assert!(validate_storage_name("").is_err());
        assert!(validate_storage_name("1invalid").is_err()); // Must start with letter
        assert!(validate_storage_name("has spaces").is_err());
        assert!(validate_storage_name("has;injection").is_err());
    }

    #[test]
    fn test_validate_node_name() {
        assert!(validate_node_name("pve1").is_ok());
        assert!(validate_node_name("proxmox-node-1").is_ok());

        assert!(validate_node_name("-invalid").is_err());
        assert!(validate_node_name("invalid-").is_err());
        assert!(validate_node_name("has spaces").is_err());
    }

    #[test]
    fn test_validate_port() {
        assert!(validate_port("80").is_ok());
        assert!(validate_port("443").is_ok());
        assert!(validate_port("80-443").is_ok());
        assert!(validate_port("80,443,8080").is_ok());
        assert!(validate_port("").is_ok()); // Empty is allowed

        assert!(validate_port("0").is_err());
        assert!(validate_port("70000").is_err());
        assert!(validate_port("abc").is_err());
    }

    #[test]
    fn test_validate_ip_or_cidr() {
        assert!(validate_ip_or_cidr("192.168.1.1").is_ok());
        assert!(validate_ip_or_cidr("10.0.0.0/8").is_ok());
        assert!(validate_ip_or_cidr("any").is_ok());
        assert!(validate_ip_or_cidr("ANY").is_ok());

        assert!(validate_ip_or_cidr("256.1.1.1").is_err());
        assert!(validate_ip_or_cidr("invalid").is_err());
    }

    #[test]
    fn test_validate_path() {
        assert!(validate_path("/var/lib/vz").is_ok());
        assert!(validate_path("/tmp/test").is_ok());

        assert!(validate_path("").is_err());
        assert!(validate_path("/path;rm -rf /").is_err());
        assert!(validate_path("/path$(cmd)").is_err());
        assert!(validate_path("/path`cmd`").is_err());
    }
}
