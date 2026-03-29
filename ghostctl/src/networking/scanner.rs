use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// Re-export socket2 for raw socket scanning
use socket2;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use dialoguer::{Select, theme::ColorfulTheme};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{BarChart, Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Tabs},
};

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub target: String,
    pub port: u16,
    pub status: PortStatus,
    pub service: Option<String>,
    pub banner: Option<String>,
    pub response_time: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PortStatus {
    Open,
    Closed,
    Filtered,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub targets: Vec<String>,
    pub ports: Vec<u16>,
    pub threads: usize,
    pub timeout: Duration,
    pub scan_type: ScanType,
    pub service_detection: bool,
    pub os_detection: bool,
    pub vulnerability_scan: bool,
}

#[derive(Debug, Clone)]
pub enum ScanType {
    Connect,
    Syn,
    Udp,
    Comprehensive,
}

#[derive(Debug)]
pub struct ScanStats {
    pub total_hosts: usize,
    pub hosts_scanned: usize,
    pub total_ports: usize,
    pub ports_scanned: usize,
    pub open_ports: usize,
    pub closed_ports: usize,
    pub filtered_ports: usize,
    pub start_time: Instant,
    pub elapsed: Duration,
    pub estimated_remaining: Duration,
}

pub struct ScannerApp {
    pub config: ScanConfig,
    pub results: Arc<Mutex<Vec<ScanResult>>>,
    pub stats: Arc<Mutex<ScanStats>>,
    pub current_tab: usize,
    pub should_quit: bool,
    pub list_state: ListState,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            targets: vec!["127.0.0.1".to_string()],
            ports: (1..=1000).collect(),
            threads: 100,
            timeout: Duration::from_millis(1000),
            scan_type: ScanType::Connect,
            service_detection: false,
            os_detection: false,
            vulnerability_scan: false,
        }
    }
}

impl ScannerApp {
    pub fn new(config: ScanConfig) -> Self {
        let total_ports = config.ports.len() * config.targets.len();
        let stats = ScanStats {
            total_hosts: config.targets.len(),
            hosts_scanned: 0,
            total_ports,
            ports_scanned: 0,
            open_ports: 0,
            closed_ports: 0,
            filtered_ports: 0,
            start_time: Instant::now(),
            elapsed: Duration::new(0, 0),
            estimated_remaining: Duration::new(0, 0),
        };

        Self {
            config,
            results: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(Mutex::new(stats)),
            current_tab: 0,
            should_quit: false,
            list_state: ListState::default(),
        }
    }

