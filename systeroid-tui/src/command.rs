use termion::event::Key;

/// Possible application commands.
#[derive(Debug)]
pub enum Command {
    /// Update the input buffer.
    UpdateInput(char),
    /// Clear the input buffer.
    ClearInput(bool),
    /// Exit the application.
    Exit,
    /// Do nothing.
    None,
}

impl Command {
    /// Parses a command from the given key.
    pub fn parse(key: Key, input_mode: bool) -> Self {
        if input_mode {
            match key {
                Key::Char(c) => Command::UpdateInput(c),
                Key::Backspace => Command::ClearInput(false),
                Key::Esc => Command::ClearInput(true),
                _ => Command::None,
            }
        } else {
            match key {
                Key::Char(':') => Command::UpdateInput(' '),
                Key::Esc => Command::Exit,
                _ => Command::None,
            }
        }
    }
}
