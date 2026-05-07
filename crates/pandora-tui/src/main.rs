
use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{
        self,
        Event,
        KeyCode,
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{
        Constraint,
        Direction,
        Layout,
    },
    style::{
        Color,
        Modifier,
        Style,
    },
    text::{
        Line,
        Span,
    },
    widgets::{
        Block,
        Borders,
        Gauge,
        Paragraph,
        Wrap,
    },
    Terminal,
};

use pandora_gene::load_genes;

struct RuntimeState {

    active_gene: String,

    generation: usize,

    avg_score: f32,

    total_runs: usize,

    mutation_count: usize,

    memory_syncs: usize,

    active_model: String,

    active_harness: String,

    tools: Vec<String>,

    systems: Vec<String>,

    event_log: Vec<String>,
}

fn main() -> Result<(), io::Error> {

    let genes =
        load_genes("genes");

    let gene =
        genes
            .iter()
            .max_by(
                |a, b| {
                    a.avg_score
                        .partial_cmp(&b.avg_score)
                        .unwrap()
                }
            )
            .unwrap();

    let state =
        RuntimeState {

            active_gene:
                gene.gene_id.clone(),

            generation:
                gene.generation,

            avg_score:
                gene.avg_score,

            total_runs:
                gene.total_runs,

            mutation_count:
                3,

            memory_syncs:
                12,

            active_model:
                "qwen2.5-coder:7b"
                    .to_string(),

            active_harness:
                "coding"
                    .to_string(),

            tools: vec![

                "read_file".to_string(),
                "write_file".to_string(),
                "call_model".to_string(),
                "mutation_engine".to_string(),
                "telemetry".to_string(),
                "runtime_events".to_string(),
            ],

            systems: vec![

                "ANUBIS".to_string(),
                "KETHER".to_string(),
                "PANOPTES".to_string(),
                "KUBER".to_string(),
                "MOLOCH".to_string(),
            ],

            event_log: vec![

                "[BOOT] PANDORA runtime initialized"
                    .to_string(),

                format!(
                    "[GENE] Loaded {}",
                    gene.gene_id
                ),

                format!(
                    "[MODEL] Selected {}",
                    "qwen2.5-coder:7b"
                ),

                "[MEMORY] ANUBIS sync complete"
                    .to_string(),

                "[RUNTIME] Harness online"
                    .to_string(),

                "[PANOPTES] Telemetry active"
                    .to_string(),
            ],
        };

    let start_time =
        Instant::now();

    let mut scroll_offset: usize = 0;

    enable_raw_mode()?;

    let mut stdout =
        io::stdout();

    execute!(
        stdout,
        EnterAlternateScreen
    )?;

    let backend =
        CrosstermBackend::new(stdout);

    let mut terminal =
        Terminal::new(backend)?;

    loop {

        let uptime =
            start_time
                .elapsed()
                .as_secs();

        terminal.draw(|f| {

            let layout =
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(10),
                        Constraint::Length(3),
                    ])
                    .split(f.size());

            let top =
                layout[0];

            let middle =
                layout[1];

            let bottom =
                layout[2];

            let chunks =
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(28),
                        Constraint::Percentage(44),
                        Constraint::Percentage(28),
                    ])
                    .split(middle);

            let banner =
                Paragraph::new(
                    Line::from(vec![

                        Span::styled(
                            " PANDORA ",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(
                                    Modifier::BOLD
                                )
                        ),

                        Span::styled(
                            "SYSTEMS",
                            Style::default()
                                .fg(Color::Magenta)
                                .add_modifier(
                                    Modifier::BOLD
                                )
                        ),
                    ])
                )
                .style(
                    Style::default()
                        .bg(Color::Black)
                )
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("RUNTIME")
                        .border_style(
                            Style::default()
                                .fg(Color::Magenta)
                        )
                );

            let left_chunks =
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(14),
                        Constraint::Length(5),
                        Constraint::Min(5),
                    ])
                    .split(chunks[0]);

            let gene_panel =
                Paragraph::new(
                    format!(
                        "ACTIVE GENE\n{}\n\nGENERATION: {}\nAVG SCORE: {:.2}\nRUNS: {}\n\nMUTATIONS: {}\nMEMORY SYNCS: {}",
                        state.active_gene,
                        state.generation,
                        state.avg_score,
                        state.total_runs,
                        state.mutation_count,
                        state.memory_syncs
                    )
                )
                .block(
                    Block::default()
                        .title("GENE")
                        .borders(Borders::ALL)
                        .border_style(
                            Style::default()
                                .fg(Color::Magenta)
                        )
                );

            let score_ratio =
                (state.avg_score / 2.0)
                    .clamp(0.0, 1.0);

            let score_gauge =
                Gauge::default()
                    .block(
                        Block::default()
                            .title("PERFORMANCE")
                            .borders(Borders::ALL)
                            .border_style(
                                Style::default()
                                    .fg(Color::Yellow)
                            )
                    )
                    .gauge_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .bg(Color::Black)
                    )
                    .ratio(
                        score_ratio as f64
                    )
                    .label(
                        format!(
                            "{:.2}",
                            state.avg_score
                        )
                    );

            let tool_registry =
                Paragraph::new(
                    state
                        .tools
                        .iter()
                        .map(
                            |t| format!("• {}", t)
                        )
                        .collect::<Vec<String>>()
                        .join("\n")
                )
                .wrap(
                    Wrap { trim: true }
                )
                .block(
                    Block::default()
                        .title("TOOLS")
                        .borders(Borders::ALL)
                        .border_style(
                            Style::default()
                                .fg(Color::Cyan)
                        )
                );

            let mut dynamic_events =
                state.event_log.clone();

            dynamic_events.push(
                format!(
                    "[UPTIME] Runtime active for {}s",
                    uptime
                )
            );

            dynamic_events.push(
                format!(
                    "[EVENTS] {} runtime events processed",
                    dynamic_events.len()
                )
            );

            let visible_events =
                dynamic_events
                    .iter()
                    .skip(scroll_offset)
                    .cloned()
                    .collect::<Vec<String>>();

            let events =
                visible_events.join("\n");

            let center =
                Paragraph::new(events)
                    .wrap(
                        Wrap { trim: true }
                    )
                    .block(
                        Block::default()
                            .title("EVENT STREAM")
                            .borders(Borders::ALL)
                            .border_style(
                                Style::default()
                                    .fg(Color::Yellow)
                            )
                    );

            let registry_text =
                format!(
                    "SYSTEMS\n{}\n\nGENES\n• coding-v1\n• coding-v2\n• coding-v3\n• business-v1\n\nSTATUS\n• ONLINE",
                    state
                        .systems
                        .iter()
                        .map(
                            |s| format!("• {}", s)
                        )
                        .collect::<Vec<String>>()
                        .join("\n")
                );

            let right_chunks =
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(18),
                        Constraint::Min(5),
                    ])
                    .split(chunks[2]);

            let registry =
                Paragraph::new(
                    registry_text
                )
                .wrap(
                    Wrap { trim: true }
                )
                .block(
                    Block::default()
                        .title("REGISTRY")
                        .borders(Borders::ALL)
                        .border_style(
                            Style::default()
                                .fg(Color::Cyan)
                        )
                );

            let telemetry =
                Paragraph::new(
                    format!(
                        "MODEL\n{}\n\nHARNESS\n{}\n\nUPTIME\n{}s\n\nEVENTS\n{}\n\nMEMORY\nACTIVE",
                        state.active_model,
                        state.active_harness,
                        uptime,
                        dynamic_events.len()
                    )
                )
                .block(
                    Block::default()
                        .title("TELEMETRY")
                        .borders(Borders::ALL)
                        .border_style(
                            Style::default()
                                .fg(Color::Green)
                        )
                );

            let status =
                Paragraph::new(
                    format!(
                        "ANUBIS ACTIVE | QWEN ONLINE | HARNESS: CODING | EVENTS: {} | UPTIME: {}s | ↑↓ SCROLL LOGS | Q QUIT",
                        dynamic_events.len(),
                        uptime
                    )
                )
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("STATUS")
                        .border_style(
                            Style::default()
                                .fg(Color::Green)
                        )
                );

            f.render_widget(
                banner,
                top
            );

            f.render_widget(
                gene_panel,
                left_chunks[0]
            );

            f.render_widget(
                score_gauge,
                left_chunks[1]
            );

            f.render_widget(
                tool_registry,
                left_chunks[2]
            );

            f.render_widget(
                center,
                chunks[1]
            );

            f.render_widget(
                registry,
                right_chunks[0]
            );

            f.render_widget(
                telemetry,
                right_chunks[1]
            );

            f.render_widget(
                status,
                bottom
            );

        })?;

        if event::poll(
            Duration::from_millis(250)
        )? {

            if let Event::Key(key) =
                event::read()? {

                match key.code {

                    KeyCode::Char('q') => {
                        break;
                    }

                    KeyCode::Down => {
                        scroll_offset =
                            scroll_offset
                                .saturating_add(1);
                    }

                    KeyCode::Up => {
                        scroll_offset =
                            scroll_offset
                                .saturating_sub(1);
                    }

                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;

    terminal.show_cursor()?;

    Ok(())
}