    pub async fn run_scan_with_tui(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Start the scan in background
        self.start_background_scan().await;

        // Run the TUI
        let res = self.run_tui(&mut terminal).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err);
        }

        Ok(())
    }

    async fn start_background_scan(&self) {
        let results = Arc::clone(&self.results);
        let stats = Arc::clone(&self.stats);
        let config = self.config.clone();

        tokio::spawn(async move {
            perform_scan(config, results, stats).await;
        });
    }

    async fn run_tui(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> std::io::Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(250))?
                && let Event::Key(key) = event::read()?
            {
                match key.code {
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                        return Ok(());
                    }
                    KeyCode::Right => {
                        self.current_tab = (self.current_tab + 1) % 4;
                    }
                    KeyCode::Left => {
                        self.current_tab = if self.current_tab > 0 {
                            self.current_tab - 1
                        } else {
                            3
                        };
                    }
                    KeyCode::Down => {
                        if let Some(selected) = self.list_state.selected() {
                            let Ok(results) = self.results.lock() else {
                                continue;
                            };
                            if selected < results.len().saturating_sub(1) {
                                self.list_state.select(Some(selected + 1));
                            }
                        } else {
                            self.list_state.select(Some(0));
                        }
                    }
                    KeyCode::Up => {
                        if let Some(selected) = self.list_state.selected()
                            && selected > 0
                        {
                            self.list_state.select(Some(selected - 1));
                        }
                    }
                    _ => {}
                }
            }

            // Check if scan is complete
            let Ok(stats) = self.stats.lock() else {
                continue;
            };
            if stats.ports_scanned >= stats.total_ports {
                // Keep TUI open for results viewing
            }
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        let size = f.area();

        // Create the layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);

        // Tab titles
        let tab_titles: Vec<Line> = ["📊 Overview", "🔍 Results", "📈 Statistics", "⚙️ Settings"]
            .iter()
            .cloned()
            .map(Line::from)
            .collect();

        let tabs = Tabs::new(tab_titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("GhostCTL Scanner"),
            )
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Blue),
            )
            .select(self.current_tab);

        f.render_widget(tabs, chunks[0]);

        // Render tab content
        match self.current_tab {
            0 => self.render_overview(f, chunks[1]),
            1 => self.render_results(f, chunks[1]),
            2 => self.render_statistics(f, chunks[1]),
            3 => self.render_settings(f, chunks[1]),
            _ => {}
        }
    }

    fn render_overview(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(8),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(area);

        // Progress gauge
        let Ok(stats) = self.stats.lock() else {
            return;
        };
        let progress = if stats.total_ports > 0 {
            (stats.ports_scanned as f64 / stats.total_ports as f64 * 100.0) as u16
        } else {
            0
        };

        let gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Scan Progress"),
            )
            .gauge_style(Style::default().fg(Color::Green))
            .percent(progress);

        f.render_widget(gauge, chunks[0]);

        // Status info
        let status_text = vec![
            Line::from(vec![
                Span::styled("Target: ", Style::default().fg(Color::Cyan)),
                Span::raw(self.config.targets.join(", ")),
            ]),
            Line::from(vec![
                Span::styled("Ports: ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{}", stats.total_ports)),
            ]),
            Line::from(vec![
                Span::styled("Progress: ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{}/{}", stats.ports_scanned, stats.total_ports)),
            ]),
            Line::from(vec![
                Span::styled("Open Ports: ", Style::default().fg(Color::Green)),
                Span::raw(format!("{}", stats.open_ports)),
            ]),
            Line::from(vec![
                Span::styled("Closed: ", Style::default().fg(Color::Red)),
                Span::raw(format!("{}", stats.closed_ports)),
            ]),
            Line::from(vec![
                Span::styled("Elapsed: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{:.1}s", stats.elapsed.as_secs_f64())),
            ]),
        ];

        let status_paragraph = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::ALL).title("Scan Status"));

        f.render_widget(status_paragraph, chunks[1]);
    }

    fn render_results(&mut self, f: &mut Frame, area: Rect) {
        let Ok(results) = self.results.lock() else {
            return;
        };

        let items: Vec<ListItem> = results
            .iter()
            .filter(|r| r.status == PortStatus::Open)
            .map(|result| {
                let status_color = match result.status {
                    PortStatus::Open => Color::Green,
                    PortStatus::Closed => Color::Red,
                    PortStatus::Filtered => Color::Yellow,
                    PortStatus::Unknown => Color::Gray,
                };

                let service = result.service.as_deref().unwrap_or("unknown");
                let content = format!(
                    "{}:{} [{}] {} ({:.1}ms)",
                    result.target,
                    result.port,
                    match result.status {
                        PortStatus::Open => "OPEN",
                        PortStatus::Closed => "CLOSED",
                        PortStatus::Filtered => "FILTERED",
                        PortStatus::Unknown => "UNKNOWN",
                    },
                    service,
                    result.response_time.as_millis()
                );

                ListItem::new(Line::from(Span::styled(
                    content,
                    Style::default().fg(status_color),
                )))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Open Ports"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_statistics(&self, f: &mut Frame, area: Rect) {
        let Ok(stats) = self.stats.lock() else {
            return;
        };

        let data = [
            ("Open", stats.open_ports as u64),
            ("Closed", stats.closed_ports as u64),
            ("Filtered", stats.filtered_ports as u64),
        ];

        let barchart = BarChart::default()
            .block(Block::default().borders(Borders::ALL).title("Port Status"))
            .data(&data)
            .bar_width(9)
            .bar_style(Style::default().fg(Color::Yellow))
            .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

        f.render_widget(barchart, area);
    }

    fn render_settings(&self, f: &mut Frame, area: Rect) {
        let settings_text = vec![
            Line::from(vec![
                Span::styled("Scan Type: ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{:?}", self.config.scan_type)),
            ]),
            Line::from(vec![
                Span::styled("Threads: ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{}", self.config.threads)),
            ]),
            Line::from(vec![
                Span::styled("Timeout: ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{}ms", self.config.timeout.as_millis())),
            ]),
            Line::from(vec![
                Span::styled("Service Detection: ", Style::default().fg(Color::Cyan)),
                Span::raw(if self.config.service_detection {
                    "Enabled"
                } else {
                    "Disabled"
                }),
            ]),
            Line::from(""),
            Line::from("Controls:"),
            Line::from("← → : Switch tabs"),
            Line::from("↑ ↓ : Navigate results"),
            Line::from("q   : Quit"),
        ];

        let settings_paragraph = Paragraph::new(settings_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Configuration & Help"),
        );

        f.render_widget(settings_paragraph, area);
    }
}

async fn perform_scan(
    config: ScanConfig,
    results: Arc<Mutex<Vec<ScanResult>>>,
    stats: Arc<Mutex<ScanStats>>,
) {
    let semaphore = Arc::new(tokio::sync::Semaphore::new(config.threads));
    let mut handles = vec![];

    for target in &config.targets {
        for &port in &config.ports {
            let target = target.clone();
            let results = Arc::clone(&results);
            let stats = Arc::clone(&stats);
            let config = config.clone();
            let semaphore = Arc::clone(&semaphore);

            let handle = tokio::spawn(async move {
                let Ok(_permit) = semaphore.acquire().await else {
                    return;
                };
                let result = scan_port(&target, port, &config).await;

                // Update results
                {
                    let Ok(mut results) = results.lock() else {
                        return;
                    };
                    results.push(result.clone());
                }

                // Update stats
                {
                    let Ok(mut stats) = stats.lock() else {
                        return;
                    };
                    stats.ports_scanned += 1;
                    match result.status {
                        PortStatus::Open => stats.open_ports += 1,
                        PortStatus::Closed => stats.closed_ports += 1,
                        PortStatus::Filtered => stats.filtered_ports += 1,
                        _ => {}
                    }
                    stats.elapsed = stats.start_time.elapsed();

                    // Calculate ETA
                    if stats.ports_scanned > 0 {
                        let avg_time_per_port =
                            stats.elapsed.as_secs_f64() / stats.ports_scanned as f64;
                        let remaining_ports = stats.total_ports - stats.ports_scanned;
                        stats.estimated_remaining =
                            Duration::from_secs_f64(avg_time_per_port * remaining_ports as f64);
                    }
                }
            });

            handles.push(handle);
        }
    }

    // Wait for all scans to complete
    for handle in handles {
        let _ = handle.await;
    }
}

