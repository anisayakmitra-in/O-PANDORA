use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::cat::RuntimeCat;
use crate::command::View;
use crate::theme;

/// Overall application state shared across widgets.
pub struct AppState {
    pub view: View,
    pub input: String,
    pub event_log: Vec<String>,
    pub cat: RuntimeCat,
    pub show_help: bool,
    pub uptime_secs: u64,
    pub personality: String,

    // Parliament subsystems
    pub service_count: usize,
    pub active_leases: usize,
    pub policy_count: usize,
    pub active_loops: usize,
    pub active_models: Vec<(String, f64)>,

    // Hardware
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub gpu_usage: f64,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            view: View::Dashboard,
            input: String::new(),
            event_log: vec![
                "[BOOT] Parliament kernel initialized".into(),
                "[BOOT] Service Registry ready".into(),
                "[BOOT] Event Bus online".into(),
                "[BOOT] Lease Manager active".into(),
                "[BOOT] Constitution Engine loaded".into(),
                "[INFO] Waiting for service registrations...".into(),
            ],
            cat: RuntimeCat::new(),
            show_help: false,
            uptime_secs: 0,
            personality: "chaotic-cat".into(),
            service_count: 0,
            active_leases: 0,
            policy_count: 0,
            active_loops: 0,
            active_models: vec![
                ("qwen2.5-coder:7b".into(), 92.0),
                ("deepseek-coder:6.7b".into(), 88.0),
                ("gemma:7b".into(), 71.0),
                ("llama3:8b".into(), 54.0),
            ],
            cpu_usage: 23.5,
            memory_usage: 41.2,
            gpu_usage: 67.8,
        }
    }

    pub fn push_event(&mut self, msg: String) {
        self.event_log.push(msg);
        if self.event_log.len() > 100 {
            self.event_log.remove(0);
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Render the full dashboard based on current view.
pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.size();

    // Top banner
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // banner
            Constraint::Min(10),   // body
            Constraint::Length(3), // prompt
        ])
        .split(area);

    render_banner(frame, main[0], state);
    render_body(frame, main[1], state);
    render_prompt(frame, main[2], state);
}

fn render_banner(frame: &mut Frame, area: Rect, state: &AppState) {
    let title = format!(" PANDORA PARLIAMENT v0.1 [{}]", state.view.title());
    let text = Line::from(vec![
        Span::styled(
            "█▀█ █▄ █ █▀▄ █▀█ █▄ █ █▀█ █▀▀",
            Style::default()
                .fg(theme::PINK)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            format!("| {}", state.personality),
            Style::default().fg(theme::GOLD),
        ),
    ]);

    let banner = Paragraph::new(text)
        .style(Style::default().bg(theme::BG))
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(theme::border_style())
                .style(Style::default().bg(theme::BG)),
        );
    frame.render_widget(banner, area);
}

fn render_body(frame: &mut Frame, area: Rect, state: &AppState) {
    match state.view {
        View::Dashboard => render_dashboard(frame, area, state),
        View::Help => render_help(frame, area),
        _ => render_dashboard(frame, area, state),
    }
}

fn render_dashboard(frame: &mut Frame, area: Rect, state: &AppState) {
    // Main layout: left (cat + telemetry), center (events), right (services + hardware)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(40),
            Constraint::Percentage(35),
        ])
        .split(area);

    render_left_panel(frame, chunks[0], state);
    render_center_panel(frame, chunks[1], state);
    render_right_panel(frame, chunks[2], state);
}

fn render_left_panel(frame: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Min(4),
        ])
        .split(area);

    // Cat panel
    let cat_art = state.cat.render();
    let cat_block = Paragraph::new(cat_art).block(
        Block::default()
            .title("RUNTIME CAT")
            .borders(Borders::ALL)
            .border_style(theme::purple_style()),
    );
    frame.render_widget(cat_block, chunks[0]);

    // Telemetry panel
    let telemetry = format!(
        "Parliament\n  Phase: operational\n  Policies: {}\n  Services: {}\n  Leases: {}\n  Loops: {}",
        state.policy_count, state.service_count, state.active_leases, state.active_loops
    );
    let tel_block = Paragraph::new(telemetry)
        .style(theme::lavender_style())
        .block(
            Block::default()
                .title("CONSTITUTIONAL STATE")
                .borders(Borders::ALL)
                .border_style(theme::gold_style()),
        );
    frame.render_widget(tel_block, chunks[1]);

    // Hardware panel
    let hw = format!(
        "CPU:  [{:25}] {:.1}%\nRAM:  [{:25}] {:.1}%\nGPU:  [{:25}] {:.1}%",
        "▰".repeat((state.cpu_usage / 4.0) as usize),
        state.cpu_usage,
        "▰".repeat((state.memory_usage / 4.0) as usize),
        state.memory_usage,
        "▰".repeat((state.gpu_usage / 4.0) as usize),
        state.gpu_usage,
    );
    let hw_block = Paragraph::new(hw).style(theme::lavender_style()).block(
        Block::default()
            .title("HARDWARE")
            .borders(Borders::ALL)
            .border_style(theme::border_style()),
    );
    frame.render_widget(hw_block, chunks[2]);
}

