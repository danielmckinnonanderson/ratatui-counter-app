#![allow(dead_code)]
#![allow(unused_imports)]

use anyhow::{Result, Context};
use commands::AppCommand;
use crossterm::event::{
    Event,
    KeyEventKind, KeyCode,
};
use ratatui::{prelude::CrosstermBackend, Terminal, Frame, widgets::Paragraph};

mod commands;

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
        terminal.draw(|frame: &mut Frame| {
            frame.render_widget(Paragraph::new(format!("Counter: {}", app_state.counter)), frame.size());
        })?;

        let key_opt = poll_for_keypress();

        let command_opt: Option<AppCommand> = if let Result::Ok(key_event) = key_opt {
            match key_event {
                Some(keycode) => AppCommand::from_key(keycode),
                None => None
            }
        } else { None };

        // If command is present, run it
        if let Some(command) = command_opt {
            // TODO - map over command_opt
            let updated = command.run(&mut app_state);
            match updated {
                Ok(_) => continue, // If all is good, move on to the next frame.
                Err(_) => break,   // If we couldn't apply the command, stop due to error.
            };
        };
    }

    // Broke out of loop, initiate shutdown.
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}


fn poll_for_keypress() -> Result<Option<KeyCode>> {
    if let Event::Key(key) = crossterm::event::read().context("Could not read event.")? {
        // TODO - Add handling for other kinds of events
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
    if crossterm::event::poll(std::time::Duration::from_millis(250))? {
    }

    Ok(())
}

