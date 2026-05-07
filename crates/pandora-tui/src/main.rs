use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
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
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use pandora_gene::load_genes;

fn main() -> Result<(), io::Error> {

    let genes =
        load_genes("genes");

    let gene =
        genes.first().unwrap();

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

        terminal.draw(|f| {

            let chunks =
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                        Constraint::Percentage(30),
                    ])
                    .split(f.size());

            let left =
                Paragraph::new(
                    format!(
                        "ACTIVE GENE\n{}\n\nGENERATION: {}\nAVG SCORE: {}\nRUNS: {}",
                        gene.gene_id,
                        gene.generation,
                        gene.avg_score,
                        gene.total_runs
                    )
                )
                .block(
                    Block::default()
                        .title("GENE")
                        .borders(Borders::ALL)
                );

            let center =
                Paragraph::new(
                    format!(
                        "Mutation lineage active\nParent: {:?}\n\nHarness active\nMemory synced",
                        gene.parent_gene
                    )
                )
                .block(
                    Block::default()
                        .title("RUNTIME")
                        .borders(Borders::ALL)
                );

            let right =
                Paragraph::new(
                    "MODEL:\nqwen2.5-coder:7b\n\nHARNESS:\ncoding\n\nSTATUS:\nACTIVE"
                )
                .block(
                    Block::default()
                        .title("TELEMETRY")
                        .borders(Borders::ALL)
                );

            f.render_widget(
                left,
                chunks[0]
            );

            f.render_widget(
                center,
                chunks[1]
            );

            f.render_widget(
                right,
                chunks[2]
            );

        })?;

        if let Event::Key(key) =
            event::read()? {

            if key.code ==
                KeyCode::Char('q') {

                break;
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
