#![allow(dead_code, unused_variables, unused_imports, clippy::collapsible_match)]
//! Pandora Terminal User Interface
//!
//! The TUI is a first-class runtime component, not a cosmetic layer.
//! Usage:
//!   pandora-tui                  Launch the operating dashboard
//!   pandora-tui --view models    Launch directly into Model Intelligence view
//!   pandora-tui --help           Show this help

mod app;
mod cat;
mod command;
mod theme;

use std::io;
use std::time::{Duration, Instant};

use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, Terminal};

use app::{render, AppState};
use command::Command;

#[derive(Parser)]
#[command(
    name = "pandora-tui",
    author = "Pandora Systems",
    version = "0.1.0",
    about = "Pandora constitutional cognition operating system dashboard",
    long_about = "Pandora TUI — a live operating system for constitutional cognition.\n\nSubscribes to runtime events through the Event Bus and renders live\nconstitutional state. No business logic lives inside widgets.\n\nSlash commands available inside the TUI: /help"
)]
struct TuiArgs {
    /// Initial view to open (dashboard, parliament, models, events, memory, etc.)
    #[arg(short, long, default_value = "dashboard")]
    view: String,

    /// Disable the runtime cat mascot
    #[arg(short = 'C', long)]
    no_cat: bool,

    /// Enable debug logging to stderr
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<(), io::Error> {
    let args = TuiArgs::parse();

    // Initialize
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut state = AppState::new();
    let start_time = Instant::now();
    let mut tick_count: u64 = 0;

    // Process CLI arguments
    if args.no_cat {
        state.cat.visible = false;
    }

    // Set initial view from --view argument
    let initial_view = args.view.to_lowercase();
    let view_cmd = Command::parse(&format!("/{}", initial_view));
    if let Some(cmd) = view_cmd {
        if let Some(view) = cmd.to_view() {
            state.view = view;
            state.push_event(format!("[TUI] Opened view: /{}", initial_view));
        }
    }

    state.push_event("[BOOT] TUI initialized. Type /help for commands.".to_string());

    // Main event loop
    loop {
        let uptime = start_time.elapsed().as_secs();
        state.uptime_secs = uptime;

        // Update cat physics every tick
        state.cat.update(uptime as f32);

        // Simulate some periodic events for demo purposes
        tick_count += 1;
        if tick_count.is_multiple_of(10) {
            state.cpu_usage = 20.0 + (uptime as f64 * 0.1).sin() * 15.0 + 15.0;
            state.memory_usage = 35.0 + (uptime as f64 * 0.05).cos() * 10.0 + 10.0;
            state.gpu_usage = 50.0 + (uptime as f64 * 0.08).sin() * 25.0 + 10.0;
        }

        // Render
        terminal.draw(|f| render(f, &state))?;

        // Handle input with timeout for smooth animation
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') if state.input.is_empty() => break,
                    KeyCode::Char('?') if state.input.is_empty() => {
                        state.show_help = !state.show_help;
                    }
                    KeyCode::Char(c) => {
                        state.input.push(c);
                    }
                    KeyCode::Backspace => {
                        state.input.pop();
                    }
                    KeyCode::Enter => {
                        let input = state.input.clone();
                        state.input.clear();

                        if input == "/quit" || input == "/q" {
                            break;
                        }

                        #[allow(clippy::collapsible_match)]
                        if let Some(cmd) = Command::parse(&input) {
                            if cmd.name == "help" || cmd.name == "h" {
                                state.show_help = !state.show_help;
                            } else if let Some(view) = cmd.to_view() {
                                state.view = view;
                                state.show_help = false;
                                state.push_event(format!("[CMD] Switched to /{}", cmd.name));
                            } else {
                                state.push_event(format!("[CMD] Unknown command: {}", input));
                            }
                        } else if !input.is_empty() {
                            state.push_event(format!("[INPUT] {}", input));
                        }
                    }
                    KeyCode::Esc => {
                        if state.show_help {
                            state.show_help = false;
                        } else if !state.input.is_empty() {
                            state.input.clear();
                        }
                    }
                    KeyCode::Tab => {
                        if state.input.starts_with('/') {
                            let partial = state.input.trim_start_matches('/');
                            for (cmd, _) in command::help_text() {
                                if cmd.trim_start_matches('/').starts_with(partial) {
                                    state.input = format!("/{}", cmd.trim_start_matches('/'));
                                    break;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
