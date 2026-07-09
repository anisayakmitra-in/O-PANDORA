//! Pandora TUI — governed cognition runtime dashboard.
//!
//! Black/white aesthetic like btop/lazygit/k9s.
//! Architecture-visible design — teaches the structure.

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Gauge, Wrap, BorderType},
    Frame, Terminal,
};
use std::{io, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Dashboard,
    Sessions,
    Pipeline,
    Providers,
    Services,
    Harnesses,
    Genes,
    Packages,
    Governance,
    Telemetry,
    DecisionLog,
    Graph,
}

impl Page {
    fn all() -> &'static [Page] {
        &[
            Page::Dashboard,
            Page::Sessions,
            Page::Pipeline,
            Page::Providers,
            Page::Services,
            Page::Harnesses,
            Page::Genes,
            Page::Packages,
            Page::Governance,
            Page::Telemetry,
            Page::DecisionLog,
            Page::Graph,
        ]
    }
    fn label(&self) -> &'static str {
        match self {
            Page::Dashboard => "  Runtime  ",
            Page::Sessions => "  Sessions  ",
            Page::Pipeline => "  Pipeline  ",
            Page::Providers => "  Providers  ",
            Page::Services => "  Services  ",
            Page::Harnesses => "  Harnesses  ",
            Page::Genes => "  Genes  ",
            Page::Packages => "  Packages  ",
            Page::Governance => "  Governance  ",
            Page::Telemetry => "  Telemetry  ",
            Page::DecisionLog => "  Decisions  ",
            Page::Graph => "  Graph  ",
        }
    }
}

const WHITE: Color = Color::White;
const GRAY: Color = Color::DarkGray;
const GREEN: Color = Color::Green;
const RED: Color = Color::Red;
const BLACK: Color = Color::Black;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_page = Page::Dashboard;
    let mut tick = 0u64;

    let res = run(&mut terminal, &mut current_page, &mut tick);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    if let Err(e) = res { eprintln!("TUI error: {}", e); }
    Ok(())
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, page: &mut Page, tick: &mut u64) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw(f, *page, *tick))?;
        *tick += 1;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Left | KeyCode::Char('h') => {
                            let pages = Page::all();
                            let idx = pages.iter().position(|p| *p == *page).unwrap_or(0);
                            *page = pages[(idx + pages.len() - 1) % pages.len()];
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            let pages = Page::all();
                            let idx = pages.iter().position(|p| *p == *page).unwrap_or(0);
                            *page = pages[(idx + 1) % pages.len()];
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn draw(frame: &mut Frame, page: Page, _tick: u64) {
    let size = frame.size();
    if size.width < 80 || size.height < 20 { return; }

    // ── Layout ──
    // Top bar (1 row), main content, bottom status bar (1 row)
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(1)])
        .split(size);

    // Main content: sidebar + content
    let content = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Min(1)])
        .split(main[1]);

    // ── Top Bar ──
    let separator: String = std::iter::repeat("─").take(size.width as usize).collect();
    let top_text = format!(" PANDORA v1.0  │ Session: exec-24af31  │ Ollama  │ Single  │ Closed  │ L0  │ Auto ");
    let top = Paragraph::new(Line::from(Span::styled(top_text, Style::default().fg(GRAY))))
        .style(Style::default().bg(BLACK));
    frame.render_widget(top, main[0]);

    // ── Bottom Status Bar ──
    let bottom_text = "  Provider: Ollama  │ qwen2.5-coder:7b  │ Pipeline: Ready  │ Telemetry: ON  │ Sessions: Persistent  ";
    let bottom = Paragraph::new(Line::from(Span::styled(bottom_text, Style::default().fg(GRAY))))
        .style(Style::default().bg(BLACK));
    frame.render_widget(bottom, main[2]);

    // ── Sidebar (architecture navigation) ──
    let sidebar_items: Vec<ListItem> = Page::all().iter().map(|p| {
        let prefix = if *p == page { "▶ " } else { "  " };
        let style = if *p == page {
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(GRAY)
        };
        ListItem::new(format!("{}{}", prefix, p.label().trim())).style(style)
    }).collect();

    let sidebar = List::new(sidebar_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(GRAY))
            .border_type(BorderType::Plain))
        .style(Style::default().bg(BLACK).fg(WHITE));
    frame.render_widget(sidebar, content[0]);

    // ── Main Content ──
    render_page(frame, &page, content[1]);
}

