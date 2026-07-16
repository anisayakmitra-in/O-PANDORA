//! Pandora TUI — terminal dashboard for the Pandora runtime.
//!
//! Layout:
//! ┌──────────────────────────────────────────┐
//! │  PANDORA SYSTEMS — Governed Execution    │
//! ├──────────┬───────────────────────────────┤
//! │ Runtime  │  Genes    │ Harnesses         │
//! │ Sessions │  Providers│ Fleet             │
//! │ Plans    │  Packages │ Marketplace       │
//! └──────────┴───────────────────────────────┘

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};
use std::io;

const PANDORA_LOGO: &str = r#"
 ██████╗  █████╗ ███╗   ██╗██████╗  ██████╗ ██████╗  █████╗
 ██╔══██╗██╔══██╗████╗  ██║██╔══██╗██╔═══██╗██╔══██╗██╔══██╗
 ██████╔╝███████║██╔██╗ ██║██║  ██║██║   ██║██████╔╝███████║
 ██╔═══╝ ██╔══██║██║╚██╗██║██║  ██║██║   ██║██╔══██╗██╔══██║
 ██║     ██║  ██║██║ ╚████║██████╔╝╚██████╔╝██║  ██║██║  ██║
 ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═══╝╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
"#;



// Additional TUI views
#[allow(dead_code)]
fn render_runtime_view(f: &mut Frame, area: ratatui::layout::Rect) {
    let text = 
        "Runtime: v0.2.0\nStatus: OK\nUptime: since start\nSessions: 0\nWorkers: 0\nConnections: default"
    );
    let block = Paragraph::new(text)
        .block(Block::default().title(" Runtime ").borders(Borders::ALL));
    f.render_widget(block, area);
}

#[allow(dead_code)]
fn render_connections_view(f: &mut Frame, area: ratatui::layout::Rect) {
    let reg = pandora_types::connection_manager::ConnectionRegistry::load();
    let lines: String = reg.list().iter().map(|c| {
        format!("{}  {}  {}ms\n", c.name, c.health_status, c.latency_ms)
    }).collect();
    let block = Paragraph::new(if lines.is_empty() { "No connections".into() } else { lines })
        .block(Block::default().title(" Connections ").borders(Borders::ALL));
    f.render_widget(block, area);
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                return Ok(());
            }
        }
    }
}

fn ui(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(8), Constraint::Min(0)].as_ref())
        .split(f.area());

    // ── Header ──
    let logo_lines: Vec<Line> = PANDORA_LOGO
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            Line::from(Span::styled(
                l,
                Style::default().fg(Color::Rgb(255, 20, 147)),
            ))
        }) // deep pink
        .collect();
    let header = Paragraph::new(logo_lines)
        .block(Block::default().borders(Borders::NONE))
        .centered();
    f.render_widget(header, chunks[0]);

    // ── Tabs / Sections ──
    let tabs = [
        " Runtime ",
        " Genes ",
        " Harnesses ",
        " Providers ",
        " Plans ",
        " Palace ",
    ];
    let tab_widget = Tabs::new(tabs.iter().map(|t| Line::from(*t)).collect::<Vec<_>>())
        .block(Block::default().borders(Borders::BOTTOM))
        .select(0)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Rgb(255, 20, 147)));
    f.render_widget(tab_widget, chunks[1]);

    // ── Main dashboard area ──
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(chunks[1]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(main_chunks[1]);

    // Load runtime data
    let builtins = pandora_kuber::builtin::all();
    let council = pandora_shadow_council::ShadowCouncil::new();
    let summary = council.summary();

    // Column 1 — Runtime + Sessions
    let runtime_text = format!(
        "Runtime: v1.0\nBuilt-in genes: {}\nInstalled harnesses: {}\nSlash commands: {}\nSessions: 0",
        builtins.len(), summary.total_harnesses, summary.slash_commands
    );
    let runtime_block = Paragraph::new(runtime_text)
        .block(
            Block::default()
                .title(" Runtime ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .style(Style::default().fg(Color::Gray));
    f.render_widget(runtime_block, body[0]);

    // Column 2 — Genes + Providers
    let gene_list: String = builtins
        .iter()
        .take(12)
        .map(|g| format!("  {} — {}\n", g.id, g.description))
        .collect();
    let gene_block = Paragraph::new(gene_list)
        .block(
            Block::default()
                .title(" Built-in Genes ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .style(Style::default().fg(Color::Gray));
    f.render_widget(gene_block, body[1]);

    // Column 3 — Marketplace + Fleet
    let palace_text = format!(
        "KUBER Palace\nServer: {}\nFeatured packages: 5\nTrending (week): 5\n\nFleet\nWorkers: 0\nRemote: none",
        if std::env::var("PALACE_URL").is_ok() { "online" } else { "offline" }
    );
    let palace_block = Paragraph::new(palace_text)
        .block(
            Block::default()
                .title(" Marketplace ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .style(Style::default().fg(Color::Gray));
    f.render_widget(palace_block, body[2]);

    // Status bar
    let status = Line::from(vec![
        Span::styled(
            " PANDORA SYSTEMS ",
            Style::default().fg(Color::Rgb(255, 20, 147)),
        ),
        Span::raw(" │ "),
        Span::styled("Runtime: OK", Style::default().fg(Color::Green)),
        Span::raw(" │ q quit · esc exit"),
    ]);
    let status_block = Paragraph::new(status).block(Block::default().borders(Borders::TOP));
    f.render_widget(status_block, main_chunks[0]);
}


#[cfg(test)]
mod tui_tests {
    #[test]
    fn logo_ascii_valid() {
        let logo = crate::PANDORA_LOGO;
        assert!(logo.contains("PANDORA"));
        assert!(logo.len() > 100);
    }

    #[test]
    fn builtin_genes_exist() {
        let genes = pandora_kuber::builtin::all();
        assert!(!genes.is_empty());
        assert!(genes.iter().any(|g| g.id == "filesystem"));
    }
}
