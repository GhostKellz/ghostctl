//! Safe command execution helpers for firewall operations
//!
//! This module provides validated command execution that:
//! - Uses argument arrays instead of shell interpolation
//! - Applies input validation before execution
//! - Prevents shell injection attacks

use crate::security::validation::{
    ValidatedIpAddress, ValidatedPort, ValidatedProtocol, ValidatedServiceName,
};
use anyhow::{Context, Result};
use std::process::{Command, Output};
use tempfile::NamedTempFile;

/// Execute a command safely with pre-validated arguments
fn run_command(program: &str, args: &[&str]) -> Result<Output> {
    Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute: {} {:?}", program, args))
}

/// Execute a sudo command safely
fn run_sudo(args: &[&str]) -> Result<Output> {
    let mut cmd_args = vec!["sudo"];
    cmd_args.extend(args);
    run_command("sudo", args)
}

/// Check command success and return stdout
fn check_output(output: Output, context: &str) -> Result<String> {
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("{}: {}", context, stderr)
    }
}

// =============================================================================
// UFW Commands
// =============================================================================

/// Enable UFW firewall
pub fn ufw_enable() -> Result<()> {
    let output = run_sudo(&["ufw", "--force", "enable"])?;
    check_output(output, "Failed to enable UFW")?;
    Ok(())
}

/// Disable UFW firewall
pub fn ufw_disable() -> Result<()> {
    let output = run_sudo(&["ufw", "disable"])?;
    check_output(output, "Failed to disable UFW")?;
    Ok(())
}

/// Get UFW status
pub fn ufw_status() -> Result<String> {
    let output = run_sudo(&["ufw", "status", "verbose"])?;
    check_output(output, "Failed to get UFW status")
}

/// Allow a port through UFW
pub fn ufw_allow_port(port: &ValidatedPort, proto: Option<&ValidatedProtocol>) -> Result<()> {
    let args: Vec<String> = match proto {
        Some(p) => vec![
            "ufw".to_string(),
            "allow".to_string(),
            format!("{}/{}", port.value(), p.to_string()),
        ],
        None => vec!["ufw".to_string(), "allow".to_string(), port.to_string()],
    };
    let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let output = run_sudo(&args_refs)?;
    check_output(output, "Failed to add UFW allow rule")?;
    Ok(())
}

/// Deny a port through UFW
pub fn ufw_deny_port(port: &ValidatedPort, proto: Option<&ValidatedProtocol>) -> Result<()> {
    let args: Vec<String> = match proto {
        Some(p) => vec![
            "ufw".to_string(),
            "deny".to_string(),
            format!("{}/{}", port.value(), p.to_string()),
        ],
        None => vec!["ufw".to_string(), "deny".to_string(), port.to_string()],
    };
    let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let output = run_sudo(&args_refs)?;
    check_output(output, "Failed to add UFW deny rule")?;
    Ok(())
}

/// Allow from IP address
pub fn ufw_allow_from_ip(ip: &ValidatedIpAddress) -> Result<()> {
    let ip_str = ip.to_string();
    let output = run_sudo(&["ufw", "allow", "from", &ip_str])?;
    check_output(output, "Failed to add UFW allow from IP")?;
    Ok(())
}

/// Allow from IP to port
pub fn ufw_allow_from_ip_to_port(ip: &ValidatedIpAddress, port: &ValidatedPort) -> Result<()> {
    let ip_str = ip.to_string();
    let port_str = port.to_string();
    let output = run_sudo(&[
        "ufw", "allow", "from", &ip_str, "to", "any", "port", &port_str,
    ])?;
    check_output(output, "Failed to add UFW allow from IP to port")?;
    Ok(())
}

/// Deny from IP address
pub fn ufw_deny_from_ip(ip: &ValidatedIpAddress) -> Result<()> {
    let ip_str = ip.to_string();
    let output = run_sudo(&["ufw", "deny", "from", &ip_str])?;
    check_output(output, "Failed to add UFW deny from IP")?;
    Ok(())
}

/// Allow a service by name
pub fn ufw_allow_service(service: &ValidatedServiceName) -> Result<()> {
    let service_str = service.to_string();
    let output = run_sudo(&["ufw", "allow", &service_str])?;
    check_output(output, "Failed to allow service")?;
    Ok(())
}

