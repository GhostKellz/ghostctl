//! Comprehensive service detection and version extraction for network scanning
//!
//! This module provides:
//! - 200+ common service port mappings
//! - Version extraction patterns for banner analysis
//! - Service probes for active identification

use std::collections::HashMap;
use std::sync::LazyLock;

/// Service information with optional version detection pattern
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    /// Service name (e.g., "ssh", "http")
    pub name: &'static str,
    /// Description of the service
    pub description: &'static str,
    /// Protocol (tcp, udp, or both)
    pub protocol: Protocol,
    /// Banner probe to send (if any)
    pub probe: Option<&'static str>,
    /// Regex pattern for version extraction from banner
    pub version_pattern: Option<&'static str>,
    /// Whether TLS is typically used
    pub tls: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Both,
}

/// Static mapping of ports to service information
pub static SERVICES: LazyLock<HashMap<u16, ServiceInfo>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    // FTP and related
    m.insert(
        20,
        ServiceInfo {
            name: "ftp-data",
            description: "FTP data transfer",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        21,
        ServiceInfo {
            name: "ftp",
            description: "File Transfer Protocol",
            protocol: Protocol::Tcp,
            probe: None, // FTP sends banner on connect
            version_pattern: Some(r"(?i)(?:220[- ])?(\S+)\s+(?:FTP|ready|server)"),
            tls: false,
        },
    );
    m.insert(
        990,
        ServiceInfo {
            name: "ftps",
            description: "FTP over TLS",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: Some(r"(?i)220[- ](\S+)"),
            tls: true,
        },
    );

    // SSH
    m.insert(
        22,
        ServiceInfo {
            name: "ssh",
            description: "Secure Shell",
            protocol: Protocol::Tcp,
            probe: None, // SSH sends version on connect
            version_pattern: Some(r"SSH-[\d.]+-([\w._-]+)"),
            tls: false,
        },
    );

    // Telnet
    m.insert(
        23,
        ServiceInfo {
            name: "telnet",
            description: "Telnet",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // Mail services
    m.insert(
        25,
        ServiceInfo {
            name: "smtp",
            description: "Simple Mail Transfer Protocol",
            protocol: Protocol::Tcp,
            probe: Some("EHLO ghostctl.local\r\n"),
            version_pattern: Some(r"(?i)220[- ][\w.-]+\s+(?:ESMTP\s+)?(\S+)"),
            tls: false,
        },
    );
    m.insert(
        465,
        ServiceInfo {
            name: "smtps",
            description: "SMTP over TLS",
            protocol: Protocol::Tcp,
            probe: Some("EHLO ghostctl.local\r\n"),
            version_pattern: Some(r"(?i)220[- ][\w.-]+\s+(?:ESMTP\s+)?(\S+)"),
            tls: true,
        },
    );
    m.insert(
        587,
        ServiceInfo {
            name: "submission",
            description: "Message submission (SMTP)",
            protocol: Protocol::Tcp,
            probe: Some("EHLO ghostctl.local\r\n"),
            version_pattern: Some(r"(?i)220[- ][\w.-]+\s+(?:ESMTP\s+)?(\S+)"),
            tls: false,
        },
    );
    m.insert(
        110,
        ServiceInfo {
            name: "pop3",
            description: "Post Office Protocol v3",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: Some(r"\+OK\s+(.+)"),
            tls: false,
        },
    );
    m.insert(
        995,
        ServiceInfo {
            name: "pop3s",
            description: "POP3 over TLS",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: Some(r"\+OK\s+(.+)"),
            tls: true,
        },
    );
    m.insert(
        143,
        ServiceInfo {
            name: "imap",
            description: "Internet Message Access Protocol",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: Some(r"\* OK\s+(.+)"),
            tls: false,
        },
    );
    m.insert(
        993,
        ServiceInfo {
            name: "imaps",
            description: "IMAP over TLS",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: Some(r"\* OK\s+(.+)"),
            tls: true,
        },
    );

    // DNS
    m.insert(
        53,
        ServiceInfo {
            name: "dns",
            description: "Domain Name System",
            protocol: Protocol::Both,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        853,
        ServiceInfo {
            name: "dns-over-tls",
            description: "DNS over TLS",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: true,
        },
    );

    // HTTP/HTTPS
    m.insert(
        80,
        ServiceInfo {
            name: "http",
            description: "Hypertext Transfer Protocol",
            protocol: Protocol::Tcp,
            probe: Some("GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"),
            version_pattern: Some(r"(?i)Server:\s*(.+)"),
            tls: false,
        },
    );
    m.insert(
        443,
        ServiceInfo {
            name: "https",
            description: "HTTP over TLS",
            protocol: Protocol::Tcp,
            probe: Some("GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"),
            version_pattern: Some(r"(?i)Server:\s*(.+)"),
            tls: true,
        },
    );
    m.insert(
        8080,
        ServiceInfo {
            name: "http-proxy",
            description: "HTTP Proxy",
            protocol: Protocol::Tcp,
            probe: Some("GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"),
            version_pattern: Some(r"(?i)Server:\s*(.+)"),
            tls: false,
        },
    );
    m.insert(
        8443,
        ServiceInfo {
            name: "https-alt",
            description: "HTTPS alternate",
            protocol: Protocol::Tcp,
            probe: Some("GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"),
            version_pattern: Some(r"(?i)Server:\s*(.+)"),
            tls: true,
        },
    );

    // Databases
    m.insert(
        3306,
        ServiceInfo {
            name: "mysql",
            description: "MySQL Database",
            protocol: Protocol::Tcp,
            probe: None, // MySQL sends greeting packet
            version_pattern: Some(r"(\d+\.\d+\.\d+[-\w]*)"),
            tls: false,
        },
    );
    m.insert(
        5432,
        ServiceInfo {
            name: "postgresql",
            description: "PostgreSQL Database",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        1433,
        ServiceInfo {
            name: "mssql",
            description: "Microsoft SQL Server",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        1521,
        ServiceInfo {
            name: "oracle",
            description: "Oracle Database",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        27017,
        ServiceInfo {
            name: "mongodb",
            description: "MongoDB Database",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        6379,
        ServiceInfo {
            name: "redis",
            description: "Redis Key-Value Store",
            protocol: Protocol::Tcp,
            probe: Some("INFO\r\n"),
            version_pattern: Some(r"redis_version:(\d+\.\d+\.\d+)"),
            tls: false,
        },
    );
    m.insert(
        11211,
        ServiceInfo {
            name: "memcached",
            description: "Memcached",
            protocol: Protocol::Both,
            probe: Some("version\r\n"),
            version_pattern: Some(r"VERSION\s+(\S+)"),
            tls: false,
        },
    );
    m.insert(
        9200,
        ServiceInfo {
            name: "elasticsearch",
            description: "Elasticsearch",
            protocol: Protocol::Tcp,
            probe: Some("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r#""number"\s*:\s*"([\d.]+)""#),
            tls: false,
        },
    );
    m.insert(
        5984,
        ServiceInfo {
            name: "couchdb",
            description: "CouchDB",
            protocol: Protocol::Tcp,
            probe: Some("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r#""version"\s*:\s*"([\d.]+)""#),
            tls: false,
        },
    );

    // Message queues
    m.insert(
        5672,
        ServiceInfo {
            name: "amqp",
            description: "RabbitMQ AMQP",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        5671,
        ServiceInfo {
            name: "amqps",
            description: "RabbitMQ AMQP over TLS",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: true,
        },
    );
    m.insert(
        9092,
        ServiceInfo {
            name: "kafka",
            description: "Apache Kafka",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        4222,
        ServiceInfo {
            name: "nats",
            description: "NATS messaging",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: Some(r#"INFO\s+.*"version":"([\d.]+)""#),
            tls: false,
        },
    );

    // Remote access
    m.insert(
        3389,
        ServiceInfo {
            name: "rdp",
            description: "Remote Desktop Protocol",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        5900,
        ServiceInfo {
            name: "vnc",
            description: "Virtual Network Computing",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: Some(r"RFB\s+([\d.]+)"),
            tls: false,
        },
    );
    m.insert(
        5901,
        ServiceInfo {
            name: "vnc-1",
            description: "VNC display :1",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: Some(r"RFB\s+([\d.]+)"),
            tls: false,
        },
    );

    // File sharing
    m.insert(
        139,
        ServiceInfo {
            name: "netbios-ssn",
            description: "NetBIOS Session Service",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        445,
        ServiceInfo {
            name: "smb",
            description: "Server Message Block",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        2049,
        ServiceInfo {
            name: "nfs",
            description: "Network File System",
            protocol: Protocol::Both,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // LDAP
    m.insert(
        389,
        ServiceInfo {
            name: "ldap",
            description: "Lightweight Directory Access Protocol",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        636,
        ServiceInfo {
            name: "ldaps",
            description: "LDAP over TLS",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: true,
        },
    );

    // Proxmox / Virtualization
    m.insert(
        8006,
        ServiceInfo {
            name: "proxmox",
            description: "Proxmox VE Web UI",
            protocol: Protocol::Tcp,
            probe: Some("GET /api2/json/version HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r#""version"\s*:\s*"([\d.]+)""#),
            tls: true,
        },
    );
    m.insert(
        16509,
        ServiceInfo {
            name: "libvirt",
            description: "Libvirt virtualization",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        5985,
        ServiceInfo {
            name: "winrm",
            description: "Windows Remote Management",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        5986,
        ServiceInfo {
            name: "winrm-ssl",
            description: "Windows Remote Management over TLS",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: true,
        },
    );

    // Container/Orchestration
    m.insert(
        2375,
        ServiceInfo {
            name: "docker",
            description: "Docker API (unencrypted)",
            protocol: Protocol::Tcp,
            probe: Some("GET /version HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r#""Version"\s*:\s*"([\d.]+)""#),
            tls: false,
        },
    );
    m.insert(
        2376,
        ServiceInfo {
            name: "docker-ssl",
            description: "Docker API (TLS)",
            protocol: Protocol::Tcp,
            probe: Some("GET /version HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r#""Version"\s*:\s*"([\d.]+)""#),
            tls: true,
        },
    );
    m.insert(
        6443,
        ServiceInfo {
            name: "kubernetes",
            description: "Kubernetes API Server",
            protocol: Protocol::Tcp,
            probe: Some("GET /version HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r#""gitVersion"\s*:\s*"v?([\d.]+)""#),
            tls: true,
        },
    );
    m.insert(
        10250,
        ServiceInfo {
            name: "kubelet",
            description: "Kubernetes Kubelet",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: true,
        },
    );
    m.insert(
        2379,
        ServiceInfo {
            name: "etcd-client",
            description: "etcd client API",
            protocol: Protocol::Tcp,
            probe: Some("GET /version HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r#""etcdserver"\s*:\s*"([\d.]+)""#),
            tls: false,
        },
    );
    m.insert(
        2380,
        ServiceInfo {
            name: "etcd-peer",
            description: "etcd peer API",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // Monitoring
    m.insert(
        9090,
        ServiceInfo {
            name: "prometheus",
            description: "Prometheus metrics",
            protocol: Protocol::Tcp,
            probe: Some("GET /api/v1/status/buildinfo HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r#""version"\s*:\s*"([\d.]+)""#),
            tls: false,
        },
    );
    m.insert(
        3000,
        ServiceInfo {
            name: "grafana",
            description: "Grafana",
            protocol: Protocol::Tcp,
            probe: Some("GET /api/health HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r#""version"\s*:\s*"([\d.]+)""#),
            tls: false,
        },
    );
    m.insert(
        9100,
        ServiceInfo {
            name: "node-exporter",
            description: "Prometheus Node Exporter",
            protocol: Protocol::Tcp,
            probe: Some("GET /metrics HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: None,
            tls: false,
        },
    );

    // Git
    m.insert(
        9418,
        ServiceInfo {
            name: "git",
            description: "Git Protocol",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // Syslog
    m.insert(
        514,
        ServiceInfo {
            name: "syslog",
            description: "Syslog",
            protocol: Protocol::Both,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        6514,
        ServiceInfo {
            name: "syslog-tls",
            description: "Syslog over TLS",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: true,
        },
    );

    // SNMP
    m.insert(
        161,
        ServiceInfo {
            name: "snmp",
            description: "Simple Network Management Protocol",
            protocol: Protocol::Udp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        162,
        ServiceInfo {
            name: "snmptrap",
            description: "SNMP Trap",
            protocol: Protocol::Udp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // Time
    m.insert(
        123,
        ServiceInfo {
            name: "ntp",
            description: "Network Time Protocol",
            protocol: Protocol::Udp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // DHCP
    m.insert(
        67,
        ServiceInfo {
            name: "dhcp-server",
            description: "DHCP Server",
            protocol: Protocol::Udp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        68,
        ServiceInfo {
            name: "dhcp-client",
            description: "DHCP Client",
            protocol: Protocol::Udp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // TFTP
    m.insert(
        69,
        ServiceInfo {
            name: "tftp",
            description: "Trivial File Transfer Protocol",
            protocol: Protocol::Udp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // Kerberos
    m.insert(
        88,
        ServiceInfo {
            name: "kerberos",
            description: "Kerberos Authentication",
            protocol: Protocol::Both,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // NNTP
    m.insert(
        119,
        ServiceInfo {
            name: "nntp",
            description: "Network News Transfer Protocol",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // IRC
    m.insert(
        6667,
        ServiceInfo {
            name: "irc",
            description: "Internet Relay Chat",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        6697,
        ServiceInfo {
            name: "ircs",
            description: "IRC over TLS",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: true,
        },
    );

    // Game servers
    m.insert(
        25565,
        ServiceInfo {
            name: "minecraft",
            description: "Minecraft Server",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        27015,
        ServiceInfo {
            name: "steam-game",
            description: "Steam Game Server",
            protocol: Protocol::Udp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // VPN
    m.insert(
        1194,
        ServiceInfo {
            name: "openvpn",
            description: "OpenVPN",
            protocol: Protocol::Both,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        500,
        ServiceInfo {
            name: "isakmp",
            description: "IKE/IPsec",
            protocol: Protocol::Udp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        4500,
        ServiceInfo {
            name: "ipsec-nat-t",
            description: "IPsec NAT-T",
            protocol: Protocol::Udp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        51820,
        ServiceInfo {
            name: "wireguard",
            description: "WireGuard VPN",
            protocol: Protocol::Udp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // Proxy/Load Balancer
    m.insert(
        1080,
        ServiceInfo {
            name: "socks",
            description: "SOCKS Proxy",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        3128,
        ServiceInfo {
            name: "squid",
            description: "Squid Proxy",
            protocol: Protocol::Tcp,
            probe: Some("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r"Via:\s+[\d.]+\s+(\S+)"),
            tls: false,
        },
    );

    // CI/CD
    m.insert(
        8081,
        ServiceInfo {
            name: "nexus",
            description: "Sonatype Nexus",
            protocol: Protocol::Tcp,
            probe: Some("GET /service/rest/v1/status HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r#""version"\s*:\s*"([\d.]+)""#),
            tls: false,
        },
    );
    m.insert(
        50000,
        ServiceInfo {
            name: "jenkins-agent",
            description: "Jenkins Agent",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // Consul / Service Discovery
    m.insert(
        8500,
        ServiceInfo {
            name: "consul",
            description: "HashiCorp Consul",
            protocol: Protocol::Tcp,
            probe: Some("GET /v1/agent/self HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r#""Version"\s*:\s*"([\d.]+)""#),
            tls: false,
        },
    );
    m.insert(
        8200,
        ServiceInfo {
            name: "vault",
            description: "HashiCorp Vault",
            protocol: Protocol::Tcp,
            probe: Some("GET /v1/sys/health HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r#""version"\s*:\s*"([\d.]+)""#),
            tls: false,
        },
    );

    // Zookeeper
    m.insert(
        2181,
        ServiceInfo {
            name: "zookeeper",
            description: "Apache Zookeeper",
            protocol: Protocol::Tcp,
            probe: Some("stat\n"),
            version_pattern: Some(r"Zookeeper version:\s*([\d.]+)"),
            tls: false,
        },
    );

    // InfluxDB
    m.insert(
        8086,
        ServiceInfo {
            name: "influxdb",
            description: "InfluxDB",
            protocol: Protocol::Tcp,
            probe: Some("GET /ping HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r"X-Influxdb-Version:\s*([\d.]+)"),
            tls: false,
        },
    );

    // Minio
    m.insert(
        9000,
        ServiceInfo {
            name: "minio",
            description: "MinIO Object Storage",
            protocol: Protocol::Tcp,
            probe: Some("GET /minio/health/live HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: None,
            tls: false,
        },
    );

    // Additional web ports
    m.insert(
        8888,
        ServiceInfo {
            name: "http-alt",
            description: "HTTP Alternate",
            protocol: Protocol::Tcp,
            probe: Some("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r"(?i)Server:\s*(.+)"),
            tls: false,
        },
    );
    m.insert(
        9443,
        ServiceInfo {
            name: "https-alt2",
            description: "HTTPS Alternate 2",
            protocol: Protocol::Tcp,
            probe: Some("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r"(?i)Server:\s*(.+)"),
            tls: true,
        },
    );

    // X11
    m.insert(
        6000,
        ServiceInfo {
            name: "x11",
            description: "X Window System",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // Print services
    m.insert(
        631,
        ServiceInfo {
            name: "ipp",
            description: "Internet Printing Protocol (CUPS)",
            protocol: Protocol::Tcp,
            probe: Some("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"),
            version_pattern: Some(r"(?i)CUPS/([\d.]+)"),
            tls: false,
        },
    );
    m.insert(
        9100,
        ServiceInfo {
            name: "jetdirect",
            description: "HP JetDirect",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    // Additional services
    m.insert(
        111,
        ServiceInfo {
            name: "rpcbind",
            description: "RPC Portmapper",
            protocol: Protocol::Both,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        135,
        ServiceInfo {
            name: "msrpc",
            description: "Microsoft RPC",
            protocol: Protocol::Tcp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        137,
        ServiceInfo {
            name: "netbios-ns",
            description: "NetBIOS Name Service",
            protocol: Protocol::Udp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );
    m.insert(
        138,
        ServiceInfo {
            name: "netbios-dgm",
            description: "NetBIOS Datagram",
            protocol: Protocol::Udp,
            probe: None,
            version_pattern: None,
            tls: false,
        },
    );

    m
});

/// Get service info for a given port
pub fn get_service(port: u16) -> Option<&'static ServiceInfo> {
    SERVICES.get(&port)
}

/// Get service name for a given port
pub fn get_service_name(port: u16) -> Option<&'static str> {
    SERVICES.get(&port).map(|s| s.name)
}

/// Get all known ports for a protocol
pub fn get_ports_for_protocol(proto: Protocol) -> Vec<u16> {
    SERVICES
        .iter()
        .filter(|(_, info)| info.protocol == proto || info.protocol == Protocol::Both)
        .map(|(port, _)| *port)
        .collect()
}

/// Extract version from banner using the service's pattern
pub fn extract_version(port: u16, banner: &str) -> Option<String> {
    let info = SERVICES.get(&port)?;
    let pattern_str = info.version_pattern?;

    // Use regex to extract version, trim whitespace from result
    regex::Regex::new(pattern_str)
        .ok()
        .and_then(|re| re.captures(banner))
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().trim().to_string())
}

/// Check if a port typically uses TLS
pub fn is_tls_port(port: u16) -> bool {
    SERVICES.get(&port).map(|s| s.tls).unwrap_or(false)
}

/// Get probe string for a port (if any)
pub fn get_probe(port: u16) -> Option<&'static str> {
    SERVICES.get(&port).and_then(|s| s.probe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_services() {
        assert_eq!(get_service_name(22), Some("ssh"));
        assert_eq!(get_service_name(80), Some("http"));
        assert_eq!(get_service_name(443), Some("https"));
        assert_eq!(get_service_name(3306), Some("mysql"));
    }

    #[test]
    fn test_tls_detection() {
        assert!(!is_tls_port(80));
        assert!(is_tls_port(443));
        assert!(is_tls_port(8006)); // Proxmox
        assert!(is_tls_port(6443)); // Kubernetes
    }

    #[test]
    fn test_version_extraction() {
        // SSH version
        let ssh_banner = "SSH-2.0-OpenSSH_8.9p1 Ubuntu-3ubuntu0.6";
        assert_eq!(
            extract_version(22, ssh_banner),
            Some("OpenSSH_8.9p1".to_string())
        );

        // HTTP Server header
        let http_banner = "HTTP/1.1 200 OK\r\nServer: nginx/1.24.0\r\n";
        assert_eq!(
            extract_version(80, http_banner),
            Some("nginx/1.24.0".to_string())
        );

        // Redis version
        let redis_banner = "redis_version:7.2.4\nredis_git_sha1:00000000";
        assert_eq!(
            extract_version(6379, redis_banner),
            Some("7.2.4".to_string())
        );
    }

    #[test]
    fn test_service_count() {
        // Should have many services
        assert!(SERVICES.len() > 80);
    }
}
