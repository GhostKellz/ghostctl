//! Input validation module for security-critical operations.
//!
//! This module provides validated types that prevent shell injection and other
//! input-based attacks by validating inputs at construction time.

use regex::Regex;
use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::LazyLock;
use thiserror::Error;

// Pre-compiled regex patterns for validation (compiled once at first use)
static IPV4_CIDR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})/(\d{1,2})$").expect("valid regex")
});
static IPV6_CIDR_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[0-9a-fA-F:]+/(\d{1,3})$").expect("valid regex"));
static INTERFACE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9._-]{0,14}$").expect("valid regex"));
static SERVICE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]{0,63}$").expect("valid regex"));
static ZONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]{0,31}$").expect("valid regex"));
static CHAIN_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]{0,63}$").expect("valid regex"));
static TABLE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]{0,31}$").expect("valid regex"));

/// Validation errors for input types
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid port number: {0}. Must be 1-65535.")]
    InvalidPort(String),

    #[error("Invalid CIDR notation: {0}. Expected format: 192.168.1.0/24 or 10.0.0.0/8")]
    InvalidCidr(String),

    #[error("Invalid IP address: {0}")]
    InvalidIpAddress(String),

    #[error("Invalid interface name: {0}. Must be alphanumeric with optional dash/underscore.")]
    InvalidInterface(String),

    #[error("Invalid service name: {0}. Must be alphanumeric.")]
    InvalidServiceName(String),

    #[error("Invalid protocol: {0}. Must be tcp, udp, or icmp.")]
    InvalidProtocol(String),

    #[error("Invalid zone name: {0}. Must be alphanumeric.")]
    InvalidZone(String),

    #[error("Invalid chain name: {0}. Must be alphanumeric with underscore.")]
    InvalidChain(String),

    #[error("Invalid table name: {0}. Must be alphanumeric.")]
    InvalidTable(String),

    #[error("Input contains shell metacharacters: {0}")]
    ShellMetacharacters(String),
}

/// Check if a string contains shell metacharacters that could enable injection
fn contains_shell_metacharacters(s: &str) -> bool {
    // Characters that have special meaning in shell contexts
    let dangerous_chars = [
        ';', '|', '&', '$', '`', '(', ')', '{', '}', '[', ']', '<', '>', '!', '\\', '"', '\'',
        '\n', '\r', '\t', '*', '?', '#', '~', '=', '%', '^',
    ];
    s.chars().any(|c| dangerous_chars.contains(&c))
}

/// A validated port number (1-65535)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidatedPort(u16);

impl ValidatedPort {
    /// Create a new validated port from a u16
    pub fn new(port: u16) -> Result<Self, ValidationError> {
        if port == 0 {
            return Err(ValidationError::InvalidPort("0".to_string()));
        }
        Ok(Self(port))
    }

    /// Create from string input
    pub fn from_input(input: &str) -> Result<Self, ValidationError> {
        let trimmed = input.trim();

        if contains_shell_metacharacters(trimmed) {
            return Err(ValidationError::ShellMetacharacters(input.to_string()));
        }

        match trimmed.parse::<u16>() {
            Ok(0) => Err(ValidationError::InvalidPort(input.to_string())),
            Ok(port) => Ok(Self(port)),
            Err(_) => Err(ValidationError::InvalidPort(input.to_string())),
        }
    }

    /// Get the port number
    pub fn value(&self) -> u16 {
        self.0
    }
}

impl fmt::Display for ValidatedPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A validated CIDR notation (e.g., 192.168.1.0/24)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedCidr(String);

