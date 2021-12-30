use crate::command::Command;

/// Application controller.
#[derive(Debug)]
pub struct App {
    /// Whether if the application is running.
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true }
    }
}

impl App {
    /// Runs the given command and updates the application.
    pub fn run_command(&mut self, command: Command) {
        match command {
            Command::Exit => {
                self.running = false;
            }
            Command::None => {}
        }
    }
}