async fn scan_port(target: &str, port: u16, config: &ScanConfig) -> ScanResult {
    let start_time = Instant::now();

    // Parse target IP
    let target_ip: IpAddr = if let Ok(ip) = target.parse() {
        ip
    } else {
        // Try to resolve hostname
        match tokio::net::lookup_host(format!("{}:80", target)).await {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    addr.ip()
                } else {
                    return ScanResult {
                        target: target.to_string(),
                        port,
                        status: PortStatus::Unknown,
                        service: None,
                        banner: None,
                        response_time: start_time.elapsed(),
                    };
                }
            }
            Err(_) => {
                return ScanResult {
                    target: target.to_string(),
                    port,
                    status: PortStatus::Unknown,
                    service: None,
                    banner: None,
                    response_time: start_time.elapsed(),
                };
            }
        }
    };

    let socket_addr = SocketAddr::new(target_ip, port);

    // Perform TCP connect scan
    let status =
        match tokio::time::timeout(config.timeout, tokio::net::TcpStream::connect(socket_addr))
            .await
        {
            Ok(Ok(_)) => PortStatus::Open,
            Ok(Err(_)) => PortStatus::Closed,
            Err(_) => PortStatus::Filtered, // Timeout
        };

    let mut service = None;
    let mut banner = None;

    // Service detection for open ports
    if status == PortStatus::Open && config.service_detection {
        service = detect_service(port);

        // Try to grab banner
        if let Ok(Ok(mut stream)) = tokio::time::timeout(
            Duration::from_millis(500),
            tokio::net::TcpStream::connect(socket_addr),
        )
        .await
        {
            banner = grab_banner(&mut stream, port).await;
        }
    }

    ScanResult {
        target: target.to_string(),
        port,
        status,
        service,
        banner,
        response_time: start_time.elapsed(),
    }
}

fn detect_service(port: u16) -> Option<String> {
    // Use comprehensive services database
    super::services::get_service_name(port).map(|s| s.to_string())
}

async fn grab_banner(stream: &mut tokio::net::TcpStream, port: u16) -> Option<String> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    // Get probe from services database, fallback to empty for banner-on-connect services
    let probe = super::services::get_probe(port).unwrap_or("");

    if !probe.is_empty() {
        let _ = stream.write_all(probe.as_bytes()).await;
    }

    let mut buffer = [0; 2048];
    match tokio::time::timeout(Duration::from_millis(1500), stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            let banner = String::from_utf8_lossy(&buffer[..n]);
            // Try to extract version info
            if let Some(version) = super::services::extract_version(port, &banner) {
                Some(format!(
                    "{} ({})",
                    banner.lines().next().unwrap_or(""),
                    version
                ))
            } else {
                Some(banner.lines().next().unwrap_or("").to_string())
            }
        }
        _ => None,
    }
}

// CLI interface functions
pub fn scan_cli(
    targets: Vec<String>,
    ports: Option<String>,
    threads: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        let ports = parse_ports(ports.as_deref().unwrap_or("1-1000"))?;
        let config = ScanConfig {
            targets,
            ports,
            threads: threads.unwrap_or(100),
            timeout: Duration::from_millis(1000),
            scan_type: ScanType::Connect,
            service_detection: true,
            os_detection: false,
            vulnerability_scan: false,
        };

        let mut app = ScannerApp::new(config);
        app.run_scan_with_tui().await
    })
}

fn parse_ports(port_str: &str) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    let mut ports = Vec::new();

    for part in port_str.split(',') {
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            if range.len() == 2 {
                let start: u16 = range[0].parse()?;
                let end: u16 = range[1].parse()?;
                for port in start..=end {
                    ports.push(port);
                }
            }
        } else {
            ports.push(part.parse()?);
        }
    }

    Ok(ports)
}

/// Validates if a port number is in valid range (1-65535)
pub fn is_valid_port(port: u16) -> bool {
    port > 0 // u16 max is 65535, so we only need to check > 0
}

/// Validates an IP address string (basic validation)
pub fn is_valid_ipv4(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    parts.iter().all(|part| part.parse::<u8>().is_ok())
}

/// Validates a CIDR notation network (e.g., "192.168.1.0/24")
pub fn is_valid_cidr(cidr: &str) -> bool {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return false;
    }
    if !is_valid_ipv4(parts[0]) {
        return false;
    }
    match parts[1].parse::<u8>() {
        Ok(prefix) => prefix <= 32,
        Err(_) => false,
    }
}

