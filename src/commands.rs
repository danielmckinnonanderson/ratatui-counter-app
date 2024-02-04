use anyhow::{Result, Context};
use crossterm::event::KeyCode;

use crate::AppState;


#[derive(Debug)]
pub enum AppCommand {
    Quit,
    Increment,
    Decrement
}

impl AppCommand {
    /// Returns a command if the given KeyCode corresponds to one
    pub fn from_key(key: KeyCode) -> Option<Self> {
        match key {
            KeyCode::Char('j') => Some(Self::Increment),
            KeyCode::Char('k') => Some(Self::Decrement),
            KeyCode::Char('q') => Some(Self::Quit),
            _ =>  None
        }
    }

    /// Apply this command to the app state
    pub fn run(&self, app_state: &mut AppState) -> Result<()> {
        match self {
            AppCommand::Quit => {
                AppCommand::quit(app_state);
                Ok(())
            },
            AppCommand::Increment => {
                AppCommand::increment_counter(app_state);
                Ok(())
            },
            AppCommand::Decrement => {
                AppCommand::decrement_counter(app_state);
                Ok(())
            },
        }
    }

    fn quit(app_state: &mut AppState) {
        app_state.should_quit = true;
    }

    fn increment_counter(app_state: &mut AppState) {
        app_state.counter += 1;
    }

    fn decrement_counter(app_state: &mut AppState) {
        app_state.counter -= 1;
    }
}

