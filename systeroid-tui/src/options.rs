use std::convert::TryFrom;

/// Available copying options.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CopyOption {
    /// Copy the name of the parameter.
    Name,
    /// Copy the value of the parameter.
    Value,
    /// Copy the documentation of the parameter.
    Documentation,
}

impl<'a> TryFrom<&'a str> for CopyOption {
    type Error = ();
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::variants()
            .iter()
            .find(|v| value == v.as_str())
            .copied()
            .ok_or(())
    }
}

impl CopyOption {
    /// Returns the string representation of the option.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Name => "Parameter name",
            Self::Value => "Parameter value",
            Self::Documentation => "Documentation",
        }
    }

    /// Returns the variants.
    pub fn variants() -> &'static [Self] {
        &[Self::Name, Self::Value, Self::Documentation]
    }
}
