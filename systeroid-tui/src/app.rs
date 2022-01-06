use crate::command::Command;
use crate::widgets::StatefulList;

/// Application controller.
#[derive(Debug)]
pub struct App {
    /// Whether if the application is running.
    pub running: bool,
    /// Input buffer.
    pub input: Option<String>,
    /// List of sysctl variables.
    pub variable_list: StatefulList<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            input: None,
            variable_list: StatefulList::with_items(vec![
                String::from("data1"),
                String::from("data2"),
            ]),
        }
    }
}

impl App {
    /// Runs the given command and updates the application.
    pub fn run_command(&mut self, command: Command) {
        match command {
            Command::ScrollUp => {
                self.variable_list.previous();
            }
            Command::ScrollDown => {
                self.variable_list.next();
            }
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