/// Delete UFW rule by number
pub fn ufw_delete_rule(rule_num: u32) -> Result<()> {
    let num_str = rule_num.to_string();
    let output = run_sudo(&["ufw", "--force", "delete", &num_str])?;
    check_output(output, "Failed to delete UFW rule")?;
    Ok(())
}

/// Reset UFW to defaults
pub fn ufw_reset() -> Result<()> {
    let output = run_sudo(&["ufw", "--force", "reset"])?;
    check_output(output, "Failed to reset UFW")?;
    Ok(())
}

// =============================================================================
// iptables Commands
// =============================================================================

/// Add iptables rule (safe argument array)
pub fn iptables_add_rule(args: &[&str]) -> Result<()> {
    let mut full_args = vec!["iptables"];
    full_args.extend(args);
    let output = run_sudo(&full_args)?;
    check_output(output, "Failed to add iptables rule")?;
    Ok(())
}

/// Allow port with iptables
pub fn iptables_allow_port(port: &ValidatedPort, proto: &ValidatedProtocol) -> Result<()> {
    let port_str = port.to_string();
    let proto_str = proto.to_string();
    let output = run_sudo(&[
        "iptables", "-A", "INPUT", "-p", &proto_str, "--dport", &port_str, "-j", "ACCEPT",
    ])?;
    check_output(output, "Failed to add iptables allow rule")?;
    Ok(())
}

/// Drop port with iptables
pub fn iptables_drop_port(port: &ValidatedPort, proto: &ValidatedProtocol) -> Result<()> {
    let port_str = port.to_string();
    let proto_str = proto.to_string();
    let output = run_sudo(&[
        "iptables", "-A", "INPUT", "-p", &proto_str, "--dport", &port_str, "-j", "DROP",
    ])?;
    check_output(output, "Failed to add iptables drop rule")?;
    Ok(())
}

/// List iptables rules
pub fn iptables_list() -> Result<String> {
    let output = run_sudo(&["iptables", "-L", "-n", "-v"])?;
    check_output(output, "Failed to list iptables rules")
}

/// Add iptables output rule with comment (for anti-cheat, etc.)
/// This avoids shell injection by passing arguments directly
pub fn iptables_allow_output(port: u16, proto: &str, comment: &str) -> Result<()> {
    // Validate protocol
    if proto != "tcp" && proto != "udp" {
        anyhow::bail!("Invalid protocol: {}", proto);
    }

    // Validate comment (alphanumeric, spaces, hyphens only)
    if !comment
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == ' ' || c == '-' || c == '_')
    {
        anyhow::bail!("Invalid comment characters: {}", comment);
    }

    let port_str = port.to_string();
    let output = run_sudo(&[
        "iptables",
        "-A",
        "OUTPUT",
        "-p",
        proto,
        "--dport",
        &port_str,
        "-j",
        "ACCEPT",
        "-m",
        "comment",
        "--comment",
        comment,
    ])?;
    check_output(output, "Failed to add iptables output rule")?;
    Ok(())
}

/// Add iptables input rule for connection tracking
pub fn iptables_allow_established() -> Result<()> {
    let output = run_sudo(&[
        "iptables",
        "-A",
        "INPUT",
        "-m",
        "conntrack",
        "--ctstate",
        "ESTABLISHED,RELATED",
        "-j",
        "ACCEPT",
    ])?;
    check_output(output, "Failed to add iptables conntrack rule")?;
    Ok(())
}

/// Drop invalid connections
pub fn iptables_drop_invalid() -> Result<()> {
    let output = run_sudo(&[
        "iptables",
        "-A",
        "INPUT",
        "-m",
        "conntrack",
        "--ctstate",
        "INVALID",
        "-j",
        "DROP",
    ])?;
    check_output(output, "Failed to add iptables drop invalid rule")?;
    Ok(())
}

/// Allow output conntrack NEW,ESTABLISHED
pub fn iptables_allow_output_conntrack() -> Result<()> {
    let output = run_sudo(&[
        "iptables",
        "-A",
        "OUTPUT",
        "-m",
        "conntrack",
        "--ctstate",
        "NEW,ESTABLISHED",
        "-j",
        "ACCEPT",
    ])?;
    check_output(output, "Failed to add output conntrack rule")?;
    Ok(())
}

