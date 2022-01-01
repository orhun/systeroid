//! systeroid-tui

#![warn(missing_docs, clippy::unwrap_used)]

/// Main application.
pub mod app;
/// Application commands.
pub mod command;
/// Error implementation.
pub mod error;
/// Event handling.
pub mod event;
/// User interface renderer.
pub mod ui;

use crate::app::App;
use crate::command::Command;
use crate::error::Result;
use crate::event::{Event, EventHandler};
use std::io::Write;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::terminal::Terminal;

/// Runs `systeroid-tui`.
pub fn run<Output: Write>(output: Output) -> Result<()> {
    let output = output.into_raw_mode()?;
    let output = MouseTerminal::from(output);
    let output = AlternateScreen::from(output);
    let backend = TermionBackend::new(output);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let event_handler = EventHandler::new(250);
    let mut app = App::default();
    while app.running {
        terminal.draw(|frame| ui::render(frame))?;
        match event_handler.next()? {
            Event::KeyPress(key) => {
                let command = Command::from(key);
                app.run_command(command);
            }
            Event::Tick => {}
        }
    }

    Ok(())
}
