use crate::sysctl::display::DisplayType;
use crate::sysctl::section::Section;
use colored::Color;
use std::collections::HashMap;

/* Macro for the concise initialization of HashMap */
macro_rules! map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

/// Configuration.
#[derive(Clone, Debug)]
pub struct Config {
    /// Whether if the verbose logging is enabled.
    pub verbose: bool,
    /// Whether if the errors should be ignored.
    pub ignore_errors: bool,
    /// Whether if the quiet mode is enabled.
    pub quiet: bool,
    /// Whether if the pager is disabled.
    pub no_pager: bool,
    /// Sections and the corresponding colors.
    pub section_colors: HashMap<Section, Color>,
    /// Default color for the output
    pub default_color: Color,
    /// Display type of the kernel parameters.
    pub display_type: DisplayType,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verbose: false,
            ignore_errors: false,
            quiet: false,
            no_pager: false,
            section_colors: map! {
                Section::Abi => Color::Red,
                Section::Fs => Color::Green,
                Section::Kernel => Color::Magenta,
                Section::Net => Color::Blue,
                Section::Sunrpc => Color::Yellow,
                Section::User => Color::Cyan,
                Section::Vm => Color::BrightRed,
                Section::Unknown => Color::White
            },
            default_color: Color::BrightBlack,
            display_type: DisplayType::default(),
        }
    }
}
