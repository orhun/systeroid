use std::str::FromStr;
use termion::event::Key;

/// Possible application commands.
#[derive(Debug)]
pub enum Command {
    /// Perform an action based on the selected entry.
    Select,
    /// Set the value of a parameter.
    Set(String, String),
    /// Scroll up on the widget.
    ScrollUp,
    /// Scroll down on the widget.
    ScrollDown,
    /// Move cursor to right/left.
    MoveCursor(u8),
    /// Enable the search mode.
    EnableSearch,
    /// Process the input.
    ProcessInput,
    /// Update the input buffer.
    UpdateInput(char),
    /// Clear the input buffer.
    ClearInput(bool),
    /// Copy selected value to clipboard.
    Copy,
    /// Refresh the application.
    Refresh,
    /// Exit the application.
    Exit,
    /// Do nothing.
    None,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "search" => Ok(Command::EnableSearch),
            "select" => Ok(Command::Select),
            "up" => Ok(Command::ScrollUp),
            "down" => Ok(Command::ScrollDown),
            "copy" => Ok(Command::Copy),
            "refresh" => Ok(Command::Refresh),
            "exit" | "quit" | "q" | "q!" => Ok(Command::Exit),
            _ => {
                if s.starts_with("set") {
                    let mut values = s.trim_start_matches("set").trim().split_whitespace();
                    Ok(Command::Set(
                        values.next().ok_or(())?.to_string(),
                        values.next().ok_or(())?.to_string(),
                    ))
                } else {
                    Err(())
                }
            }
        }
    }
}

impl Command {
    /// Parses a command from the given key.
    pub fn parse(key: Key, input_mode: bool) -> Self {
        if input_mode {
            match key {
                Key::Char('\n') => Command::ProcessInput,
                Key::Char(c) => Command::UpdateInput(c),
                Key::Backspace => Command::ClearInput(false),
                Key::Delete => Command::ClearInput(true),
                Key::Left => Command::MoveCursor(1),
                Key::Right => Command::MoveCursor(0),
                Key::Esc => Command::Exit,
                _ => Command::None,
            }
        } else {
            match key {
                Key::Up => Command::ScrollUp,
                Key::Down => Command::ScrollDown,
                Key::Char(':') => Command::UpdateInput(' '),
                Key::Char('/') => Command::EnableSearch,
                Key::Char('\n') => Command::Select,
                Key::Char('c') => Command::Copy,
                Key::Char('r') => Command::Refresh,
                Key::Esc => Command::Exit,
                _ => Command::None,
            }
        }
    }
}
