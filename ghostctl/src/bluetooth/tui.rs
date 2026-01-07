//! Bluetooth TUI Module
//!
//! Interactive terminal interface for Bluetooth management using ratatui.

use anyhow::Result;
use bluer::{Adapter, Address, Device, Session};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::StreamExt;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{
    io,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::mpsc;

/// UI focus state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusedBlock {
    Adapters,
    PairedDevices,
    DiscoveredDevices,
    DeviceInfo,
    Help,
}

/// Device information cache
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub address: Address,
    pub name: String,
    pub connected: bool,
    pub paired: bool,
    pub trusted: bool,
    pub rssi: Option<i16>,
    pub icon: Option<String>,
}

/// Notification message
#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub level: NotificationLevel,
    pub ttl: u8,
}

#[derive(Debug, Clone)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// Application state
pub struct App {
    pub running: bool,
    pub focused: FocusedBlock,
    pub session: Option<Arc<Session>>,
    pub adapters: Vec<String>,
    pub adapter_state: ListState,
    pub current_adapter: Option<Arc<Adapter>>,
    pub paired_devices: Vec<DeviceInfo>,
    pub paired_state: ListState,
    pub discovered_devices: Vec<DeviceInfo>,
    pub discovered_state: ListState,
    pub notifications: Vec<Notification>,
    pub scanning: bool,
    pub help_visible: bool,
    pub last_refresh: Instant,
}

impl App {
    pub fn new() -> Self {
        let mut adapter_state = ListState::default();
        adapter_state.select(Some(0));

        Self {
            running: true,
            focused: FocusedBlock::Adapters,
            session: None,
            adapters: Vec::new(),
            adapter_state,
            current_adapter: None,
            paired_devices: Vec::new(),
            paired_state: ListState::default(),
            discovered_devices: Vec::new(),
            discovered_state: ListState::default(),
            notifications: Vec::new(),
            scanning: false,
            help_visible: false,
            last_refresh: Instant::now(),
        }
    }

    pub fn add_notification(&mut self, message: &str, level: NotificationLevel) {
        self.notifications.push(Notification {
            message: message.to_string(),
            level,
            ttl: 50, // ~5 seconds at 100ms tick
        });
    }

    pub fn tick(&mut self) {
        // Decay notifications
        self.notifications.retain_mut(|n| {
            n.ttl = n.ttl.saturating_sub(1);
            n.ttl > 0
        });
    }

    fn selected_paired_device(&self) -> Option<&DeviceInfo> {
        self.paired_state
            .selected()
            .and_then(|i| self.paired_devices.get(i))
    }

    fn selected_discovered_device(&self) -> Option<&DeviceInfo> {
        self.discovered_state
            .selected()
            .and_then(|i| self.discovered_devices.get(i))
    }
}

/// Event types for the TUI
#[derive(Debug)]
pub enum AppEvent {
    Tick,
    Key(event::KeyEvent),
    SessionReady(Arc<Session>),
    AdaptersLoaded(Vec<String>),
    AdapterSelected(Arc<Adapter>),
    DevicesRefreshed(Vec<DeviceInfo>, Vec<DeviceInfo>),
    DeviceDiscovered(DeviceInfo),
    ScanComplete,
    Error(String),
}

