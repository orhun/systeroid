use termion::event::Key;

/// Possible application commands.
#[derive(Debug)]
pub enum Command {
    /// Exit the application.
    Exit,
    /// Do nothing.
    None,
}

impl From<Key> for Command {
    fn from(key: Key) -> Self {
        match key {
            Key::Esc => Command::Exit,
            _ => Command::None,
        }
    }
}
