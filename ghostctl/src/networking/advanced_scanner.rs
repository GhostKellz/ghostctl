use anyhow::{Context, Result};
use ipnet::Ipv4Net;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::net::IpAddr;
use std::time::{Duration, Instant, SystemTime};
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Advanced scanning techniques beyond basic TCP connect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanTechnique {
    TcpConnect, // Current implementation - full TCP handshake
    TcpSyn,     // Half-open scanning (requires raw sockets)
    TcpAck,     // ACK scanning for firewall detection
    TcpWindow,  // Window scanning for OS detection
    TcpMaimon,  // Maimon scanning technique
    UdpScan,    // UDP port scanning
    IcmpScan,   // ICMP ping scanning
    ArpScan,    // ARP discovery scanning
    ScriptScan, // NSE-like script scanning
}

/// Timing templates for different scanning scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimingTemplate {
    Paranoid,   // T0: Very slow, maximum IDS evasion
    Sneaky,     // T1: Slow, good IDS evasion
    Polite,     // T2: Slower, less bandwidth usage
    Normal,     // T3: Default timing (current implementation)
    Aggressive, // T4: Faster, assumes fast reliable network
    Insane,     // T5: Very fast, assumes extremely fast network
}

impl TimingTemplate {
    pub fn get_timing_config(&self) -> TimingConfig {
        match self {
            TimingTemplate::Paranoid => TimingConfig {
                min_rtt_timeout: Duration::from_millis(10000),
                max_rtt_timeout: Duration::from_millis(15000),
                max_parallelism: 1,
                scan_delay: Duration::from_millis(5000),
                max_retries: 10,
            },
            TimingTemplate::Sneaky => TimingConfig {
                min_rtt_timeout: Duration::from_millis(5000),
                max_rtt_timeout: Duration::from_millis(10000),
                max_parallelism: 5,
                scan_delay: Duration::from_millis(1000),
                max_retries: 5,
            },
            TimingTemplate::Polite => TimingConfig {
                min_rtt_timeout: Duration::from_millis(2500),
                max_rtt_timeout: Duration::from_millis(5000),
                max_parallelism: 10,
                scan_delay: Duration::from_millis(400),
                max_retries: 3,
            },
            TimingTemplate::Normal => TimingConfig {
                min_rtt_timeout: Duration::from_millis(1000),
                max_rtt_timeout: Duration::from_millis(3000),
                max_parallelism: 50,
                scan_delay: Duration::from_millis(0),
                max_retries: 2,
            },
            TimingTemplate::Aggressive => TimingConfig {
                min_rtt_timeout: Duration::from_millis(500),
                max_rtt_timeout: Duration::from_millis(1500),
                max_parallelism: 100,
                scan_delay: Duration::from_millis(0),
                max_retries: 1,
            },
            TimingTemplate::Insane => TimingConfig {
                min_rtt_timeout: Duration::from_millis(250),
                max_rtt_timeout: Duration::from_millis(750),
                max_parallelism: 300,
                scan_delay: Duration::from_millis(0),
                max_retries: 0,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimingConfig {
    pub min_rtt_timeout: Duration,
    pub max_rtt_timeout: Duration,
    pub max_parallelism: usize,
    pub scan_delay: Duration,
    pub max_retries: u8,
}

/// Enhanced scan configuration with enterprise features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedScanConfig {
    pub technique: ScanTechnique,
    pub timing_template: TimingTemplate,
    pub source_port: Option<u16>,
    pub spoof_source: Option<IpAddr>,
    pub fragment_packets: bool,
    pub randomize_hosts: bool,
    pub randomize_ports: bool,
    pub adaptive_timing: bool,
    pub os_detection: bool,
    pub service_detection: bool,
    pub script_scanning: bool,
    pub vulnerability_scanning: bool,
    pub traceroute: bool,
    pub dns_resolution: bool,
    pub exclude_ranges: Vec<String>,
    pub include_ranges: Vec<String>,
}

impl Default for AdvancedScanConfig {
    fn default() -> Self {
        Self {
            technique: ScanTechnique::TcpConnect,
            timing_template: TimingTemplate::Normal,
            source_port: None,
            spoof_source: None,
            fragment_packets: false,
            randomize_hosts: false,
            randomize_ports: false,
            adaptive_timing: true,
            os_detection: false,
            service_detection: true,
            script_scanning: false,
            vulnerability_scanning: false,
            traceroute: false,
            dns_resolution: true,
            exclude_ranges: vec![],
            include_ranges: vec![],
        }
    }
}

/// Enhanced service information with version detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: Option<String>,
    pub product: Option<String>,
    pub extrainfo: Option<String>,
    pub hostname: Option<String>,
    pub ostype: Option<String>,
    pub confidence: f32,
    pub cpe: Vec<String>, // Common Platform Enumeration
    pub banner: Option<String>,
    pub ssl_info: Option<SslInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslInfo {
    pub version: String,
    pub cipher: String,
    pub certificate: Option<CertificateInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateInfo {
    pub subject: String,
    pub issuer: String,
    pub validity_start: SystemTime,
    pub validity_end: SystemTime,
    pub serial_number: String,
    pub signature_algorithm: String,
}

/// OS fingerprinting results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsFingerprint {
    pub os_family: String,
    pub os_version: Option<String>,
    pub device_type: Option<String>,
    pub confidence: f32,
    pub fingerprint_tests: Vec<FingerprintTest>,
    pub tcp_sequence: Option<TcpSequence>,
    pub ip_id_sequence: Option<IpIdSequence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintTest {
    pub test_name: String,
    pub result: String,
    pub expected_patterns: Vec<String>,
    pub match_confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpSequence {
    pub difficulty: String,
    pub index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpIdSequence {
    pub class: String,
    pub values: Vec<u16>,
}

/// Vulnerability scanning results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityInfo {
    pub cve_id: String,
    pub severity: VulnerabilitySeverity,
    pub description: String,
    pub solution: Option<String>,
    pub references: Vec<String>,
    pub cvss_score: Option<f32>,
    pub exploitability: ExploitabilityInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VulnerabilitySeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExploitabilityInfo {
    pub is_exploitable: bool,
    pub exploit_complexity: String,
    pub available_exploits: Vec<String>,
}

/// Network range management for large scans
pub struct NetworkRange {
    pub cidr: String,
    pub exclude_ranges: Vec<String>,
    pub include_ranges: Vec<String>,
}

impl NetworkRange {
    pub fn new(cidr: &str) -> Self {
        Self {
            cidr: cidr.to_string(),
            exclude_ranges: vec![],
            include_ranges: vec![],
        }
    }

    pub fn exclude_range(&mut self, range: &str) {
        self.exclude_ranges.push(range.to_string());
    }

    pub fn include_range(&mut self, range: &str) {
        self.include_ranges.push(range.to_string());
    }

    /// Efficiently expand CIDR to target IP list with exclusions
    pub fn expand_to_targets(&self) -> Result<Vec<IpAddr>> {
        let mut targets = Vec::new();

        // Parse main CIDR range
        let network: Ipv4Net = self.cidr.parse().context("Invalid CIDR format")?;

        // Generate all hosts in the network
        for ip in network.hosts() {
            if !self.is_excluded(ip.into()) {
                targets.push(ip.into());
            }
        }

        // Add any specifically included ranges
        for include_range in &self.include_ranges {
            let include_net: Ipv4Net = include_range
                .parse()
                .context("Invalid include range format")?;
            for ip in include_net.hosts() {
                if !targets.contains(&ip.into()) {
                    targets.push(ip.into());
                }
            }
        }

        Ok(targets)
    }

    fn is_excluded(&self, ip: IpAddr) -> bool {
        for exclude_range in &self.exclude_ranges {
            if let Ok(exclude_net) = exclude_range.parse::<Ipv4Net>()
                && let IpAddr::V4(ipv4) = ip
                && exclude_net.contains(&ipv4)
            {
                return true;
            }
        }
        false
    }
}

/// Adaptive rate limiting based on network conditions
#[derive(Debug)]
pub struct AdaptiveRateLimiter {
    current_rate: f64,
    target_rate: f64,
    max_rate: f64,
    min_rate: f64,
    rtt_history: VecDeque<Duration>,
    success_rate: f64,
    last_adjustment: Instant,
    adjustment_interval: Duration,
}

impl AdaptiveRateLimiter {
    pub fn new(initial_rate: f64, max_rate: f64) -> Self {
        Self {
            current_rate: initial_rate,
            target_rate: initial_rate,
            max_rate,
            min_rate: 1.0,
            rtt_history: VecDeque::with_capacity(100),
            success_rate: 1.0,
            last_adjustment: Instant::now(),
            adjustment_interval: Duration::from_secs(5),
        }
    }

    pub fn adjust_rate(&mut self, success: bool, rtt: Duration) {
        self.rtt_history.push_back(rtt);
        if self.rtt_history.len() > 100 {
            self.rtt_history.pop_front();
        }

        // Update success rate (exponential moving average)
        let alpha = 0.1;
        self.success_rate =
            alpha * if success { 1.0 } else { 0.0 } + (1.0 - alpha) * self.success_rate;

        // Only adjust rate periodically
        if self.last_adjustment.elapsed() < self.adjustment_interval {
            return;
        }

        let avg_rtt = self.calculate_average_rtt();

        // Adjust rate based on RTT and success rate
        if avg_rtt > Duration::from_millis(500) || self.success_rate < 0.8 {
            self.current_rate *= 0.8; // Decrease rate
        } else if avg_rtt < Duration::from_millis(100) && self.success_rate > 0.95 {
            self.current_rate *= 1.2; // Increase rate
        }

        self.current_rate = self.current_rate.clamp(self.min_rate, self.max_rate);
        self.last_adjustment = Instant::now();
    }

    fn calculate_average_rtt(&self) -> Duration {
        if self.rtt_history.is_empty() {
            return Duration::from_millis(100);
        }

        let sum: Duration = self.rtt_history.iter().sum();
        sum / self.rtt_history.len() as u32
    }

    pub fn current_rate(&self) -> f64 {
        self.current_rate
    }

    pub async fn wait_for_rate(&self) {
        let delay = Duration::from_secs_f64(1.0 / self.current_rate);
        tokio::time::sleep(delay).await;
    }
}

/// Service detection probe database
pub struct ServiceProbe {
    pub name: String,
    pub probe_string: Vec<u8>,
    pub ports: Vec<u16>,
    pub protocol: Protocol,
    pub match_patterns: Vec<ServiceMatch>,
    pub fallback: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Protocol {
    Tcp,
    Udp,
    Sctp,
}

#[derive(Debug, Clone)]
pub struct ServiceMatch {
    pub pattern: Regex,
    pub service: String,
    pub version_extract: Option<String>,
    pub product_extract: Option<String>,
    pub confidence: f32,
}

/// Enhanced service detection with probe database
pub async fn advanced_service_detection(
    target: &str,
    port: u16,
    initial_banner: Option<&str>,
) -> Option<ServiceInfo> {
    let probes = load_service_probes();

    // Try specific probes for this port
    for probe in probes.iter().filter(|p| p.ports.contains(&port)) {
        if let Some(service_info) = send_probe_and_analyze(target, port, probe).await {
            return Some(service_info);
        }
    }

    // Fallback to banner analysis
    if let Some(banner) = initial_banner {
        return analyze_banner_for_service(banner, port);
    }

    None
}

async fn send_probe_and_analyze(
    target: &str,
    port: u16,
    probe: &ServiceProbe,
) -> Option<ServiceInfo> {
    // Connect to target
    let addr = format!("{}:{}", target, port);
    let mut stream = timeout(Duration::from_secs(5), TcpStream::connect(&addr))
        .await
        .ok()
        .and_then(|r| r.ok())?;

    // Send probe
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    if stream.write_all(&probe.probe_string).await.is_err() {
        return None;
    }

    // Read response
    let mut buffer = vec![0; 4096];
    let bytes_read = timeout(Duration::from_secs(3), stream.read(&mut buffer))
        .await
        .ok()
        .and_then(|r| r.ok())?;

    let response = String::from_utf8_lossy(&buffer[..bytes_read]);

    // Analyze response against patterns
    for pattern_match in &probe.match_patterns {
        if let Some(captures) = pattern_match.pattern.captures(&response) {
            return Some(ServiceInfo {
                name: pattern_match.service.clone(),
                version: extract_version(&captures, &pattern_match.version_extract),
                product: extract_product(&captures, &pattern_match.product_extract),
                extrainfo: None,
                hostname: None,
                ostype: None,
                confidence: pattern_match.confidence,
                cpe: vec![],
                banner: Some(response.to_string()),
                ssl_info: None,
            });
        }
    }

    None
}

fn extract_version(captures: &regex::Captures, extract_pattern: &Option<String>) -> Option<String> {
    if let Some(pattern) = extract_pattern
        && let Some(version_match) = captures.get(1)
    {
        return Some(version_match.as_str().to_string());
    }
    None
}

fn extract_product(captures: &regex::Captures, extract_pattern: &Option<String>) -> Option<String> {
    if let Some(pattern) = extract_pattern
        && let Some(product_match) = captures.get(2)
    {
        return Some(product_match.as_str().to_string());
    }
    None
}

fn analyze_banner_for_service(banner: &str, port: u16) -> Option<ServiceInfo> {
    // Common service patterns
    let patterns = [
        (r"SSH-(\d+\.\d+)", "ssh"),
        (r"HTTP/(\d+\.\d+)", "http"),
        (r"220.*FTP", "ftp"),
        (r"220.*SMTP", "smtp"),
        (r"\+OK.*POP3", "pop3"),
        (r"\* OK.*IMAP", "imap"),
        (r"MySQL", "mysql"),
        (r"PostgreSQL", "postgresql"),
    ];

    for (pattern, service) in &patterns {
        if let Ok(regex) = Regex::new(pattern)
            && let Some(captures) = regex.captures(banner)
        {
            return Some(ServiceInfo {
                name: service.to_string(),
                version: captures.get(1).map(|m| m.as_str().to_string()),
                product: None,
                extrainfo: None,
                hostname: None,
                ostype: None,
                confidence: 0.8,
                cpe: vec![],
                banner: Some(banner.to_string()),
                ssl_info: None,
            });
        }
    }

    None
}

fn load_service_probes() -> Vec<ServiceProbe> {
    // This would normally load from a configuration file
    // For now, return some basic probes
    vec![
        ServiceProbe {
            name: "http_get".to_string(),
            probe_string: b"GET / HTTP/1.0\r\n\r\n".to_vec(),
            ports: vec![80, 8080, 8000, 443],
            protocol: Protocol::Tcp,
            match_patterns: vec![ServiceMatch {
                pattern: Regex::new(r"Server: ([^\r\n]+)").unwrap(),
                service: "http".to_string(),
                version_extract: Some("$1".to_string()),
                product_extract: None,
                confidence: 0.9,
            }],
            fallback: None,
        },
        ServiceProbe {
            name: "ssh_version".to_string(),
            probe_string: b"SSH-2.0-GhostCTL_Scanner\r\n".to_vec(),
            ports: vec![22],
            protocol: Protocol::Tcp,
            match_patterns: vec![ServiceMatch {
                pattern: Regex::new(r"SSH-([0-9.]+)-([^\r\n]+)").unwrap(),
                service: "ssh".to_string(),
                version_extract: Some("$1".to_string()),
                product_extract: Some("$2".to_string()),
                confidence: 0.95,
            }],
            fallback: None,
        },
    ]
}

/// Export functions for integration with existing scanner
pub fn get_advanced_scan_config() -> AdvancedScanConfig {
    AdvancedScanConfig::default()
}

pub fn create_network_range(cidr: &str) -> NetworkRange {
    NetworkRange::new(cidr)
}

pub fn create_adaptive_rate_limiter(initial_rate: f64, max_rate: f64) -> AdaptiveRateLimiter {
    AdaptiveRateLimiter::new(initial_rate, max_rate)
}