impl ValidatedCidr {
    /// Create from string input
    pub fn from_input(input: &str) -> Result<Self, ValidationError> {
        let trimmed = input.trim();

        if contains_shell_metacharacters(trimmed) {
            return Err(ValidationError::ShellMetacharacters(input.to_string()));
        }

        // Use pre-compiled static regexes
        if let Some(caps) = IPV4_CIDR_REGEX.captures(trimmed) {
            // Validate each octet is 0-255
            for i in 1..=4 {
                if let Some(octet) = caps.get(i) {
                    let val: u32 = octet.as_str().parse().unwrap_or(256);
                    if val > 255 {
                        return Err(ValidationError::InvalidCidr(input.to_string()));
                    }
                }
            }
            // Validate prefix is 0-32
            if let Some(prefix) = caps.get(5) {
                let val: u32 = prefix.as_str().parse().unwrap_or(33);
                if val > 32 {
                    return Err(ValidationError::InvalidCidr(input.to_string()));
                }
            }
            return Ok(Self(trimmed.to_string()));
        }

        if IPV6_CIDR_REGEX.is_match(trimmed) {
            // Basic IPv6 CIDR validation
            let parts: Vec<&str> = trimmed.split('/').collect();
            if parts.len() == 2 {
                let prefix: u32 = parts[1].parse().unwrap_or(129);
                if prefix <= 128 {
                    return Ok(Self(trimmed.to_string()));
                }
            }
        }

        Err(ValidationError::InvalidCidr(input.to_string()))
    }

    /// Get the CIDR string
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidatedCidr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A validated IP address (IPv4 or IPv6)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedIpAddress(String);

impl ValidatedIpAddress {
    /// Create from string input
    pub fn from_input(input: &str) -> Result<Self, ValidationError> {
        let trimmed = input.trim();

        if contains_shell_metacharacters(trimmed) {
            return Err(ValidationError::ShellMetacharacters(input.to_string()));
        }

        // Use std::net to validate IP address
        match IpAddr::from_str(trimmed) {
            Ok(_) => Ok(Self(trimmed.to_string())),
            Err(_) => Err(ValidationError::InvalidIpAddress(input.to_string())),
        }
    }

    /// Get the IP address string
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidatedIpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A validated network interface name (e.g., eth0, wlan0, br-lan)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedInterface(String);

impl ValidatedInterface {
    /// Create from string input
    pub fn from_input(input: &str) -> Result<Self, ValidationError> {
        let trimmed = input.trim();

        if contains_shell_metacharacters(trimmed) {
            return Err(ValidationError::ShellMetacharacters(input.to_string()));
        }

        // Interface names: alphanumeric, dash, underscore, dot. Max 15 chars (Linux limit)
        if INTERFACE_REGEX.is_match(trimmed) {
            Ok(Self(trimmed.to_string()))
        } else {
            Err(ValidationError::InvalidInterface(input.to_string()))
        }
    }

    /// Get the interface name
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidatedInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A validated service name (e.g., ssh, http, https)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedServiceName(String);

impl ValidatedServiceName {
    /// Well-known services that are always allowed
    const KNOWN_SERVICES: &'static [&'static str] = &[
        "ssh",
        "http",
        "https",
        "ftp",
        "smtp",
        "pop3",
        "imap",
        "dns",
        "ntp",
        "snmp",
        "ldap",
        "mysql",
        "postgresql",
        "redis",
        "mongodb",
        "elasticsearch",
        "docker",
        "kubernetes",
        "prometheus",
        "grafana",
        "nginx",
        "apache",
        "tomcat",
        "jenkins",
        "gitlab",
        "git",
        "telnet",
        "rdp",
        "vnc",
        "smb",
        "nfs",
        "iscsi",
        "syslog",
        "kerberos",
        "dhcp",
        "tftp",
        "openvpn",
        "wireguard",
        "ipsec",
        "l2tp",
        "pptp",
    ];

    /// Create from string input
    pub fn from_input(input: &str) -> Result<Self, ValidationError> {
        let trimmed = input.trim().to_lowercase();

        if contains_shell_metacharacters(&trimmed) {
            return Err(ValidationError::ShellMetacharacters(input.to_string()));
        }

        // Allow known services
        if Self::KNOWN_SERVICES.contains(&trimmed.as_str()) {
            return Ok(Self(trimmed));
        }

        // For unknown services, require strict alphanumeric format
        if SERVICE_REGEX.is_match(&trimmed) {
            Ok(Self(trimmed))
        } else {
            Err(ValidationError::InvalidServiceName(input.to_string()))
        }
    }