/// Allow loopback interface
pub fn iptables_allow_loopback() -> Result<()> {
    run_sudo(&["iptables", "-A", "INPUT", "-i", "lo", "-j", "ACCEPT"])?;
    run_sudo(&["iptables", "-A", "OUTPUT", "-o", "lo", "-j", "ACCEPT"])?;
    Ok(())
}

/// Allow input port with comment
pub fn iptables_allow_input(port: u16, proto: &str, comment: &str) -> Result<()> {
    if proto != "tcp" && proto != "udp" {
        anyhow::bail!("Invalid protocol: {}", proto);
    }
    if !comment
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == ' ' || c == '-' || c == '_')
    {
        anyhow::bail!("Invalid comment characters: {}", comment);
    }

    let port_str = port.to_string();
    let output = run_sudo(&[
        "iptables",
        "-A",
        "INPUT",
        "-p",
        proto,
        "--dport",
        &port_str,
        "-j",
        "ACCEPT",
        "-m",
        "comment",
        "--comment",
        comment,
    ])?;
    check_output(output, "Failed to add iptables input rule")?;
    Ok(())
}

/// Allow output to port (no comment)
pub fn iptables_allow_output_port(port: u16, proto: &str) -> Result<()> {
    if proto != "tcp" && proto != "udp" {
        anyhow::bail!("Invalid protocol: {}", proto);
    }
    let port_str = port.to_string();
    let output = run_sudo(&[
        "iptables", "-A", "OUTPUT", "-p", proto, "--dport", &port_str, "-j", "ACCEPT",
    ])?;
    check_output(output, "Failed to add iptables output port rule")?;
    Ok(())
}

/// Allow input from sport (source port)
pub fn iptables_allow_input_sport(port: u16, proto: &str) -> Result<()> {
    if proto != "tcp" && proto != "udp" {
        anyhow::bail!("Invalid protocol: {}", proto);
    }
    let port_str = port.to_string();
    let output = run_sudo(&[
        "iptables", "-A", "INPUT", "-p", proto, "--sport", &port_str, "-j", "ACCEPT",
    ])?;
    check_output(output, "Failed to add iptables input sport rule")?;
    Ok(())
}

/// UFW allow outbound with comment
pub fn ufw_allow_out(port: u16, proto: Option<&str>, comment: &str) -> Result<()> {
    if !comment
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == ' ' || c == '-' || c == '_')
    {
        anyhow::bail!("Invalid comment characters: {}", comment);
    }

    let port_spec = match proto {
        Some(p) => format!("{}/{}", port, p),
        None => port.to_string(),
    };

    let output = run_sudo(&["ufw", "allow", "out", &port_spec, "comment", comment])?;
    check_output(output, "Failed to add UFW outbound rule")?;
    Ok(())
}

/// UFW allow inbound with comment
pub fn ufw_allow_in(port: u16, proto: Option<&str>, comment: &str) -> Result<()> {
    if !comment
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == ' ' || c == '-' || c == '_')
    {
        anyhow::bail!("Invalid comment characters: {}", comment);
    }

    let port_spec = match proto {
        Some(p) => format!("{}/{}", port, p),
        None => port.to_string(),
    };

    let output = run_sudo(&["ufw", "allow", &port_spec, "comment", comment])?;
    check_output(output, "Failed to add UFW inbound rule")?;
    Ok(())
}

/// Firewalld add service
pub fn firewalld_add_service_simple(service: &str) -> Result<()> {
    // Validate service name
    if !service
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!("Invalid service name: {}", service);
    }

    let service_arg = format!("--add-service={}", service);
    let output = run_sudo(&["firewall-cmd", "--permanent", &service_arg])?;
    check_output(output, "Failed to add firewalld service")?;
    Ok(())
}

/// Firewalld add port
pub fn firewalld_add_port_simple(port: u16, proto: &str) -> Result<()> {
    if proto != "tcp" && proto != "udp" {
        anyhow::bail!("Invalid protocol: {}", proto);
    }

    let port_arg = format!("--add-port={}/{}", port, proto);
    let output = run_sudo(&["firewall-cmd", "--permanent", &port_arg])?;
    check_output(output, "Failed to add firewalld port")?;
    Ok(())
}

