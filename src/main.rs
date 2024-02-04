#![allow(dead_code)]
#![allow(unused_imports)]

use anyhow::{Result, Context};
use crossterm::event::{
    Event,
    KeyEventKind, KeyCode,
};
use ratatui::{prelude::CrosstermBackend, Terminal, Frame, widgets::Paragraph};

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
    let mut app = AppState {
        counter: 0,
        should_quit: false,
    };


    loop {
        terminal.draw(|frame: &mut Frame| {
            frame.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), frame.size());
            update().map_
            match update() {

            }
        })?;

        // Poll for keyboard events

    }

    // Broke out of loop, initiate shutdown
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

pub enum AppCommand {
    Quit,
    Increment,
    Decrement
}

impl AppCommand {
    pub fn from_key(key: KeyCode) -> Option<Self> {
        match key {
            KeyCode::Char('j') => Some(Self::Increment),
            KeyCode::Char('k') => Some(Self::Decrement),
            KeyCode::Char('q') => Some(Self::Quit),
            _ =>  None
        }
    }
}

fn update(app_state: &mut AppState) -> Result<()> {

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

    let key_event_opt = poll_for_keypress().context("Could not poll for keypress.");

    let command_opt = if let Result::Ok(key_event) = key_event_opt {
        match key_event {
            Some(keycode) => Some(AppCommand::from_key(keycode)),
            None => None
        }
    } else { None };

    // If command is present, run it
    if let Some(command) = command_opt {
        // TODO - map over command_opt

        match command {
            AppCommand::Quit => {
                // FIXME
                Ok(())
            },
            AppCommand::Increment => {
                app_state.counter += 1;
                Ok(())
            },
            AppCommand::Decrement => {
                app_state.counter -=1;
                Ok(())
            }
        }
    } else { Ok(()) }
}

fn wait() -> Result<()> {
    if crossterm::event::poll(std::time::Duration::from_millis(250))? {
    }

    Ok(())
}