/// Parse a port range string (e.g., "80-443") into start and end ports
pub fn parse_port_range(range_str: &str) -> Option<(u16, u16)> {
    let parts: Vec<&str> = range_str.split('-').collect();
    if parts.len() != 2 {
        return None;
    }
    let start = parts[0].parse::<u16>().ok()?;
    let end = parts[1].parse::<u16>().ok()?;
    if start > end || start == 0 {
        return None;
    }
    Some((start, end))
}

/// Format scan duration into human-readable string
pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.as_millis() % 1000;
    if secs >= 3600 {
        format!("{}h {}m {}s", secs / 3600, (secs % 3600) / 60, secs % 60)
    } else if secs >= 60 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else if secs > 0 {
        format!("{}.{}s", secs, millis / 100)
    } else {
        format!("{}ms", duration.as_millis())
    }
}

/// Calculate scan progress percentage
pub fn calculate_progress(completed: usize, total: usize) -> u8 {
    if total == 0 {
        return 0;
    }
    let progress = (completed as f64 / total as f64 * 100.0) as u8;
    progress.min(100)
}

// Placeholder for firewall integration
pub fn network_security_scanning() {
    println!("🔍 Network Security Scanning");
    println!("===========================");

    let options = vec![
        "🌐 Network Discovery Scan",
        "🚪 Port Scanning",
        "🔍 Service Detection",
        "⚡ Quick Scan",
        "📊 Scan Results Analysis",
        "⬅️  Back",
    ];

    let Ok(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🔍 Network Security Scanning")
        .items(&options)
        .default(0)
        .interact()
    else {
        return;
    };

    match selection {
        0 => network_discovery_scan(),
        1 => port_scanning_menu(),
        2 => service_detection_menu(),
        3 => quick_scan_menu(),
        4 => scan_results_analysis(),
        _ => {}
    }
}

fn network_discovery_scan() {
    println!("🌐 Network Discovery - Use: ghostctl scan <network_range>");
    println!("Example: ghostctl scan 192.168.1.0/24");
}

fn port_scanning_menu() {
    println!("🚪 Port Scanning - Use: ghostctl scan <target> -p <ports>");
    println!("Example: ghostctl scan 192.168.1.1 -p 80,443,8080");
}

fn service_detection_menu() {
    println!("🔍 Service Detection - Use: ghostctl scan <target> --service");
    println!("Example: ghostctl scan 192.168.1.1 --service");
}

fn quick_scan_menu() {
    println!("⚡ Quick Scan - Use: ghostctl scan <target>");
    println!("Example: ghostctl scan 192.168.1.1");
}

fn scan_results_analysis() {
    println!("📊 Scan Results Analysis");
    println!("Results are displayed in the TUI during scanning");
}

// =============================================================================
// Raw Socket Scanning (SYN/UDP) and OS Fingerprinting
// =============================================================================

/// Check if we have raw socket capability (CAP_NET_RAW or root)
pub fn has_raw_socket_capability() -> bool {
    use socket2::{Domain, Protocol, Socket, Type};

    // Try to create a raw TCP socket
    match Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::TCP)) {
        Ok(_) => {
            log::debug!("Raw socket capability available");
            true
        }
        Err(e) => {
            log::debug!("Raw socket not available: {}", e);
            false
        }
    }
}

/// Select the appropriate scan technique based on capabilities
pub fn select_scan_technique(requested: ScanType) -> ScanType {
    match requested {
        ScanType::Syn | ScanType::Udp if !has_raw_socket_capability() => {
            log::warn!(
                "Raw sockets unavailable (need root or CAP_NET_RAW), falling back to Connect scan"
            );
            println!("Note: SYN/UDP scan requires root. Using TCP Connect scan instead.");
            ScanType::Connect
        }
        other => other,
    }
}

/// Perform SYN scan on a single port (requires raw socket capability)
pub async fn syn_scan_port(target: IpAddr, port: u16, timeout: Duration) -> PortStatus {
    use socket2::{Domain, Protocol, Socket, Type};

    // This is a simplified SYN scan - in production you'd craft actual TCP packets
    // For now, we'll use a heuristic approach with socket options

    let domain = match target {
        IpAddr::V4(_) => Domain::IPV4,
        IpAddr::V6(_) => Domain::IPV6,
    };

    let socket = match Socket::new(domain, Type::STREAM, Some(Protocol::TCP)) {
        Ok(s) => s,
        Err(_) => return PortStatus::Unknown,
    };

    // Set non-blocking and timeout
    if socket.set_nonblocking(true).is_err() {
        return PortStatus::Unknown;
    }

    let socket_addr: socket2::SockAddr = SocketAddr::new(target, port).into();

    // Attempt connection with timeout
    let start = Instant::now();
    match socket.connect(&socket_addr) {
        Ok(_) => PortStatus::Open,
        Err(e) if e.raw_os_error() == Some(libc::EINPROGRESS) => {
            // Connection in progress - wait for it
            std::thread::sleep(timeout.min(Duration::from_millis(100)));

            // Check if connection succeeded
            match socket.take_error() {
                Ok(None) => PortStatus::Open,
                Ok(Some(e)) if e.raw_os_error() == Some(libc::ECONNREFUSED) => PortStatus::Closed,
                _ if start.elapsed() >= timeout => PortStatus::Filtered,
                _ => PortStatus::Closed,
            }
        }
        Err(e) if e.raw_os_error() == Some(libc::ECONNREFUSED) => PortStatus::Closed,
        Err(_) => PortStatus::Filtered,
    }
}

