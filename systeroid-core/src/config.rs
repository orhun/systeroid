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

/// General configuration.
#[derive(Debug, Default)]
pub struct Config {
    /// Application configuration.
    pub app: AppConfig,
    /// Sysctl configuration.
    pub sysctl: SysctlConfig,
}

/// Sysctl configuration.
#[derive(Debug)]
pub struct AppConfig {
    /// Whether if the quiet mode is enabled.
    pub quiet: bool,
    /// Whether if the colors are disabled.
    pub no_color: bool,
    /// Whether if the pager is disabled.
    pub no_pager: bool,
    /// Sections and the corresponding colors.
    pub section_colors: HashMap<Section, Color>,
    /// Default color for the output
    pub default_color: Color,
    /// Display type of the kernel parameters.
    pub display_type: DisplayType,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            quiet: false,
            no_color: false,
            no_pager: false,
            section_colors: map! {
                Section::Abi => Color::Red,
                Section::Fs => Color::Green,
                Section::Kernel => Color::Magenta,
                Section::Net => Color::Blue,
                Section::Sunrpc => Color::Yellow,
                Section::User => Color::Cyan,
                Section::Vm => Color::BrightRed,
                Section::Unknown => Color::BrightBlack
            },
            default_color: Color::BrightBlack,
            display_type: DisplayType::default(),
        }
    }
}

/// Sysctl configuration.
#[derive(Debug, Default)]
pub struct SysctlConfig {
    /// Whether if the verbose logging is enabled.
    pub verbose: bool,
    /// Whether if the errors should be ignored.
    pub ignore_errors: bool,
}
