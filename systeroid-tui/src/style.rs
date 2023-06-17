use crate::error::Result;
use std::str::FromStr;
use tui::style::{Color as TuiColor, Style};

/// Color configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Colors {
    /// Background color.
    bg: TuiColor,
    /// Foreground color.
    fg: TuiColor,
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            bg: TuiColor::Reset,
            fg: TuiColor::Reset,
        }
    }
}

impl Colors {
    /// Constructs a new instance.
    pub fn new(background: &str, foreground: &str) -> Result<Self> {
        Ok(Self {
            bg: TuiColor::from_str(background)?,
            fg: TuiColor::from_str(foreground)?,
        })
    }

    /// Returns the background/foreground colors with default style.
    pub fn get_style(&self) -> Style {
        Style::default().bg(self.fg).fg(self.bg)
    }

    /// Returns the background color with default style.
    pub fn get_bg_style(&self) -> Style {
        Style::default().bg(self.bg)
    }

    /// Returns the foreground color with default style.
    pub fn get_fg_style(&self) -> Style {
        Style::default().fg(self.fg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_style() -> Result<()> {
        assert_eq!(
            Colors {
                bg: TuiColor::from_str("red")?,
                fg: TuiColor::from_str("blue")?,
            },
            Colors::new("red", "blue")?
        );
        assert_eq!(
            Style::default().fg(TuiColor::Green),
            Colors::new("reset", "Green")?.get_fg_style()
        );
        assert_eq!(
            Style::default().bg(TuiColor::Yellow),
            Colors::new("YELLOW", "reset")?.get_bg_style()
        );
        assert_eq!(
            Style::default()
                .bg(TuiColor::DarkGray)
                .fg(TuiColor::Magenta),
            Colors::new("Magenta", "DarkGray")?.get_style()
        );
        Ok(())
    }
}
