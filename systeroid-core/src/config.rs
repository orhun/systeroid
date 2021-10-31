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
    /// Sysctl configuration.
    pub sysctl: SysctlConfig,
}

/// Sysctl configuration.
#[derive(Debug)]
pub struct SysctlConfig {
    /// Sections and the corresponding colors.
    pub section_colors: HashMap<Section, Color>,
    /// Default color for the output
    pub default_color: Color,
}

impl Default for SysctlConfig {
    fn default() -> Self {
        Self {
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
