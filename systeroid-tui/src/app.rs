use crate::command::Command;
use crate::error::Result;
use crate::options::{CopyOption, Direction, ScrollArea};
use crate::widgets::SelectableList;
#[cfg(feature = "clipboard")]
use copypasta_ext::{display::DisplayServer, prelude::ClipboardProvider};
use std::str::FromStr;
use std::time::Instant;
use systeroid_core::sysctl::controller::Sysctl;
use systeroid_core::sysctl::parameter::Parameter;
use systeroid_core::sysctl::section::Section;
use unicode_width::UnicodeWidthStr;

/// Representation of a key binding.
pub struct KeyBinding<'a> {
    /// Pressed key.
    pub key: &'a str,
    /// Action to perform.
    pub action: &'a str,
}

/// Help text to show.
pub const HELP_TEXT: &str = concat!(
    "\u{2800} _    __/_ _   '_/\n",
    "_) (/_) /(-/ ()/(/\n",
    "/           \u{2800}\n",
    env!("CARGO_PKG_NAME"),
    " v",
    env!("CARGO_PKG_VERSION"),
    "\n",
    env!("CARGO_PKG_REPOSITORY"),
    "\nwritten by ",
    env!("CARGO_PKG_AUTHORS"),
);

/// Key bindings of the application.
pub const KEY_BINDINGS: &[&KeyBinding] = &[
    &KeyBinding {
        key: "[?], f1",
        action: "show help",
    },
    &KeyBinding {
        key: "up/down, k/j, pgup/pgdown",
        action: "scroll list",
    },
    &KeyBinding {
        key: "t/b",
        action: "scroll to top/bottom",
    },
    &KeyBinding {
        key: "left/right, h/l",
        action: "scroll documentation",
    },
    &KeyBinding {
        key: "tab, [`]",
        action: "next/previous section",
    },
    &KeyBinding {
        key: "[:]",
        action: "command",
    },
    &KeyBinding {
        key: "[/]",
        action: "search",
    },
    &KeyBinding {
        key: "enter",
        action: "select / set value",
    },
    &KeyBinding {
        key: "c",
        action: "copy to clipboard",
    },
    &KeyBinding {
        key: "r, f5",
        action: "refresh",
    },
    &KeyBinding {
        key: "esc",
        action: "cancel / exit",
    },
    &KeyBinding {
        key: "q, ctrl-c/ctrl-d",
        action: "exit",
    },
];

/// Duration of prompt messages.
const MESSAGE_DURATION: u128 = 1750;