/// Main TUI entry point
pub fn bluetooth_tui() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Create tokio runtime for async operations
    let rt = tokio::runtime::Runtime::new()?;

    // Create event channel
    let (tx, mut rx) = mpsc::unbounded_channel::<AppEvent>();

    // Initialize session in background
    let tx_init = tx.clone();
    rt.spawn(async move {
        match Session::new().await {
            Ok(session) => {
                let session = Arc::new(session);
                let _ = tx_init.send(AppEvent::SessionReady(session.clone()));

                // Load adapters
                match session.adapter_names().await {
                    Ok(names) => {
                        let _ = tx_init.send(AppEvent::AdaptersLoaded(names));
                    }
                    Err(e) => {
                        let _ = tx_init
                            .send(AppEvent::Error(format!("Failed to list adapters: {}", e)));
                    }
                }
            }
            Err(e) => {
                let _ = tx_init.send(AppEvent::Error(format!(
                    "Failed to connect to BlueZ. Is bluetooth service running?\nError: {}",
                    e
                )));
            }
        }
    });

    // Spawn tick generator
    let tx_tick = tx.clone();
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(100));
        if tx_tick.send(AppEvent::Tick).is_err() {
            break;
        }
    });

    // Spawn key event handler
    let tx_key = tx.clone();
    std::thread::spawn(move || loop {
        if event::poll(Duration::from_millis(50)).unwrap_or(false)
            && let Ok(Event::Key(key)) = event::read()
                && tx_key.send(AppEvent::Key(key)).is_err() {
                    break;
                }
    });

    // Main loop
    while app.running {
        // Draw UI
        terminal.draw(|f| ui(f, &mut app))?;

        // Handle events
        if let Ok(event) = rx.try_recv() {
            match event {
                AppEvent::Tick => {
                    app.tick();
                }
                AppEvent::Key(key) => {
                    handle_key_event(&mut app, key, &tx, &rt);
                }
                AppEvent::SessionReady(session) => {
                    app.session = Some(session);
                    app.add_notification("Connected to BlueZ", NotificationLevel::Success);
                }
                AppEvent::AdaptersLoaded(names) => {
                    app.adapters = names;
                    if !app.adapters.is_empty() {
                        app.adapter_state.select(Some(0));
                        // Auto-select first adapter
                        if let Some(session) = &app.session
                            && let Ok(adapter) = session.adapter(&app.adapters[0]) {
                                let adapter = Arc::new(adapter);
                                let _ = tx.send(AppEvent::AdapterSelected(adapter));
                            }
                    }
                }
                AppEvent::AdapterSelected(adapter) => {
                    app.current_adapter = Some(adapter.clone());
                    // Refresh devices
                    let tx_refresh = tx.clone();
                    rt.spawn(async move {
                        let mut paired = Vec::new();
                        let mut discovered = Vec::new();

                        if let Ok(addresses) = adapter.device_addresses().await {
                            for addr in addresses {
                                if let Ok(device) = adapter.device(addr) {
                                    let info = get_device_info(&device).await;
                                    if info.paired {
                                        paired.push(info);
                                    } else {
                                        discovered.push(info);
                                    }
                                }
                            }
                        }

                        let _ = tx_refresh.send(AppEvent::DevicesRefreshed(paired, discovered));
                    });
                }
                AppEvent::DevicesRefreshed(paired, discovered) => {
                    app.paired_devices = paired;
                    app.discovered_devices = discovered;
                    if !app.paired_devices.is_empty() && app.paired_state.selected().is_none() {
                        app.paired_state.select(Some(0));
                    }
                    if !app.discovered_devices.is_empty()
                        && app.discovered_state.selected().is_none()
                    {
                        app.discovered_state.select(Some(0));
                    }
                }
                AppEvent::DeviceDiscovered(info) => {
                    // Add to discovered if not already there
                    if !app
                        .discovered_devices
                        .iter()
                        .any(|d| d.address == info.address)
                        && !app.paired_devices.iter().any(|d| d.address == info.address)
                    {
                        app.discovered_devices.push(info);
                        if app.discovered_state.selected().is_none() {
                            app.discovered_state.select(Some(0));
                        }
                    }
                }
                AppEvent::ScanComplete => {
                    app.scanning = false;
                    app.add_notification("Scan complete", NotificationLevel::Info);
                }
                AppEvent::Error(msg) => {
                    app.add_notification(&msg, NotificationLevel::Error);
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn get_device_info(device: &Device) -> DeviceInfo {
    DeviceInfo {
        address: device.address(),
        name: device
            .name()
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| "Unknown".to_string()),
        connected: device.is_connected().await.unwrap_or(false),
        paired: device.is_paired().await.unwrap_or(false),
        trusted: device.is_trusted().await.unwrap_or(false),
        rssi: device.rssi().await.ok().flatten(),
        icon: device.icon().await.ok().flatten(),
    }
}

fn handle_key_event(
    app: &mut App,
    key: event::KeyEvent,
    tx: &mpsc::UnboundedSender<AppEvent>,
    rt: &tokio::runtime::Runtime,
) {
    // Global keys
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            if app.help_visible {
                app.help_visible = false;
            } else {
                app.running = false;
            }
            return;
        }
        KeyCode::Char('?') => {
            app.help_visible = !app.help_visible;
            return;
        }
        KeyCode::Tab => {
            app.focused = match app.focused {
                FocusedBlock::Adapters => FocusedBlock::PairedDevices,
                FocusedBlock::PairedDevices => FocusedBlock::DiscoveredDevices,
                FocusedBlock::DiscoveredDevices => FocusedBlock::Adapters,
                _ => FocusedBlock::Adapters,
            };
            return;
        }
        KeyCode::BackTab => {
            app.focused = match app.focused {
                FocusedBlock::Adapters => FocusedBlock::DiscoveredDevices,
                FocusedBlock::PairedDevices => FocusedBlock::Adapters,
                FocusedBlock::DiscoveredDevices => FocusedBlock::PairedDevices,
                _ => FocusedBlock::Adapters,
            };
            return;
        }
        _ => {}
    }

    // Context-specific keys
    match app.focused {
        FocusedBlock::Adapters => handle_adapter_keys(app, key, tx, rt),
        FocusedBlock::PairedDevices => handle_paired_keys(app, key, tx, rt),
        FocusedBlock::DiscoveredDevices => handle_discovered_keys(app, key, tx, rt),
        _ => {}
    }
}

fn handle_adapter_keys(
    app: &mut App,
    key: event::KeyEvent,
    tx: &mpsc::UnboundedSender<AppEvent>,
    rt: &tokio::runtime::Runtime,
) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(selected) = app.adapter_state.selected()
                && selected > 0 {
                    app.adapter_state.select(Some(selected - 1));
                }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(selected) = app.adapter_state.selected()
                && selected < app.adapters.len().saturating_sub(1) {
                    app.adapter_state.select(Some(selected + 1));
                }
        }
        KeyCode::Enter => {
            if let (Some(session), Some(idx)) = (&app.session, app.adapter_state.selected())
                && let Some(name) = app.adapters.get(idx)
                    && let Ok(adapter) = session.adapter(name) {
                        let _ = tx.send(AppEvent::AdapterSelected(Arc::new(adapter)));
                    }
        }
        KeyCode::Char('p') => {
            // Toggle power
            if let Some(adapter) = &app.current_adapter {
                let adapter = adapter.clone();
                let tx = tx.clone();
                rt.spawn(async move {
                    let current = adapter.is_powered().await.unwrap_or(false);
                    if let Err(e) = adapter.set_powered(!current).await {
                        let _ = tx.send(AppEvent::Error(format!("Failed to toggle power: {}", e)));
                    }
                });
            }
        }
        KeyCode::Char('s') => {
            // Start scan
            if !app.scanning {
                // Clone adapter first to avoid borrow issues
                let adapter = app.current_adapter.clone();
                if let Some(adapter) = adapter {
                    app.scanning = true;
                    app.add_notification("Scanning for devices...", NotificationLevel::Info);

                    let adapter = adapter.clone();
                    let tx = tx.clone();
                    rt.spawn(async move {
                        // Ensure powered
                        let _ = adapter.set_powered(true).await;

                        match adapter.discover_devices().await {
                            Ok(mut stream) => {
                                let timeout = tokio::time::sleep(Duration::from_secs(10));
                                tokio::pin!(timeout);

                                loop {
                                    tokio::select! {
                                        _ = &mut timeout => break,
                                        event = stream.next() => {
                                            match event {
                                                Some(bluer::AdapterEvent::DeviceAdded(addr)) => {
                                                    if let Ok(device) = adapter.device(addr) {
                                                        let info = get_device_info(&device).await;
                                                        let _ = tx.send(AppEvent::DeviceDiscovered(info));
                                                    }
                                                }
                                                None => break,
                                                _ => {}
                                            }
                                        }
                                    }
                                }

                                let _ = tx.send(AppEvent::ScanComplete);
                            }
                            Err(e) => {
                                let _ = tx.send(AppEvent::Error(format!("Scan failed: {}", e)));
                                let _ = tx.send(AppEvent::ScanComplete);
                            }
                        }
                    });
                }
            }
        }
        _ => {}
    }
}