// =============================================================================
// Sysctl Commands
// =============================================================================

/// Set a sysctl parameter
pub fn sysctl_set(param: &str, value: &str) -> Result<()> {
    // Validate parameter name (alphanumeric, dots, underscores only)
    if !param
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_')
    {
        anyhow::bail!("Invalid sysctl parameter: {}", param);
    }

    // Value validation depends on the parameter, but basic safety check
    if value.contains(';') || value.contains('|') || value.contains('$') {
        anyhow::bail!("Invalid characters in sysctl value");
    }

    let setting = format!("{}={}", param, value);
    let output = run_sudo(&["sysctl", "-w", &setting])?;
    check_output(output, "Failed to set sysctl")?;
    Ok(())
}

// =============================================================================
// Traffic Control (tc) Commands
// =============================================================================

/// Validate network interface name
pub fn validate_interface_name(iface: &str) -> Result<()> {
    // Interface names: alphanumeric plus some special chars
    if iface.is_empty() || iface.len() > 15 {
        anyhow::bail!("Invalid interface name length");
    }
    if !iface
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
    {
        anyhow::bail!("Invalid interface name characters");
    }
    Ok(())
}

/// Run a tc command with validated interface
pub fn tc_run(args: &[&str]) -> Result<()> {
    let output = run_sudo(&[&["tc"], args].concat())?;
    check_output(output, "tc command failed")?;
    Ok(())
}

/// Add a qdisc to an interface
pub fn tc_add_qdisc(iface: &str, qdisc_args: &[&str]) -> Result<()> {
    validate_interface_name(iface)?;
    let mut args = vec!["qdisc", "add", "dev", iface];
    args.extend(qdisc_args);
    tc_run(&args)
}

/// Add a tc class
pub fn tc_add_class(iface: &str, class_args: &[&str]) -> Result<()> {
    validate_interface_name(iface)?;
    let mut args = vec!["class", "add", "dev", iface];
    args.extend(class_args);
    tc_run(&args)
}

/// Add a tc filter
pub fn tc_add_filter(iface: &str, filter_args: &[&str]) -> Result<()> {
    validate_interface_name(iface)?;
    let mut args = vec!["filter", "add", "dev", iface];
    args.extend(filter_args);
    tc_run(&args)
}

/// Show tc class stats
pub fn tc_show_class(iface: &str) -> Result<String> {
    validate_interface_name(iface)?;
    let output = run_sudo(&["tc", "-s", "class", "show", "dev", iface])?;
    check_output(output, "tc show failed")
}

/// Write content to a file using sudo (for system config files)
pub fn sudo_write_file(path: &str, content: &str) -> Result<()> {
    use std::io::Write;

    // Validate path (no shell metacharacters)
    if path.contains(';')
        || path.contains('|')
        || path.contains('$')
        || path.contains('`')
        || path.contains('\n')
    {
        anyhow::bail!("Invalid characters in path");
    }

    // Use NamedTempFile for secure temp file creation with:
    // - Unpredictable filename (prevents symlink race attacks)
    // - Restrictive default permissions (0600)
    // - Automatic cleanup on drop if not persisted
    let mut temp_file = NamedTempFile::new().context("Failed to create secure temp file")?;

    temp_file
        .write_all(content.as_bytes())
        .context("Failed to write to temp file")?;

    // Sync to ensure content is written before moving
    temp_file.flush()?;

    let temp_path = temp_file.path().to_string_lossy().to_string();

    // Set target permissions before moving (0644 for config files)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o644);
        std::fs::set_permissions(temp_file.path(), perms)?;
    }

    // Keep the temp file open until sudo mv completes to prevent deletion
    let temp_file = temp_file.into_temp_path();

    // Move to final location with sudo
    let output = run_sudo(&["mv", &temp_path, path])?;
    if !output.status.success() {
        // temp_file will be cleaned up automatically on drop
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to move config file: {}", stderr);
    }

    // Prevent auto-cleanup since mv succeeded
    temp_file.close()?;

    Ok(())
}

