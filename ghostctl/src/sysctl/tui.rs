//! Sysctl TUI Module
//!
//! Interactive terminal interface for browsing kernel parameters.

use super::{KernelParam, get_all_parameters};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};
use std::{collections::HashMap, io, time::Duration};

/// UI focus state
#[derive(Debug, Clone, Copy, PartialEq)]
enum FocusedBlock {
    Categories,
    Parameters,
    Search,
    Help,
}

/// Application state
struct App {
    running: bool,
    focused: FocusedBlock,
    all_params: Vec<KernelParam>,
    categories: Vec<String>,
    category_state: ListState,
    filtered_params: Vec<KernelParam>,
    param_state: ListState,
    search_query: String,
    help_visible: bool,
}

impl App {
    fn new() -> Self {
        let all_params = get_all_parameters();

        // Get unique categories
        let mut cats: HashMap<String, usize> = HashMap::new();
        for param in &all_params {
            *cats.entry(param.category.clone()).or_insert(0) += 1;
        }
        let mut categories: Vec<String> = cats.keys().cloned().collect();
        categories.sort();

        let mut category_state = ListState::default();
        category_state.select(Some(0));

        let filtered_params = if !categories.is_empty() {
            all_params
                .iter()
                .filter(|p| p.category == categories[0])
                .cloned()
                .collect()
        } else {
            Vec::new()
        };

        let mut param_state = ListState::default();
        if !filtered_params.is_empty() {
            param_state.select(Some(0));
        }

        Self {
            running: true,
            focused: FocusedBlock::Categories,
            all_params,
            categories,
            category_state,
            filtered_params,
            param_state,
            search_query: String::new(),
            help_visible: false,
        }
    }

    fn filter_by_category(&mut self, category: &str) {
        self.filtered_params = self
            .all_params
            .iter()
            .filter(|p| p.category == category)
            .cloned()
            .collect();

        self.param_state = ListState::default();
        if !self.filtered_params.is_empty() {
            self.param_state.select(Some(0));
        }
    }

    fn search(&mut self, query: &str) {
        if query.is_empty() {
            // Reset to category filter
            if let Some(idx) = self.category_state.selected() {
                let cat = self.categories.get(idx).cloned();
                if let Some(cat) = cat {
                    self.filter_by_category(&cat);
                }
            }
        } else {
            let query_lower = query.to_lowercase();
            self.filtered_params = self
                .all_params
                .iter()
                .filter(|p| {
                    p.name.to_lowercase().contains(&query_lower)
                        || p.description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&query_lower))
                            .unwrap_or(false)
                })
                .cloned()
                .collect();

            self.param_state = ListState::default();
            if !self.filtered_params.is_empty() {
                self.param_state.select(Some(0));
            }
        }
    }

    fn selected_param(&self) -> Option<&KernelParam> {
        self.param_state
            .selected()
            .and_then(|i| self.filtered_params.get(i))
    }
}

/// Main TUI entry point
pub fn sysctl_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    while app.running {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            handle_key_event(&mut app, key);
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn handle_key_event(app: &mut App, key: event::KeyEvent) {
    // Global keys
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            if app.help_visible {
                app.help_visible = false;
            } else if app.focused == FocusedBlock::Search && !app.search_query.is_empty() {
                app.search_query.clear();
                app.search("");
                app.focused = FocusedBlock::Parameters;
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
                FocusedBlock::Categories => FocusedBlock::Parameters,
                FocusedBlock::Parameters => FocusedBlock::Categories,
                FocusedBlock::Search => FocusedBlock::Parameters,
                _ => FocusedBlock::Categories,
            };
            return;
        }
        KeyCode::Char('/') => {
            app.focused = FocusedBlock::Search;
            return;
        }
        _ => {}
    }

    match app.focused {
        FocusedBlock::Categories => handle_category_keys(app, key),
        FocusedBlock::Parameters => handle_param_keys(app, key),
        FocusedBlock::Search => handle_search_keys(app, key),
        _ => {}
    }
}

fn handle_category_keys(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(selected) = app.category_state.selected()
                && selected > 0
            {
                let new_idx = selected - 1;
                app.category_state.select(Some(new_idx));
                let cat = app.categories.get(new_idx).cloned();
                if let Some(cat) = cat {
                    app.filter_by_category(&cat);
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(selected) = app.category_state.selected()
                && selected < app.categories.len().saturating_sub(1)
            {
                let new_idx = selected + 1;
                app.category_state.select(Some(new_idx));
                let cat = app.categories.get(new_idx).cloned();
                if let Some(cat) = cat {
                    app.filter_by_category(&cat);
                }
            }
        }
        KeyCode::Enter => {
            app.focused = FocusedBlock::Parameters;
        }
        _ => {}
    }
}

