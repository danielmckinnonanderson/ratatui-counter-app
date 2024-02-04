#![allow(dead_code)]
#![allow(unused_imports)]

use anyhow::{Result, Context};
use commands::AppCommand;
use crossterm::event::{
    Event,
    KeyEventKind, KeyCode,
};
use lazy_static::lazy_static;
use ratatui::{prelude::CrosstermBackend, Terminal, Frame, widgets::Paragraph};

mod commands;

pub const FRAME_RATE_MILLIS: u64 = 16;
lazy_static! {
    pub static ref FRAME_WAIT_DURATION: std::time::Duration = std::time::Duration::from_millis(FRAME_RATE_MILLIS);
}


pub struct AppState {
    counter: i32,
    should_quit: bool,
}

fn main() -> Result<()> {
    // Enable raw mode
    crossterm::terminal::enable_raw_mode()
        .context("Failed to enable raw mode.")?;
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
        let command_opt: Option<AppCommand> = if let Result::Ok(key_event) = key_opt {
            match key_event {
                Some(keycode) => AppCommand::from_key(keycode),
                None => None
            }
        } else { None };

        // If the pressed key corresponds to a command, run the command.
        if let Some(command) = command_opt {
            // TODO - map over command_opt
            let updated = command.run(&mut app_state);
            match updated {
                Ok(_) => {},     // If all is good, move on and draw the TUI.
                Err(_) => break, // If we couldn't apply the command, stop due to error.
            };
        };

        // Draw the TUI for the current frame
        terminal.draw(|frame: &mut Frame| {
            println!("Closure");
            frame.render_widget(Paragraph::new(
                format!("Counter: {}", app_state.counter)
            ),
            frame.size());
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
                KeyCode::Char('j') => Ok(Some(KeyCode::Char('j'))),
                KeyCode::Char('k') => Ok(Some(KeyCode::Char('k'))),
                KeyCode::Char('q') => Ok(Some(KeyCode::Char('q'))),
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
    if crossterm::event::poll(*FRAME_WAIT_DURATION)? {

    }

    Ok(())
}