fn handle_paired_keys(
    app: &mut App,
    key: event::KeyEvent,
    tx: &mpsc::UnboundedSender<AppEvent>,
    rt: &tokio::runtime::Runtime,
) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(selected) = app.paired_state.selected()
                && selected > 0 {
                    app.paired_state.select(Some(selected - 1));
                }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(selected) = app.paired_state.selected()
                && selected < app.paired_devices.len().saturating_sub(1) {
                    app.paired_state.select(Some(selected + 1));
                }
        }
        KeyCode::Enter | KeyCode::Char('c') => {
            // Connect/disconnect
            if let (Some(adapter), Some(device_info)) =
                (&app.current_adapter, app.selected_paired_device())
            {
                let addr = device_info.address;
                let was_connected = device_info.connected;
                let adapter = adapter.clone();
                let tx = tx.clone();

                rt.spawn(async move {
                    if let Ok(device) = adapter.device(addr) {
                        let result = if was_connected {
                            device.disconnect().await
                        } else {
                            device.connect().await
                        };

                        if let Err(e) = result {
                            let action = if was_connected {
                                "disconnect"
                            } else {
                                "connect"
                            };
                            let _ =
                                tx.send(AppEvent::Error(format!("Failed to {}: {}", action, e)));
                        }

                        // Refresh device list
                        if let Some(adapter) = Some(adapter) {
                            let _ = tx.send(AppEvent::AdapterSelected(adapter));
                        }
                    }
                });
            }
        }
        KeyCode::Char('t') => {
            // Toggle trusted
            if let (Some(adapter), Some(device_info)) =
                (&app.current_adapter, app.selected_paired_device())
            {
                let addr = device_info.address;
                let was_trusted = device_info.trusted;
                let adapter = adapter.clone();
                let tx = tx.clone();

                rt.spawn(async move {
                    if let Ok(device) = adapter.device(addr) {
                        if let Err(e) = device.set_trusted(!was_trusted).await {
                            let _ =
                                tx.send(AppEvent::Error(format!("Failed to toggle trust: {}", e)));
                        }
                        let _ = tx.send(AppEvent::AdapterSelected(adapter));
                    }
                });
            }
        }
        KeyCode::Char('r') | KeyCode::Delete => {
            // Remove device
            if let (Some(adapter), Some(device_info)) =
                (&app.current_adapter, app.selected_paired_device())
            {
                let addr = device_info.address;
                let adapter = adapter.clone();
                let tx = tx.clone();

                rt.spawn(async move {
                    if let Err(e) = adapter.remove_device(addr).await {
                        let _ = tx.send(AppEvent::Error(format!("Failed to remove device: {}", e)));
                    }
                    let _ = tx.send(AppEvent::AdapterSelected(adapter));
                });
            }
        }
        _ => {}
    }
}