/// Save iptables rules
pub fn iptables_save() -> Result<String> {
    let output = run_sudo(&["iptables-save"])?;
    check_output(output, "Failed to save iptables rules")
}

// =============================================================================
// nftables Commands
// =============================================================================

/// List nftables ruleset
pub fn nft_list_ruleset() -> Result<String> {
    let output = run_sudo(&["nft", "list", "ruleset"])?;
    check_output(output, "Failed to list nftables ruleset")
}

/// Add nftables table
pub fn nft_add_table(family: &str, name: &str) -> Result<()> {
    // Validate family
    let valid_families = ["ip", "ip6", "inet", "arp", "bridge", "netdev"];
    if !valid_families.contains(&family) {
        anyhow::bail!("Invalid nftables family: {}", family);
    }

    // Validate table name (alphanumeric + underscore only)
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        anyhow::bail!("Invalid table name: {}", name);
    }

    let output = run_sudo(&["nft", "add", "table", family, name])?;
    check_output(output, "Failed to add nftables table")?;
    Ok(())
}

/// Add nftables chain
pub fn nft_add_chain(
    family: &str,
    table: &str,
    chain: &str,
    chain_type: Option<&str>,
) -> Result<()> {
    // Validate inputs
    let valid_families = ["ip", "ip6", "inet", "arp", "bridge", "netdev"];
    if !valid_families.contains(&family) {
        anyhow::bail!("Invalid nftables family: {}", family);
    }

    if !table.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        anyhow::bail!("Invalid table name: {}", table);
    }

    if !chain.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        anyhow::bail!("Invalid chain name: {}", chain);
    }

    let output = if let Some(ct) = chain_type {
        // Validate chain type
        let valid_types = ["filter", "nat", "route"];
        if !valid_types.contains(&ct) {
            anyhow::bail!("Invalid chain type: {}", ct);
        }
        run_sudo(&[
            "nft",
            "add",
            "chain",
            family,
            table,
            chain,
            &format!("{{ type {} hook input priority 0; }}", ct),
        ])?
    } else {
        run_sudo(&["nft", "add", "chain", family, table, chain])?
    };

    check_output(output, "Failed to add nftables chain")?;
    Ok(())
}

/// Delete nftables table
pub fn nft_delete_table(family: &str, name: &str) -> Result<()> {
    // Validate family
    let valid_families = ["ip", "ip6", "inet", "arp", "bridge", "netdev"];
    if !valid_families.contains(&family) {
        anyhow::bail!("Invalid nftables family: {}", family);
    }

    // Validate table name
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        anyhow::bail!("Invalid table name: {}", name);
    }

    let output = run_sudo(&["nft", "delete", "table", family, name])?;
    check_output(output, "Failed to delete nftables table")?;
    Ok(())
}

/// Flush nftables ruleset
pub fn nft_flush_ruleset() -> Result<()> {
    let output = run_sudo(&["nft", "flush", "ruleset"])?;
    check_output(output, "Failed to flush nftables ruleset")?;
    Ok(())
}

// =============================================================================
// firewalld Commands
// =============================================================================

/// Get firewalld state
pub fn firewalld_state() -> Result<String> {
    let output = run_sudo(&["firewall-cmd", "--state"])?;
    check_output(output, "Failed to get firewalld state")
}

/// Add port to firewalld
pub fn firewalld_add_port(port: &ValidatedPort, proto: &ValidatedProtocol) -> Result<()> {
    let port_proto = format!("{}/{}", port.value(), proto);
    let output = run_sudo(&["firewall-cmd", "--add-port", &port_proto, "--permanent"])?;
    check_output(output, "Failed to add firewalld port")?;
    Ok(())
}

/// Remove port from firewalld
pub fn firewalld_remove_port(port: &ValidatedPort, proto: &ValidatedProtocol) -> Result<()> {
    let port_proto = format!("{}/{}", port.value(), proto);
    let output = run_sudo(&["firewall-cmd", "--remove-port", &port_proto, "--permanent"])?;
    check_output(output, "Failed to remove firewalld port")?;
    Ok(())
}

