#![allow(dead_code)]
#![allow(unused_imports)]

use std::time::Instant;

use anyhow::{Context, Result};
use commands::AppCommand;
use crossterm::{
    event::{Event, KeyCode, KeyEventKind},
    style::Colored, terminal::enable_raw_mode,
};
use lazy_static::lazy_static;
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

mod commands;

pub const FRAMES_PER_SECOND: u64 = 30;
lazy_static! {
    pub static ref FRAME_WAIT_DURATION: std::time::Duration =
        std::time::Duration::from_millis(FRAMES_PER_SECOND);
}

pub struct AppState {
    counter: i32,
    should_quit: bool,
}

fn main() -> Result<()> {
    // Enable raw mode
    crossterm::terminal::enable_raw_mode().context("Failed to enable raw mode.")?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)
        .context("Failed to enter alternate screen.")?;

    // Initialize the terminal backend
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))
        .context("Failed to start new Terminal with CrosstermBackend.")?;

    // Define application state
    let mut app_state = AppState {
        counter: 0,
        should_quit: false,
    };

    let mut last_frame_time = Instant::now();

    while !app_state.should_quit {
        // Draw the TUI for the current frame
        terminal.draw(|frame: &mut Frame| {
            // Draw a popup window
            let popup_area = centered_rect(frame.size(), 36, 36);

            frame.render_widget(
                Block::default().borders(Borders::all()).title("Main"),
                popup_area,
            )
        })?;

        // Check for keypress events.
        let key_opt = poll_for_keypress();
        let command_opt: Option<AppCommand> = key_opt
            .ok()
            .and_then(|key_event| key_event.and_then(AppCommand::from_key));

        // If the pressed key corresponds to a command, run the command.
        if let Some(command) = command_opt {
            if command.run(&mut app_state).is_err() {
                // If running the command returned an error, stop looping.
                break;
            }
        };

        // Sleep until the next frame
        let elapsed = Instant::now().duration_since(last_frame_time);

        // Update the last frame time now that we have evaluated time since last frame.
        last_frame_time = std::time::Instant::now();

        if elapsed < *FRAME_WAIT_DURATION {
            std::thread::sleep(*FRAME_WAIT_DURATION - elapsed);
        }
    }

    // Broke out of loop, initiate shutdown.
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

fn poll_for_keypress() -> Result<Option<KeyCode>> {
    if let Event::Key(key) = crossterm::event::read().context("Could not read event.")? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('j') | KeyCode::Char('k') | KeyCode::Char('q') => Ok(Some(key.code)),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn ui_yellow_span(_app_state: &AppState, frame: &mut Frame) {
    let span = "This text is yellow.".yellow();
    frame.render_widget(span, frame.size());
}