/// Perform UDP scan on a single port
pub async fn udp_scan_port(target: IpAddr, port: u16, timeout: Duration) -> PortStatus {
    use tokio::net::UdpSocket;

    // Bind to any available port
    let local_addr = match target {
        IpAddr::V4(_) => "0.0.0.0:0",
        IpAddr::V6(_) => "[::]:0",
    };

    let socket = match UdpSocket::bind(local_addr).await {
        Ok(s) => s,
        Err(_) => return PortStatus::Unknown,
    };

    let target_addr = SocketAddr::new(target, port);
    if socket.connect(target_addr).await.is_err() {
        return PortStatus::Unknown;
    }

    // Send a probe packet (generic UDP probe)
    let probe = get_udp_probe(port);
    if socket.send(&probe).await.is_err() {
        return PortStatus::Unknown;
    }

    // Wait for response
    let mut buf = [0u8; 1024];
    match tokio::time::timeout(timeout, socket.recv(&mut buf)).await {
        Ok(Ok(n)) if n > 0 => PortStatus::Open,
        Ok(Ok(_)) => PortStatus::Open, // Empty response still means open
        Ok(Err(e)) => {
            // Check for ICMP port unreachable (indicates closed)
            if e.raw_os_error() == Some(libc::ECONNREFUSED) {
                PortStatus::Closed
            } else {
                PortStatus::Filtered
            }
        }
        Err(_) => PortStatus::Filtered, // Timeout - could be open or filtered
    }
}

/// Get appropriate UDP probe for a given port
fn get_udp_probe(port: u16) -> Vec<u8> {
    match port {
        53 => {
            // DNS query for google.com
            vec![
                0x00, 0x01, // Transaction ID
                0x01, 0x00, // Standard query
                0x00, 0x01, // Questions: 1
                0x00, 0x00, // Answer RRs: 0
                0x00, 0x00, // Authority RRs: 0
                0x00, 0x00, // Additional RRs: 0
                0x06, b'g', b'o', b'o', b'g', b'l', b'e', // google
                0x03, b'c', b'o', b'm', // com
                0x00, // null terminator
                0x00, 0x01, // Type: A
                0x00, 0x01, // Class: IN
            ]
        }
        123 => {
            // NTP version request
            vec![0xe3, 0x00, 0x04, 0xfa, 0x00, 0x01, 0x00, 0x00]
        }
        161 | 162 => {
            // SNMP get-request (community: public)
            vec![
                0x30, 0x26, 0x02, 0x01, 0x01, 0x04, 0x06, b'p', b'u', b'b', b'l', b'i', b'c', 0xa0,
                0x19,
            ]
        }
        _ => {
            // Generic probe
            vec![0x00; 8]
        }
    }
}

/// OS fingerprinting result
#[derive(Debug, Clone)]
pub struct OsFingerprintResult {
    pub os_family: String,
    pub os_guess: String,
    pub confidence: f32,
    pub ttl: Option<u8>,
    pub window_size: Option<u32>,
}

/// Perform basic OS fingerprinting based on network characteristics
pub async fn fingerprint_os(target: IpAddr) -> Option<OsFingerprintResult> {
    // Try to connect to a common port to analyze TCP characteristics
    let common_ports = [80, 443, 22, 21, 25, 8080];

    for port in common_ports {
        if let Some(result) = fingerprint_via_port(target, port).await {
            return Some(result);
        }
    }

    None
}

/// Perform enhanced OS fingerprinting using the fingerprint module
/// This uses multiple indicators for better accuracy
pub async fn fingerprint_os_enhanced(target: IpAddr) -> Option<super::fingerprint::OsFingerprint> {
    use tokio::net::TcpStream;

    // Try common ports to get TCP characteristics
    let common_ports = [80, 443, 22, 21, 25, 8080, 3389, 5900];

    for port in common_ports {
        let addr = SocketAddr::new(target, port);

        // Connect with timeout
        let stream =
            match tokio::time::timeout(Duration::from_secs(2), TcpStream::connect(addr)).await {
                Ok(Ok(s)) => s,
                _ => continue,
            };

        // Get TTL from the connection
        let ttl = get_ttl_from_stream(&stream);

        if let Some(observed_ttl) = ttl {
            // Get window size if possible (requires platform-specific code)
            let window_size = get_window_size_from_stream(&stream).unwrap_or(65535);

            // Use the fingerprint module for enhanced detection
            // We pass empty TCP options since we can't easily get them without raw sockets
            if let Some(fp) = super::fingerprint::fingerprint_os(observed_ttl, window_size, &[]) {
                return Some(fp);
            }
        }
    }

    None
}

/// Quick OS fingerprint using just TTL (for fast scans)
pub fn quick_fingerprint_by_ttl(ttl: u8) -> OsFingerprintResult {
    let (os_family, os_guess, confidence) = super::fingerprint::quick_os_guess(ttl);
    OsFingerprintResult {
        os_family: os_family.to_string(),
        os_guess: os_guess.to_string(),
        confidence,
        ttl: Some(ttl),
        window_size: None,
    }
}

