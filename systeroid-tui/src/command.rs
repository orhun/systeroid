use crate::options::{Direction, ScrollArea};
use std::str::FromStr;
use termion::event::Key;
use tui_logger::TuiWidgetEvent;

/// Possible logger widget commands.
#[derive(Debug, PartialEq)]
pub struct LoggerCommand(pub TuiWidgetEvent);

impl Eq for LoggerCommand {}

impl LoggerCommand {
    /// Parses a logger command from the given key.
    pub fn parse(key: Key) -> Option<Self> {
        match key {
            Key::Char(' ') => Some(Self(TuiWidgetEvent::SpaceKey)),
            Key::Esc => Some(Self(TuiWidgetEvent::EscapeKey)),
            Key::PageUp => Some(Self(TuiWidgetEvent::PrevPageKey)),
            Key::PageDown => Some(Self(TuiWidgetEvent::NextPageKey)),
            Key::Up => Some(Self(TuiWidgetEvent::UpKey)),
            Key::Down => Some(Self(TuiWidgetEvent::DownKey)),
            Key::Left => Some(Self(TuiWidgetEvent::LeftKey)),
            Key::Right => Some(Self(TuiWidgetEvent::RightKey)),
            Key::Char('+') => Some(Self(TuiWidgetEvent::PlusKey)),
            Key::Char('-') => Some(Self(TuiWidgetEvent::MinusKey)),
            Key::Char('h') => Some(Self(TuiWidgetEvent::HideKey)),
            Key::Char('f') => Some(Self(TuiWidgetEvent::FocusKey)),
            _ => None,
        }
    }
}

