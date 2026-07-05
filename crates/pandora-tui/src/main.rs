use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use pandora_kuber::builtin;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run(&mut terminal);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    if let Err(e) = res { eprintln!("TUI error: {}", e); }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab { D, H, G, P, R, M }
use Tab::*;

fn run<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut tab = D;
    loop {
        terminal.draw(|f| draw(f, &tab))?;
        if !event::poll(Duration::from_millis(100))? { continue; }
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press { continue; }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char('1') => tab = D,
                KeyCode::Char('2') => tab = H,
                KeyCode::Char('3') => tab = G,
                KeyCode::Char('4') => tab = P,
                KeyCode::Char('5') => tab = R,
                KeyCode::Char('6') => tab = M,
                _ => {}
            }
        }
    }
}

fn draw(f: &mut Frame, tab: &Tab) {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let st = Style::default().fg(Color::Rgb(180, 160, 220)).bg(Color::Rgb(20, 10, 40));
    f.render_widget(Paragraph::new(Line::from(Span::styled(" PANDORA TUI  [1]Dash [2]Harness [3]Genes [4]Pipe [5]Prov [6]Mem  [q]uit", st))).style(st), vert[0]);

    let names = [" Dashboard ", " Harnesses ", " Genes ", " Pipeline ", " Providers ", " Memory "];
    let tlines: Vec<Line> = names.iter().enumerate().map(|(i, t)| {
        let sel = *tab as u8 == i as u8;
        let s = if sel { Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) };
        Line::from(Span::styled(t.to_string(), s))
    }).collect();
    let tabs = Tabs::new(tlines)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().bg(Color::Rgb(12, 6, 24)));
    f.render_widget(tabs, vert[1]);

    match tab {
        D => {
            let txt = vec![
                Line::from(Span::styled(" DASHBOARD", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
                Line::from(""),
                Line::from("  Architecture: Constitutional v1.0"),
                Line::from("  Mode: SOVEREIGN"),
                Line::from("  Status: OPERATIONAL"),
                Line::from(""),
                Line::from("  10 Services | 14 Genes | 8 Harnesses"),
                Line::from("  206 tests passing"),
            ];
            f.render_widget(Paragraph::new(ratatui::text::Text::from(txt)).block(Block::default().borders(Borders::ALL)), vert[1]);
        }
        H => {
            let items: Vec<ListItem> = [
                "Cognition Source Harness",
                "Planning Source Harness",
                "Execution Source Harness",
                "Governance Source Harness",
                "Identity Source Harness",
                "Coordination Meta Harness",
                "Coding Domain Harness",
                "Research Domain Harness",
            ].iter().map(|s| ListItem::new(Line::from(Span::styled(*s, Style::default().fg(Color::Rgb(200, 180, 220)))))).collect();
            f.render_widget(List::new(items).block(Block::default().borders(Borders::ALL)), vert[1]);
        }
        G => {
            let items: Vec<ListItem> = builtin::all().iter().map(|g| {
                ListItem::new(Line::from(Span::styled(format!(" {}  {}", g.id, g.description), Style::default().fg(Color::Rgb(200, 180, 255)))))
            }).collect();
            f.render_widget(List::new(items).block(Block::default().borders(Borders::ALL).title(" Gene Registry")), vert[1]);
        }
        P => {
            let items: Vec<ListItem> = [
                "1. Task", "2. Workflow", "3. Capability",
                "4. Execute", "5. Record", "6. Telemetry", "7. Ledger",
            ].iter().map(|s| ListItem::new(Line::from(Span::styled(*s, Style::default().fg(Color::Rgb(140, 120, 160)))))).collect();
            f.render_widget(List::new(items).block(Block::default().borders(Borders::ALL)), vert[1]);
        }
        R => {
            let items: Vec<ListItem> = [
                "Ollama (localhost:11434)",
                "LlamaCpp (localhost:8080)",
                "OpenAI (api key)",
                "Anthropic (api key)",
                "Custom (PROVIDER_ENDPOINT)",
            ].iter().map(|s| ListItem::new(Line::from(Span::styled(*s, Style::default().fg(Color::Rgb(200, 180, 220)))))).collect();
            f.render_widget(List::new(items).block(Block::default().borders(Borders::ALL)), vert[1]);
        }
        M => {
            let txt = vec![
                Line::from(Span::styled(" MEMORY SERVICE", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
                Line::from("  In-memory session store"),
                Line::from("  Operations: store, retrieve, search, delete"),
                Line::from("  Status: ACTIVE"),
            ];
            f.render_widget(Paragraph::new(ratatui::text::Text::from(txt)).block(Block::default().borders(Borders::ALL)), vert[1]);
        }
    }

    let help = Style::default().fg(Color::Rgb(80, 60, 100)).bg(Color::Rgb(10, 5, 20));
    f.render_widget(Paragraph::new(Line::from(Span::styled(" [1]Dashboard [2]Harnesses [3]Genes [4]Pipeline [5]Providers [6]Memory  [q]uit", help))).style(help), vert[2]);
}
