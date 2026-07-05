use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
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
    Parliament,
    Services,
    Council,
    Harnesses,
    Genes,
    Execution,
    Providers,
    Telemetry,
    Kuber,
    Skills,
    Settings,
}

fn nav_items() -> &'static [(&'static str, Page)] {
    &[
        (" Parliament", Page::Parliament),
        (" Services", Page::Services),
        (" Council", Page::Council),
        (" Harnesses", Page::Harnesses),
        (" Genes", Page::Genes),
        (" Execution", Page::Execution),
        (" Providers", Page::Providers),
        (" Telemetry", Page::Telemetry),
        (" KUBER", Page::Kuber),
        (" Skills", Page::Skills),
        (" Settings", Page::Settings),
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
                KeyCode::Char('1') => { page = Page::Parliament; sel = 0; }
                KeyCode::Char('2') => { page = Page::Services; sel = 0; }
                KeyCode::Char('3') => { page = Page::Council; sel = 0; }
                KeyCode::Char('4') => { page = Page::Harnesses; sel = 0; }
                KeyCode::Char('5') => { page = Page::Genes; sel = 0; }
                KeyCode::Char('6') => { page = Page::Execution; sel = 0; }
                KeyCode::Char('7') => { page = Page::Providers; sel = 0; }
                KeyCode::Char('8') => { page = Page::Telemetry; sel = 0; }
                KeyCode::Char('9') => { page = Page::Kuber; sel = 0; }
                KeyCode::Char('0') => { page = Page::Skills; sel = 0; }
                KeyCode::Down => sel = sel.saturating_add(1),
                KeyCode::Up => sel = sel.saturating_sub(1),
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
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    // Top bar
    let top_st = Style::default().fg(Color::Rgb(255, 200, 220)).bg(Color::Rgb(40, 10, 25));
    f.render_widget(Paragraph::new(Line::from(Span::styled(
        " PANDORA  v0.2  |  Architecture Control Plane  |  [1-0]nav [Tab]next [q]uit", top_st,
    ))).style(top_st), vert[0]);

    // Sidebar + content
    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)])
        .split(vert[1]);

    f.render_widget(Block::default().borders(Borders::RIGHT).border_style(Style::default().fg(Color::Rgb(80, 20, 45))), horiz[0]);

    let items: Vec<ListItem> = nav_items().iter().map(|(name, p)| {
        let active = *p == *page;
        ListItem::new(Line::from(Span::styled(name.to_string(),
            if active { Style::default().fg(Color::Yellow).bg(Color::Rgb(70, 18, 40)).add_modifier(Modifier::BOLD) }
            else { Style::default().fg(Color::Rgb(200, 150, 170)) }
        )))
    }).collect();
    f.render_widget(List::new(items), horiz[0]);

    // Content — build page text
    let lines: Vec<Line<'static>> = match page {
        Page::Parliament => {
            let sc = ShadowCouncil::new();
            let s = sc.summary();
            vec![
                sl(" PARLIAMENT", Color::Rgb(255, 150, 200), true),
                sl(" Constitutional runtime layer", Color::Rgb(200, 150, 170), false),
                sl("", Color::White, false),
                sl(" ServiceRegistry  — manages service lifecycle", Color::Rgb(230, 170, 190), false),
                sl(" ConstitutionEngine — policy evaluation", Color::Rgb(230, 170, 190), false),
                sl(" LeaseManager   — capability lease tracking", Color::Rgb(230, 170, 190), false),
                sl(" EventBus       — inter-service events", Color::Rgb(230, 170, 190), false),
                sl("", Color::White, false),
                sl(" Architecture Constitution v1.0", Color::Yellow, true),
                sl("", Color::White, false),
                sl(&format!(" Shadow Council: {} harnesses, {} genes", s.total_harnesses, s.genes), Color::Cyan, false),
                sl(&format!("  Source: {}  Meta: {}  Domain: {}", s.source_count, s.meta_count, s.domain_count), Color::Cyan, false),
                sl(&format!("  Slash commands: {}  Capabilities: {}", s.slash_commands, s.capabilities), Color::Cyan, false),
            ]
        }
        Page::Services => {
            let names = ["Memory", "Planning", "Execution", "Governance", "Identity",
                         "Sandbox", "Workflow", "Scheduler", "Ledger", "Provider", "Telemetry"];
            let mut v = vec![
                sl(" CONSTITUTIONAL SERVICES", Color::Rgb(255, 150, 200), true),
                sl(" All 10 services have real implementations", Color::Rgb(200, 150, 170), false),
                sl("", Color::White, false),
            ];
            for n in &names {
                let check = "\u{2713}";
                v.push(sl(&format!("  {}  {}  Service", check, n), Color::Rgb(160, 200, 160), false));
            }
            v.push(sl("", Color::White, false));
            v.push(sl(" Owner: Parliament", Color::Rgb(200, 150, 170), false));
            v
        }
        Page::Council => {
            let sc = ShadowCouncil::new();
            let s = sc.summary();
            vec![
                sl(" SHADOW COUNCIL", Color::Rgb(255, 150, 200), true),
                sl(" Lifecycle, routing, capability resolution", Color::Rgb(200, 150, 170), false),
                sl("", Color::White, false),
                sl(&format!(" Harnesses: {} total", s.total_harnesses), Color::Cyan, false),
                sl(&format!("  Source: {}  Meta: {}  Domain: {}", s.source_count, s.meta_count, s.domain_count), Color::Rgb(230, 170, 190), false),
                sl("", Color::White, false),
                sl(&format!(" Genes: {} installed, {} enabled", s.genes, s.genes_enabled), Color::Cyan, false),
                sl("", Color::White, false),
                sl(&format!(" Slash commands: {}  Capabilities: {}", s.slash_commands, s.capabilities), Color::Cyan, false),
                sl("", Color::White, false),
                sl(" Routing: first-register-wins for slash commands", Color::Rgb(200, 150, 170), false),
                sl(" Policy: capability-based resolution", Color::Rgb(200, 150, 170), false),
            ]
        }
        Page::Harnesses => {
            let h = [
                ("SOURCE (5)", &[
                    ("Memory Source Harness", ".memory.search, .memory.graph"),
                    ("Planning Source Harness", ".plan, .plan.review"),
                    ("Execution Source Harness", ".execute, .profile"),
                    ("Governance Source Harness", ".policy, .audit"),
                    ("Identity Source Harness", ".identity, .fork"),
                ][..]),
                ("META (1)", &[("Coordination Meta Harness", ".delegate, .route, .orchestrate")][..]),
                ("DOMAIN (2)", &[
                    ("Coding Domain Harness", ".build, .test, .lint, .review"),
                    ("Research Domain Harness", ".search, .extract, .summarize"),
                ][..]),
            ];
            let mut v = vec![sl(" HARNESSES", Color::Rgb(255, 150, 200), true), sl("", Color::White, false)];
            for (title, items) in &h {
                v.push(sl(title, Color::Cyan, true));
                for &(name, cmds) in *items {
                    v.push(sl(&format!("  {}  {}", name, cmds), Color::Rgb(220, 160, 180), false));
                }
                v.push(sl("", Color::White, false));
            }
            v.push(sl(" Source + Meta + Domain = complete harness model", Color::Rgb(200, 150, 170), false));
            v
        }
        Page::Genes => {
            let genes = builtin::all();
            let mut v = vec![
                sl(&format!(" GENE REGISTRY ({} first-party)", genes.len()), Color::Rgb(255, 150, 200), true),
                sl("", Color::White, false),
            ];
            for g in &genes {
                let kind_color = match g.kind.as_str() {
                    "Workflow" => Color::Rgb(200, 180, 100),
                    "MCP" => Color::Rgb(100, 200, 255),
                    "Benchmark" => Color::Rgb(255, 180, 100),
                    "Agent" => Color::Rgb(200, 150, 255),
                    _ => Color::Rgb(160, 200, 160),
                };
                v.push(sl(&format!("  {}  v{}  {}", g.id, g.version, g.description), kind_color, false));
                v.push(sl(&format!("      kind: {}  caps: {:?}", g.kind, g.capabilities), Color::Rgb(180, 120, 140), false));
            }
            v.push(sl("", Color::White, false));
            v.push(sl(" Install: pandora install <name>", Color::Rgb(200, 150, 170), false));
            v
        }
        Page::Execution => {
            let stages = [
                ("1. TASK", "Receive user request and parse intent"),
                ("2. INSTRUCTION", "Convert to structured instruction"),
                ("3. WORKFLOW", "Generate execution plan via Planning Service"),
                ("4. CAPABILITY", "Resolve required capabilities via Shadow Council"),
                ("5. TARGET", "Select execution target via policy"),
                ("6. EXECUTE", "Run via chosen provider"),
                ("7. RECORD", "Capture execution frame via Recorder"),
                ("8. TELEMETRY", "Trace and span via Telemetry Engine"),
                ("9. KNOWLEDGE", "Distill insights via Knowledge Engine"),
                ("10. LEDGER", "Persist outcome in Execution Ledger"),
            ];
            let idx = *sel % stages.len();
            let mut v = vec![
                sl(" EXECUTION PIPELINE (10 stages)", Color::Rgb(255, 150, 200), true),
                sl(" Every execution flows through all stages", Color::Rgb(200, 150, 170), false),
                sl("", Color::White, false),
            ];
            for (i, (name, desc)) in stages.iter().enumerate() {
                let c = if i == 5 { Color::Rgb(255, 200, 0) } else { Color::Rgb(140, 120, 160) };
                let arrow = if i == idx { " >" } else { "  " };
                v.push(sl(&format!("{} {}  {}", arrow, name, desc), c, i == idx));
            }
            v.push(sl("", Color::White, false));
            v.push(sl(" Active stage indicator via [up/down]", Color::Rgb(200, 150, 170), false));
            v
        }
        Page::Providers => {
            let provs = [
                ("Ollama", "localhost:11434", "OLLAMA_HOST", Color::Rgb(0, 200, 100)),
                ("LlamaCpp", "localhost:8080", "LLAMA_CPP_HOST", Color::Rgb(0, 200, 100)),
                ("LM Studio", "localhost:1234", "auto-detected", Color::Rgb(160, 200, 100)),
                ("vLLM", "localhost:8000", "auto-detected", Color::Rgb(160, 200, 100)),
                ("OpenAI", "api.openai.com", "API key", Color::Rgb(100, 160, 255)),
                ("Anthropic", "api.anthropic.com", "API key", Color::Rgb(255, 160, 100)),
                ("Groq", "api.groq.com", "API key", Color::Rgb(255, 200, 100)),
                ("OpenRouter", "openrouter.ai", "API key", Color::Rgb(200, 100, 255)),
                ("Custom", "PROVIDER_ENDPOINT", "Bearer token", Color::Rgb(255, 150, 200)),
            ];
            let mut v = vec![
                sl(" AI PROVIDERS", Color::Rgb(255, 150, 200), true),
                sl(" Provider-agnostic — models are interchangeable", Color::Rgb(200, 150, 170), false),
                sl("", Color::White, false),
            ];
            for (name, ep, cfg, clr) in &provs {
                v.push(sl(&format!("  {}  {}  ({})", name, ep, cfg), *clr, false));
            }
            v.push(sl("", Color::White, false));
            v.push(sl(" Resolution:", Color::Cyan, true));
            v.push(sl("  1. Check ExecutionTarget (hints + policy)", Color::Rgb(200, 150, 170), false));
            v.push(sl("  2. Fall back to env vars (OLLAMA_HOST, etc.)", Color::Rgb(200, 150, 170), false));
            v.push(sl("  3. Scan local discovery endpoints", Color::Rgb(200, 150, 170), false));
            v
        }
        Page::Telemetry => vec![
            sl(" TELEMETRY", Color::Rgb(255, 150, 200), true),
            sl(" Execution observability and tracing", Color::Rgb(200, 150, 170), false),
            sl("", Color::White, false),
            sl(" Trace  — full execution trace", Color::Cyan, false),
            sl(" Span   — individual operation timing", Color::Cyan, false),
            sl(" Events — state transitions and decisions", Color::Cyan, false),
            sl(" Timing — wall-clock + cpu per stage", Color::Cyan, false),
            sl(" Errors — captured failures with context", Color::Cyan, false),
            sl("", Color::White, false),
            sl(" Components:", Color::Cyan, true),
            sl("  TelemetryEngine — begin_trace / begin_span / add_span / end_trace", Color::Rgb(220, 160, 180), false),
            sl("  Recorder — captures ExecutionFrames", Color::Rgb(220, 160, 180), false),
            sl("  Ledger — records outcomes (Success/Failure)", Color::Rgb(220, 160, 180), false),
            sl("", Color::White, false),
            sl(" Session model:", Color::Cyan, true),
            sl("  Session -> Trace -> Spans -> Events -> Ledger", Color::Rgb(200, 150, 170), false),
            sl("  Sessions are replayable from the Ledger", Color::Rgb(200, 150, 170), false),
        ],
        Page::Kuber => vec![
            sl(" KUBER — Distribution", Color::Rgb(255, 150, 200), true),
            sl(" Package registry and installation system", Color::Rgb(200, 150, 170), false),
            sl("", Color::White, false),
            sl(&format!(" Built-in packages: {}", builtin::all().len()), Color::Cyan, false),
            sl(" Install: pandora install <id>", Color::Rgb(230, 170, 190), false),
            sl(" Search: pandora search <query>", Color::Rgb(230, 170, 190), false),
            sl(" Update: pandora update <id>", Color::Rgb(230, 170, 190), false),
            sl("", Color::White, false),
            sl(" Scoring: security, compatibility, capabilities,", Color::Rgb(200, 150, 170), false),
            sl("  dependencies, tests, governance, trust, performance", Color::Rgb(200, 150, 170), false),
            sl("", Color::White, false),
            sl(" Sources: local filesystem, remote URLs, built-in", Color::Rgb(200, 150, 170), false),
        ],
        Page::Skills => vec![
            sl(" SKILLS", Color::Rgb(255, 150, 200), true),
            sl(" Declarative bundles of genes and harnesses", Color::Rgb(200, 150, 170), false),
            sl("", Color::White, false),
            sl(" Install: pandora new skill <name>", Color::Rgb(200, 150, 170), false),
            sl(" Scaffold: creates skill.toml + template", Color::Rgb(200, 150, 170), false),
        ],
        Page::Settings => vec![
            sl(" SETTINGS", Color::Rgb(255, 150, 200), true),
            sl("", Color::White, false),
            sl(" Architecture: v1.0 (frozen)", Color::Cyan, false),
            sl(" Mode: SOVEREIGN", Color::Cyan, false),
            sl("", Color::White, false),
            sl(" Environment:", Color::Cyan, true),
            sl("  OLLAMA_HOST      — Ollama endpoint", Color::Rgb(230, 170, 190), false),
            sl("  LLAMA_CPP_HOST   — LlamaCpp endpoint", Color::Rgb(230, 170, 190), false),
            sl("  PROVIDER_ENDPOINT — Custom API endpoint", Color::Rgb(230, 170, 190), false),
            sl("  PROVIDER_API_KEY  — Bearer token", Color::Rgb(230, 170, 190), false),
            sl("  PANDORA_WEB_PORT — Web dashboard port", Color::Rgb(230, 170, 190), false),
            sl("", Color::White, false),
            sl(" Keyboard:", Color::Cyan, true),
            sl("  [1-0]   Navigation tabs", Color::Rgb(200, 150, 170), false),
            sl("  [Tab]   Next tab", Color::Rgb(200, 150, 170), false),
            sl("  [up/dn] Stage selector", Color::Rgb(200, 150, 170), false),
            sl("  [q/Esc] Quit", Color::Rgb(200, 150, 170), false),
        ],
    };

    f.render_widget(Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false }), horiz[1]);

    // Bottom help
    let bg = Style::default().fg(Color::Rgb(80, 60, 100)).bg(Color::Rgb(30, 8, 18));
    f.render_widget(Paragraph::new(Line::from(Span::styled(
        " [1]Parl [2]Svc [3]Council [4]Harness [5]Genes [6]Exec [7]Prov [8]Tel [9]KUBER [0]Skills [Tab]next [q]uit", bg,
    ))).style(bg), vert[2]);
}
