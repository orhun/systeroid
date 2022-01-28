use crate::options::{Direction, ScrollArea};
use std::str::FromStr;
use termion::event::Key;

/// Possible application commands.
#[derive(Debug, PartialEq)]
pub enum Command {
    /// Perform an action based on the selected entry.
    Select,
    /// Set the value of a parameter.
    Set(String, String),
    /// Scroll the widget.
    Scroll(ScrollArea, Direction, u8),
    /// Move cursor to right/left.
    MoveCursor(u8),
    /// Enable the search mode.
    Search,
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
    Nothing,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "search" => Ok(Command::Search),
            "select" => Ok(Command::Select),
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
                } else if s.starts_with("scroll") {
                    let mut values = s.trim_start_matches("scroll").trim().split_whitespace();
                    Ok(Command::Scroll(
                        ScrollArea::try_from(values.next().ok_or(())?)?,
                        Direction::try_from(values.next().ok_or(())?)?,
                        values.next().and_then(|v| v.parse().ok()).unwrap_or(1),
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
                _ => Command::Nothing,
            }
        } else {
            match key {
                Key::Up => Command::Scroll(ScrollArea::List, Direction::Up, 1),
                Key::Down => Command::Scroll(ScrollArea::List, Direction::Down, 1),
                Key::PageUp => Command::Scroll(ScrollArea::List, Direction::Up, 4),
                Key::PageDown => Command::Scroll(ScrollArea::List, Direction::Down, 4),
                Key::Char('t') => Command::Scroll(ScrollArea::List, Direction::Top, 0),
                Key::Char('b') => Command::Scroll(ScrollArea::List, Direction::Bottom, 0),
                Key::Left => Command::Scroll(ScrollArea::Documentation, Direction::Up, 1),
                Key::Right => Command::Scroll(ScrollArea::Documentation, Direction::Down, 1),
                Key::Char('`') => Command::Scroll(ScrollArea::Section, Direction::Up, 1),
                Key::Char('\t') => Command::Scroll(ScrollArea::Section, Direction::Down, 1),
                Key::Char(':') => Command::UpdateInput(' '),
                Key::Char('/') => Command::Search,
                Key::Char('\n') => Command::Select,
                Key::Char('c') => Command::Copy,
                Key::Char('r') => Command::Refresh,
                Key::Esc => Command::Exit,
                _ => Command::Nothing,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_command_parser {
        (input_mode: $input_mode: expr,
         $($key: expr => $command: expr,)+
        ) => {
            $(assert_eq!($command, Command::parse($key, $input_mode)));+
        };
    }

    #[test]
    fn test_command() {
        for (command, value) in vec![
            (Command::Search, "search"),
            (Command::Select, "select"),
            (Command::Copy, "copy"),
            (Command::Refresh, "refresh"),
            (Command::Exit, "exit"),
            (
                Command::Set(String::from("a"), String::from("b")),
                "set a b",
            ),
            (
                Command::Scroll(ScrollArea::List, Direction::Up, 1),
                "scroll list up 1",
            ),
            (
                Command::Scroll(ScrollArea::Documentation, Direction::Down, 4),
                "scroll docs down 4",
            ),
        ] {
            assert_eq!(command, Command::from_str(value).unwrap());
        }
        assert!(Command::from_str("---").is_err());
        assert_command_parser! {
            input_mode: true,
            Key::Char('\n') => Command::ProcessInput,
            Key::Char('a') => Command::UpdateInput('a'),
            Key::Backspace => Command::ClearInput(false),
            Key::Delete => Command::ClearInput(true),
            Key::Left => Command::MoveCursor(1),
            Key::Right => Command::MoveCursor(0),
            Key::Esc => Command::Exit,
        }
        assert_command_parser! {
            input_mode: false,
            Key::Up => Command::Scroll(ScrollArea::List, Direction::Up, 1),
            Key::Down => Command::Scroll(ScrollArea::List, Direction::Down, 1),
            Key::PageUp => Command::Scroll(ScrollArea::List, Direction::Up, 4),
            Key::PageDown => Command::Scroll(ScrollArea::List, Direction::Down, 4),
            Key::Char('t') => Command::Scroll(ScrollArea::List, Direction::Top, 0),
            Key::Char('b') => Command::Scroll(ScrollArea::List, Direction::Bottom, 0),
            Key::Left => Command::Scroll(ScrollArea::Documentation, Direction::Up, 1),
            Key::Right => Command::Scroll(ScrollArea::Documentation, Direction::Down, 1),
            Key::Char('`') => Command::Scroll(ScrollArea::Section, Direction::Up, 1),
            Key::Char('\t') => Command::Scroll(ScrollArea::Section, Direction::Down, 1),
            Key::Char(':') => Command::UpdateInput(' '),
            Key::Char('/') => Command::Search,
            Key::Char('\n') => Command::Select,
            Key::Char('c') => Command::Copy,
            Key::Char('r') => Command::Refresh,
            Key::Esc => Command::Exit,
        }
        assert_eq!(Command::Nothing, Command::parse(Key::PageDown, true));
        assert_eq!(Command::Nothing, Command::parse(Key::Char('#'), false));
    }
}
