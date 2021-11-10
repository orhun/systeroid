use crate::sysctl::Section;
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
    /// Color configuration.
    pub color: ColorConfig,
}

/// Sysctl configuration.
#[derive(Debug)]
pub struct ColorConfig {
    /// Whether if the colors are disabled.
    pub no_color: bool,
    /// Sections and the corresponding colors.
    pub section_colors: HashMap<Section, Color>,
    /// Default color for the output
    pub default_color: Color,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            no_color: false,
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
        }
    }
}
