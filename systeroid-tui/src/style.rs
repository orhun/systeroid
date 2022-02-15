use crate::error::Result;
use colorsys::{ParseError, Rgb};
use std::result::Result as StdResult;
use std::str::FromStr;
use tui::style::{Color as TuiColor, Style};

/// Color configuration.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Colors {
    /// Background color.
    bg: Color,
    /// Foreground color.
    fg: Color,
}

impl Colors {
    /// Constructs a new instance.
    pub fn new(background: &str, foreground: &str) -> Result<Self> {
        Ok(Self {
            bg: Color::from_str(background)?,
            fg: Color::from_str(foreground)?,
        })
    }

    /// Returns the background/foreground colors with default style.
    pub fn get_style(&self) -> Style {
        Style::default().bg(self.fg.get()).fg(self.bg.get())
    }

    /// Returns the background color with default style.
    pub fn get_bg_style(&self) -> Style {
        Style::default().bg(self.bg.get())
    }

    /// Returns the foreground color with default style.
    pub fn get_fg_style(&self) -> Style {
        Style::default().fg(self.fg.get())
    }
}

/// Wrapper for widget colors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    /// Inner type.
    inner: TuiColor,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            inner: TuiColor::Reset,
        }
    }
}

impl Color {
    /// Returns the underlying [`Color`] type.
    ///
    /// [`Color`]: tui::style::Color
    pub fn get(self) -> TuiColor {
        self.inner
    }
}

impl FromStr for Color {
    type Err = ParseError;
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(Self {
            inner: match s.to_lowercase().as_ref() {
                "reset" => TuiColor::Reset,
                "black" => TuiColor::Black,
                "red" => TuiColor::Red,
                "green" => TuiColor::Green,
                "yellow" => TuiColor::Yellow,
                "blue" => TuiColor::Blue,
                "magenta" => TuiColor::Magenta,
                "cyan" => TuiColor::Cyan,
                "gray" => TuiColor::Gray,
                "darkgray" => TuiColor::DarkGray,
                "lightred" => TuiColor::LightRed,
                "lightgreen" => TuiColor::LightGreen,
                "lightyellow" => TuiColor::LightYellow,
                "lightblue" => TuiColor::LightBlue,
                "lightmagenta" => TuiColor::LightMagenta,
                "lightcyan" => TuiColor::LightCyan,
                "white" => TuiColor::White,
                _ => {
                    let rgb = Rgb::from_hex_str(&format!("#{}", s))?;
                    TuiColor::Rgb(rgb.red() as u8, rgb.green() as u8, rgb.blue() as u8)
                }
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_color() -> Result<()> {
        assert_eq!(TuiColor::Reset, Color::default().get());
        assert_eq!(TuiColor::Gray, Color::from_str("gray")?.get());
        assert_eq!(TuiColor::Black, Color::from_str("black")?.get());
        assert_eq!(TuiColor::Green, Color::from_str("green")?.get());
        assert_eq!(
            TuiColor::Rgb(152, 157, 69),
            Color::from_str("989D45")?.get()
        );
        assert_eq!(TuiColor::Rgb(18, 49, 47), Color::from_str("12312F")?.get());
        assert_eq!(
            TuiColor::Rgb(255, 242, 255),
            Color::from_str("FFF2FF")?.get()
        );
        Ok(())
    }
    #[test]
    fn test_style() -> Result<()> {
        assert_eq!(
            Colors {
                bg: Color::from_str("red")?,
                fg: Color::from_str("blue")?,
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