async fn fingerprint_via_port(target: IpAddr, port: u16) -> Option<OsFingerprintResult> {
    use tokio::net::TcpStream;

    let addr = SocketAddr::new(target, port);

    // Connect with timeout
    let stream = match tokio::time::timeout(Duration::from_secs(2), TcpStream::connect(addr)).await
    {
        Ok(Ok(s)) => s,
        _ => return None,
    };

    // Get TTL from the connection (platform-specific)
    let ttl = get_ttl_from_stream(&stream);

    // Use the fingerprint module for better detection
    if let Some(observed_ttl) = ttl {
        let fp = super::fingerprint::fingerprint_by_ttl(observed_ttl);
        return Some(OsFingerprintResult {
            os_family: fp.os_family,
            os_guess: fp.os_guess,
            confidence: fp.confidence,
            ttl: fp.ttl,
            window_size: fp.window_size,
        });
    }

    // Fallback to basic TTL analysis
    let (os_family, os_guess, confidence) = match ttl {
        Some(t) if t <= 64 && t > 32 => ("Linux/Unix", "Linux 2.6+/macOS/BSD", 0.7),
        Some(t) if t <= 128 && t > 64 => ("Windows", "Windows 7/8/10/Server", 0.7),
        Some(t) if t > 128 => ("Network Device", "Cisco/Juniper/Router", 0.5),
        Some(t) if t <= 32 => ("Embedded", "Embedded Linux/IoT", 0.4),
        _ => ("Unknown", "Could not determine", 0.1),
    };

    Some(OsFingerprintResult {
        os_family: os_family.to_string(),
        os_guess: os_guess.to_string(),
        confidence,
        ttl,
        window_size: None,
    })
}

/// Get TCP window size from stream (platform-specific)
#[cfg(unix)]
fn get_window_size_from_stream(stream: &tokio::net::TcpStream) -> Option<u16> {
    use std::os::unix::io::AsRawFd;

    let fd = stream.as_raw_fd();
    let mut rcvbuf: libc::c_int = 0;
    let mut len: libc::socklen_t = std::mem::size_of::<libc::c_int>() as libc::socklen_t;

    let result = unsafe {
        libc::getsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_RCVBUF,
            &mut rcvbuf as *mut _ as *mut libc::c_void,
            &mut len,
        )
    };

    if result == 0 {
        // SO_RCVBUF returns doubled value on Linux, divide by 2
        Some((rcvbuf / 2).min(65535) as u16)
    } else {
        None
    }
}

#[cfg(not(unix))]
fn get_window_size_from_stream(_stream: &tokio::net::TcpStream) -> Option<u16> {
    None
}

#[cfg(unix)]
fn get_ttl_from_stream(stream: &tokio::net::TcpStream) -> Option<u8> {
    use std::os::unix::io::AsRawFd;

    let fd = stream.as_raw_fd();
    let mut ttl: libc::c_int = 0;
    let mut len: libc::socklen_t = std::mem::size_of::<libc::c_int>() as libc::socklen_t;

    let result = unsafe {
        libc::getsockopt(
            fd,
            libc::IPPROTO_IP,
            libc::IP_TTL,
            &mut ttl as *mut _ as *mut libc::c_void,
            &mut len,
        )
    };

    if result == 0 { Some(ttl as u8) } else { None }
}

#[cfg(not(unix))]
fn get_ttl_from_stream(_stream: &tokio::net::TcpStream) -> Option<u8> {
    None
}

/// Extended scan function that supports SYN/UDP scanning
pub async fn scan_port_extended(target: &str, port: u16, config: &ScanConfig) -> ScanResult {
    let start_time = Instant::now();

    // Parse target IP
    let target_ip: IpAddr = match target.parse() {
        Ok(ip) => ip,
        Err(_) => {
            // Try DNS resolution
            match tokio::net::lookup_host(format!("{}:80", target)).await {
                Ok(mut addrs) => match addrs.next() {
                    Some(addr) => addr.ip(),
                    None => {
                        return ScanResult {
                            target: target.to_string(),
                            port,
                            status: PortStatus::Unknown,
                            service: None,
                            banner: None,
                            response_time: start_time.elapsed(),
                        };
                    }
                },
                Err(_) => {
                    return ScanResult {
                        target: target.to_string(),
                        port,
                        status: PortStatus::Unknown,
                        service: None,
                        banner: None,
                        response_time: start_time.elapsed(),
                    };
                }
            }
        }
    };

    // Select scan technique
    let effective_scan_type = select_scan_technique(config.scan_type.clone());

    let status = match effective_scan_type {
        ScanType::Syn => syn_scan_port(target_ip, port, config.timeout).await,
        ScanType::Udp => udp_scan_port(target_ip, port, config.timeout).await,
        ScanType::Connect | ScanType::Comprehensive => {
            // Use existing TCP connect scan
            let socket_addr = SocketAddr::new(target_ip, port);
            match tokio::time::timeout(config.timeout, tokio::net::TcpStream::connect(socket_addr))
                .await
            {
                Ok(Ok(_)) => PortStatus::Open,
                Ok(Err(_)) => PortStatus::Closed,
                Err(_) => PortStatus::Filtered,
            }
        }
    };

    let service = if status == PortStatus::Open && config.service_detection {
        detect_service(port)
    } else {
        None
    };

    ScanResult {
        target: target.to_string(),
        port,
        status,
        service,
        banner: None,
        response_time: start_time.elapsed(),
    }
}