/// Add service to firewalld
pub fn firewalld_add_service(service: &ValidatedServiceName) -> Result<()> {
    let service_str = service.to_string();
    let output = run_sudo(&["firewall-cmd", "--add-service", &service_str, "--permanent"])?;
    check_output(output, "Failed to add firewalld service")?;
    Ok(())
}

/// Reload firewalld
pub fn firewalld_reload() -> Result<()> {
    let output = run_sudo(&["firewall-cmd", "--reload"])?;
    check_output(output, "Failed to reload firewalld")?;
    Ok(())
}

/// List firewalld zones
pub fn firewalld_list_zones() -> Result<String> {
    let output = run_sudo(&["firewall-cmd", "--get-zones"])?;
    check_output(output, "Failed to list firewalld zones")
}

// =============================================================================
// Utility Functions
// =============================================================================

/// Filter output lines containing a specific string (replaces grep piping)
pub fn filter_lines(output: &str, pattern: &str) -> Vec<String> {
    output
        .lines()
        .filter(|line| line.contains(pattern))
        .map(|s| s.to_string())
        .collect()
}

/// Check if a port is in use (replaces shell grep patterns)
pub fn check_port_in_ufw(port: &ValidatedPort) -> Result<bool> {
    let status = ufw_status()?;
    let port_str = port.to_string();
    Ok(status.lines().any(|line| line.contains(&port_str)))
}

/// Check if a port is in iptables
pub fn check_port_in_iptables(port: &ValidatedPort) -> Result<bool> {
    let rules = iptables_list()?;
    let port_str = port.to_string();
    Ok(rules.lines().any(|line| line.contains(&port_str)))
}

// =============================================================================
// Traffic Control (tc) Additional Commands
// =============================================================================

/// Delete qdisc from interface (for clearing before reconfiguring)
pub fn tc_del_qdisc(iface: &str, parent: &str) -> Result<()> {
    validate_interface_name(iface)?;
    // parent should be "root" or a valid handle like "1:0"
    if parent != "root"
        && !parent
            .chars()
            .all(|c| c.is_ascii_digit() || c == ':' || c == '0')
    {
        anyhow::bail!("Invalid tc parent: {}", parent);
    }
    // Ignore errors when deleting (qdisc may not exist)
    let _ = run_sudo(&["tc", "qdisc", "del", "dev", iface, parent]);
    Ok(())
}

// =============================================================================
// Ethtool Commands
// =============================================================================

/// Run ethtool with validated interface and options
pub fn ethtool_run(iface: &str, args: &[&str]) -> Result<String> {
    validate_interface_name(iface)?;
    let mut full_args = vec!["ethtool"];
    full_args.extend(args);
    full_args.push(iface);
    let output = run_sudo(&full_args)?;
    // Don't fail on ethtool errors as feature may not be supported
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Set ethtool ring buffer sizes
pub fn ethtool_set_ring(iface: &str, rx: u32, tx: u32) -> Result<()> {
    validate_interface_name(iface)?;
    let rx_str = rx.to_string();
    let tx_str = tx.to_string();
    let _ = run_sudo(&["ethtool", "-G", iface, "rx", &rx_str, "tx", &tx_str]);
    Ok(())
}

/// Set ethtool coalescing
pub fn ethtool_set_coalesce(iface: &str, rx_usecs: u32, tx_usecs: u32) -> Result<()> {
    validate_interface_name(iface)?;
    let rx_str = rx_usecs.to_string();
    let tx_str = tx_usecs.to_string();
    let _ = run_sudo(&[
        "ethtool", "-C", iface, "rx-usecs", &rx_str, "tx-usecs", &tx_str,
    ]);
    Ok(())
}

/// Disable latency-adding features
pub fn ethtool_disable_offloads(iface: &str) -> Result<()> {
    validate_interface_name(iface)?;
    let _ = run_sudo(&[
        "ethtool", "-K", iface, "tso", "off", "gso", "off", "gro", "off", "lro", "off",
    ]);
    Ok(())
}

/// Enable performance features
pub fn ethtool_enable_features(iface: &str) -> Result<()> {
    validate_interface_name(iface)?;
    let _ = run_sudo(&["ethtool", "-K", iface, "rx", "on", "tx", "on", "sg", "on"]);
    Ok(())
}

/// Show ethtool settings
pub fn ethtool_show(iface: &str) -> Result<String> {
    validate_interface_name(iface)?;
    let output = run_sudo(&["ethtool", iface])?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// =============================================================================
// System Info Commands (read-only, safe)
// =============================================================================

/// Get listening ports count
pub fn ss_listening_count() -> Result<u32> {
    let output = Command::new("ss")
        .args(["-tuln"])
        .output()
        .context("Failed to run ss")?;
    let count = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| l.contains("LISTEN"))
        .count();
    Ok(count as u32)
}

/// Get established connections count
pub fn ss_established_count() -> Result<u32> {
    let output = Command::new("ss")
        .args(["-tan"])
        .output()
        .context("Failed to run ss")?;
    let count = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| l.contains("ESTAB"))
        .count();
    Ok(count as u32)
}