    /// Get the service name
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidatedServiceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A validated network protocol (tcp, udp, icmp)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidatedProtocol {
    Tcp,
    Udp,
    Icmp,
    All,
}

impl ValidatedProtocol {
    /// Create from string input
    pub fn from_input(input: &str) -> Result<Self, ValidationError> {
        match input.trim().to_lowercase().as_str() {
            "tcp" => Ok(Self::Tcp),
            "udp" => Ok(Self::Udp),
            "icmp" => Ok(Self::Icmp),
            "all" | "" => Ok(Self::All),
            _ => Err(ValidationError::InvalidProtocol(input.to_string())),
        }
    }

    /// Get the protocol as a string for commands
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Icmp => "icmp",
            Self::All => "all",
        }
    }
}

impl fmt::Display for ValidatedProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A validated firewall zone name (e.g., public, internal, dmz)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedZone(String);

impl ValidatedZone {
    /// Common firewall zones
    const KNOWN_ZONES: &'static [&'static str] = &[
        "public", "private", "internal", "external", "dmz", "work", "home", "trusted", "drop",
        "block",
    ];

    /// Create from string input
    pub fn from_input(input: &str) -> Result<Self, ValidationError> {
        let trimmed = input.trim().to_lowercase();

        if contains_shell_metacharacters(&trimmed) {
            return Err(ValidationError::ShellMetacharacters(input.to_string()));
        }

        // Allow known zones
        if Self::KNOWN_ZONES.contains(&trimmed.as_str()) {
            return Ok(Self(trimmed));
        }

        // For custom zones, require strict alphanumeric format
        if ZONE_REGEX.is_match(&trimmed) {
            Ok(Self(trimmed))
        } else {
            Err(ValidationError::InvalidZone(input.to_string()))
        }
    }

    /// Get the zone name
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidatedZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A validated iptables/nftables chain name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedChain(String);

impl ValidatedChain {
    /// Standard chains
    const STANDARD_CHAINS: &'static [&'static str] =
        &["INPUT", "OUTPUT", "FORWARD", "PREROUTING", "POSTROUTING"];

    /// Create from string input
    pub fn from_input(input: &str) -> Result<Self, ValidationError> {
        let trimmed = input.trim();

        if contains_shell_metacharacters(trimmed) {
            return Err(ValidationError::ShellMetacharacters(input.to_string()));
        }

        // Allow standard chains (case-insensitive match, but preserve original case)
        let upper = trimmed.to_uppercase();
        if Self::STANDARD_CHAINS.contains(&upper.as_str()) {
            return Ok(Self(upper));
        }

        // For custom chains, require alphanumeric with underscore
        if CHAIN_REGEX.is_match(trimmed) {
            Ok(Self(trimmed.to_string()))
        } else {
            Err(ValidationError::InvalidChain(input.to_string()))
        }
    }

    /// Get the chain name
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidatedChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A validated iptables/nftables table name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedTable(String);

impl ValidatedTable {
    /// Standard tables
    const STANDARD_TABLES: &'static [&'static str] =
        &["filter", "nat", "mangle", "raw", "security", "inet"];

    /// Create from string input
    pub fn from_input(input: &str) -> Result<Self, ValidationError> {
        let trimmed = input.trim().to_lowercase();

        if contains_shell_metacharacters(&trimmed) {
            return Err(ValidationError::ShellMetacharacters(input.to_string()));
        }

        // Allow standard tables
        if Self::STANDARD_TABLES.contains(&trimmed.as_str()) {
            return Ok(Self(trimmed));
        }

        // For custom tables, require alphanumeric
        if TABLE_REGEX.is_match(&trimmed) {
            Ok(Self(trimmed))
        } else {
            Err(ValidationError::InvalidTable(input.to_string()))
        }
    }

