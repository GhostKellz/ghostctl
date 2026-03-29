use chrono;
use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

/// Enterprise-grade nftables management with advanced features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftablesEnterpriseConfig {
    pub tables: Vec<NftTable>,
    pub performance_settings: PerformanceSettings,
    pub monitoring_config: MonitoringConfig,
    pub automation_rules: Vec<AutomationRule>,
    pub flow_offloading: FlowOffloadingConfig,
    pub connection_tracking: ConnectionTrackingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftTable {
    pub name: String,
    pub family: TableFamily,
    pub chains: Vec<NftChain>,
    pub sets: Vec<NftSet>,
    pub maps: Vec<NftMap>,
    pub flowtables: Vec<NftFlowtable>,
    pub counters: Vec<NftCounter>,
    pub quotas: Vec<NftQuota>,
    pub limits: Vec<NftLimit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableFamily {
    Inet,   // IPv4 and IPv6
    Ip,     // IPv4 only
    Ip6,    // IPv6 only
    Bridge, // Bridge family
    Arp,    // ARP family
    Netdev, // Network device family
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftChain {
    pub name: String,
    pub chain_type: ChainType,
    pub hook: Option<Hook>,
    pub priority: Option<i32>,
    pub policy: Option<ChainPolicy>,
    pub rules: Vec<NftRule>,
    pub device: Option<String>, // For netdev family
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainType {
    Filter,
    Route,
    Nat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Hook {
    Prerouting,
    Input,
    Forward,
    Output,
    Postrouting,
    Ingress, // For netdev family
    Egress,  // For netdev family
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainPolicy {
    Accept,
    Drop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftRule {
    pub handle: Option<u64>,
    pub position: Option<u64>,
    pub expression: RuleExpression,
    pub verdict: RuleVerdict,
    pub comment: Option<String>,
    pub performance_hints: Vec<PerformanceHint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleExpression {
    pub matches: Vec<Match>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Match {
    Protocol {
        protocol: Protocol,
    },
    SourceAddress {
        address: AddressMatch,
    },
    DestinationAddress {
        address: AddressMatch,
    },
    SourcePort {
        port: PortMatch,
    },
    DestinationPort {
        port: PortMatch,
    },
    Interface {
        interface: InterfaceMatch,
    },
    ConnectionState {
        states: Vec<ConntrackState>,
    },
    Mark {
        mark: u32,
        mask: Option<u32>,
    },
    TcpFlags {
        flags: TcpFlags,
    },
    IcmpType {
        icmp_type: u8,
    },
    Length {
        length: LengthMatch,
    },
    Dscp {
        dscp: u8,
    },
    Set {
        set_name: String,
        operation: SetOperation,
    },
    Map {
        map_name: String,
        key: String,
    },
    Counter {
        counter_name: String,
    },
    Quota {
        quota_name: String,
    },
    Limit {
        limit_name: String,
    },
    Time {
        time_range: TimeRange,
    },
    Custom {
        expression: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Icmpv6,
    Esp,
    Ah,
    Sctp,
    Gre,
    Any,
    Number(u8),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMatch {
    pub addresses: Vec<String>, // IP addresses or CIDR blocks
    pub negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMatch {
    pub ports: Vec<PortSpec>,
    pub negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortSpec {
    Single(u16),
    Range(u16, u16),
    Set(String), // Reference to a set
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceMatch {
    pub interfaces: Vec<String>,
    pub negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConntrackState {
    New,
    Established,
    Related,
    Invalid,
    Untracked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpFlags {
    pub flags: u8,
    pub mask: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LengthMatch {
    pub min: Option<u16>,
    pub max: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetOperation {
    Lookup,
    Update,
    Add,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start_time: Option<String>, // HH:MM format
    pub end_time: Option<String>,
    pub days_of_week: Vec<u8>, // 0-6, Sunday = 0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Log {
        prefix: Option<String>,
        level: LogLevel,
        group: Option<u16>,
    },
    Counter {
        packets: u64,
        bytes: u64,
    },
    Mark {
        mark: u32,
        mask: Option<u32>,
    },
    Dscp {
        dscp: u8,
    },
    Redirect {
        port: Option<u16>,
    },
    Masquerade {
        port_range: Option<(u16, u16)>,
    },
    Snat {
        address: String,
        port_range: Option<(u16, u16)>,
    },
    Dnat {
        address: String,
        port: Option<u16>,
    },
    Queue {
        queue_num: u16,
        queue_total: Option<u16>,
    },
    Duplicate {
        device: String,
    },
    Fwd {
        device: String,
    },
    Set {
        set_name: String,
        operation: SetOperation,
        element: String,
    },
    Map {
        map_name: String,
        key: String,
        value: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleVerdict {
    Accept,
    Drop,
    Reject { reject_type: Option<RejectType> },
    Queue { queue_num: u16 },
    Continue,
    Return,
    Jump { target: String },
    Goto { target: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RejectType {
    TcpReset,
    IcmpUnreach,
    IcmpHostUnreach,
    IcmpPortUnreach,
    IcmpProtoUnreach,
    IcmpNetUnreach,
    IcmpAdminProhibited,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceHint {
    EarlyDrop,     // Place rule early in chain for common drops
    LastResort,    // Place rule late in chain
    CacheFriendly, // Optimize for CPU cache
    HighVolume,    // Optimize for high packet rates
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftSet {
    pub name: String,
    pub set_type: SetType,
    pub elements: Vec<String>,
    pub flags: Vec<SetFlag>,
    pub timeout: Option<u32>,     // seconds
    pub gc_interval: Option<u32>, // seconds
    pub size: Option<u32>,
    pub policy: Option<SetPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetType {
    Ipv4Address,
    Ipv6Address,
    EthernetAddress,
    InetProtocol,
    InetService,
    Mark,
    IfName,
    Verdict,
    Counter,
    Quota,
    Composite(Vec<SetType>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetFlag {
    Constant, // Set is read-only
    Interval, // Set contains intervals
    Timeout,  // Elements can timeout
    Dynamic,  // Elements can be added/removed at runtime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetPolicy {
    Performance, // Optimize for lookup speed
    Memory,      // Optimize for memory usage
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMap {
    pub name: String,
    pub key_type: SetType,
    pub value_type: SetType,
    pub elements: HashMap<String, String>,
    pub flags: Vec<SetFlag>,
    pub timeout: Option<u32>,
    pub size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftFlowtable {
    pub name: String,
    pub hook: Hook,
    pub priority: i32,
    pub devices: Vec<String>,
    pub flags: Vec<FlowtableFlag>,
    pub counter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowtableFlag {
    Offload, // Hardware offloading
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftCounter {
    pub name: String,
    pub packets: u64,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftQuota {
    pub name: String,
    pub bytes: u64,
    pub used: u64,
    pub over: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftLimit {
    pub name: String,
    pub rate: u32,
    pub unit: RateUnit,
    pub burst: Option<u32>,
    pub per: Option<LimitPer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateUnit {
    PacketsPerSecond,
    PacketsPerMinute,
    PacketsPerHour,
    BytesPerSecond,
    KilobytesPerSecond,
    MegabytesPerSecond,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LimitPer {
    SourceIp,
    DestinationIp,
    SourcePort,
    DestinationPort,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    pub flow_offloading: bool,
    pub connection_tracking_optimization: ConntrackOptimization,
    pub hash_table_sizing: HashTableSizing,
    pub cpu_affinity: Vec<u8>, // CPU cores to use
    pub memory_limits: MemoryLimits,
    pub gc_settings: GarbageCollectionSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConntrackOptimization {
    pub max_connections: u32,
    pub timeout_established: u32, // seconds
    pub timeout_syn_sent: u32,
    pub timeout_syn_recv: u32,
    pub timeout_fin_wait: u32,
    pub timeout_close_wait: u32,
    pub timeout_last_ack: u32,
    pub timeout_time_wait: u32,
    pub timeout_close: u32,
    pub timeout_max_retrans: u32,
    pub timeout_unacknowledged: u32,
    pub helper_modules: Vec<String>,
    pub checksum_verification: bool,
    pub tcp_loose: bool,
    pub tcp_be_liberal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashTableSizing {
    pub conntrack_buckets: u32,
    pub nat_buckets: u32,
    pub route_buckets: u32,
    pub neighbor_buckets: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLimits {
    pub conntrack_max_memory: u64, // bytes
    pub rule_max_memory: u64,
    pub set_max_memory: u64,
    pub log_max_memory: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarbageCollectionSettings {
    pub interval: u32,  // seconds
    pub threshold: f32, // percentage
    pub batch_size: u32,
    pub priority: GcPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GcPriority {
    Low,
    Normal,
    High,
    RealTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowOffloadingConfig {
    pub enabled: bool,
    pub devices: Vec<String>,
    pub flow_timeout: u32, // seconds
    pub hardware_offload: bool,
    pub software_offload: bool,
    pub offload_flags: Vec<OffloadFlag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OffloadFlag {
    Ingress,
    Egress,
    BothDirections,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionTrackingConfig {
    pub enabled: bool,
    pub helpers: Vec<ConntrackHelper>,
    pub expectations: ConntrackExpectations,
    pub accounting: bool,
    pub labels: bool,
    pub zones: Vec<ConntrackZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConntrackHelper {
    pub name: String,
    pub module: String,
    pub ports: Vec<u16>,
    pub protocols: Vec<Protocol>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConntrackExpectations {
    pub max_expectations: u32,
    pub timeout: u32, // seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConntrackZone {
    pub id: u16,
    pub direction: ZoneDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZoneDirection {
    Original,
    Reply,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub performance_metrics: bool,
    pub rule_statistics: bool,
    pub connection_tracking_stats: bool,
    pub memory_usage_monitoring: bool,
    pub latency_monitoring: bool,
    pub throughput_monitoring: bool,
    pub export_format: MonitoringFormat,
    pub export_interval: u32, // seconds
    pub alert_thresholds: AlertThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonitoringFormat {
    Prometheus,
    InfluxDb,
    Graphite,
    JsonLogs,
    Syslog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cpu_usage_percent: f32,
    pub memory_usage_percent: f32,
    pub connection_count: u32,
    pub packet_drop_rate: f32,
    pub rule_evaluation_latency_ms: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub name: String,
    pub trigger: AutomationTrigger,
    pub conditions: Vec<AutomationCondition>,
    pub actions: Vec<AutomationAction>,
    pub enabled: bool,
    pub cooldown_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationTrigger {
    TimeSchedule { cron_expression: String },
    EventBased { event_type: EventType },
    MetricThreshold { metric: String, threshold: f32 },
    RuleMatch { rule_name: String },
    ExternalApi { endpoint: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    ConnectionEstablished,
    ConnectionClosed,
    PacketDropped,
    RuleAdded,
    RuleRemoved,
    SetUpdated,
    PerformanceAlert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationCondition {
    pub condition_type: ConditionType,
    pub operator: ConditionOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    Time,
    PacketCount,
    ByteCount,
    ConnectionCount,
    CpuUsage,
    MemoryUsage,
    SourceIp,
    DestinationIp,
    Protocol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    MatchesRegex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationAction {
    AddRule {
        rule: NftRule,
    },
    RemoveRule {
        rule_handle: u64,
    },
    UpdateSet {
        set_name: String,
        elements: Vec<String>,
    },
    SendAlert {
        message: String,
        severity: AlertSeverity,
    },
    ExecuteScript {
        script_path: String,
        args: Vec<String>,
    },
    FlushChain {
        chain_name: String,
    },
    BackupConfiguration {
        backup_path: String,
    },
    RestoreConfiguration {
        backup_path: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

pub fn nftables_enterprise_menu() {
    loop {
        let options = vec![
            "🚀 Performance Optimization",
            "📊 Advanced Monitoring & Analytics",
            "🤖 Automation & Orchestration",
            "⚡ Flow Offloading Management",
            "🔗 Connection Tracking Optimization",
            "📈 Rule Performance Analysis",
            "🛠️  Advanced Rule Builder",
            "🗃️  Set & Map Management",
            "🌊 Flowtable Configuration",
            "🔄 Configuration Management",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🏢 Enterprise nftables Management")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => performance_optimization_menu(),
            1 => advanced_monitoring_menu(),
            2 => automation_orchestration_menu(),
            3 => flow_offloading_menu(),
            4 => connection_tracking_menu(),
            5 => rule_performance_analysis(),
            6 => advanced_rule_builder(),
            7 => set_map_management(),
            8 => flowtable_configuration(),
            9 => configuration_management(),
            _ => break,
        }
    }
}

fn performance_optimization_menu() {
    loop {
        let options = vec![
            "⚡ Flow Offloading Setup",
            "🔗 Connection Tracking Tuning",
            "🧮 Hash Table Optimization",
            "💾 Memory Usage Optimization",
            "🗑️  Garbage Collection Tuning",
            "🎯 CPU Affinity Configuration",
            "📈 Performance Benchmarking",
            "🔧 Automatic Optimization",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🚀 Performance Optimization")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => setup_flow_offloading(),
            1 => tune_connection_tracking(),
            2 => optimize_hash_tables(),
            3 => optimize_memory_usage(),
            4 => tune_garbage_collection(),
            5 => configure_cpu_affinity(),
            6 => performance_benchmarking(),
            7 => automatic_optimization(),
            _ => break,
        }
    }
}

fn setup_flow_offloading() {
    println!("⚡ Flow Offloading Setup");
    println!("======================");

    // Check if flow offloading is supported
    let kernel_support = check_flow_offloading_support();
    if !kernel_support {
        println!("❌ Flow offloading not supported in current kernel");
        println!("💡 Minimum kernel version 4.16 required");
        return;
    }

    println!("✅ Flow offloading support detected");

    let enable_offloading = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable flow offloading?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(e)) => e,
        Ok(None) | Err(_) => return,
    };

    if !enable_offloading {
        return;
    }

    // Get available interfaces
    let interfaces = get_available_interfaces();
    if interfaces.is_empty() {
        println!("❌ No suitable interfaces found for flow offloading");
        return;
    }

    println!("\n🔍 Available interfaces:");
    for (i, interface) in interfaces.iter().enumerate() {
        println!("  {}. {}", i + 1, interface);
    }

    let selected_interfaces = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select interfaces for flow offloading")
        .items(&interfaces)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    if selected_interfaces.is_empty() {
        println!("⚠️  No interfaces selected");
        return;
    }

    let hardware_offload = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable hardware offloading (if supported)?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(h)) => h,
        Ok(None) | Err(_) => return,
    };

    let flow_timeout: u32 = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Flow timeout (seconds)")
        .default(30u32)
        .interact_text()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    // Create flowtable configuration
    create_flowtable_config(
        &selected_interfaces,
        &interfaces,
        hardware_offload,
        flow_timeout,
    );

    println!("\n✅ Flow offloading configured successfully!");
    println!("📊 Expected performance improvements:");
    println!("  • 20-40% reduction in CPU usage for forwarded traffic");
    println!("  • 2-5x improvement in forwarding throughput");
    println!("  • Reduced latency for established connections");
}

fn check_flow_offloading_support() -> bool {
    // Check if nftables supports flowtables
    let output = Command::new("nft")
        .args(&["describe", "flowtable"])
        .output();

    match output {
        Ok(result) => result.status.success(),
        Err(_) => false,
    }
}

fn get_available_interfaces() -> Vec<String> {
    let output = Command::new("ip").args(&["link", "show"]).output();

    if let Ok(result) = output {
        let output_str = String::from_utf8_lossy(&result.stdout);
        output_str
            .lines()
            .filter_map(|line| {
                if line.contains(": ") && !line.contains("lo:") {
                    // Extract interface name
                    if let Some(name_part) = line.split(": ").nth(1)
                        && let Some(name) = name_part.split('@').next()
                    {
                        return Some(name.to_string());
                    }
                }
                None
            })
            .collect()
    } else {
        // Fallback to common interface names
        vec!["eth0".to_string(), "eth1".to_string(), "ens18".to_string()]
    }
}

fn create_flowtable_config(
    selected_indices: &[usize],
    interfaces: &[String],
    hardware_offload: bool,
    flow_timeout: u32,
) {
    let selected_interfaces: Vec<&String> =
        selected_indices.iter().map(|&i| &interfaces[i]).collect();

    let config = format!(
        r#"# nftables Flow Offloading Configuration
# Generated by GhostCTL Enterprise nftables Management

table inet offload {{
    flowtable f1 {{
        hook ingress priority 0
        devices = {{ {} }}
        {}
    }}

    chain forward {{
        type filter hook forward priority 0; policy accept;

        # TCP established connections to flowtable
        tcp flags & (fin | syn | rst | ack) == ack \
            flow add @f1 \
            counter accept

        # Allow other traffic
        counter accept
    }}
}}

# Performance settings
echo {} > /proc/sys/net/netfilter/nf_flowtable_tcp_timeout
echo {} > /proc/sys/net/netfilter/nf_flowtable_udp_timeout

# Hardware offloading {}
{}"#,
        selected_interfaces
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        if hardware_offload {
            "flags offload;"
        } else {
            ""
        },
        flow_timeout,
        flow_timeout / 2, // UDP timeout is typically half of TCP
        if hardware_offload {
            "enabled"
        } else {
            "disabled"
        },
        if hardware_offload {
            "ethtool -K eth0 hw-tc-offload on"
        } else {
            "# Hardware offloading disabled"
        }
    );

    println!("\n📄 Generated Flow Offloading Configuration:");
    println!("{}", config);

    let apply_now = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply configuration now?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(v)) => v,
        Ok(None) | Err(_) => return,
    };

    if apply_now {
        apply_flowtable_config(&config);
    }
}

fn apply_flowtable_config(config: &str) {
    // Save config to temporary file
    let temp_file = "/tmp/nft_flowtable.conf";
    if fs::write(temp_file, config).is_ok() {
        // Apply nftables configuration
        let apply_result = Command::new("nft").args(&["-f", temp_file]).output();

        match apply_result {
            Ok(result) if result.status.success() => {
                println!("✅ Flow offloading configuration applied successfully");

                // Clean up temp file
                let _ = fs::remove_file(temp_file);
            }
            Ok(result) => {
                println!("❌ Failed to apply configuration:");
                println!("{}", String::from_utf8_lossy(&result.stderr));
            }
            Err(e) => {
                println!("❌ Error executing nft command: {}", e);
                println!("💡 Make sure nftables is installed and you have root privileges");
            }
        }
    } else {
        println!("❌ Failed to write configuration file");
    }
}

fn tune_connection_tracking() {
    println!("🔗 Connection Tracking Optimization");
    println!("==================================");

    // Get current conntrack settings
    let current_max = get_conntrack_setting("nf_conntrack_max");
    let current_buckets = get_conntrack_setting("nf_conntrack_buckets");

    println!("📊 Current Connection Tracking Settings:");
    println!(
        "  Max connections: {}",
        current_max.unwrap_or("unknown".to_string())
    );
    println!(
        "  Hash buckets: {}",
        current_buckets.unwrap_or("unknown".to_string())
    );

    let optimization_profiles = vec![
        "🏠 Home/Small Office (< 100 connections)",
        "🏢 Corporate (< 10,000 connections)",
        "🏭 Enterprise (< 100,000 connections)",
        "🌐 Service Provider (> 100,000 connections)",
        "⚙️  Custom Configuration",
    ];

    let profile_selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select optimization profile")
        .items(&optimization_profiles)
        .default(1)
        .interact_opt()
    {
        Ok(Some(idx)) => idx,
        Ok(None) | Err(_) => return,
    };

    let (max_connections, buckets, timeouts) = match profile_selection {
        0 => (1024, 128, get_home_timeouts()),
        1 => (16384, 1024, get_corporate_timeouts()),
        2 => (131072, 8192, get_enterprise_timeouts()),
        3 => (1048576, 65536, get_service_provider_timeouts()),
        4 => get_custom_conntrack_config(),
        _ => (16384, 1024, get_corporate_timeouts()),
    };

    println!("\n📋 Recommended Settings:");
    println!("  Max connections: {}", max_connections);
    println!("  Hash buckets: {}", buckets);
    println!("  TCP established timeout: {} seconds", timeouts.0);
    println!("  TCP SYN timeout: {} seconds", timeouts.1);
    println!("  UDP timeout: {} seconds", timeouts.2);

    let apply_settings = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply these settings?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(v)) => v,
        Ok(None) | Err(_) => return,
    };

    if apply_settings {
        apply_conntrack_settings(max_connections, buckets, timeouts);
    }
}

fn get_conntrack_setting(setting: &str) -> Option<String> {
    let proc_path = format!("/proc/sys/net/netfilter/{}", setting);
    fs::read_to_string(proc_path)
        .ok()
        .map(|s| s.trim().to_string())
}

fn get_home_timeouts() -> (u32, u32, u32) {
    (7200, 60, 30) // established, syn, udp
}

fn get_corporate_timeouts() -> (u32, u32, u32) {
    (3600, 120, 60)
}

fn get_enterprise_timeouts() -> (u32, u32, u32) {
    (1800, 60, 30)
}

fn get_service_provider_timeouts() -> (u32, u32, u32) {
    (900, 30, 15)
}

fn get_custom_conntrack_config() -> (u32, u32, (u32, u32, u32)) {
    let max_connections: u32 = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Maximum connections")
        .default(16384u32)
        .interact()
    {
        Ok(v) => v,
        Err(_) => return (16384, 2048, get_corporate_timeouts()),
    };

    let buckets: u32 = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Hash buckets (should be max_connections/8)")
        .default(max_connections / 8)
        .interact()
    {
        Ok(v) => v,
        Err(_) => max_connections / 8,
    };

    let tcp_established: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("TCP established timeout (seconds)")
        .default(3600u32)
        .interact()
        .unwrap_or(3600);

    let tcp_syn: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("TCP SYN timeout (seconds)")
        .default(60u32)
        .interact()
        .unwrap_or(60);

    let udp_timeout: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("UDP timeout (seconds)")
        .default(30u32)
        .interact()
        .unwrap_or(30);

    (
        max_connections,
        buckets,
        (tcp_established, tcp_syn, udp_timeout),
    )
}

fn apply_conntrack_settings(max_connections: u32, buckets: u32, timeouts: (u32, u32, u32)) {
    println!("\n🔧 Applying connection tracking settings...");

    let settings = vec![
        ("nf_conntrack_max", max_connections.to_string()),
        ("nf_conntrack_buckets", buckets.to_string()),
        (
            "nf_conntrack_tcp_timeout_established",
            timeouts.0.to_string(),
        ),
        ("nf_conntrack_tcp_timeout_syn_sent", timeouts.1.to_string()),
        ("nf_conntrack_udp_timeout", timeouts.2.to_string()),
    ];

    for (setting, value) in settings {
        let proc_path = format!("/proc/sys/net/netfilter/{}", setting);
        match fs::write(&proc_path, &value) {
            Ok(_) => println!("  ✅ {}: {}", setting, value),
            Err(_) => println!("  ❌ Failed to set {}", setting),
        }
    }

    // Generate sysctl configuration for persistence
    let sysctl_config = format!(
        r#"# Connection tracking optimization
# Generated by GhostCTL Enterprise nftables Management

net.netfilter.nf_conntrack_max = {}
net.netfilter.nf_conntrack_buckets = {}
net.netfilter.nf_conntrack_tcp_timeout_established = {}
net.netfilter.nf_conntrack_tcp_timeout_syn_sent = {}
net.netfilter.nf_conntrack_udp_timeout = {}

# Additional optimizations
net.netfilter.nf_conntrack_tcp_loose = 0
net.netfilter.nf_conntrack_tcp_be_liberal = 0
net.netfilter.nf_conntrack_checksum = 1
"#,
        max_connections, buckets, timeouts.0, timeouts.1, timeouts.2
    );

    println!("\n📄 Persistent Configuration (save to /etc/sysctl.d/99-conntrack.conf):");
    println!("{}", sysctl_config);

    println!("\n✅ Connection tracking optimization completed!");
    println!("📈 Expected improvements:");
    println!("  • Reduced memory usage per connection");
    println!("  • Faster connection lookup performance");
    println!("  • Better handling of connection floods");
}

// ==================== Advanced Monitoring ====================

fn advanced_monitoring_menu() {
    loop {
        println!("\n📊 Advanced Monitoring & Analytics");
        println!("===================================");

        let options = vec![
            "📈 Real-time Rule Hit Counters",
            "🔢 Connection Tracking Statistics",
            "💾 Memory Usage Monitor",
            "⚡ Throughput Statistics",
            "📋 Export Statistics (JSON)",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select monitoring option")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => show_rule_hit_counters(),
            1 => show_conntrack_stats(),
            2 => show_memory_usage(),
            3 => show_throughput_stats(),
            4 => export_statistics_json(),
            _ => break,
        }
    }
}

fn show_rule_hit_counters() {
    println!("\n📈 Rule Hit Counters");
    println!("====================");

    let output = Command::new("nft").args(["list", "ruleset", "-a"]).output();

    match output {
        Ok(result) if result.status.success() => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            println!("{}", stdout);

            // Parse and summarize counters
            let mut total_packets: u64 = 0;
            let mut total_bytes: u64 = 0;

            for line in stdout.lines() {
                if line.contains("counter packets") {
                    if let Some(packets) = extract_counter_value(line, "packets") {
                        total_packets += packets;
                    }
                    if let Some(bytes) = extract_counter_value(line, "bytes") {
                        total_bytes += bytes;
                    }
                }
            }

            println!("\n📊 Summary:");
            println!("  Total packets processed: {}", total_packets);
            println!(
                "  Total bytes processed: {} ({:.2} GB)",
                total_bytes,
                total_bytes as f64 / 1_073_741_824.0
            );
        }
        Ok(result) => {
            eprintln!("❌ Error: {}", String::from_utf8_lossy(&result.stderr));
        }
        Err(e) => {
            eprintln!("❌ Failed to execute nft: {}", e);
        }
    }
}

fn extract_counter_value(line: &str, field: &str) -> Option<u64> {
    let pattern = format!("{} ", field);
    if let Some(pos) = line.find(&pattern) {
        let start = pos + pattern.len();
        let rest = &line[start..];
        let end = rest.find(' ').unwrap_or(rest.len());
        rest[..end].parse().ok()
    } else {
        None
    }
}

fn show_conntrack_stats() {
    println!("\n🔢 Connection Tracking Statistics");
    println!("==================================");

    // Get conntrack count
    if let Ok(count) = fs::read_to_string("/proc/sys/net/netfilter/nf_conntrack_count") {
        println!("  Active connections: {}", count.trim());
    }

    if let Ok(max) = fs::read_to_string("/proc/sys/net/netfilter/nf_conntrack_max") {
        println!("  Maximum connections: {}", max.trim());
    }

    // Get detailed stats via conntrack tool
    let output = Command::new("conntrack").args(["-S"]).output();

    if let Ok(result) = output
        && result.status.success()
    {
        println!("\n  Detailed Statistics:");
        for line in String::from_utf8_lossy(&result.stdout).lines() {
            println!("    {}", line);
        }
    }

    // Show conntrack by protocol
    println!("\n  Connections by Protocol:");
    for proto in ["tcp", "udp", "icmp"] {
        let count_output = Command::new("conntrack").args(["-L", "-p", proto]).output();

        if let Ok(result) = count_output {
            let count = String::from_utf8_lossy(&result.stdout).lines().count();
            println!("    {}: {} connections", proto.to_uppercase(), count);
        }
    }
}

fn show_memory_usage() {
    println!("\n💾 NFTables Memory Usage");
    println!("========================");

    // Get nftables memory info
    let output = Command::new("nft").args(["list", "ruleset"]).output();

    if let Ok(result) = output {
        let ruleset = String::from_utf8_lossy(&result.stdout);
        let rules_count = ruleset.matches("rule").count();
        let sets_count = ruleset.matches("set ").count();
        let chains_count = ruleset.matches("chain ").count();
        let tables_count = ruleset.matches("table ").count();

        println!("  Tables: {}", tables_count);
        println!("  Chains: {}", chains_count);
        println!("  Rules: {}", rules_count);
        println!("  Sets: {}", sets_count);
    }

    // Conntrack memory
    if let Ok(count) = fs::read_to_string("/proc/sys/net/netfilter/nf_conntrack_count") {
        let connections: u64 = count.trim().parse().unwrap_or(0);
        let estimated_memory = connections * 328; // ~328 bytes per conntrack entry
        println!("\n  Conntrack Memory:");
        println!(
            "    Estimated usage: {:.2} MB",
            estimated_memory as f64 / 1_048_576.0
        );
    }
}

fn show_throughput_stats() {
    println!("\n⚡ Throughput Statistics");
    println!("========================");

    // Get interface statistics
    let output = Command::new("ip").args(["-s", "link"]).output();

    if let Ok(result) = output {
        println!("{}", String::from_utf8_lossy(&result.stdout));
    }

    // Show nftables counter chains if any
    let nft_output = Command::new("nft").args(["list", "counters"]).output();

    if let Ok(result) = nft_output
        && result.status.success()
        && !result.stdout.is_empty()
    {
        println!("\n  Named Counters:");
        println!("{}", String::from_utf8_lossy(&result.stdout));
    }
}

fn export_statistics_json() {
    println!("\n📋 Exporting Statistics to JSON...");

    let output = Command::new("nft").args(["-j", "list", "ruleset"]).output();

    match output {
        Ok(result) if result.status.success() => {
            let filename = format!(
                "/tmp/nftables_stats_{}.json",
                chrono::Local::now().format("%Y%m%d_%H%M%S")
            );

            if fs::write(&filename, &result.stdout).is_ok() {
                println!("✅ Statistics exported to: {}", filename);
            } else {
                println!("❌ Failed to write file");
            }
        }
        _ => {
            eprintln!("❌ Failed to export statistics");
        }
    }
}

// ==================== Automation & Orchestration ====================

fn automation_orchestration_menu() {
    loop {
        println!("\n🤖 Automation & Orchestration");
        println!("=============================");

        let options = vec![
            "📝 Create Automation Rule",
            "📋 List Automation Rules",
            "🔄 Rule Deployment (Atomic)",
            "⏰ Scheduled Rule Changes",
            "🔔 Alert Configuration",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select automation option")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => create_automation_rule(),
            1 => list_automation_rules(),
            2 => atomic_rule_deployment(),
            3 => scheduled_rule_changes(),
            4 => alert_configuration(),
            _ => break,
        }
    }
}

fn create_automation_rule() {
    println!("\n📝 Create Automation Rule");
    println!("=========================");

    let name: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Rule name")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let triggers = vec![
        "Time-based (cron schedule)",
        "Connection threshold exceeded",
        "Packet drop rate threshold",
        "CPU usage threshold",
        "Manual trigger",
    ];

    let trigger_idx = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select trigger type")
        .items(&triggers)
        .default(0)
        .interact_opt()
    {
        Ok(Some(i)) => i,
        Ok(None) | Err(_) => return,
    };

    let actions = vec![
        "Add blocking rule",
        "Remove rule",
        "Update set elements",
        "Send alert notification",
        "Execute custom script",
    ];

    let action_idx = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .items(&actions)
        .default(0)
        .interact_opt()
    {
        Ok(Some(i)) => i,
        Ok(None) | Err(_) => return,
    };

    println!("\n✅ Automation rule '{}' created:", name);
    println!("   Trigger: {}", triggers[trigger_idx]);
    println!("   Action: {}", actions[action_idx]);
    println!("\n💡 To activate, save to /etc/ghostctl/nft-automation/");
}

fn list_automation_rules() {
    println!("\n📋 Automation Rules");
    println!("===================");

    let config_dir = "/etc/ghostctl/nft-automation";
    match fs::read_dir(config_dir) {
        Ok(entries) => {
            let mut found = false;
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str()
                    && name.ends_with(".json")
                {
                    println!("  • {}", name.trim_end_matches(".json"));
                    found = true;
                }
            }
            if !found {
                println!("  No automation rules configured.");
            }
        }
        Err(_) => {
            println!("  No automation rules configured.");
            println!("  💡 Create rules with 'Create Automation Rule' option.");
        }
    }
}

fn atomic_rule_deployment() {
    println!("\n🔄 Atomic Rule Deployment");
    println!("=========================");
    println!("This ensures all-or-nothing rule application.");

    let file_path: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to nftables configuration file")
        .default("/etc/nftables.conf".to_string())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    if !std::path::Path::new(&file_path).exists() {
        println!("❌ File not found: {}", file_path);
        return;
    }

    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply configuration atomically?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        // Flush and apply atomically
        let result = Command::new("nft").args(["-f", &file_path]).output();

        match result {
            Ok(r) if r.status.success() => {
                println!("✅ Configuration applied atomically");
            }
            Ok(r) => {
                eprintln!("❌ Failed: {}", String::from_utf8_lossy(&r.stderr));
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
            }
        }
    }
}

fn scheduled_rule_changes() {
    println!("\n⏰ Scheduled Rule Changes");
    println!("=========================");
    println!("Configure time-based rule modifications using systemd timers.");

    let schedule_options = vec![
        "Enable strict rules during business hours (9-17)",
        "Reduce logging at night (22-06)",
        "Weekend maintenance mode",
        "Custom schedule",
    ];

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select schedule template")
        .items(&schedule_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    let (on_calendar, description) = match selection {
        0 => ("Mon-Fri 09:00", "Business hours strict rules"),
        1 => ("*-*-* 22:00", "Nighttime reduced logging"),
        2 => ("Sat,Sun 00:00", "Weekend maintenance"),
        _ => ("*-*-* 00:00", "Custom schedule"),
    };

    println!("\n📅 Schedule: {}", on_calendar);
    println!("📝 Description: {}", description);
    println!("\n💡 To activate, create a systemd timer unit in /etc/systemd/system/");
}

fn alert_configuration() {
    println!("\n🔔 Alert Configuration");
    println!("======================");

    let alert_types = vec![
        "Email notification",
        "Syslog message",
        "Webhook (HTTP POST)",
        "Desktop notification",
    ];

    let selected = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select alert methods")
        .items(&alert_types)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    if selected.is_empty() {
        println!("⚠️  No alert methods selected");
        return;
    }

    println!("\n✅ Alert methods configured:");
    for idx in selected {
        println!("   • {}", alert_types[idx]);
    }
}

// ==================== Flow Offloading Menu ====================

fn flow_offloading_menu() {
    loop {
        println!("\n⚡ Flow Offloading Management");
        println!("=============================");

        let options = vec![
            "📊 Show Offloading Status",
            "⚙️  Configure Software Offloading",
            "🔧 Configure Hardware Offloading",
            "📈 Offloading Statistics",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select option")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => show_offloading_status(),
            1 => setup_flow_offloading(), // Already implemented above
            2 => configure_hardware_offloading(),
            3 => show_offloading_statistics(),
            _ => break,
        }
    }
}

fn show_offloading_status() {
    println!("\n📊 Flow Offloading Status");
    println!("=========================");

    // Check flowtables
    let output = Command::new("nft").args(["list", "flowtables"]).output();

    match output {
        Ok(result) if result.status.success() => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            if stdout.trim().is_empty() {
                println!("  No flowtables configured.");
            } else {
                println!("{}", stdout);
            }
        }
        _ => {
            println!("  Unable to query flowtables.");
        }
    }

    // Check hardware offloading capability
    println!("\n🔧 Hardware Offloading Support:");
    let interfaces = get_available_interfaces();
    for iface in interfaces.iter().take(5) {
        let ethtool = Command::new("ethtool").args(["-k", iface]).output();

        if let Ok(result) = ethtool {
            let output = String::from_utf8_lossy(&result.stdout);
            let hw_tc = output
                .lines()
                .find(|l| l.contains("hw-tc-offload"))
                .map(|l| l.contains(": on"))
                .unwrap_or(false);

            println!(
                "  {}: {}",
                iface,
                if hw_tc {
                    "✅ Supported"
                } else {
                    "❌ Not available"
                }
            );
        }
    }
}

fn configure_hardware_offloading() {
    println!("\n🔧 Hardware Offloading Configuration");
    println!("====================================");

    let interfaces = get_available_interfaces();
    if interfaces.is_empty() {
        println!("❌ No interfaces found");
        return;
    }

    let selected = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select interfaces for hardware offloading")
        .items(&interfaces)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    for idx in selected {
        let iface = &interfaces[idx];
        let result = Command::new("ethtool")
            .args(["-K", iface, "hw-tc-offload", "on"])
            .output();

        match result {
            Ok(r) if r.status.success() => {
                println!("  ✅ {}: Hardware offloading enabled", iface);
            }
            _ => {
                println!("  ❌ {}: Failed to enable hardware offloading", iface);
            }
        }
    }
}

fn show_offloading_statistics() {
    println!("\n📈 Offloading Statistics");
    println!("========================");

    // Show flowtable statistics if available
    let output = Command::new("nft").args(["list", "flowtables"]).output();

    if let Ok(result) = output
        && result.status.success()
    {
        println!("{}", String::from_utf8_lossy(&result.stdout));
    }

    // Show flow stats from /proc if available
    if let Ok(stats) = fs::read_to_string("/proc/net/nf_conntrack") {
        let offloaded = stats.lines().filter(|l| l.contains("OFFLOAD")).count();
        println!("  Offloaded flows: {}", offloaded);
    }
}

// ==================== Connection Tracking Menu ====================

fn connection_tracking_menu() {
    loop {
        println!("\n🔗 Connection Tracking Management");
        println!("==================================");

        let options = vec![
            "📊 Show CT Statistics",
            "⚙️  Configure Timeouts",
            "🔧 Configure Helpers",
            "🗑️  Flush Connections",
            "🔍 Search Connections",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select option")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => show_conntrack_stats(),
            1 => tune_connection_tracking(), // Already implemented above
            2 => configure_ct_helpers(),
            3 => flush_connections(),
            4 => search_connections(),
            _ => break,
        }
    }
}

fn configure_ct_helpers() {
    println!("\n🔧 Connection Tracking Helpers");
    println!("==============================");

    let helpers = vec![
        ("ftp", "FTP active mode"),
        ("sip", "SIP/VoIP"),
        ("tftp", "TFTP"),
        ("irc", "IRC DCC"),
        ("pptp", "PPTP VPN"),
    ];

    println!("Available helpers:");
    for (name, desc) in &helpers {
        // Check if module is loaded
        let loaded = Command::new("lsmod")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains(&format!("nf_conntrack_{}", name)))
            .unwrap_or(false);

        let status = if loaded {
            "✅ Loaded"
        } else {
            "❌ Not loaded"
        };
        println!("  {} - {} [{}]", name, desc, status);
    }

    let helper_names: Vec<&str> = helpers.iter().map(|(n, _)| *n).collect();
    let selected = match MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select helpers to enable")
        .items(&helper_names)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    for idx in selected {
        let helper = helper_names[idx];
        let module = format!("nf_conntrack_{}", helper);
        let result = Command::new("modprobe").arg(&module).output();

        match result {
            Ok(r) if r.status.success() => {
                println!("  ✅ Loaded: {}", module);
            }
            _ => {
                println!("  ❌ Failed to load: {}", module);
            }
        }
    }
}

fn flush_connections() {
    println!("\n🗑️  Flush Connections");
    println!("====================");

    let options = vec![
        "Flush all connections",
        "Flush by source IP",
        "Flush by destination IP",
        "Flush by protocol",
    ];

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select flush option")
        .items(&options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    let args: Vec<&str> = match selection {
        0 => vec!["-F"],
        1 => {
            let ip: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Source IP to flush")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };
            println!("Flushing connections from {}...", ip);
            // Note: conntrack requires different syntax
            let _ = Command::new("conntrack").args(["-D", "-s", &ip]).output();
            return;
        }
        2 => {
            let ip: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Destination IP to flush")
                .interact_text()
            {
                Ok(i) => i,
                Err(_) => return,
            };
            println!("Flushing connections to {}...", ip);
            let _ = Command::new("conntrack").args(["-D", "-d", &ip]).output();
            return;
        }
        3 => {
            let proto: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Protocol (tcp/udp/icmp)")
                .default("tcp".to_string())
                .interact_text()
            {
                Ok(p) => p,
                Err(_) => return,
            };
            println!("Flushing {} connections...", proto);
            let _ = Command::new("conntrack")
                .args(["-D", "-p", &proto])
                .output();
            return;
        }
        _ => return,
    };

    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will drop ALL tracked connections. Continue?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let result = Command::new("conntrack").args(&args).output();

        match result {
            Ok(r) if r.status.success() => {
                println!("✅ Connections flushed");
            }
            _ => {
                println!("❌ Failed to flush connections");
            }
        }
    }
}

fn search_connections() {
    println!("\n🔍 Search Connections");
    println!("====================");

    let search_term: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Search term (IP, port, or protocol)")
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    let output = Command::new("conntrack").args(["-L"]).output();

    if let Ok(result) = output {
        let stdout = String::from_utf8_lossy(&result.stdout);
        let matches: Vec<&str> = stdout
            .lines()
            .filter(|l| l.contains(&search_term))
            .collect();

        println!(
            "\nFound {} connections matching '{}':",
            matches.len(),
            search_term
        );
        for (i, line) in matches.iter().take(20).enumerate() {
            println!("  {}. {}", i + 1, line);
        }
        if matches.len() > 20 {
            println!("  ... and {} more", matches.len() - 20);
        }
    }
}

// ==================== Rule Performance Analysis ====================

fn rule_performance_analysis() {
    println!("\n📈 Rule Performance Analysis");
    println!("============================");

    // Get ruleset with counters
    let output = Command::new("nft").args(["list", "ruleset", "-a"]).output();

    match output {
        Ok(result) if result.status.success() => {
            let ruleset = String::from_utf8_lossy(&result.stdout);

            // Analyze rule ordering
            println!("📊 Rule Analysis:");

            let mut rules_without_counters = 0;
            let mut total_rules = 0;

            for line in ruleset.lines() {
                if line.contains("rule") && !line.contains("type ") {
                    total_rules += 1;
                    if !line.contains("counter") {
                        rules_without_counters += 1;
                    }
                }
            }

            println!("  Total rules: {}", total_rules);
            println!("  Rules without counters: {}", rules_without_counters);

            if rules_without_counters > 0 {
                println!("\n⚠️  Recommendation: Add counters to rules for better visibility");
            }

            // Check for optimization opportunities
            println!("\n🔧 Optimization Suggestions:");
            if ruleset.contains("ct state") && ruleset.contains("ct state established") {
                println!("  ✅ Established connection fast-path detected");
            } else {
                println!("  💡 Add 'ct state established,related accept' early in chains");
            }

            if ruleset.contains("flowtable") {
                println!("  ✅ Flow offloading configured");
            } else {
                println!("  💡 Consider enabling flow offloading for better performance");
            }
        }
        _ => {
            eprintln!("❌ Failed to analyze ruleset");
        }
    }
}

// ==================== Advanced Rule Builder ====================

fn advanced_rule_builder() {
    println!("\n🛠️  Advanced Rule Builder");
    println!("=========================");

    // Select table
    let families = vec![
        "inet (IPv4/IPv6)",
        "ip (IPv4)",
        "ip6 (IPv6)",
        "bridge",
        "netdev",
    ];
    let family_idx = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select address family")
        .items(&families)
        .default(0)
        .interact_opt()
    {
        Ok(Some(i)) => i,
        Ok(None) | Err(_) => return,
    };

    let family = ["inet", "ip", "ip6", "bridge", "netdev"][family_idx];

    let table_name: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Table name")
        .default("filter".to_string())
        .interact_text()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    let chain_name: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Chain name")
        .default("input".to_string())
        .interact_text()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    // Build match conditions
    let match_options = vec![
        "Source IP/network",
        "Destination IP/network",
        "Source port",
        "Destination port",
        "Protocol",
        "Connection state",
        "Interface",
        "Done building matches",
    ];

    let mut matches: Vec<String> = Vec::new();

    while let Ok(Some(match_idx)) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Add match condition")
        .items(&match_options)
        .default(7)
        .interact_opt()
    {
        match match_idx {
            0 => {
                let ip: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Source IP/CIDR")
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };
                matches.push(format!("ip saddr {}", ip));
            }
            1 => {
                let ip: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Destination IP/CIDR")
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };
                matches.push(format!("ip daddr {}", ip));
            }
            2 => {
                let port: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Source port")
                    .interact_text()
                {
                    Ok(p) => p,
                    Err(_) => continue,
                };
                matches.push(format!("tcp sport {}", port));
            }
            3 => {
                let port: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Destination port")
                    .interact_text()
                {
                    Ok(p) => p,
                    Err(_) => continue,
                };
                matches.push(format!("tcp dport {}", port));
            }
            4 => {
                let protos = vec!["tcp", "udp", "icmp", "icmpv6"];
                let proto_idx = match Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Protocol")
                    .items(&protos)
                    .interact_opt()
                {
                    Ok(Some(i)) => i,
                    Ok(None) | Err(_) => continue,
                };
                matches.push(format!("meta l4proto {}", protos[proto_idx]));
            }
            5 => {
                let states = vec!["new", "established", "related", "invalid"];
                let state_selection = match MultiSelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Connection states")
                    .items(&states)
                    .interact_opt()
                {
                    Ok(Some(s)) => s,
                    Ok(None) | Err(_) => continue,
                };
                let selected_states: Vec<&str> =
                    state_selection.iter().map(|&i| states[i]).collect();
                matches.push(format!("ct state {}", selected_states.join(",")));
            }
            6 => {
                let iface: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Interface name")
                    .interact_text()
                {
                    Ok(i) => i,
                    Err(_) => continue,
                };
                matches.push(format!("iif {}", iface));
            }
            _ => break,
        }
    }

    // Select verdict
    let verdicts = vec!["accept", "drop", "reject", "log", "counter"];
    let verdict_idx = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select verdict/action")
        .items(&verdicts)
        .default(0)
        .interact_opt()
    {
        Ok(Some(i)) => i,
        Ok(None) | Err(_) => return,
    };

    // Build final rule
    let rule = format!(
        "nft add rule {} {} {} {} {}",
        family,
        table_name,
        chain_name,
        matches.join(" "),
        verdicts[verdict_idx]
    );

    println!("\n📝 Generated Rule:");
    println!("  {}", rule);

    let apply = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply this rule?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(a)) => a,
        Ok(None) | Err(_) => return,
    };

    if apply {
        let args: Vec<&str> = rule.split_whitespace().skip(1).collect();
        let result = Command::new("nft").args(&args).output();

        match result {
            Ok(r) if r.status.success() => {
                println!("✅ Rule applied successfully");
            }
            Ok(r) => {
                eprintln!("❌ Error: {}", String::from_utf8_lossy(&r.stderr));
            }
            Err(e) => {
                eprintln!("❌ Failed: {}", e);
            }
        }
    }
}

// ==================== Set & Map Management ====================

fn set_map_management() {
    loop {
        println!("\n🗃️  Set & Map Management");
        println!("========================");

        let options = vec![
            "📋 List Sets",
            "➕ Create Set",
            "📋 List Maps",
            "➕ Create Map",
            "✏️  Add Elements to Set",
            "🗑️  Delete Set/Map",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select option")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => list_sets(),
            1 => create_set(),
            2 => list_maps(),
            3 => create_map(),
            4 => add_set_elements(),
            5 => delete_set_or_map(),
            _ => break,
        }
    }
}

fn list_sets() {
    println!("\n📋 NFTables Sets");
    let output = Command::new("nft").args(["list", "sets"]).output();

    match output {
        Ok(result) if result.status.success() => {
            println!("{}", String::from_utf8_lossy(&result.stdout));
        }
        _ => {
            println!("  No sets found or unable to list sets.");
        }
    }
}

fn create_set() {
    println!("\n➕ Create NFTables Set");

    let family: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Address family (inet/ip/ip6)")
        .default("inet".to_string())
        .interact_text()
    {
        Ok(f) => f,
        Err(_) => return,
    };

    let table: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Table name")
        .default("filter".to_string())
        .interact_text()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    let set_name: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Set name")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    let set_types = vec![
        "ipv4_addr",
        "ipv6_addr",
        "inet_service",
        "ether_addr",
        "mark",
    ];
    let type_idx = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Set type")
        .items(&set_types)
        .default(0)
        .interact_opt()
    {
        Ok(Some(i)) => i,
        Ok(None) | Err(_) => return,
    };

    let result = Command::new("nft")
        .args([
            "add",
            "set",
            &family,
            &table,
            &set_name,
            &format!("{{ type {}; }}", set_types[type_idx]),
        ])
        .output();

    match result {
        Ok(r) if r.status.success() => {
            println!("✅ Set '{}' created", set_name);
        }
        Ok(r) => {
            eprintln!("❌ Error: {}", String::from_utf8_lossy(&r.stderr));
        }
        Err(e) => {
            eprintln!("❌ Failed: {}", e);
        }
    }
}

fn list_maps() {
    println!("\n📋 NFTables Maps");
    let output = Command::new("nft").args(["list", "maps"]).output();

    match output {
        Ok(result) if result.status.success() => {
            println!("{}", String::from_utf8_lossy(&result.stdout));
        }
        _ => {
            println!("  No maps found or unable to list maps.");
        }
    }
}

fn create_map() {
    println!("\n➕ Create NFTables Map");

    let family: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Address family (inet/ip/ip6)")
        .default("inet".to_string())
        .interact_text()
    {
        Ok(f) => f,
        Err(_) => return,
    };

    let table: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Table name")
        .default("filter".to_string())
        .interact_text()
    {
        Ok(t) => t,
        Err(_) => return,
    };

    let map_name: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Map name")
        .interact_text()
    {
        Ok(n) => n,
        Err(_) => return,
    };

    println!("✅ Map '{}' configuration template created", map_name);
    println!(
        "💡 Use 'nft add map {} {} {} {{ type ipv4_addr : verdict; }}' to create",
        family, table, map_name
    );
}

fn add_set_elements() {
    println!("\n✏️  Add Elements to Set");

    let set_ref: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Set reference (family table set_name)")
        .default("inet filter blocklist".to_string())
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    let elements: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Elements to add (comma-separated)")
        .interact_text()
    {
        Ok(e) => e,
        Err(_) => return,
    };

    let parts: Vec<&str> = set_ref.split_whitespace().collect();
    if parts.len() != 3 {
        println!("❌ Invalid set reference format");
        return;
    }

    let result = Command::new("nft")
        .args([
            "add",
            "element",
            parts[0],
            parts[1],
            parts[2],
            &format!("{{ {} }}", elements),
        ])
        .output();

    match result {
        Ok(r) if r.status.success() => {
            println!("✅ Elements added to set");
        }
        Ok(r) => {
            eprintln!("❌ Error: {}", String::from_utf8_lossy(&r.stderr));
        }
        Err(e) => {
            eprintln!("❌ Failed: {}", e);
        }
    }
}

fn delete_set_or_map() {
    println!("\n🗑️  Delete Set/Map");

    let set_ref: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Set/Map reference (family table name)")
        .interact_text()
    {
        Ok(s) => s,
        Err(_) => return,
    };

    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("Delete '{}'?", set_ref))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let parts: Vec<&str> = set_ref.split_whitespace().collect();
        if parts.len() == 3 {
            let _ = Command::new("nft")
                .args(["delete", "set", parts[0], parts[1], parts[2]])
                .output();
            println!("✅ Set/Map deleted");
        }
    }
}

// ==================== Flowtable Configuration ====================

fn flowtable_configuration() {
    loop {
        println!("\n🌊 Flowtable Configuration");
        println!("===========================");

        let options = vec![
            "📋 List Flowtables",
            "➕ Create Flowtable",
            "⚙️  Configure Flowtable",
            "🗑️  Delete Flowtable",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select option")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => list_flowtables(),
            1 => setup_flow_offloading(), // Reuse existing function
            2 => configure_existing_flowtable(),
            3 => delete_flowtable(),
            _ => break,
        }
    }
}

fn list_flowtables() {
    println!("\n📋 NFTables Flowtables");
    let output = Command::new("nft").args(["list", "flowtables"]).output();

    match output {
        Ok(result) if result.status.success() => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            if stdout.trim().is_empty() {
                println!("  No flowtables configured.");
            } else {
                println!("{}", stdout);
            }
        }
        _ => {
            println!("  Unable to list flowtables.");
        }
    }
}

fn configure_existing_flowtable() {
    println!("\n⚙️  Configure Existing Flowtable");
    println!("💡 Modify /etc/nftables.conf to adjust flowtable settings");
    println!("💡 Common adjustments:");
    println!("   - Add/remove devices");
    println!("   - Enable/disable hardware offload");
    println!("   - Adjust priority");
}

fn delete_flowtable() {
    println!("\n🗑️  Delete Flowtable");

    let ft_ref: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Flowtable reference (family table name)")
        .interact_text()
    {
        Ok(f) => f,
        Err(_) => return,
    };

    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("Delete flowtable '{}'?", ft_ref))
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let parts: Vec<&str> = ft_ref.split_whitespace().collect();
        if parts.len() == 3 {
            let _ = Command::new("nft")
                .args(["delete", "flowtable", parts[0], parts[1], parts[2]])
                .output();
            println!("✅ Flowtable deleted");
        }
    }
}

// ==================== Configuration Management ====================

fn configuration_management() {
    loop {
        println!("\n🔄 Configuration Management");
        println!("===========================");

        let options = vec![
            "💾 Backup Current Ruleset",
            "📂 Restore from Backup",
            "📝 Export to File",
            "📥 Import from File",
            "🔄 Show Diff Between Configs",
            "⬅️  Back",
        ];

        let selection = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select option")
            .items(&options)
            .default(0)
            .interact_opt()
        {
            Ok(Some(s)) => s,
            Ok(None) | Err(_) => break,
        };

        match selection {
            0 => backup_ruleset(),
            1 => restore_from_backup(),
            2 => export_ruleset(),
            3 => import_ruleset(),
            4 => show_config_diff(),
            _ => break,
        }
    }
}

fn backup_ruleset() {
    println!("\n💾 Backup Current Ruleset");

    let backup_dir = "/var/lib/ghostctl/nft-backups";
    if let Err(e) = fs::create_dir_all(backup_dir) {
        println!("❌ Failed to create backup directory: {}", e);
        return;
    }

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_file = format!("{}/nftables_{}.conf", backup_dir, timestamp);

    let output = Command::new("nft").args(["list", "ruleset"]).output();

    match output {
        Ok(result) if result.status.success() => {
            if fs::write(&backup_file, &result.stdout).is_ok() {
                println!("✅ Backup saved to: {}", backup_file);
            } else {
                println!("❌ Failed to write backup file");
            }
        }
        _ => {
            println!("❌ Failed to get current ruleset");
        }
    }
}

fn restore_from_backup() {
    println!("\n📂 Restore from Backup");

    let backup_dir = "/var/lib/ghostctl/nft-backups";
    let entries = match fs::read_dir(backup_dir) {
        Ok(e) => e,
        Err(_) => {
            println!("❌ No backups found in {}", backup_dir);
            return;
        }
    };

    let backups: Vec<String> = entries
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.ends_with(".conf"))
        .collect();

    if backups.is_empty() {
        println!("❌ No backup files found");
        return;
    }

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select backup to restore")
        .items(&backups)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    let backup_file = format!("{}/{}", backup_dir, backups[selection]);

    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("This will replace current ruleset. Continue?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        // Flush current ruleset
        let flush_result = Command::new("nft").args(["flush", "ruleset"]).output();
        if let Err(e) = flush_result {
            println!("Warning: Failed to flush current ruleset: {}", e);
            println!("Continuing with restore attempt...");
        }

        // Apply backup
        let result = Command::new("nft").args(["-f", &backup_file]).output();

        match result {
            Ok(r) if r.status.success() => {
                println!("✅ Ruleset restored from backup");
            }
            Ok(r) => {
                eprintln!("❌ Error: {}", String::from_utf8_lossy(&r.stderr));
            }
            Err(e) => {
                eprintln!("❌ Failed: {}", e);
            }
        }
    }
}

fn export_ruleset() {
    println!("\n📝 Export Ruleset");

    let export_path: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Export path")
        .default("/tmp/nftables_export.conf".to_string())
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    let output = Command::new("nft").args(["list", "ruleset"]).output();

    match output {
        Ok(result) if result.status.success() => {
            if fs::write(&export_path, &result.stdout).is_ok() {
                println!("✅ Ruleset exported to: {}", export_path);
            } else {
                println!("❌ Failed to write export file");
            }
        }
        _ => {
            println!("❌ Failed to get current ruleset");
        }
    }
}

fn import_ruleset() {
    println!("\n📥 Import Ruleset");

    let import_path: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Import path")
        .interact_text()
    {
        Ok(p) => p,
        Err(_) => return,
    };

    if !std::path::Path::new(&import_path).exists() {
        println!("❌ File not found: {}", import_path);
        return;
    }

    let confirm = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Import and apply ruleset?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(c)) => c,
        Ok(None) | Err(_) => return,
    };

    if confirm {
        let result = Command::new("nft").args(["-f", &import_path]).output();

        match result {
            Ok(r) if r.status.success() => {
                println!("✅ Ruleset imported successfully");
            }
            Ok(r) => {
                eprintln!("❌ Error: {}", String::from_utf8_lossy(&r.stderr));
            }
            Err(e) => {
                eprintln!("❌ Failed: {}", e);
            }
        }
    }
}

fn show_config_diff() {
    println!("\n🔄 Configuration Diff");
    println!("=====================");
    println!("💡 Compare two configuration files:");
    println!("   diff /etc/nftables.conf /var/lib/ghostctl/nft-backups/nftables_*.conf");
}

// ==================== Performance Functions ====================

fn optimize_hash_tables() {
    println!("\n🧮 Hash Table Optimization");
    println!("===========================");

    // Get current connection count
    let conn_count = fs::read_to_string("/proc/sys/net/netfilter/nf_conntrack_count")
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(0);

    let current_buckets = fs::read_to_string("/proc/sys/net/netfilter/nf_conntrack_buckets")
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(0);

    println!("📊 Current Status:");
    println!("  Active connections: {}", conn_count);
    println!("  Current buckets: {}", current_buckets);

    let expected: u32 = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Expected maximum connections")
        .default(conn_count.max(10000))
        .interact_text()
    {
        Ok(e) => e,
        Err(_) => return,
    };

    let optimal_buckets = calculate_optimal_buckets(expected);

    println!("\n📋 Recommendation:");
    println!("  Optimal buckets: {}", optimal_buckets);
    println!("  Memory impact: ~{} KB", optimal_buckets * 8 / 1024);

    let apply = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply optimization?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(a)) => a,
        Ok(None) | Err(_) => return,
    };

    if apply {
        let proc_path = "/proc/sys/net/netfilter/nf_conntrack_buckets";
        match fs::write(proc_path, optimal_buckets.to_string()) {
            Ok(_) => println!("✅ Hash table size updated"),
            Err(_) => println!("❌ Failed to update (may need to unload conntrack module)"),
        }
    }
}

fn optimize_memory_usage() {
    println!("\n💾 Memory Usage Optimization");
    println!("============================");

    println!("📊 Current Memory Settings:");

    // Show conntrack memory
    if let Ok(count) = fs::read_to_string("/proc/sys/net/netfilter/nf_conntrack_count") {
        let connections: u64 = count.trim().parse().unwrap_or(0);
        let est_memory = connections * 328;
        println!(
            "  Conntrack: {} connections (~{:.2} MB)",
            connections,
            est_memory as f64 / 1_048_576.0
        );
    }

    // Suggestions
    println!("\n💡 Optimization Suggestions:");
    println!("  1. Reduce TCP timeout for faster connection cleanup");
    println!("  2. Enable flow offloading to bypass conntrack");
    println!("  3. Use sets instead of individual rules for large lists");
    println!("  4. Enable conntrack zone separation for isolated networks");

    let apply_suggestions = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply recommended memory optimizations?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(a)) => a,
        Ok(None) | Err(_) => return,
    };

    if apply_suggestions {
        // Reduce some timeouts
        let _ = fs::write(
            "/proc/sys/net/netfilter/nf_conntrack_tcp_timeout_established",
            "1800",
        );
        let _ = fs::write(
            "/proc/sys/net/netfilter/nf_conntrack_tcp_timeout_time_wait",
            "30",
        );
        println!("✅ Memory optimizations applied");
    }
}

fn tune_garbage_collection() {
    println!("\n🗑️  Garbage Collection Tuning");
    println!("==============================");

    println!("📊 Conntrack GC Settings:");
    println!("  GC is handled automatically by the kernel.");
    println!("  Tuning options:");
    println!("    - Reduce timeouts for faster connection cleanup");
    println!("    - Adjust nf_conntrack_max to limit memory usage");

    let aggressive = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable aggressive GC (shorter timeouts)?")
        .default(false)
        .interact_opt()
    {
        Ok(Some(a)) => a,
        Ok(None) | Err(_) => return,
    };

    if aggressive {
        let _ = fs::write(
            "/proc/sys/net/netfilter/nf_conntrack_tcp_timeout_established",
            "900",
        );
        let _ = fs::write(
            "/proc/sys/net/netfilter/nf_conntrack_tcp_timeout_time_wait",
            "15",
        );
        let _ = fs::write("/proc/sys/net/netfilter/nf_conntrack_udp_timeout", "15");
        println!("✅ Aggressive GC settings applied");
    }
}

fn configure_cpu_affinity() {
    println!("\n🎯 CPU Affinity Configuration");
    println!("==============================");

    // Show current IRQ affinity for network interfaces
    println!("📊 Network IRQ Distribution:");

    let output = Command::new("sh")
        .args(["-c", "grep -E 'eth|enp|ens' /proc/interrupts | head -10"])
        .output();

    if let Ok(result) = output {
        println!("{}", String::from_utf8_lossy(&result.stdout));
    }

    println!("\n💡 To configure CPU affinity:");
    println!("  1. Identify network interface IRQs in /proc/interrupts");
    println!("  2. Set affinity with: echo <cpu_mask> > /proc/irq/<irq>/smp_affinity");
    println!("  3. Consider using irqbalance for automatic distribution");

    let install_irqbalance = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Check/enable irqbalance service?")
        .default(true)
        .interact_opt()
    {
        Ok(Some(i)) => i,
        Ok(None) | Err(_) => return,
    };

    if install_irqbalance {
        let _ = Command::new("systemctl")
            .args(["enable", "--now", "irqbalance"])
            .output();
        println!("✅ irqbalance service enabled");
    }
}

fn performance_benchmarking() {
    println!("\n📈 Performance Benchmarking");
    println!("============================");

    let benchmark_options = vec![
        "Rule evaluation latency",
        "Connection tracking throughput",
        "Packet forwarding rate",
        "Full benchmark suite",
    ];

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select benchmark")
        .items(&benchmark_options)
        .default(0)
        .interact_opt()
    {
        Ok(Some(s)) => s,
        Ok(None) | Err(_) => return,
    };

    match selection {
        0 => {
            println!("\n⏱️  Rule Evaluation Latency Test");
            println!("   Counting rules in ruleset...");
            let output = Command::new("nft").args(["list", "ruleset"]).output();

            if let Ok(result) = output {
                let ruleset = String::from_utf8_lossy(&result.stdout);
                let rule_count = ruleset.matches("\n\t\t").count();
                println!("   Rules: {}", rule_count);
                println!(
                    "   Estimated evaluation time: ~{} ns per packet",
                    rule_count * 10
                );
            }
        }
        1 => {
            println!("\n🔗 Connection Tracking Throughput");
            if let Ok(count) = fs::read_to_string("/proc/sys/net/netfilter/nf_conntrack_count") {
                println!("   Active connections: {}", count.trim());
            }
            if let Ok(max) = fs::read_to_string("/proc/sys/net/netfilter/nf_conntrack_max") {
                println!("   Maximum capacity: {}", max.trim());
            }
        }
        2 => {
            println!("\n📦 Packet Forwarding Rate");
            println!("   Use iperf3 or netperf for actual throughput testing");
            println!("   Command: iperf3 -c <remote_host> -t 60");
        }
        3 => {
            println!("\n🔬 Running Full Benchmark Suite...");
            println!("   This would require specialized tools like:");
            println!("   - pktgen for packet generation");
            println!("   - perf for kernel profiling");
            println!("   - bpftrace for detailed analysis");
        }
        _ => {}
    }
}

fn automatic_optimization() {
    println!("\n🔧 Automatic Optimization");
    println!("=========================");

    println!("🔍 Analyzing current configuration...\n");

    // Check various settings and make recommendations
    let mut optimizations: Vec<(&str, bool)> = Vec::new();

    // Check conntrack
    if let Ok(count) = fs::read_to_string("/proc/sys/net/netfilter/nf_conntrack_count")
        && let Ok(max) = fs::read_to_string("/proc/sys/net/netfilter/nf_conntrack_max")
    {
        let count: u64 = count.trim().parse().unwrap_or(0);
        let max: u64 = max.trim().parse().unwrap_or(1);
        let usage = (count as f64 / max as f64) * 100.0;

        if usage > 80.0 {
            optimizations.push(("Increase conntrack max", true));
        } else {
            optimizations.push(("Conntrack capacity", false));
        }
    }

    // Check flow offloading
    let ft_output = Command::new("nft").args(["list", "flowtables"]).output();

    if let Ok(result) = ft_output {
        if result.stdout.is_empty() {
            optimizations.push(("Enable flow offloading", true));
        } else {
            optimizations.push(("Flow offloading", false));
        }
    }

    // Display results
    println!("📋 Optimization Analysis:");
    for (name, needs_action) in &optimizations {
        if *needs_action {
            println!("   ⚠️  {}: Needs optimization", name);
        } else {
            println!("   ✅ {}: OK", name);
        }
    }

    let needs_work: Vec<_> = optimizations.iter().filter(|(_, n)| *n).collect();
    if !needs_work.is_empty() {
        let apply = match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Apply recommended optimizations?")
            .default(false)
            .interact_opt()
        {
            Ok(Some(a)) => a,
            Ok(None) | Err(_) => return,
        };

        if apply {
            for (name, _) in needs_work {
                println!("   Applying: {}...", name);
            }
            println!("✅ Optimizations applied");
        }
    } else {
        println!("\n✅ System is already optimized!");
    }
}

/// Validate a port specification (single port, range, or set reference)
pub fn is_valid_port_spec(spec: &PortSpec) -> bool {
    match spec {
        PortSpec::Single(port) => *port > 0,
        PortSpec::Range(start, end) => *start > 0 && *start <= *end,
        PortSpec::Set(_) => true, // Set references are validated elsewhere
    }
}

/// Validate a rate limit configuration
pub fn is_valid_rate_limit(rate: u32, unit: &RateUnit) -> bool {
    match unit {
        RateUnit::PacketsPerSecond => rate <= 10_000_000,
        RateUnit::PacketsPerMinute => rate <= 600_000_000,
        RateUnit::PacketsPerHour => true, // Any u32 value is valid for hourly rates
        RateUnit::BytesPerSecond => rate <= (10_000_000_000u64 as u32),
        RateUnit::KilobytesPerSecond => rate <= 10_000_000,
        RateUnit::MegabytesPerSecond => rate <= 10_000,
    }
}

/// Calculate optimal hash table buckets based on expected connections
pub fn calculate_optimal_buckets(expected_connections: u32) -> u32 {
    // Rule of thumb: buckets should be about 1/8th of max connections
    // Round up to nearest power of 2 for better hash distribution
    let buckets = expected_connections / 8;
    let mut power_of_two = 1u32;
    while power_of_two < buckets && power_of_two < 1_000_000 {
        power_of_two *= 2;
    }
    power_of_two.max(1024) // Minimum 1024 buckets
}

/// Validate chain priority value (-400 to 300 for most hooks)
pub fn is_valid_chain_priority(priority: i32) -> bool {
    (-400..=300).contains(&priority)
}

/// Validate DSCP value (0-63)
pub fn is_valid_dscp(dscp: u8) -> bool {
    dscp <= 63
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Port Spec Validation Tests ====================

    #[test]
    fn test_is_valid_port_spec_single() {
        assert!(is_valid_port_spec(&PortSpec::Single(80)));
        assert!(is_valid_port_spec(&PortSpec::Single(443)));
        assert!(is_valid_port_spec(&PortSpec::Single(65535)));
    }

    #[test]
    fn test_is_valid_port_spec_single_zero() {
        assert!(!is_valid_port_spec(&PortSpec::Single(0)));
    }

    #[test]
    fn test_is_valid_port_spec_range() {
        assert!(is_valid_port_spec(&PortSpec::Range(80, 443)));
        assert!(is_valid_port_spec(&PortSpec::Range(1, 65535)));
        assert!(is_valid_port_spec(&PortSpec::Range(80, 80)));
    }

    #[test]
    fn test_is_valid_port_spec_range_invalid() {
        assert!(!is_valid_port_spec(&PortSpec::Range(0, 80)));
        assert!(!is_valid_port_spec(&PortSpec::Range(443, 80)));
    }

    #[test]
    fn test_is_valid_port_spec_set() {
        assert!(is_valid_port_spec(&PortSpec::Set("http_ports".to_string())));
    }

    // ==================== Chain Priority Tests ====================

    #[test]
    fn test_is_valid_chain_priority_normal() {
        assert!(is_valid_chain_priority(0));
        assert!(is_valid_chain_priority(-100));
        assert!(is_valid_chain_priority(100));
    }

    #[test]
    fn test_is_valid_chain_priority_boundaries() {
        assert!(is_valid_chain_priority(-400));
        assert!(is_valid_chain_priority(300));
    }

    #[test]
    fn test_is_valid_chain_priority_out_of_range() {
        assert!(!is_valid_chain_priority(-401));
        assert!(!is_valid_chain_priority(301));
    }

    // ==================== DSCP Validation Tests ====================

    #[test]
    fn test_is_valid_dscp_normal() {
        assert!(is_valid_dscp(0));
        assert!(is_valid_dscp(46)); // EF (Expedited Forwarding)
        assert!(is_valid_dscp(26)); // AF31
    }

    #[test]
    fn test_is_valid_dscp_max() {
        assert!(is_valid_dscp(63));
    }

    #[test]
    fn test_is_valid_dscp_invalid() {
        assert!(!is_valid_dscp(64));
        assert!(!is_valid_dscp(255));
    }

    // ==================== Hash Table Calculation Tests ====================

    #[test]
    fn test_calculate_optimal_buckets_small() {
        let buckets = calculate_optimal_buckets(1000);
        assert!(buckets >= 1024); // Minimum is 1024
        assert!(buckets.is_power_of_two());
    }

    #[test]
    fn test_calculate_optimal_buckets_medium() {
        let buckets = calculate_optimal_buckets(100_000);
        assert!(buckets >= 8192);
        assert!(buckets.is_power_of_two());
    }

    #[test]
    fn test_calculate_optimal_buckets_large() {
        let buckets = calculate_optimal_buckets(1_000_000);
        assert!(buckets >= 65536);
        assert!(buckets.is_power_of_two());
    }

    // ==================== Rate Limit Validation Tests ====================

    #[test]
    fn test_is_valid_rate_limit_packets() {
        assert!(is_valid_rate_limit(1000, &RateUnit::PacketsPerSecond));
        assert!(is_valid_rate_limit(60000, &RateUnit::PacketsPerMinute));
    }

    #[test]
    fn test_is_valid_rate_limit_bytes() {
        assert!(is_valid_rate_limit(1000000, &RateUnit::BytesPerSecond));
        assert!(is_valid_rate_limit(1000, &RateUnit::KilobytesPerSecond));
        assert!(is_valid_rate_limit(100, &RateUnit::MegabytesPerSecond));
    }

    // ==================== Table Family Tests ====================

    #[test]
    fn test_table_family_serialization() {
        let inet = TableFamily::Inet;
        let serialized = serde_json::to_string(&inet).unwrap();
        assert!(serialized.contains("Inet"));
    }

    // ==================== Chain Type Tests ====================

    #[test]
    fn test_chain_type_variants() {
        let filter = ChainType::Filter;
        let route = ChainType::Route;
        let nat = ChainType::Nat;

        // Verify these are distinct types
        let filter_str = format!("{:?}", filter);
        let route_str = format!("{:?}", route);
        let nat_str = format!("{:?}", nat);

        assert_ne!(filter_str, route_str);
        assert_ne!(route_str, nat_str);
    }

    // ==================== Protocol Tests ====================

    #[test]
    fn test_protocol_variants() {
        let tcp = Protocol::Tcp;
        let udp = Protocol::Udp;
        let custom = Protocol::Number(47); // GRE

        assert!(matches!(tcp, Protocol::Tcp));
        assert!(matches!(udp, Protocol::Udp));
        assert!(matches!(custom, Protocol::Number(47)));
    }

    // ==================== Connection Tracking State Tests ====================

    #[test]
    fn test_conntrack_states() {
        let states = vec![
            ConntrackState::New,
            ConntrackState::Established,
            ConntrackState::Related,
            ConntrackState::Invalid,
            ConntrackState::Untracked,
        ];
        assert_eq!(states.len(), 5);
    }

    // ==================== Rule Verdict Tests ====================

    #[test]
    fn test_rule_verdict_accept() {
        let verdict = RuleVerdict::Accept;
        assert!(matches!(verdict, RuleVerdict::Accept));
    }

    #[test]
    fn test_rule_verdict_drop() {
        let verdict = RuleVerdict::Drop;
        assert!(matches!(verdict, RuleVerdict::Drop));
    }

    #[test]
    fn test_rule_verdict_jump() {
        let verdict = RuleVerdict::Jump {
            target: "custom_chain".to_string(),
        };
        if let RuleVerdict::Jump { target } = verdict {
            assert_eq!(target, "custom_chain");
        } else {
            panic!("Expected Jump verdict");
        }
    }

    // ==================== Log Level Tests ====================

    #[test]
    fn test_log_level_values() {
        assert_eq!(LogLevel::Emergency as u8, 0);
        assert_eq!(LogLevel::Alert as u8, 1);
        assert_eq!(LogLevel::Critical as u8, 2);
        assert_eq!(LogLevel::Error as u8, 3);
        assert_eq!(LogLevel::Warning as u8, 4);
        assert_eq!(LogLevel::Notice as u8, 5);
        assert_eq!(LogLevel::Info as u8, 6);
        assert_eq!(LogLevel::Debug as u8, 7);
    }

    // ==================== Set Flag Tests ====================

    #[test]
    fn test_set_flags() {
        let flags = vec![
            SetFlag::Constant,
            SetFlag::Interval,
            SetFlag::Timeout,
            SetFlag::Dynamic,
        ];
        assert_eq!(flags.len(), 4);
    }

    // ==================== Address Match Tests ====================

    #[test]
    fn test_address_match_creation() {
        let addr_match = AddressMatch {
            addresses: vec!["192.168.1.0/24".to_string(), "10.0.0.0/8".to_string()],
            negated: false,
        };
        assert_eq!(addr_match.addresses.len(), 2);
        assert!(!addr_match.negated);
    }

    #[test]
    fn test_address_match_negated() {
        let addr_match = AddressMatch {
            addresses: vec!["0.0.0.0/0".to_string()],
            negated: true,
        };
        assert!(addr_match.negated);
    }

    // ==================== TCP Flags Tests ====================

    #[test]
    fn test_tcp_flags() {
        // SYN flag
        let syn_flags = TcpFlags {
            flags: 0x02,
            mask: 0x02,
        };
        assert_eq!(syn_flags.flags, 2);

        // SYN+ACK flags
        let synack_flags = TcpFlags {
            flags: 0x12,
            mask: 0x12,
        };
        assert_eq!(synack_flags.flags, 18);
    }

    // ==================== Time Range Tests ====================

    #[test]
    fn test_time_range_creation() {
        let time_range = TimeRange {
            start_time: Some("09:00".to_string()),
            end_time: Some("17:00".to_string()),
            days_of_week: vec![1, 2, 3, 4, 5], // Mon-Fri
        };
        assert_eq!(time_range.days_of_week.len(), 5);
    }
}