/// Count iptables rules
pub fn iptables_rule_count() -> Result<u32> {
    let output = run_sudo(&["iptables", "-L", "-n"])?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().count() as u32)
}

/// Get iptables chain policies
pub fn iptables_get_policies() -> Result<Vec<String>> {
    let output = run_sudo(&["iptables", "-L", "-n"])?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout
        .lines()
        .filter(|l| l.starts_with("Chain"))
        .map(|s| s.to_string())
        .collect())
}

/// Get recent firewall-related journal entries
pub fn journalctl_firewall_recent(count: u32) -> Result<Vec<String>> {
    let count_str = count.to_string();
    let output = run_sudo(&["journalctl", "-xe", "-n", &count_str, "--no-pager"])?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout
        .lines()
        .filter(|l| {
            let lower = l.to_lowercase();
            lower.contains("block")
                || lower.contains("drop")
                || lower.contains("reject")
                || lower.contains("firewall")
                || lower.contains("ufw")
                || lower.contains("iptables")
                || lower.contains("netfilter")
        })
        .map(|s| s.to_string())
        .collect())
}

// =============================================================================
// IRQ Affinity Commands
// =============================================================================

/// Set IRQ CPU affinity
pub fn set_irq_affinity(irq: u32, cpu_mask: &str) -> Result<()> {
    // Validate cpu_mask (hex value)
    if !cpu_mask
        .chars()
        .all(|c| c.is_ascii_hexdigit() || c == 'x' || c == 'X')
    {
        anyhow::bail!("Invalid CPU mask: {}", cpu_mask);
    }

    let path = format!("/proc/irq/{}/smp_affinity", irq);
    sudo_write_file(&path, cpu_mask)
}

/// Get IRQs for a network interface
pub fn get_interface_irqs(iface: &str) -> Result<Vec<u32>> {
    validate_interface_name(iface)?;
    let output = Command::new("grep")
        .args([iface, "/proc/interrupts"])
        .output()
        .context("Failed to read /proc/interrupts")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut irqs = Vec::new();

    for line in stdout.lines() {
        if let Some(irq_str) = line.split_whitespace().next() {
            let irq_clean = irq_str.trim_end_matches(':');
            if let Ok(irq) = irq_clean.parse::<u32>() {
                irqs.push(irq);
            }
        }
    }

    Ok(irqs)
}

// =============================================================================
// Network Interface Queue Commands
// =============================================================================

/// Write to sysfs with sudo
pub fn sysfs_write(path: &str, value: &str) -> Result<()> {
    // Validate path starts with /sys/ or /proc/
    if !path.starts_with("/sys/") && !path.starts_with("/proc/") {
        anyhow::bail!("Invalid sysfs path: {}", path);
    }
    sudo_write_file(path, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_lines() {
        let output = "allow 22/tcp\nallow 80/tcp\ndeny 443/tcp";
        let filtered = filter_lines(output, "allow");
        assert_eq!(filtered.len(), 2);
        assert!(filtered[0].contains("22"));
        assert!(filtered[1].contains("80"));
    }

    #[test]
    fn test_validate_interface_name() {
        assert!(validate_interface_name("eth0").is_ok());
        assert!(validate_interface_name("enp3s0").is_ok());
        assert!(validate_interface_name("wlan0").is_ok());
        assert!(validate_interface_name("").is_err());
        assert!(validate_interface_name("eth;rm").is_err());
    }
}