/// Possible application commands.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    /// Show help.
    Help,
    /// Show logs.
    Logs,
    /// Logger event.
    LoggerEvent(LoggerCommand),
    /// Perform an action based on the selected entry.
    Select,
    /// Save the value of a parameter to a file.
    Save,
    /// Set the value of a parameter.
    Set(String, String, bool),
    /// Scroll the widget.
    Scroll(ScrollArea, Direction, u8),
    /// Move cursor..
    MoveCursor(Direction),
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
    /// Cancel the operation.
    Cancel,
    /// Exit the application.
    Exit,
    /// Do nothing.
    Nothing,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "help" => Ok(Command::Help),
            "logs" => Ok(Command::Logs),
            "search" => Ok(Command::Search),
            "select" => Ok(Command::Select),
            "copy" => Ok(Command::Copy),
            "refresh" => Ok(Command::Refresh),
            "exit" | "quit" | "q" | "q!" => Ok(Command::Exit),
            _ => {
                if s.starts_with("set") || s.starts_with("save") {
                    let values: Vec<&str> = s
                        .trim_start_matches("set")
                        .trim_start_matches("save")
                        .split_whitespace()
                        .collect();
                    Ok(Command::Set(
                        values.first().ok_or(())?.to_string(),
                        values[1..].join(" "),
                        s.starts_with("save"),
                    ))
                } else if s.starts_with("scroll") {
                    let mut values = s.trim_start_matches("scroll").split_whitespace();
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
                Key::Left => Command::MoveCursor(Direction::Left),
                Key::Right => Command::MoveCursor(Direction::Right),
                Key::Esc => Command::Cancel,
                _ => Command::Nothing,
            }
        } else {
            match key {
                Key::Char('?') | Key::F(1) => Command::Help,
                Key::Ctrl('l') | Key::F(2) => Command::Logs,
                Key::Up | Key::Char('k') => Command::Scroll(ScrollArea::List, Direction::Up, 1),
                Key::Down | Key::Char('j') => Command::Scroll(ScrollArea::List, Direction::Down, 1),
                Key::PageUp => Command::Scroll(ScrollArea::List, Direction::Up, 4),
                Key::PageDown => Command::Scroll(ScrollArea::List, Direction::Down, 4),
                Key::Char('t') => Command::Scroll(ScrollArea::List, Direction::Top, 0),
                Key::Char('b') => Command::Scroll(ScrollArea::List, Direction::Bottom, 0),
                Key::Left | Key::Char('h') => {
                    Command::Scroll(ScrollArea::Documentation, Direction::Up, 1)
                }
                Key::Right | Key::Char('l') => {
                    Command::Scroll(ScrollArea::Documentation, Direction::Down, 1)
                }
                Key::Char('`') => Command::Scroll(ScrollArea::Section, Direction::Left, 1),
                Key::Char('\t') => Command::Scroll(ScrollArea::Section, Direction::Right, 1),
                Key::Char(':') => Command::UpdateInput(' '),
                Key::Char('s') => Command::Save,
                Key::Char('/') => Command::Search,
                Key::Char('\n') => Command::Select,
                Key::Char('c') => Command::Copy,
                Key::Char('r') | Key::F(5) => Command::Refresh,
                Key::Esc => Command::Cancel,
                Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('d') => Command::Exit,
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
            (Command::Help, "help"),
            (Command::Logs, "logs"),
            (Command::Search, "search"),
            (Command::Select, "select"),
            (Command::Copy, "copy"),
            (Command::Refresh, "refresh"),
            (Command::Exit, "quit"),
            (
                Command::Set(String::from("a"), String::from("b c"), false),
                "set a b c",
            ),
            (
                Command::Set(String::from("a"), String::from("b c"), true),
                "save a b c",
            ),
            (
                Command::Scroll(ScrollArea::List, Direction::Up, 1),
                "scroll list up 1",
            ),
            (
                Command::Scroll(ScrollArea::Documentation, Direction::Down, 4),
                "scroll docs down 4",
            ),
            (
                Command::Scroll(ScrollArea::Section, Direction::Top, 1),
                "scroll section top 1",
            ),
            (
                Command::Scroll(ScrollArea::List, Direction::Bottom, 1),
                "scroll list bottom 1",
            ),
        ] {
            assert_eq!(Ok(command), Command::from_str(value));
        }
        assert!(Command::from_str("---").is_err());
        assert_command_parser! {
            input_mode: true,
            Key::Char('\n') => Command::ProcessInput,
            Key::Char('a') => Command::UpdateInput('a'),
            Key::Backspace => Command::ClearInput(false),
            Key::Delete => Command::ClearInput(true),
            Key::Left => Command::MoveCursor(Direction::Left),
            Key::Right => Command::MoveCursor(Direction::Right),
            Key::Esc => Command::Cancel,
        }
        assert_command_parser! {
            input_mode: false,
            Key::Char('?') => Command::Help,
            Key::Ctrl('l') => Command::Logs,
            Key::Up => Command::Scroll(ScrollArea::List, Direction::Up, 1),
            Key::Down => Command::Scroll(ScrollArea::List, Direction::Down, 1),
            Key::PageUp => Command::Scroll(ScrollArea::List, Direction::Up, 4),
            Key::PageDown => Command::Scroll(ScrollArea::List, Direction::Down, 4),
            Key::Char('t') => Command::Scroll(ScrollArea::List, Direction::Top, 0),
            Key::Char('b') => Command::Scroll(ScrollArea::List, Direction::Bottom, 0),
            Key::Left => Command::Scroll(ScrollArea::Documentation, Direction::Up, 1),
            Key::Right => Command::Scroll(ScrollArea::Documentation, Direction::Down, 1),
            Key::Char('`') => Command::Scroll(ScrollArea::Section, Direction::Left, 1),
            Key::Char('\t') => Command::Scroll(ScrollArea::Section, Direction::Right, 1),
            Key::Char(':') => Command::UpdateInput(' '),
            Key::Char('/') => Command::Search,
            Key::Char('\n') => Command::Select,
            Key::Char('c') => Command::Copy,
            Key::Char('r') => Command::Refresh,
            Key::Esc => Command::Cancel,
            Key::Ctrl('c') => Command::Exit,
        }
        assert_eq!(Command::Nothing, Command::parse(Key::PageDown, true));
        assert_eq!(Command::Nothing, Command::parse(Key::Char('#'), false));
    }
}
