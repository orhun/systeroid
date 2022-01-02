use crate::command::Command;

/// Application controller.
#[derive(Debug)]
pub struct App {
    /// Whether if the application is running.
    pub running: bool,
    /// Input buffer.
    pub input: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            input: None,
        }
    }
}

impl App {
    /// Runs the given command and updates the application.
    pub fn run_command(&mut self, command: Command) {
        match command {
            Command::UpdateInput(v) => match self.input.as_mut() {
                Some(input) => {
                    input.push(v);
                }
                None => {
                    self.input = Some(String::new());
                }
            },
            Command::ClearInput(cancel) => {
                if cancel {
                    self.input = None
                } else if let Some(input) = self.input.as_mut() {
                    if input.pop().is_none() {
                        self.input = None;
                    }
                }
            }
            Command::Exit => {
                self.running = false;
            }
            Command::None => {}
        }
    }
}