fn render_page(frame: &mut Frame, page: &Page, area: Rect) {
    let text = match page {
        Page::Dashboard => render_dashboard(),
        Page::Sessions => render_sessions(),
        Page::Pipeline => render_pipeline(),
        Page::Providers => render_providers(),
        Page::Services => render_services(),
        Page::Harnesses => render_harnesses(),
        Page::Genes => render_genes(),
        Page::Packages => render_packages(),
        Page::Governance => render_governance(),
        Page::Telemetry => render_telemetry(),
        Page::DecisionLog => render_decisions(),
        Page::Graph => render_graph(),
    };

    let block = Block::default()
        .title(format!(" {} ", page.label().trim()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(GRAY))
        .border_type(BorderType::Plain);

    let p = Paragraph::new(text)
        .block(block)
        .style(Style::default().bg(BLACK).fg(WHITE))
        .wrap(Wrap { trim: false });
    frame.render_widget(p, area);
}

fn render_dashboard() -> Text<'static> {
    Text::from(
        "\n\
         Architecture: v1.0\n\n\
         Pipeline\n\
           Running:   1\n\
           Completed: 214\n\
           Failed:    3\n\n\
         Sessions\n\
           Stored:    215\n\n\
         Providers\n\
           Healthy:   3\n\n\
         Genes\n\
           Installed: 21\n\n\
         Harnesses\n\
           Loaded:    10\n\n\
         Packages\n\
           Installed: 17\n\n\
         Type /help for runtime commands.\n\
         ← → to navigate pages."
    )
}

fn render_pipeline() -> Text<'static> {
    Text::from(
        "\n\
         Execution Pipeline\n\n\
            ✓ Instruction\n\
            ✓ Workflow\n\
            ✓ Governance\n\
            ✓ Harness Dispatch\n\
            ✓ Capability Resolution\n\
            ► Provider Execution\n\
            □ Recorder\n\
            □ Telemetry\n\
            □ Knowledge\n\
            □ Ledger\n\n\
         Current stage: Provider Execution\n\
         Elapsed: 4.9s"
    )
}

fn render_sessions() -> Text<'static> {
    Text::from(
        "\n\
         Recent Sessions\n\n\
         ok exec-24af31   Implement JWT auth\n\
         ok exec-24af30   Refactor API routes\n\
         ok exec-24af29   Add database migrations\n\
         ok exec-24af28   Fix memory leak\n\
         ok exec-24af27   Design system tokens\n\
         ok exec-24af26   Security audit deps\n\
         ok exec-24af25   Benchmark providers\n\
         ok exec-24af24   Code review PR #42\n\
         ok exec-24af23   Generate API docs\n\
         ok exec-24af22   Run cargo-audit"
    )
}

fn render_providers() -> Text<'static> {
    Text::from(
        "\n\
         Name        Status   Models  Latency\n\
         ─────────────────────────────────────\n\
         Ollama      OK       12      5 ms\n\
         OpenAI      OK       GPT-5   140 ms\n\
         LlamaCpp    OFFLINE  0       --\n\
         LM Studio   OK       4       8 ms\n\n\
         Select: ← → providers   / to filter"
    )
}

