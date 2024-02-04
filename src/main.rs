#![allow(dead_code)]
#![allow(unused_imports)]

use crossterm::event::{
    Event,
    KeyEventKind, KeyCode,
};
use ratatui::{prelude::CrosstermBackend, Terminal, Frame, widgets::Paragraph};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable raw mode
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal backend
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Define application state
    let mut ctr = 0;

    loop {
        terminal.draw(|frame: &mut Frame| {
            frame.render_widget(Paragraph::new(format!("Counter: {ctr}")), frame.size());
        })?;

        // Poll for keyboard events
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            // If a key occurs handle it
            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('j') => ctr += 1,
                        KeyCode::Char('k') => ctr -= 1,
                        KeyCode::Char('q') => break,
                        _ => {},
                    }
                }
            }
        }
    }

    // Broke out of loop, initiate shutdown
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

