use crate::command::Command;
use crate::widgets::StatefulList;
use std::str::FromStr;
use systeroid_core::sysctl::controller::Sysctl;
use systeroid_core::sysctl::parameter::Parameter;

/// Application controller.
#[derive(Debug)]
pub struct App<'a> {
    /// Whether if the application is running.
    pub running: bool,
    /// Input buffer.
    pub input: Option<String>,
    /// List of sysctl variables.
    pub variable_list: StatefulList<Parameter>,
    /// Sysctl controller.
    sysctl: &'a mut Sysctl,
}

impl<'a> App<'a> {
    /// Constructs a new instance.
    pub fn new(sysctl: &'a mut Sysctl) -> Self {
        Self {
            running: true,
            input: None,
            variable_list: StatefulList::with_items(sysctl.parameters.clone()),
            sysctl,
        }
    }

    /// Runs the given command and updates the application.
    pub fn run_command(&mut self, command: Command) {
        match command {
            Command::ScrollUp => {
                self.variable_list.previous();
            }
            Command::ScrollDown => {
                self.variable_list.next();
            }
            Command::ProcessInput => {
                if let Some(input) = &self.input {
                    if let Ok(command) = Command::from_str(input) {
                        self.run_command(command)
                    } else {
                        self.input = None;
                    }
                }
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
            Command::Refresh => {
                self.input = None;
                self.variable_list = StatefulList::with_items(self.sysctl.parameters.clone());
            }
            Command::Exit => {
                self.running = false;
            }
            Command::None => {}
        }
    }
}