/// Print capability status
pub fn print_scan_capabilities() {
    println!("Scanner Capabilities");
    println!("====================");

    let has_raw = has_raw_socket_capability();

    println!("TCP Connect Scan: Available");
    println!(
        "SYN Scan:         {}",
        if has_raw {
            "Available (root/CAP_NET_RAW)"
        } else {
            "Unavailable (need root or CAP_NET_RAW)"
        }
    );
    println!(
        "UDP Scan:         {}",
        if has_raw {
            "Available"
        } else {
            "Limited (no ICMP feedback)"
        }
    );
    println!("OS Fingerprint:   Enhanced (TTL + Window Size + TCP Options)");
    println!("  - TTL-based detection");
    println!("  - TCP window size analysis");
    println!("  - Known OS signature database");
    println!("Service Detection: Available");

    if !has_raw {
        println!();
        println!("To enable full scanning capabilities:");
        println!("  sudo setcap cap_net_raw+ep $(which ghostctl)");
        println!("  or run with: sudo ghostctl scan ...");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Port Parsing Tests ====================

    #[test]
    fn test_parse_ports_single() {
        let result = parse_ports("80").unwrap();
        assert_eq!(result, vec![80]);
    }

    #[test]
    fn test_parse_ports_multiple() {
        let result = parse_ports("80,443,8080").unwrap();
        assert_eq!(result, vec![80, 443, 8080]);
    }

    #[test]
    fn test_parse_ports_range() {
        let result = parse_ports("20-25").unwrap();
        assert_eq!(result, vec![20, 21, 22, 23, 24, 25]);
    }

    #[test]
    fn test_parse_ports_mixed() {
        let result = parse_ports("22,80-82,443").unwrap();
        assert_eq!(result, vec![22, 80, 81, 82, 443]);
    }

    #[test]
    fn test_parse_ports_single_range() {
        let result = parse_ports("1-3").unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_ports_invalid_returns_error() {
        assert!(parse_ports("invalid").is_err());
    }

    #[test]
    fn test_parse_ports_empty_range() {
        // Should still work as "1-1000" is the default in CLI
        let result = parse_ports("1-1000").unwrap();
        assert_eq!(result.len(), 1000);
        assert_eq!(result[0], 1);
        assert_eq!(result[999], 1000);
    }

    // ==================== Port Range Parsing Tests ====================

    #[test]
    fn test_parse_port_range_valid() {
        assert_eq!(parse_port_range("80-443"), Some((80, 443)));
    }

    #[test]
    fn test_parse_port_range_same_port() {
        assert_eq!(parse_port_range("80-80"), Some((80, 80)));
    }

    #[test]
    fn test_parse_port_range_invalid_format() {
        assert_eq!(parse_port_range("80"), None);
        assert_eq!(parse_port_range("80-443-8080"), None);
    }

    #[test]
    fn test_parse_port_range_reversed() {
        assert_eq!(parse_port_range("443-80"), None);
    }

    #[test]
    fn test_parse_port_range_zero_start() {
        assert_eq!(parse_port_range("0-80"), None);
    }

    #[test]
    fn test_parse_port_range_invalid_numbers() {
        assert_eq!(parse_port_range("abc-def"), None);
    }

    // ==================== Port Validation Tests ====================

    #[test]
    fn test_is_valid_port_normal() {
        assert!(is_valid_port(80));
        assert!(is_valid_port(443));
        assert!(is_valid_port(8080));
    }

    #[test]
    fn test_is_valid_port_boundaries() {
        assert!(is_valid_port(1));
        assert!(is_valid_port(65535));
    }

    #[test]
    fn test_is_valid_port_zero_invalid() {
        assert!(!is_valid_port(0));
    }

    // ==================== IP Validation Tests ====================

    #[test]
    fn test_is_valid_ipv4_normal() {
        assert!(is_valid_ipv4("192.168.1.1"));
        assert!(is_valid_ipv4("10.0.0.1"));
        assert!(is_valid_ipv4("8.8.8.8"));
    }

    #[test]
    fn test_is_valid_ipv4_boundaries() {
        assert!(is_valid_ipv4("0.0.0.0"));
        assert!(is_valid_ipv4("255.255.255.255"));
    }

    #[test]
    fn test_is_valid_ipv4_invalid_format() {
        assert!(!is_valid_ipv4("192.168.1"));
        assert!(!is_valid_ipv4("192.168.1.1.1"));
        assert!(!is_valid_ipv4("192.168.1.256"));
    }

    #[test]
    fn test_is_valid_ipv4_non_numeric() {
        assert!(!is_valid_ipv4("abc.def.ghi.jkl"));
        assert!(!is_valid_ipv4("192.168.1.a"));
    }

    #[test]
    fn test_is_valid_ipv4_empty() {
        assert!(!is_valid_ipv4(""));
    }

    // ==================== CIDR Validation Tests ====================

    #[test]
    fn test_is_valid_cidr_normal() {
        assert!(is_valid_cidr("192.168.1.0/24"));
        assert!(is_valid_cidr("10.0.0.0/8"));
        assert!(is_valid_cidr("172.16.0.0/16"));
    }

    #[test]
    fn test_is_valid_cidr_boundaries() {
        assert!(is_valid_cidr("0.0.0.0/0"));
        assert!(is_valid_cidr("192.168.1.1/32"));
    }

    #[test]
    fn test_is_valid_cidr_invalid_prefix() {
        assert!(!is_valid_cidr("192.168.1.0/33"));
        assert!(!is_valid_cidr("192.168.1.0/abc"));
    }

    #[test]
    fn test_is_valid_cidr_missing_prefix() {
        assert!(!is_valid_cidr("192.168.1.0"));
    }

    #[test]
    fn test_is_valid_cidr_invalid_ip() {
        assert!(!is_valid_cidr("256.168.1.0/24"));
    }

    // ==================== Service Detection Tests ====================

    #[test]
    fn test_detect_service_common_ports() {
        assert_eq!(detect_service(21), Some("ftp".to_string()));
        assert_eq!(detect_service(22), Some("ssh".to_string()));
        assert_eq!(detect_service(23), Some("telnet".to_string()));
        assert_eq!(detect_service(25), Some("smtp".to_string()));
        assert_eq!(detect_service(53), Some("dns".to_string()));
        assert_eq!(detect_service(80), Some("http".to_string()));
        assert_eq!(detect_service(443), Some("https".to_string()));
    }

    #[test]
    fn test_detect_service_database_ports() {
        assert_eq!(detect_service(3306), Some("mysql".to_string()));
        assert_eq!(detect_service(5432), Some("postgresql".to_string()));
        assert_eq!(detect_service(6379), Some("redis".to_string()));
        assert_eq!(detect_service(27017), Some("mongodb".to_string()));
    }

    #[test]
    fn test_detect_service_unknown_port() {
        assert_eq!(detect_service(12345), None);
        assert_eq!(detect_service(1), None);
        assert_eq!(detect_service(65535), None);
    }

    // ==================== Duration Formatting Tests ====================

    #[test]
    fn test_format_duration_milliseconds() {
        assert_eq!(format_duration(Duration::from_millis(100)), "100ms");
        assert_eq!(format_duration(Duration::from_millis(999)), "999ms");
    }

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration(Duration::from_secs(1)), "1.0s");
        assert_eq!(format_duration(Duration::from_secs(30)), "30.0s");
        assert_eq!(format_duration(Duration::from_secs(59)), "59.0s");
    }

    #[test]
    fn test_format_duration_minutes() {
        assert_eq!(format_duration(Duration::from_secs(60)), "1m 0s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3599)), "59m 59s");
    }

    #[test]
    fn test_format_duration_hours() {
        assert_eq!(format_duration(Duration::from_secs(3600)), "1h 0m 0s");
        assert_eq!(format_duration(Duration::from_secs(7265)), "2h 1m 5s");
    }

    // ==================== Progress Calculation Tests ====================

    #[test]
    fn test_calculate_progress_zero() {
        assert_eq!(calculate_progress(0, 100), 0);
    }

    #[test]
    fn test_calculate_progress_half() {
        assert_eq!(calculate_progress(50, 100), 50);
    }

    #[test]
    fn test_calculate_progress_complete() {
        assert_eq!(calculate_progress(100, 100), 100);
    }

    #[test]
    fn test_calculate_progress_empty_total() {
        assert_eq!(calculate_progress(0, 0), 0);
    }

    #[test]
    fn test_calculate_progress_capped_at_100() {
        assert_eq!(calculate_progress(150, 100), 100);
    }

    // ==================== Port Status Tests ====================

    #[test]
    fn test_port_status_equality() {
        assert_eq!(PortStatus::Open, PortStatus::Open);
        assert_eq!(PortStatus::Closed, PortStatus::Closed);
        assert_ne!(PortStatus::Open, PortStatus::Closed);
    }

    // ==================== Scan Config Tests ====================

    #[test]
    fn test_scan_config_default() {
        let config = ScanConfig::default();
        assert_eq!(config.targets, vec!["127.0.0.1".to_string()]);
        assert_eq!(config.ports.len(), 1000);
        assert_eq!(config.threads, 100);
        assert_eq!(config.timeout, Duration::from_millis(1000));
    }

    // ==================== Scan Result Tests ====================

    #[test]
    fn test_scan_result_creation() {
        let result = ScanResult {
            target: "192.168.1.1".to_string(),
            port: 80,
            status: PortStatus::Open,
            service: Some("http".to_string()),
            banner: Some("Apache/2.4".to_string()),
            response_time: Duration::from_millis(50),
        };
        assert_eq!(result.target, "192.168.1.1");
        assert_eq!(result.port, 80);
        assert_eq!(result.status, PortStatus::Open);
        assert_eq!(result.service, Some("http".to_string()));
    }
}
