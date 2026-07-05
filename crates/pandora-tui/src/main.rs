mod app;
use app::{App, Tab};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    if let Err(e) = res { eprintln!("TUI error: {}", e); }
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>, app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if !event::poll(Duration::from_millis(100))? { continue; }
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press { continue; }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char(c) if c >= '1' && c <= '6' => {
                    let tabs = [Tab::Overview, Tab::Services, Tab::Harnesses,
                                Tab::Genes, Tab::Pipeline, Tab::Providers];
                    app.current_tab = tabs[(c as u8 - b'1') as usize];
                    app.list_selected = 0;
                }
                KeyCode::Down => {
                    if app.list_selected < app.list_len().saturating_sub(1) {
                        app.list_selected += 1;
                    }
                }
                KeyCode::Up => {
                    app.list_selected = app.list_selected.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let area = f.area();
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let status = format!(
        " Pandora v{} | {} services | {} genes | {} harnesses | [1-6] tabs [q]uit ",
        env!("CARGO_PKG_VERSION"), app.service_count,
        app.gene_count + app.builtin_genes, app.harness_count,
    );
    let sbar = Paragraph::new(Line::from(Span::styled(
        &status, Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 100)),
    ))).style(Style::default().bg(Color::Rgb(30, 30, 100)));
    f.render_widget(sbar, vert[0]);

    let tab_names = [" Overview ", " Services ", " Harnesses ", " Genes ", " Pipeline ", " Providers "];
    let tab_lines: Vec<Line> = tab_names.iter().copied().enumerate().map(|(i, t)| {
        let sel = i as u8 == app.current_tab as u8;
        let s = if sel { Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD) }
                else { Style::default().fg(Color::DarkGray) };
        Line::from(Span::styled(t, s))
    }).collect();
    let tabs = Tabs::new(tab_lines)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Rgb(60, 60, 100))))
        .style(Style::default().bg(Color::Rgb(15, 15, 30)));
    f.render_widget(tabs, vert[1]);

    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(vert[1]);

    let lbl = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 100)))
        .style(Style::default().bg(Color::Rgb(12, 12, 28)));
    f.render_widget(lbl, horiz[0]);
    let li = Layout::default().margin(1)
        .constraints([Constraint::Length(1), Constraint::Min(0)]).split(horiz[0]);
    let titles = [" System Overview", " Constitutional Services", " Harnesses",
               " Genes", " Execution Pipeline", " AI Providers"];
    f.render_widget(Paragraph::new(Line::from(Span::styled(
        titles[app.current_tab as usize],
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ))), li[0]);
    let li_items = app.list_items();
    let items: Vec<ListItem> = li_items.iter().enumerate().map(|(i, item)| {
        let sel = i == app.list_selected;
        let s = if sel { Style::default().fg(Color::Yellow).bg(Color::Rgb(40, 40, 80)) }
                else { Style::default().fg(Color::Rgb(200, 200, 220)) };
        ListItem::new(Text::from(Line::from(Span::raw(item)))).style(s)
    }).collect();
    f.render_widget(List::new(items), li[1]);

    let rbl = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 100)))
        .style(Style::default().bg(Color::Rgb(12, 12, 28)));
    f.render_widget(rbl, horiz[1]);
    let ri = Layout::default().margin(1)
        .constraints([Constraint::Length(1), Constraint::Min(0)]).split(horiz[1]);
    f.render_widget(Paragraph::new(Line::from(Span::styled(
        " Details", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ))), ri[0]);
    let det_text = app.detail_text();
    let det: Vec<Line> = det_text.iter().map(|s| {
        let st = if s.starts_with("  ") { Style::default().fg(Color::Rgb(160, 160, 180)) }
                 else { Style::default().fg(Color::Rgb(200, 200, 220)) };
        Line::from(Span::styled(s, st))
    }).collect();
    f.render_widget(Paragraph::new(Text::from(det)).wrap(Wrap { trim: false }), ri[1]);

    let help = Line::from(Span::styled(
        " [1]Overview [2]Services [3]Harnesses [4]Genes [5]Pipeline [6]Providers  [q]uit  [up/down]nav",
        Style::default().fg(Color::Rgb(100, 100, 140)).bg(Color::Rgb(10, 10, 20)),
    ));
    f.render_widget(Paragraph::new(help).style(Style::default().bg(Color::Rgb(10, 10, 20))), vert[2]);
}
