use crate::error::Result;
use crate::sysctl::r#type::{DisplayType, OutputType};
use crate::sysctl::section::Section;
use colored::Color;
use ini::Ini;
use std::collections::HashMap;
use std::path::PathBuf;

/// Default configuration file.
pub const DEFAULT_CONFIG: &str = "systeroid.conf";

/// Environment variable for setting the path of the configuration file.
pub const CONFIG_ENV: &str = "SYSTEROID_CONFIG";

lazy_static! {
    /// Default locations for the configuration file.
    pub static ref DEFAULT_CONFIG_PATHS: Vec<Option<PathBuf>> = vec![
        dirs_next::config_dir().map(|p| p.join("systeroid").join(DEFAULT_CONFIG)),
        dirs_next::home_dir().map(|p| p.join(".systeroid").join(DEFAULT_CONFIG)),
    ];
}

/// Macro for the concise initialization of HashMap
macro_rules! map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

/// Macro for parsing a boolean value from INI format
macro_rules! parse_ini_flag {
    ($self: ident, $config: ident, $section: ident, $name: ident) => {
        if let Some($name) = $section.get(stringify!($name)) {
            $self.$config.$name = $name == "true";
        }
    };
}

/// Configuration.
#[derive(Clone, Debug)]
pub struct Config {
    /// Whether if the deprecated variables should be included while listing.
    pub display_deprecated: bool,
    /// Path of the Linux kernel documentation.
    pub kernel_docs: Option<PathBuf>,
    /// CLI configuration.
    pub cli: CliConfig,
    /// TUI configuration.
    pub tui: TuiConfig,
}

/// CLI configuration.
#[derive(Clone, Debug)]
pub struct CliConfig {
    /// Whether if the verbose logging is enabled.
    pub verbose: bool,
    /// Whether if the errors should be ignored.
    pub ignore_errors: bool,
    /// Whether if the quiet mode is enabled.
    pub quiet: bool,
    /// Whether if the pager is disabled.
    pub no_pager: bool,
    /// Display type of the kernel parameters.
    pub display_type: DisplayType,
    /// Output type of the application.
    pub output_type: OutputType,
    /// Color configuration.
    pub color: CliColorConfig,
}

/// CLI color configuration.
#[derive(Clone, Debug)]
pub struct CliColorConfig {
    /// Default color for the output
    pub default_color: Color,
    /// Sections and the corresponding colors.
    pub section_colors: HashMap<Section, Color>,
}

/// TUI configuration.
#[derive(Clone, Debug)]
pub struct TuiConfig {
    /// Refresh rate of the terminal.
    pub tick_rate: u64,
    /// Do not parse/show Linux kernel documentation.
    pub no_docs: bool,
    /// Path for saving the changed kernel parameters.
    pub save_path: Option<PathBuf>,
    /// File to save the logs.
    pub log_file: Option<String>,
    /// Color configuration.
    pub color: TuiColorConfig,
}

/// TUI color configuration.
#[derive(Clone, Debug)]
pub struct TuiColorConfig {
    /// Foreground color.
    pub fg_color: String,
    /// Background color.
    pub bg_color: String,
}

