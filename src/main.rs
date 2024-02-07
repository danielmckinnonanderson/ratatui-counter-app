#![allow(dead_code)]
#![allow(unused_imports)]

use anyhow::{Context, Result};
use commands::AppCommand;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use lazy_static::lazy_static;
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph, Frame, Terminal};

mod commands;

pub const FRAME_RATE_MILLIS: u64 = 16;
lazy_static! {
    pub static ref FRAME_WAIT_DURATION: std::time::Duration =
        std::time::Duration::from_millis(FRAME_RATE_MILLIS);
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

    while !app_state.should_quit {
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

        // Draw the TUI for the current frame
        terminal.draw(|frame: &mut Frame| {
            frame.render_widget(
                Paragraph::new(format!("Counter: {}", app_state.counter)),
                frame.size(),
            );
        })?;
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

fn wait() -> Result<()> {
    crossterm::event::poll(*FRAME_WAIT_DURATION)?;
    Ok(())
}