fn handle_param_keys(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(selected) = app.param_state.selected()
                && selected > 0
            {
                app.param_state.select(Some(selected - 1));
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(selected) = app.param_state.selected()
                && selected < app.filtered_params.len().saturating_sub(1)
            {
                app.param_state.select(Some(selected + 1));
            }
        }
        KeyCode::PageUp => {
            if let Some(selected) = app.param_state.selected() {
                let new_idx = selected.saturating_sub(10);
                app.param_state.select(Some(new_idx));
            }
        }
        KeyCode::PageDown => {
            if let Some(selected) = app.param_state.selected() {
                let new_idx = (selected + 10).min(app.filtered_params.len().saturating_sub(1));
                app.param_state.select(Some(new_idx));
            }
        }
        KeyCode::Home => {
            app.param_state.select(Some(0));
        }
        KeyCode::End => {
            if !app.filtered_params.is_empty() {
                app.param_state.select(Some(app.filtered_params.len() - 1));
            }
        }
        _ => {}
    }
}

fn handle_search_keys(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            app.search_query.push(c);
            app.search(&app.search_query.clone());
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            app.search(&app.search_query.clone());
        }
        KeyCode::Enter => {
            app.focused = FocusedBlock::Parameters;
        }
        _ => {}
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Search bar
            Constraint::Min(10),   // Main content
            Constraint::Length(6), // Detail view
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new(format!(
        "GhostCTL Kernel Parameter Browser - {} parameters loaded",
        app.all_params.len()
    ))
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Search bar
    let search_style = if app.focused == FocusedBlock::Search {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };
    let search_text = if app.search_query.is_empty() && app.focused != FocusedBlock::Search {
        "Press / to search".to_string()
    } else {
        format!("Search: {}_", app.search_query)
    };
    let search = Paragraph::new(search_text)
        .style(search_style)
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(search, chunks[1]);

    // Main content - 2 columns
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(chunks[2]);

    // Categories
    render_categories(f, app, main_chunks[0]);

    // Parameters
    render_parameters(f, app, main_chunks[1]);

    // Detail view
    render_detail(f, app, chunks[3]);

    // Help overlay
    if app.help_visible {
        render_help(f);
    }
}

fn render_categories(f: &mut Frame, app: &mut App, area: Rect) {
    let style = if app.focused == FocusedBlock::Categories {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };

    let items: Vec<ListItem> = app
        .categories
        .iter()
        .map(|cat| {
            let count = app.all_params.iter().filter(|p| p.category == *cat).count();
            ListItem::new(format!("{} ({})", cat, count))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Categories")
                .border_style(style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, area, &mut app.category_state);
}

fn render_parameters(f: &mut Frame, app: &mut App, area: Rect) {
    let style = if app.focused == FocusedBlock::Parameters {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };

    let items: Vec<ListItem> = app
        .filtered_params
        .iter()
        .map(|param| {
            let value_display = if param.value.len() > 30 {
                format!("{}...", &param.value[..30])
            } else {
                param.value.clone()
            };
            ListItem::new(format!("{} = {}", param.name, value_display))
        })
        .collect();

    let title = format!("Parameters ({})", app.filtered_params.len());
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, area, &mut app.param_state);
}

fn render_detail(f: &mut Frame, app: &App, area: Rect) {
    let content = if let Some(param) = app.selected_param() {
        let desc = param
            .description
            .as_ref()
            .map(|d| format!("\nDescription: {}", d))
            .unwrap_or_default();

        vec![
            Line::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&param.name),
            ]),
            Line::from(vec![
                Span::styled("Value: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(&param.value, Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("Path: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&param.path),
            ]),
            Line::from(if let Some(d) = &param.description {
                vec![
                    Span::styled("Info: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(d),
                ]
            } else {
                vec![Span::raw("")]
            }),
        ]
    } else {
        vec![Line::from("Select a parameter to view details")]
    };

    let detail = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Details"))
        .wrap(Wrap { trim: false });

    f.render_widget(detail, area);
}

fn render_help(f: &mut Frame) {
    let area = centered_rect(50, 50, f.area());
    f.render_widget(Clear, area);

    let help_text = vec![
        Line::from(Span::styled(
            "Keyboard Shortcuts",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Tab          Switch panels"),
        Line::from("  /            Search parameters"),
        Line::from("  j/k, Up/Down Navigate"),
        Line::from("  PgUp/PgDown  Page up/down"),
        Line::from("  Home/End     Jump to start/end"),
        Line::from("  ?            Toggle help"),
        Line::from("  q/Esc        Quit"),
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