fn handle_discovered_keys(
    app: &mut App,
    key: event::KeyEvent,
    tx: &mpsc::UnboundedSender<AppEvent>,
    rt: &tokio::runtime::Runtime,
) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(selected) = app.discovered_state.selected()
                && selected > 0 {
                    app.discovered_state.select(Some(selected - 1));
                }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(selected) = app.discovered_state.selected()
                && selected < app.discovered_devices.len().saturating_sub(1) {
                    app.discovered_state.select(Some(selected + 1));
                }
        }
        KeyCode::Enter | KeyCode::Char('c') => {
            // Pair and connect
            if let (Some(adapter), Some(device_info)) =
                (&app.current_adapter, app.selected_discovered_device())
            {
                let addr = device_info.address;
                let adapter = adapter.clone();
                let tx = tx.clone();

                rt.spawn(async move {
                    if let Ok(device) = adapter.device(addr) {
                        // Try to pair first
                        if !device.is_paired().await.unwrap_or(false)
                            && let Err(e) = device.pair().await {
                                let _ = tx.send(AppEvent::Error(format!("Pairing failed: {}", e)));
                                return;
                            }

                        // Then connect
                        if let Err(e) = device.connect().await {
                            let _ = tx.send(AppEvent::Error(format!("Connection failed: {}", e)));
                        }

                        let _ = tx.send(AppEvent::AdapterSelected(adapter));
                    }
                });
            }
        }
        _ => {}
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Status/notifications
        ])
        .split(f.area());

    // Header
    let scanning_indicator = if app.scanning { " [Scanning...]" } else { "" };
    let header = Paragraph::new(format!("GhostCTL Bluetooth Manager{}", scanning_indicator))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Main content - 3 columns
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Adapters
            Constraint::Percentage(40), // Paired devices
            Constraint::Percentage(35), // Discovered devices
        ])
        .split(chunks[1]);

    // Adapters list
    render_adapters(f, app, main_chunks[0]);

    // Paired devices
    render_paired_devices(f, app, main_chunks[1]);

    // Discovered devices
    render_discovered_devices(f, app, main_chunks[2]);

    // Status bar
    let status = if !app.notifications.is_empty() {
        let n = &app.notifications[app.notifications.len() - 1];
        let color = match n.level {
            NotificationLevel::Info => Color::Blue,
            NotificationLevel::Success => Color::Green,
            NotificationLevel::Warning => Color::Yellow,
            NotificationLevel::Error => Color::Red,
        };
        Paragraph::new(n.message.clone())
            .style(Style::default().fg(color))
            .block(Block::default().borders(Borders::ALL).title("Status"))
    } else {
        Paragraph::new("Press ? for help | Tab to switch panels | q to quit")
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title("Help"))
    };
    f.render_widget(status, chunks[2]);

    // Help overlay
    if app.help_visible {
        render_help(f);
    }
}

