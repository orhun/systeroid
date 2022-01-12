use crate::command::Command;
use crate::error::Result;
use crate::widgets::StatefulTable;
use std::str::FromStr;
use std::time::Instant;
use systeroid_core::sysctl::controller::Sysctl;
use systeroid_core::sysctl::parameter::Parameter;

/// Duration of prompt messages.
const MESSAGE_DURATION: u128 = 1750;

/// Application controller.
#[derive(Debug)]
pub struct App<'a> {
    /// Whether if the application is running.
    pub running: bool,
    /// Input buffer.
    pub input: Option<String>,
    /// Time tracker for measuring the time for clearing the input.
    pub input_time: Option<Instant>,
    /// Whether if the search mode is enabled.
    pub search_mode: bool,
    /// List of sysctl parameters.
    pub parameter_list: StatefulTable<Parameter>,
    /// Sysctl controller.
    sysctl: &'a mut Sysctl,
}

impl<'a> App<'a> {
    /// Constructs a new instance.
    pub fn new(sysctl: &'a mut Sysctl) -> Self {
        Self {
            running: true,
            input: None,
            input_time: None,
            search_mode: false,
            parameter_list: StatefulTable::with_items(sysctl.parameters.clone()),
            sysctl,
        }
    }

    /// Returns true if the app is in input mode.
    pub fn is_input_mode(&self) -> bool {
        self.input.is_some() && self.input_time.is_none()
    }

    /// Performs a search operation in the kernel parameter list.
    fn search(&mut self) {
        if let Some(query) = &self.input {
            self.parameter_list.items = self
                .sysctl
                .parameters
                .clone()
                .into_iter()
                .filter(|param| param.name.contains(query))
                .collect();
            if self.parameter_list.items.is_empty() {
                self.parameter_list.state.select(None);
            } else {
                self.parameter_list.state.select(Some(0));
            }
        } else {
            self.parameter_list = StatefulTable::with_items(self.sysctl.parameters.clone());
        }
    }

    /// Runs the given command and updates the application.
    pub fn run_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::ScrollUp => {
                self.parameter_list.previous();
            }
            Command::ScrollDown => {
                self.parameter_list.next();
            }
            Command::EnableSearch => {
                if self.input_time.is_some() {
                    self.input_time = None;
                }
                self.search_mode = true;
                self.search();
                self.input = Some(String::new());
            }
            Command::ProcessInput => {
                if self.input_time.is_some() {
                    return Ok(());
                } else if self.search_mode {
                    self.input = None;
                    self.search_mode = false;
                } else if let Some(input) = &self.input {
                    if let Ok(command) = Command::from_str(input) {
                        self.run_command(command)?;
                    } else {
                        self.input = Some(String::from("Unknown command"));
                        self.input_time = Some(Instant::now());
                    }
                }
            }
            Command::UpdateInput(v) => {
                match self.input.as_mut() {
                    Some(input) => {
                        if self.input_time.is_some() {
                            self.input_time = None;
                            self.input = Some(String::new());
                        } else {
                            input.push(v);
                        }
                    }
                    None => {
                        self.input = Some(String::new());
                        self.search_mode = false;
                    }
                }
                if self.search_mode {
                    self.search();
                }
            }
            Command::ClearInput(cancel) => {
                if self.input_time.is_some() {
                    return Ok(());
                } else if cancel {
                    self.input = None
                } else if let Some(input) = self.input.as_mut() {
                    if input.pop().is_none() {
                        self.input = None;
                    }
                }
                if self.search_mode {
                    self.search();
                }
            }
            Command::Refresh => {
                self.input = None;
                let parameters = Sysctl::init(self.sysctl.config.clone())?.parameters;
                self.sysctl.parameters.iter_mut().for_each(|parameter| {
                    if let Some(param) =
                        parameters.iter().find(|param| param.name == parameter.name)
                    {
                        parameter.value = param.value.to_string();
                    }
                });
                self.parameter_list = StatefulTable::with_items(self.sysctl.parameters.clone());
            }
            Command::Exit => {
                self.running = false;
            }
            Command::None => {}
        }
        Ok(())
    }

    /// Handles the terminal tick event.
    pub fn tick(&mut self) {
        if let Some(instant) = self.input_time {
            if instant.elapsed().as_millis() > MESSAGE_DURATION {
                self.input = None;
                self.input_time = None;
            }
        }
    }
}