impl Config {
    /// Parses the configuration file and overrides values.
    pub fn parse(&mut self, path: Option<PathBuf>) -> Result<()> {
        log::trace!("Parsing configuration from {:?}", path);
        let mut config_paths = DEFAULT_CONFIG_PATHS.clone();
        if path.is_some() {
            config_paths.insert(0, path);
        }
        let mut config_path = None;
        for path in config_paths.into_iter().flatten() {
            if path.exists() {
                config_path = Some(path);
                break;
            }
        }
        if let Some(path) = config_path {
            let ini = Ini::load_from_file(path)?;
            if let Some(general_section) = ini.section(Some("general")) {
                if let Some(display_deprecated) = general_section.get("display_deprecated") {
                    self.display_deprecated = display_deprecated == "true";
                }
                if let Some(kernel_docs) = general_section.get("kernel_docs") {
                    self.kernel_docs = Some(PathBuf::from(kernel_docs));
                }
            }
            if let Some(section) = ini.section(Some("cli")) {
                parse_ini_flag!(self, cli, section, verbose);
                parse_ini_flag!(self, cli, section, ignore_errors);
                parse_ini_flag!(self, cli, section, quiet);
                parse_ini_flag!(self, cli, section, no_pager);
                if let Some(display_type) = section.get("display_type").map(DisplayType::from) {
                    self.cli.display_type = display_type;
                }
                if let Some(output_type) = section.get("output_type").map(OutputType::from) {
                    self.cli.output_type = output_type;
                }
            }
            if let Some(section) = ini.section(Some("cli.colors")) {
                if let Some(default_color) = section
                    .get("default_color")
                    .and_then(|v| Color::try_from(v).ok())
                {
                    self.cli.color.default_color = default_color;
                }
                for (key, value) in section.iter() {
                    if key.starts_with("section_") {
                        if let (sysctl_section, Some(color)) = (
                            Section::from(key.trim_start_matches("section_").to_string()),
                            Color::try_from(value).ok(),
                        ) {
                            self.cli.color.section_colors.insert(sysctl_section, color);
                        }
                    }
                }
            }
            if let Some(section) = ini.section(Some("tui")) {
                if let Some(tick_rate) = section.get("tick_rate").and_then(|v| v.parse().ok()) {
                    self.tui.tick_rate = tick_rate;
                }
                if let Some(save_path) = section.get("save_path") {
                    self.tui.save_path = Some(PathBuf::from(save_path));
                }
                if let Some(log_file) = section.get("log_file") {
                    self.tui.log_file = Some(log_file.to_string());
                }
                parse_ini_flag!(self, tui, section, no_docs);
            }
            if let Some(section) = ini.section(Some("tui.colors")) {
                if let Some(fg_color) = section.get("fg_color") {
                    self.tui.color.fg_color = fg_color.to_string();
                }
                if let Some(bg_color) = section.get("bg_color") {
                    self.tui.color.bg_color = bg_color.to_string();
                }
            }
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display_deprecated: false,
            kernel_docs: None,
            cli: CliConfig {
                verbose: false,
                ignore_errors: false,
                quiet: false,
                no_pager: false,
                display_type: DisplayType::Default,
                output_type: OutputType::Default,
                color: CliColorConfig {
                    default_color: Color::BrightBlack,
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
                },
            },
            tui: TuiConfig {
                tick_rate: 250,
                no_docs: false,
                save_path: None,
                log_file: None,
                color: TuiColorConfig {
                    fg_color: String::from("white"),
                    bg_color: String::from("black"),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config() -> Result<()> {
        let mut config = Config {
            display_deprecated: true,
            ..Default::default()
        };
        config.cli.display_type = DisplayType::Value;
        config.cli.color.default_color = Color::Blue;
        config.cli.color.section_colors = HashMap::new();
        config.tui.tick_rate = 3000;
        config.tui.color.fg_color = String::new();
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("parent directory not found")
            .join("config")
            .join(DEFAULT_CONFIG);
        config.parse(Some(path))?;
        assert_eq!(
            Config::default().display_deprecated,
            config.display_deprecated
        );
        assert_eq!(
            Some(PathBuf::from("/usr/share/doc/linux")),
            config.kernel_docs
        );
        assert_eq!(Config::default().cli.display_type, config.cli.display_type);
        assert_eq!(
            Config::default().cli.color.default_color,
            config.cli.color.default_color
        );
        assert_eq!(
            Config::default().cli.color.section_colors,
            config.cli.color.section_colors
        );
        assert_eq!(Config::default().tui.tick_rate, config.tui.tick_rate);
        assert_eq!(
            Config::default().tui.color.fg_color,
            config.tui.color.fg_color
        );
        Ok(())
    }
}
