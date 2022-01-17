use crate::command::Command;
use crate::error::Result;
use crate::options::CopyOption;
use crate::widgets::StatefulTable;
#[cfg(feature = "clipboard")]
use copypasta_ext::{display::DisplayServer, prelude::ClipboardProvider};
use std::str::FromStr;
use std::time::Instant;
use systeroid_core::sysctl::controller::Sysctl;
use systeroid_core::sysctl::parameter::Parameter;
use unicode_width::UnicodeWidthStr;

/// Duration of prompt messages.
const MESSAGE_DURATION: u128 = 1750;

/// Application controller.
pub struct App<'a> {
    /// Whether if the application is running.
    pub running: bool,
    /// Input buffer.
    pub input: Option<String>,
    /// Time tracker for measuring the time for clearing the input.
    pub input_time: Option<Instant>,
    /// Cursor position.
    pub input_cursor: u16,
    /// Whether if the search mode is enabled.
    pub search_mode: bool,
    /// Entries of the options menu.
    pub options: Option<StatefulTable<&'a str>>,
    /// List of sysctl parameters.
    pub parameter_list: StatefulTable<Parameter>,
    #[cfg(feature = "clipboard")]
    /// Clipboard context.
    clipboard: Option<Box<dyn ClipboardProvider>>,
    /// Sysctl controller.
    sysctl: &'a mut Sysctl,
}

impl<'a> App<'a> {
    /// Constructs a new instance.
    pub fn new(sysctl: &'a mut Sysctl) -> Self {
        let mut app = Self {
            running: true,
            input: None,
            input_time: None,
            input_cursor: 0,
            search_mode: false,
            options: None,
            parameter_list: StatefulTable::default(),
            #[cfg(feature = "clipboard")]
            clipboard: None,
            sysctl,
        };
        app.parameter_list.items = app.sysctl.parameters.clone();
        #[cfg(feature = "clipboard")]
        {
            app.clipboard = match DisplayServer::select().try_context() {
                None => {
                    app.input = Some(String::from(
                        "Failed to initialize clipboard, no suitable clipboard provider found",
                    ));
                    app.input_time = Some(Instant::now());
                    None
                }
                clipboard => clipboard,
            }
        }
        app
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

    /// Copies the selected entry to the clipboard.
    #[cfg(feature = "clipboard")]
    fn copy_to_clipboard(&mut self, copy_option: CopyOption) -> Result<()> {
        self.input = Some(if let Some(clipboard) = self.clipboard.as_mut() {
            if let Some(parameter) = self.parameter_list.selected() {
                match copy_option {
                    CopyOption::Name => clipboard.set_contents(parameter.name.clone()),
                    CopyOption::Value => clipboard.set_contents(parameter.value.clone()),
                    CopyOption::Documentation => {
                        clipboard.set_contents(parameter.get_documentation().unwrap_or_default())
                    }
                }
                .map_err(|e| crate::error::Error::ClipboardError(e.to_string()))?;
                String::from("Copied to clipboard!")
            } else {
                String::from("No parameter is selected")
            }
        } else {
            String::from("Clipboard is not initialized")
        });
        self.input_time = Some(Instant::now());
        Ok(())
    }

    /// Shows a message about clipboard being not enabled.
    #[cfg(not(feature = "clipboard"))]
    fn copy_to_clipboard(&mut self, _: CopyOption) -> Result<()> {
        self.input = Some(String::from("Clipboard support is not enabled"));
        self.input_time = Some(Instant::now());
        Ok(())
    }

    /// Runs the given command and updates the application.
    pub fn run_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Select => {
                if let Some(copy_option) = self
                    .options
                    .as_ref()
                    .and_then(|v| v.selected())
                    .and_then(|v| CopyOption::try_from(*v).ok())
                {
                    self.copy_to_clipboard(copy_option)?;
                }
                self.options = None;
            }
            Command::ScrollUp => {
                if let Some(options) = self.options.as_mut() {
                    options.previous();
                } else if !self.parameter_list.items.is_empty() {
                    self.parameter_list.previous();
                }
            }
            Command::ScrollDown => {
                if let Some(options) = self.options.as_mut() {
                    options.next();
                } else if !self.parameter_list.items.is_empty() {
                    self.parameter_list.next();
                }
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
                            input.insert(input.width() - self.input_cursor as usize, v);
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
            Command::ClearInput(remove_end) => {
                if self.input_time.is_some() {
                    return Ok(());
                } else if let Some(input) = self.input.as_mut() {
                    if remove_end {
                        self.input_cursor = self
                            .input_cursor
                            .checked_sub(1)
                            .unwrap_or(self.input_cursor);
                    }
                    if let Some(remove_index) =
                        input.width().checked_sub((self.input_cursor + 1).into())
                    {
                        input.remove(remove_index);
                    } else if input.is_empty() {
                        self.input = None;
                    }
                }
                if self.search_mode {
                    self.search();
                }
            }
            Command::MoveCursor(direction) => {
                if let Some(input) = &self.input {
                    if direction == 0 {
                        if let Some(cursor_position) = self.input_cursor.checked_sub(1) {
                            self.input_cursor = cursor_position as u16;
                        }
                    } else if self.input_cursor != input.width() as u16 {
                        self.input_cursor += direction as u16;
                    }
                }
            }
            Command::Copy => {
                if self.parameter_list.selected().is_some() {
                    let mut copy_options = CopyOption::variants().to_vec();
                    if self
                        .parameter_list
                        .selected()
                        .and_then(|parameter| parameter.get_documentation())
                        .is_none()
                    {
                        copy_options.retain(|v| v != &CopyOption::Documentation)
                    }
                    self.options = Some(StatefulTable::with_items(
                        copy_options.iter().map(|v| v.as_str()).collect(),
                    ));
                } else {
                    self.input = Some(String::from("No parameter is selected"));
                    self.input_time = Some(Instant::now());
                }
            }
            Command::Refresh => {
                self.input = None;
                self.sysctl.parameters = Sysctl::init(self.sysctl.config.clone())?.parameters;
                self.parameter_list.items.iter_mut().for_each(|parameter| {
                    if let Some(param) = self
                        .sysctl
                        .parameters
                        .iter()
                        .find(|param| param.name == parameter.name)
                    {
                        parameter.value = param.value.to_string();
                    }
                });
            }
            Command::Exit => {
                if self.input.is_some() {
                    self.input = None;
                    self.input_time = None;
                } else if self.options.is_some() {
                    self.options = None;
                } else {
                    self.running = false;
                }
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
