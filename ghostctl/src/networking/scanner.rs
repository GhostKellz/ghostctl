use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

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
                            let results = self.results.lock().unwrap();
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
            let stats = self.stats.lock().unwrap();
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
        let stats = self.stats.lock().unwrap();
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
        let results = self.results.lock().unwrap();

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
        let stats = self.stats.lock().unwrap();

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
                let _permit = semaphore.acquire().await.unwrap();
                let result = scan_port(&target, port, &config).await;

                // Update results
                {
                    let mut results = results.lock().unwrap();
                    results.push(result.clone());
                }

                // Update stats
                {
                    let mut stats = stats.lock().unwrap();
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
    let common_services = [
        (21, "ftp"),
        (22, "ssh"),
        (23, "telnet"),
        (25, "smtp"),
        (53, "dns"),
        (80, "http"),
        (110, "pop3"),
        (143, "imap"),
        (443, "https"),
        (993, "imaps"),
        (995, "pop3s"),
        (3306, "mysql"),
        (3389, "rdp"),
        (5432, "postgresql"),
        (6379, "redis"),
        (27017, "mongodb"),
    ];

    common_services
        .iter()
        .find(|(p, _)| *p == port)
        .map(|(_, service)| service.to_string())
}

async fn grab_banner(stream: &mut tokio::net::TcpStream, port: u16) -> Option<String> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    // Send appropriate probe based on port
    let probe = match port {
        21 => "", // FTP sends banner on connect
        22 => "", // SSH sends banner on connect
        25 => "EHLO ghostctl\r\n",
        80 => "GET / HTTP/1.0\r\n\r\n",
        443 => "", // HTTPS needs TLS handshake
        _ => "",
    };

    if !probe.is_empty() {
        let _ = stream.write_all(probe.as_bytes()).await;
    }

    let mut buffer = [0; 1024];
    match tokio::time::timeout(Duration::from_millis(1000), stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            let banner = String::from_utf8_lossy(&buffer[..n]);
            Some(banner.lines().next().unwrap_or("").to_string())
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

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🔍 Network Security Scanning")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

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
