use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::path::Path;

/// Sections of the sysctl documentation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Section {
    /// Documentation for `/proc/sys/abi/*`
    Abi,
    /// Documentation for `/proc/sys/fs/*`
    Fs,
    /// Documentation for `/proc/sys/kernel/*`
    Kernel,
    /// Documentation for `/proc/sys/net/*`
    Net,
    /// Documentation for `/proc/sys/sunrpc/*`
    Sunrpc,
    /// Documentation for `/proc/sys/user/*`
    User,
    /// Documentation for `/proc/sys/vm/*`
    Vm,
    /// Unknown.
    Unknown,
}

impl From<String> for Section {
    fn from(value: String) -> Self {
        for section in Self::variants() {
            if value.starts_with(&format!("{}.", section)) {
                return *section;
            }
        }
        Self::Unknown
    }
}

impl<'a> From<&'a Path> for Section {
    fn from(value: &'a Path) -> Self {
        for section in Self::variants() {
            if value.file_stem().and_then(|v| v.to_str()) == Some(&section.to_string()) {
                return *section;
            }
        }
        Self::Net
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Section {
    /// Returns the variants.
    pub fn variants() -> &'static [Section] {
        &[
            Self::Abi,
            Self::Fs,
            Self::Kernel,
            Self::Net,
            Self::Sunrpc,
            Self::User,
            Self::Vm,
        ]
    }
}