fn render_adapters(f: &mut Frame, app: &mut App, area: Rect) {
    let style = if app.focused == FocusedBlock::Adapters {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };

    let items: Vec<ListItem> = app
        .adapters
        .iter()
        .map(|name| ListItem::new(name.as_str()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Adapters [p=power s=scan]")
                .border_style(style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, area, &mut app.adapter_state);
}

fn render_paired_devices(f: &mut Frame, app: &mut App, area: Rect) {
    let style = if app.focused == FocusedBlock::PairedDevices {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };

    let items: Vec<ListItem> = app
        .paired_devices
        .iter()
        .map(|d| {
            let status = if d.connected { "[C]" } else { "[ ]" };
            let trust = if d.trusted { "T" } else { " " };
            ListItem::new(format!("{} {} {} ({})", status, trust, d.name, d.address))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Paired [c=connect t=trust r=remove]")
                .border_style(style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, area, &mut app.paired_state);
}

fn render_discovered_devices(f: &mut Frame, app: &mut App, area: Rect) {
    let style = if app.focused == FocusedBlock::DiscoveredDevices {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };

    let items: Vec<ListItem> = app
        .discovered_devices
        .iter()
        .map(|d| {
            let rssi = d.rssi.map(|r| format!(" {}dBm", r)).unwrap_or_default();
            ListItem::new(format!("{}{}", d.name, rssi))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Discovered [Enter=pair+connect]")
                .border_style(style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, area, &mut app.discovered_state);
}

fn render_help(f: &mut Frame) {
    let area = centered_rect(60, 70, f.area());
    f.render_widget(Clear, area);

    let help_text = vec![
        Line::from(Span::styled(
            "Keyboard Shortcuts",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  Tab/Shift+Tab  Switch between panels"),
        Line::from("  j/k or Up/Down Navigate lists"),
        Line::from("  Enter          Select/connect"),
        Line::from(""),
        Line::from("Adapter Panel:"),
        Line::from("  p              Toggle adapter power"),
        Line::from("  s              Start device scan"),
        Line::from(""),
        Line::from("Paired Devices:"),
        Line::from("  c/Enter        Connect/disconnect device"),
        Line::from("  t              Toggle device trust"),
        Line::from("  r/Delete       Remove device"),
        Line::from(""),
        Line::from("Discovered Devices:"),
        Line::from("  Enter/c        Pair and connect"),
        Line::from(""),
        Line::from("General:"),
        Line::from("  ?              Toggle this help"),
        Line::from("  q/Esc          Quit"),
    ];

    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(help, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
