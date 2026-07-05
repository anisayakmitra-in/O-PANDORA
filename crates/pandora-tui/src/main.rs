use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Gauge, Wrap},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use pandora_kuber::builtin;
use pandora_shadow_council::ShadowCouncil;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Parliament, Services, Council, Harnesses, Genes,
    Execution, Providers, Telemetry, Kuber, Skills, Settings,
}

fn nav_items() -> &'static [(&'static str, Page)] {
    &[
        ("  Parliament", Page::Parliament),
        ("  Services", Page::Services),
        ("  Council", Page::Council),
        ("  Harnesses", Page::Harnesses),
        ("  Genes", Page::Genes),
        ("  Execution", Page::Execution),
        ("  Providers", Page::Providers),
        ("  Telemetry", Page::Telemetry),
        ("  KUBER", Page::Kuber),
        ("  Skills", Page::Skills),
    ]
}

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

fn run<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut page = Page::Parliament;
    let mut sel: usize = 0;
    loop {
        terminal.draw(|f| draw(f, &page, &sel))?;
        if !event::poll(Duration::from_millis(100))? { continue; }
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press { continue; }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Down => sel = sel.saturating_add(1).min(99),
                KeyCode::Up => sel = sel.saturating_sub(1),
                KeyCode::Enter | KeyCode::Right => {
                    let items = nav_items();
                    if sel < items.len() { page = items[sel].1; }
                }
                KeyCode::Left => {
                    let items = nav_items();
                    let idx = items.iter().position(|(_, p)| *p == page).unwrap_or(0);
                    if idx > 0 { page = items[idx - 1].1; }
                }
                KeyCode::Tab => {
                    let items = nav_items();
                    let idx = items.iter().position(|(_, p)| *p == page).unwrap_or(0);
                    page = items[(idx + 1) % items.len()].1;
                    sel = 0;
                }
                _ => {}
            }
        }
    }
}

fn sl(s: &str, fg: Color, bold: bool) -> Line<'static> {
    let mut st = Style::default().fg(fg);
    if bold { st = st.add_modifier(Modifier::BOLD); }
    Line::from(Span::styled(s.to_string(), st))
}