fn render_center_panel(frame: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(5)])
        .split(area);

    // Model rankings
    let model_items: Vec<ListItem> = state
        .active_models
        .iter()
        .map(|(name, score)| {
            let bar_len = (*score as usize) / 4;
            let bar = "▰".repeat(bar_len);
            let content = format!(" {}  [{:25}]  {:.0}", name, bar, score);
            ListItem::new(Line::from(vec![Span::styled(
                content,
                theme::lavender_style(),
            )]))
        })
        .collect();

    let model_list = List::new(model_items).block(
        Block::default()
            .title("MODEL INTELLIGENCE RANKINGS")
            .borders(Borders::ALL)
            .border_style(theme::border_style()),
    );
    frame.render_widget(model_list, chunks[0]);

    // Event log
    let visible_events: Vec<Line> = state
        .event_log
        .iter()
        .rev()
        .take(15)
        .map(|e| {
            let style = if e.contains("ERROR") || e.contains("FAIL") {
                theme::error_style()
            } else if e.contains("WARN") {
                theme::warn_style()
            } else {
                theme::lavender_style()
            };
            Line::from(Span::styled(e.clone(), style))
        })
        .collect();

    let events = Paragraph::new(visible_events)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title("EVENT BUS")
                .borders(Borders::ALL)
                .border_style(theme::gold_style()),
        );
    frame.render_widget(events, chunks[1]);
}

fn render_right_panel(frame: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Services
    let services = format!(
        "Memory Service:    {}\nExecution Service:  {}\nPlanning Service:   {}\nGovernance Service: {}\nEvolution Service:  {}\nIdentity Service:   {}\nSecurity Service:   {}",
        if state.service_count > 0 { "ONLINE" } else { "PENDING" },
        if state.service_count > 1 { "ONLINE" } else { "PENDING" },
        "PENDING",
        "PENDING",
        "PENDING",
        "PENDING",
        "PENDING",
    );
    let svc_block = Paragraph::new(services)
        .style(theme::lavender_style())
        .block(
            Block::default()
                .title("SERVICE REGISTRY")
                .borders(Borders::ALL)
                .border_style(theme::pink_style()),
        );
    frame.render_widget(svc_block, chunks[0]);

    // Runtime info + status
    let rt = format!(
        "PID:         {}\nUptime:      {}s\nPersonality: {}\nServices:    {} registered\nLeases:      {} active\nLoops:       {} running\nPolicies:    {} active\nView:        /{}",
        std::process::id(),
        state.uptime_secs,
        state.personality,
        state.service_count,
        state.active_leases,
        state.active_loops,
        state.policy_count,
        match state.view {
            View::Dashboard => "dashboard",
            _ => "?",
        },
    );
    let rt_block = Paragraph::new(rt).style(theme::lavender_style()).block(
        Block::default()
            .title("RUNTIME")
            .borders(Borders::ALL)
            .border_style(theme::purple_style()),
    );
    frame.render_widget(rt_block, chunks[1]);
}

fn render_prompt(frame: &mut Frame, area: Rect, state: &AppState) {
    let prompt_text = if state.show_help {
        " Type /help for commands. Use TAB to autocomplete. ESC to close. Q to quit.".to_string()
    } else {
        format!(" PINK_GRID_> {}", state.input)
    };

    let prompt = Paragraph::new(prompt_text)
        .style(Style::default().fg(theme::PINK).bg(theme::BG))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme::border_style()),
        );
    frame.render_widget(prompt, area);
}

fn render_help(frame: &mut Frame, area: Rect) {
    use crate::command::help_text;
    let items: Vec<ListItem> = help_text()
        .iter()
        .map(|(cmd, desc)| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!(" {:15}", cmd),
                    theme::pink_style().add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("  {}", desc), theme::lavender_style()),
            ]))
        })
        .collect();

    let help_list = List::new(items).block(
        Block::default()
            .title("COMMANDS")
            .borders(Borders::ALL)
            .border_style(theme::gold_style()),
    );
    frame.render_widget(help_list, area);
}