    /// Get the table name
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidatedTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A validated port range (e.g., "80", "80:443", "1024:65535")
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedPortRange {
    start: u16,
    end: u16,
}

impl ValidatedPortRange {
    /// Create from string input
    pub fn from_input(input: &str) -> Result<Self, ValidationError> {
        let trimmed = input.trim();

        if contains_shell_metacharacters(trimmed) {
            return Err(ValidationError::ShellMetacharacters(input.to_string()));
        }

        // Single port
        if let Ok(port) = trimmed.parse::<u16>() {
            if port == 0 {
                return Err(ValidationError::InvalidPort(input.to_string()));
            }
            return Ok(Self {
                start: port,
                end: port,
            });
        }

        // Port range (start:end)
        let parts: Vec<&str> = trimmed.split(':').collect();
        if parts.len() == 2 {
            let start: u16 = parts[0]
                .parse()
                .map_err(|_| ValidationError::InvalidPort(input.to_string()))?;
            let end: u16 = parts[1]
                .parse()
                .map_err(|_| ValidationError::InvalidPort(input.to_string()))?;

            if start == 0 || end == 0 || start > end {
                return Err(ValidationError::InvalidPort(input.to_string()));
            }

            return Ok(Self { start, end });
        }

        Err(ValidationError::InvalidPort(input.to_string()))
    }

    /// Check if this is a single port
    pub fn is_single(&self) -> bool {
        self.start == self.end
    }

    /// Get start port
    pub fn start(&self) -> u16 {
        self.start
    }

    /// Get end port
    pub fn end(&self) -> u16 {
        self.end
    }
}

impl fmt::Display for ValidatedPortRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_single() {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}:{}", self.start, self.end)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_validation() {
        assert!(ValidatedPort::from_input("80").is_ok());
        assert!(ValidatedPort::from_input("443").is_ok());
        assert!(ValidatedPort::from_input("65535").is_ok());
        assert!(ValidatedPort::from_input("1").is_ok());

        assert!(ValidatedPort::from_input("0").is_err());
        assert!(ValidatedPort::from_input("-1").is_err());
        assert!(ValidatedPort::from_input("65536").is_err());
        assert!(ValidatedPort::from_input("abc").is_err());
        assert!(ValidatedPort::from_input("80; rm -rf /").is_err());
        assert!(ValidatedPort::from_input("80`whoami`").is_err());
    }

    #[test]
    fn test_cidr_validation() {
        assert!(ValidatedCidr::from_input("192.168.1.0/24").is_ok());
        assert!(ValidatedCidr::from_input("10.0.0.0/8").is_ok());
        assert!(ValidatedCidr::from_input("0.0.0.0/0").is_ok());

        assert!(ValidatedCidr::from_input("192.168.1.0").is_err());
        assert!(ValidatedCidr::from_input("192.168.1.0/33").is_err());
        assert!(ValidatedCidr::from_input("256.1.1.1/24").is_err());
        assert!(ValidatedCidr::from_input("192.168.1.0/24; rm -rf /").is_err());
    }

    #[test]
    fn test_interface_validation() {
        assert!(ValidatedInterface::from_input("eth0").is_ok());
        assert!(ValidatedInterface::from_input("wlan0").is_ok());
        assert!(ValidatedInterface::from_input("br-lan").is_ok());
        assert!(ValidatedInterface::from_input("docker0").is_ok());
        assert!(ValidatedInterface::from_input("veth123abc").is_ok());

        assert!(ValidatedInterface::from_input("eth0; whoami").is_err());
        assert!(ValidatedInterface::from_input("").is_err());
        assert!(ValidatedInterface::from_input("this_is_a_very_long_interface_name").is_err());
    }

    #[test]
    fn test_service_validation() {
        assert!(ValidatedServiceName::from_input("ssh").is_ok());
        assert!(ValidatedServiceName::from_input("HTTP").is_ok());
        assert!(ValidatedServiceName::from_input("my_service").is_ok());

        assert!(ValidatedServiceName::from_input("ssh; id").is_err());
        assert!(ValidatedServiceName::from_input("$(whoami)").is_err());
    }

    #[test]
    fn test_shell_metacharacter_detection() {
        assert!(contains_shell_metacharacters("; rm -rf /"));
        assert!(contains_shell_metacharacters("| cat /etc/passwd"));
        assert!(contains_shell_metacharacters("$(whoami)"));
        assert!(contains_shell_metacharacters("`id`"));
        assert!(contains_shell_metacharacters("foo && bar"));

        assert!(!contains_shell_metacharacters("192.168.1.1"));
        assert!(!contains_shell_metacharacters("eth0"));
        assert!(!contains_shell_metacharacters("ssh"));
    }
}