fn render_services() -> Text<'static> {
    Text::from(
        "\n\
         Constitutional Services\n\n\
         Memory      DefaultMemoryService          OK\n\
         Planning    DefaultPlanningService         OK\n\
         Execution   DefaultExecutionService        OK\n\
         Governance  DefaultGovernanceService       OK\n\
         Identity    DefaultIdentityService         OK\n\
         Provider    DefaultProviderRegistrySvc     OK\n\
         Ledger      DefaultLedgerService           OK\n\
         Scheduler   DefaultSchedulerService         OK\n\
         Workflow    DefaultWorkflowService         OK"
    )
}

fn render_harnesses() -> Text<'static> {
    Text::from(
        "\n\
         Source (5)\n\
           Memory     ─ Augments Memory Service\n\
           Planning   ─ Augments Planning Service\n\
           Execution  ─ Augments Execution Service\n\
           Governance ─ Augments Governance Service\n\
           Identity   ─ Augments Identity Service\n\n\
         Meta (1)\n\
           Coordination ─ Coordinates services\n\n\
         Domain (4)\n\
           Coding     ─ Build, test, lint, review, simplify\n\
           Research   ─ Search and summarize\n\
           Security   ─ Audit, scan, secrets detection\n\
           Design     ─ UI/UX, animation, brand, review"
    )
}

fn render_genes() -> Text<'static> {
    Text::from(
        "\n\
         Built-in (21)\n\n\
         filesystem     git            http\n\
         shell          rust-tool      python-tool\n\
         workflow       docker         docker-compose\n\
         terraform      kubectl        browser\n\
         sqlite         github         mcp\n\
         code-review    benchmark      postgres\n\
         go             node           java\n\n\
         Use ← → to select a gene for details."
    )
}

fn render_packages() -> Text<'static> {
    Text::from(
        "\n\
         Installed Packages\n\n\
         filesystem     v0.1.0   Built-in\n\
         shell          v0.1.0   Built-in\n\
         git            v0.1.0   Built-in\n\
         ...\n\
         postgres       v0.1.0   Built-in\n\n\
         KUBER: 17 installed, 0 external"
    )
}

fn render_governance() -> Text<'static> {
    Text::from(
        "\n\
         Governance\n\n\
         Policy: default (allow all with configured providers)\n\n\
         Rules:\n\
           shell execution        → requires approval\n\
           file write             → requires approval\n\
           provider switch        → requires approval\n\
           sandbox violation      → blocks execution\n\n\
         Audit Log:\n\
           All decisions recorded in DecisionLog\n\
           Session persistence: enabled"
    )
}

fn render_telemetry() -> Text<'static> {
    Text::from(
        "\n\
         Telemetry\n\n\
         Current Session\n\
           Traces:      12 spans\n\
           Decision:    7 choices recorded\n\
           Duration:    4.9s\n\n\
         All Sessions\n\
           Total traces: 2,847\n\
           Avg latency:  1.2s\n\
           Error rate:   1.4%"
    )
}

fn render_decisions() -> Text<'static> {
    Text::from(
        "\n\
         Decision Log — exec-24af31\n\n\
         Stage 2 — Harness Dispatch\n\
           ✓ Coding\n\
             Reason: Domain capability match\n\
           ✗ Research\n\
             Reason: No matching capability\n\
           ✗ Security\n\
             Reason: No audit requested\n\n\
         ──────────────────────────────────────\n\n\
         Stage 3 — Provider Selection\n\
           ✓ Ollama\n\
             Reason: Lowest latency (4ms)\n\
           ✗ OpenAI\n\
             Reason: Offline policy\n\
           ✗ LlamaCpp\n\
             Reason: Model unavailable"
    )
}

fn render_graph() -> Text<'static> {
    Text::from(
        "\n\
         Execution Graph\n\n\
           Instruction\n\
             ↓\n\
           Workflow\n\
             ↓\n\
           Coding Harness\n\
             ↓\n\
           Rust Gene × Filesystem Gene\n\
             ↓\n\
           Ollama\n\
             ↓\n\
           Recorder → Telemetry → Knowledge\n\
             ↓\n\
           Ledger\n\
             ↓\n\
           Session"
    )
}