/// Application controller.
pub struct App<'a> {
    /// Whether if the application is running.
    pub running: bool,
    /// Whether if the help message is shown.
    pub show_help: bool,
    /// Input buffer.
    pub input: Option<String>,
    /// Time tracker for measuring the time for clearing the input.
    pub input_time: Option<Instant>,
    /// Cursor position.
    pub input_cursor: u16,
    /// Whether if the search mode is enabled.
    pub search_mode: bool,
    /// Y-scroll offset for the documentation.
    pub docs_scroll_amount: u16,
    /// Entries of the options menu.
    pub options: Option<SelectableList<&'a str>>,
    /// List of sysctl parameters.
    pub parameter_list: SelectableList<Parameter>,
    /// List of sysctl sections.
    pub section_list: SelectableList<String>,
    /// List of key bindings.
    pub key_bindings: SelectableList<&'a KeyBinding<'a>>,
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
            show_help: false,
            input: None,
            input_time: None,
            input_cursor: 0,
            search_mode: false,
            docs_scroll_amount: 0,
            options: None,
            parameter_list: SelectableList::default(),
            section_list: SelectableList::with_items({
                let mut sections = Section::variants()
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>();
                sections.insert(0, String::from("all"));
                sections
            }),
            key_bindings: SelectableList::default(),
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
    pub fn search(&mut self) {
        let section = self
            .section_list
            .selected()
            .map(|v| Section::from(v.to_string()))
            .unwrap_or(Section::Unknown);
        if let Some(query) = &self.input {
            self.parameter_list.items = self
                .sysctl
                .parameters
                .clone()
                .into_iter()
                .filter(|param| {
                    let mut found = param.name.contains(query);
                    if section != Section::Unknown {
                        found = found && section == param.section
                    }
                    found
                })
                .collect();
            if self.parameter_list.items.is_empty() {
                self.parameter_list.state.select(None);
            } else {
                self.parameter_list.state.select(Some(0));
            }
        } else {
            self.parameter_list = SelectableList::with_items(
                self.sysctl
                    .parameters
                    .clone()
                    .into_iter()
                    .filter(|param| section == Section::Unknown || param.section == section)
                    .collect(),
            );
        }
        self.docs_scroll_amount = 0;
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
        let mut hide_popup = true;
        match command {
            Command::Help => {
                self.options = None;
                self.key_bindings = SelectableList::with_items(KEY_BINDINGS.to_vec());
                self.key_bindings.state.select(None);
                self.show_help = true;
                hide_popup = false;
            }
            Command::Select => {
                if let Some(copy_option) = self
                    .options
                    .as_ref()
                    .and_then(|v| v.selected())
                    .and_then(|v| CopyOption::try_from(*v).ok())
                {
                    self.copy_to_clipboard(copy_option)?;
                    self.options = None;
                } else if self.show_help {
                    self.key_bindings.state.select(None);
                } else if let Some(parameter) = self.parameter_list.selected() {
                    self.search_mode = false;
                    self.input_time = None;
                    self.input = Some(format!("set {} {}", parameter.name, parameter.value));
                }
            }
            Command::Set(param_name, new_value) => {
                if let Some(parameter) = self
                    .parameter_list
                    .items
                    .iter_mut()
                    .find(|param| param.name == param_name)
                {
                    match parameter.update_value(&new_value, &self.sysctl.config, &mut Vec::new()) {
                        Ok(()) => {
                            self.run_command(Command::Refresh)?;
                        }
                        Err(e) => {
                            self.input = Some(e.to_string());
                            self.input_time = Some(Instant::now());
                        }
                    }
                } else {
                    self.input = Some(String::from("Unknown parameter"));
                    self.input_time = Some(Instant::now());
                }
            }
            Command::Scroll(ScrollArea::List, Direction::Up, amount) => {
                if self.show_help {
                    self.key_bindings.previous();
                    hide_popup = false;
                } else if let Some(options) = self.options.as_mut() {
                    options.previous();
                    hide_popup = false;
                } else if !self.parameter_list.items.is_empty() {
                    self.docs_scroll_amount = 0;
                    if amount == 1 {
                        self.parameter_list.previous();
                    } else {
                        self.parameter_list.state.select(
                            self.parameter_list
                                .state
                                .selected()
                                .and_then(|v| v.checked_sub(amount.into()))
                                .or(Some(0)),
                        )
                    }
                }
            }
            Command::Scroll(ScrollArea::List, Direction::Down, amount) => {
                if self.show_help {
                    self.key_bindings.next();
                    hide_popup = false;
                } else if let Some(options) = self.options.as_mut() {
                    options.next();
                    hide_popup = false;
                } else if !self.parameter_list.items.is_empty() {
                    self.docs_scroll_amount = 0;
                    if amount == 1 {
                        self.parameter_list.next();
                    } else {
                        self.parameter_list.state.select(
                            self.parameter_list
                                .state
                                .selected()
                                .and_then(|v| v.checked_add(amount.into()))
                                .map(|mut index| {
                                    if index > self.parameter_list.items.len() {
                                        index = self.parameter_list.items.len() - 1;
                                    }
                                    index
                                }),
                        )
                    }
                }
            }
            Command::Scroll(ScrollArea::List, Direction::Top, _) => {
                if !self.parameter_list.items.is_empty() {
                    self.docs_scroll_amount = 0;
                    self.parameter_list.state.select(Some(0));
                }
            }
            Command::Scroll(ScrollArea::List, Direction::Bottom, _) => {
                if let Some(last_index) = self.parameter_list.items.len().checked_sub(1) {
                    self.docs_scroll_amount = 0;
                    self.parameter_list.state.select(Some(last_index))
                }
            }
            Command::Scroll(ScrollArea::Documentation, Direction::Up, amount) => {
                self.docs_scroll_amount = self
                    .docs_scroll_amount
                    .checked_sub(amount.into())
                    .unwrap_or_default();
            }
            Command::Scroll(ScrollArea::Documentation, Direction::Down, amount) => {
                self.docs_scroll_amount = self
                    .docs_scroll_amount
                    .checked_add(amount.into())
                    .unwrap_or(self.docs_scroll_amount);
            }
            Command::Scroll(ScrollArea::Section, direction, _) => {
                match direction {
                    Direction::Right => self.section_list.next(),
                    _ => self.section_list.previous(),
                }
                self.search();
                if self.parameter_list.items.is_empty() {
                    self.parameter_list.state.select(None);
                } else {
                    self.parameter_list.state.select(Some(0));
                }
            }
            Command::Scroll(_, _, _) => {}
            Command::Search => {
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
                        self.input = None;
                        self.run_command(command)?;
                        hide_popup = false;
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
                    if direction == Direction::Right {
                        if let Some(cursor_position) = self.input_cursor.checked_sub(1) {
                            self.input_cursor = cursor_position as u16;
                        }
                    } else if self.input_cursor != input.width() as u16 {
                        self.input_cursor += 1;
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
                    self.options = Some(SelectableList::with_items(
                        copy_options.iter().map(|v| v.as_str()).collect(),
                    ));
                    hide_popup = false;
                    self.show_help = false;
                } else {
                    self.input = Some(String::from("No parameter is selected"));
                    self.input_time = Some(Instant::now());
                }
            }
            Command::Refresh => {
                self.input = None;
                self.docs_scroll_amount = 0;
                let parameters = Sysctl::init(self.sysctl.config.clone())?.parameters;
                self.sysctl.parameters.iter_mut().for_each(|parameter| {
                    if let Some(param) =
                        parameters.iter().find(|param| param.name == parameter.name)
                    {
                        parameter.value = param.value.to_string();
                    }
                });
            }
            Command::Cancel => {
                if self.input.is_some() {
                    self.input = None;
                    self.input_time = None;
                } else if self.options.is_none() && !self.show_help {
                    self.running = false;
                }
            }
            Command::Exit => {
                self.running = false;
            }
            Command::Nothing => {}
        }
        if hide_popup {
            self.options = None;
            self.show_help = false;
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