fn draw(f: &mut Frame, page: &Page, sel: &usize) {
    let area = f.area();

    // Hermes-style three-column layout
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Min(20), Constraint::Length(24)])
        .split(area);

    // ── Left sidebar (Hermes: sessions list) ──
    let left_bg = Style::default().bg(Color::Rgb(25, 8, 15));
    let left_border = Block::default()
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(Color::Rgb(80, 20, 45)));
    f.render_widget(left_border, cols[0]);

    let sidebar = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(cols[0]);

    // Pandora logo/title in sidebar
    f.render_widget(Paragraph::new(Line::from(Span::styled(
        " PANDORA",
        Style::default().fg(Color::Rgb(255, 150, 200)).add_modifier(Modifier::BOLD),
    ))).style(left_bg), sidebar[0]);
    f.render_widget(Paragraph::new(Line::from(Span::styled(
        " v0.2  Architecture",
        Style::default().fg(Color::Rgb(180, 120, 140)),
    ))).style(left_bg), sidebar[0]);

    // Navigation list (Hermes: Skills & Tools, Messaging, Artifacts)
    let items: Vec<ListItem> = nav_items().iter().map(|(name, p)| {
        let active = *p == *page;
        ListItem::new(Line::from(Span::styled(name.to_string(),
            if active { Style::default().fg(Color::Rgb(255, 255, 255)).bg(Color::Rgb(70, 18, 40)).add_modifier(Modifier::BOLD) }
            else { Style::default().fg(Color::Rgb(200, 150, 170)) }
        )))
    }).collect();
    f.render_widget(List::new(items).style(left_bg), sidebar[1]);

    // Bottom sidebar (Hermes: profiles)
    let sc = ShadowCouncil::new();
    let s = sc.summary();
    f.render_widget(Paragraph::new(vec![
        Line::from(Span::styled(" Status", Style::default().fg(Color::Rgb(180, 120, 140)))),
        Line::from(Span::styled(format!(" {} harnesses", s.total_harnesses), Style::default().fg(Color::Rgb(200, 150, 170)))),
        Line::from(Span::styled(format!(" {} genes", s.genes + builtin::all().len()), Style::default().fg(Color::Rgb(200, 150, 170)))),
    ]).style(left_bg), sidebar[2]);

    // ── Main content (Hermes: chat area) ──
    let main_border = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(80, 20, 45)));
    f.render_widget(main_border, cols[1]);

    let main_inner = Layout::default()
        .margin(1)
        .constraints([Constraint::Min(0)])
        .split(cols[1]);

    let lines = build_content(page, sel);
    f.render_widget(
        Paragraph::new(Text::from(lines))
            .wrap(Wrap { trim: false })
            .style(Style::default().bg(Color::Rgb(15, 5, 10))),
        main_inner[0],
    );

    // ── Right panel (Hermes: gateway, agents, cron) ──
    let right_bg = Style::default().bg(Color::Rgb(25, 8, 15));
    let right_border = Block::default()
        .borders(Borders::LEFT)
        .border_style(Style::default().fg(Color::Rgb(80, 20, 45)));
    f.render_widget(right_border, cols[2]);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(8), Constraint::Length(8), Constraint::Min(0)])
        .split(cols[2]);

    // Right panel: Services (Hermes: Gateway)
    f.render_widget(Paragraph::new(vec![
        Line::from(Span::styled(" Services", Style::default().fg(Color::Rgb(255, 150, 200)).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("  Memory", Style::default().fg(Color::Rgb(0, 200, 100)))),
        Line::from(Span::styled("  Planning", Style::default().fg(Color::Rgb(0, 200, 100)))),
        Line::from(Span::styled("  Execution", Style::default().fg(Color::Rgb(0, 200, 100)))),
        Line::from(Span::styled("  Governance", Style::default().fg(Color::Rgb(0, 200, 100)))),
        Line::from(Span::styled("  +6 more", Style::default().fg(Color::Rgb(180, 120, 140)))),
    ]).style(right_bg), right[0]);

    // Right panel: Harnesses (Hermes: Agents)
    f.render_widget(Paragraph::new(vec![
        Line::from(Span::styled(" Harnesses", Style::default().fg(Color::Rgb(255, 150, 200)).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled(format!("  Source: {}", s.source_count), Style::default().fg(Color::Rgb(200, 150, 170)))),
        Line::from(Span::styled(format!("  Meta: {}", s.meta_count), Style::default().fg(Color::Rgb(200, 150, 170)))),
        Line::from(Span::styled(format!("  Domain: {}", s.domain_count), Style::default().fg(Color::Rgb(200, 150, 170)))),
    ]).style(right_bg), right[1]);

    // Right panel: Runtime (Hermes: Cron)
    f.render_widget(Paragraph::new(vec![
        Line::from(Span::styled(" Runtime", Style::default().fg(Color::Rgb(255, 150, 200)).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("  Constitutional", Style::default().fg(Color::Rgb(0, 200, 100)))),
        Line::from(Span::styled("  Architecture v1.0", Style::default().fg(Color::Rgb(200, 150, 170)))),
        Line::from(Span::styled("  206 tests", Style::default().fg(Color::Rgb(200, 150, 170)))),
    ]).style(right_bg), right[2]);
}

fn build_content(page: &Page, sel: &usize) -> Vec<Line<'static>> {
    match page {
        Page::Parliament => vec![
            sl(" Parliament", Color::Rgb(255, 150, 200), true),
            sl(" Constitutional runtime layer", Color::Rgb(180, 120, 140), false),
            sl("", Color::White, false),
            sl(" ServiceRegistry   — service lifecycle management", Color::Rgb(230, 170, 190), false),
            sl(" ConstitutionEngine — policy evaluation", Color::Rgb(230, 170, 190), false),
            sl(" LeaseManager     — capability lease tracking", Color::Rgb(230, 170, 190), false),
            sl(" EventBus         — inter-service events", Color::Rgb(230, 170, 190), false),
            sl("", Color::White, false),
            sl(" Architecture Constitution v1.0", Color::Rgb(255, 200, 100), true),
            sl("", Color::White, false),
            sl("  Shadow Council -> Harnesses -> Genes -> Pipeline", Color::Rgb(180, 120, 140), false),
        ],
        Page::Services => {
            let names = ["Memory", "Planning", "Execution", "Governance", "Identity",
                         "Sandbox", "Workflow", "Scheduler", "Ledger", "Provider", "Telemetry"];
            let mut v = vec![
                sl(" Services", Color::Rgb(255, 150, 200), true),
                sl(" 10 constitutional services", Color::Rgb(180, 120, 140), false),
                sl("", Color::White, false),
            ];
            for n in &names { v.push(sl(&format!("  {}  Service", n), Color::Rgb(160, 200, 160), false)); }
            v.push(sl("", Color::White, false));
            v.push(sl(" All services have real implementations", Color::Rgb(180, 120, 140), false));
            v
        }
        Page::Council => {
            let sc = ShadowCouncil::new();
            let s = sc.summary();
            vec![
                sl(" Shadow Council", Color::Rgb(255, 150, 200), true),
                sl(" Lifecycle, routing, capability resolution", Color::Rgb(180, 120, 140), false),
                sl("", Color::White, false),
                sl(&format!(" Harnesses: {} total", s.total_harnesses), Color::Cyan, false),
                sl(&format!("  Source: {}  Meta: {}  Domain: {}", s.source_count, s.meta_count, s.domain_count), Color::Rgb(230, 170, 190), false),
                sl("", Color::White, false),
                sl(&format!(" Genes: {} installed, {} enabled", s.genes, s.genes_enabled), Color::Cyan, false),
                sl("", Color::White, false),
                sl(&format!(" Slash commands: {}  Capabilities: {}", s.slash_commands, s.capabilities), Color::Cyan, false),
                sl("", Color::White, false),
                sl(" Routing: first-register-wins", Color::Rgb(180, 120, 140), false),
            ]
        }
        Page::Harnesses => {
            let mut v = vec![
                sl(" Harnesses", Color::Rgb(255, 150, 200), true),
                sl(" Source | Meta | Domain", Color::Rgb(180, 120, 140), false),
                sl("", Color::White, false),
                sl(" Source (5)", Color::Cyan, true),
            ];
            for n in &["Memory", "Planning", "Execution", "Governance", "Identity"] {
                v.push(sl(&format!("    {} Source Harness", n), Color::Rgb(160, 200, 160), false));
            }
            v.push(sl("", Color::White, false));
            v.push(sl(" Meta (1)", Color::Cyan, true));
            v.push(sl("    Coordination Meta Harness", Color::Rgb(200, 180, 220), false));
            v.push(sl("", Color::White, false));
            v.push(sl(" Domain (2)", Color::Cyan, true));
            v.push(sl("    Coding Domain Harness", Color::Rgb(200, 180, 100), false));
            v.push(sl("    Research Domain Harness", Color::Rgb(200, 180, 100), false));
            v.push(sl("", Color::White, false));
            v.push(sl(" Source augments services | Meta coordinates", Color::Rgb(180, 120, 140), false));
            v.push(sl(" Domain packages experiences", Color::Rgb(180, 120, 140), false));
            v
        }
        Page::Genes => {
            let genes = builtin::all();
            let mut v = vec![
                sl(&format!(" Genes  ({} first-party)", genes.len()), Color::Rgb(255, 150, 200), true),
                sl("", Color::White, false),
            ];
            for g in &genes {
                let c = match g.kind.as_str() {
                    "Workflow" => Color::Rgb(200, 180, 100),
                    "MCP" => Color::Rgb(100, 200, 255),
                    "Benchmark" => Color::Rgb(255, 180, 100),
                    "Agent" => Color::Rgb(200, 150, 255),
                    _ => Color::Rgb(160, 200, 160),
                };
                v.push(sl(&format!("  {}  v{}  {}", g.id, g.version, g.description), c, false));
            }
            v.push(sl("", Color::White, false));
            v.push(sl(" pandora install <name>", Color::Rgb(180, 120, 140), false));
            v
        }
        Page::Execution => {
            let stages = [
                "1. TASK", "2. INSTRUCTION", "3. WORKFLOW", "4. CAPABILITY",
                "5. TARGET", "6. EXECUTE", "7. RECORD", "8. TELEMETRY", "9. LEDGER",
            ];
            let idx = *sel % stages.len();
            let mut v = vec![
                sl(" Execution Pipeline", Color::Rgb(255, 150, 200), true),
                sl(" 9 stages, selectable via [up/down]", Color::Rgb(180, 120, 140), false),
                sl("", Color::White, false),
            ];
            for (i, stage) in stages.iter().enumerate() {
                let c = if i == idx { Color::Rgb(255, 200, 0) } else if i == 5 { Color::Rgb(255, 180, 100) } else { Color::Rgb(200, 150, 170) };
                v.push(sl(&format!(" {}  {}", if i == idx { ">" } else { " " }, stage), c, i == idx));
            }
            v
        }
        Page::Providers => {
            let mut v = vec![
                sl(" Providers", Color::Rgb(255, 150, 200), true),
                sl(" Provider-agnostic execution", Color::Rgb(180, 120, 140), false),
                sl("", Color::White, false),
            ];
            for (name, ep) in &[("Ollama", "localhost:11434"), ("LlamaCpp", "localhost:8080"),
                ("LM Studio", "localhost:1234"), ("vLLM", "localhost:8000"),
                ("OpenAI", "api.openai.com"), ("Anthropic", "api.anthropic.com"),
                ("Groq", "api.groq.com"), ("OpenRouter", "openrouter.ai"),
                ("Custom", "PROVIDER_ENDPOINT")] {
                v.push(sl(&format!("  {}  {}", name, ep), Color::Rgb(200, 150, 170), false));
            }
            v
        }
        Page::Telemetry => vec![
            sl(" Telemetry", Color::Rgb(255, 150, 200), true),
            sl(" Execution observability", Color::Rgb(180, 120, 140), false),
            sl("", Color::White, false),
            sl("  Trace  — full execution trace", Color::Cyan, false),
            sl("  Span   — operation timing", Color::Cyan, false),
            sl("  Events — state transitions", Color::Cyan, false),
            sl("  Errors — captured failures", Color::Cyan, false),
            sl("", Color::White, false),
            sl(" TelemetryEngine + Recorder + Ledger", Color::Rgb(200, 150, 170), false),
            sl(" Session -> Trace -> Spans -> Ledger", Color::Rgb(200, 150, 170), false),
        ],
        Page::Kuber => vec![
            sl(" KUBER", Color::Rgb(255, 150, 200), true),
            sl(" Package distribution", Color::Rgb(180, 120, 140), false),
            sl("", Color::White, false),
            sl(&format!(" Built-in: {} packages", builtin::all().len()), Color::Cyan, false),
            sl(" Install: pandora install <id>", Color::Rgb(200, 150, 170), false),
            sl(" Search: pandora search <query>", Color::Rgb(200, 150, 170), false),
            sl(" Scoring: 8 dimensions", Color::Rgb(200, 150, 170), false),
        ],
        Page::Skills => vec![
            sl(" Skills", Color::Rgb(255, 150, 200), true),
            sl(" Declarative gene/harness bundles", Color::Rgb(180, 120, 140), false),
            sl("", Color::White, false),
            sl(" pandora new skill <name>", Color::Rgb(200, 150, 170), false),
            sl(" Creates skill.toml + template", Color::Rgb(200, 150, 170), false),
        ],
        Page::Settings => vec![
            sl(" Settings", Color::Rgb(255, 150, 200), true),
            sl("", Color::White, false),
            sl(" OLLAMA_HOST     — Ollama endpoint", Color::Rgb(200, 150, 170), false),
            sl(" LLAMA_CPP_HOST  — LlamaCpp endpoint", Color::Rgb(200, 150, 170), false),
            sl(" PROVIDER_ENDPOINT — Custom API", Color::Rgb(200, 150, 170), false),
            sl(" PROVIDER_API_KEY  — Bearer token", Color::Rgb(200, 150, 170), false),
            sl("", Color::White, false),
            sl(" [up/down] navigate  [Enter] select", Color::Rgb(180, 120, 140), false),
            sl(" [Left/Right] prev/next page  [Tab] cycle", Color::Rgb(180, 120, 140), false),
            sl(" [q/Esc] quit", Color::Rgb(180, 120, 140), false),
        ],
    }
}
