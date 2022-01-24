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

impl Section {
    /// Returns the section of the given parameter name.
    pub fn from_name(name: String) -> Self {
        for section in Self::variants() {
            if name.starts_with(&format!("{}.", section)) {
                return *section;
            }
        }
        Self::Unknown
    }
}

impl From<String> for Section {
    fn from(value: String) -> Self {
        for section in Self::variants() {
            if value.to_lowercase() == section.to_string() {
                return *section;
            }
        }
        Self::Unknown
    }
}

impl<'a> From<&'a Path> for Section {
    fn from(value: &'a Path) -> Self {
        if value.components().any(|v| v.as_os_str() == "networking") {
            return Self::Net;
        }
        for section in Self::variants() {
            if let Some(file_stem) = value.file_stem().and_then(|v| v.to_str()) {
                if file_stem.starts_with(&section.to_string().to_lowercase()) {
                    return *section;
                }
            }
        }
        Section::Unknown
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysctl_section() {
        assert_eq!(Section::Net, Section::from_name(String::from("net.xyz")));
        assert_eq!(
            Section::User,
            Section::from_name(String::from("user.aaa.bbb"))
        );
        assert_eq!(Section::Unknown, Section::from_name(String::from("test")));
        assert_eq!(Section::Sunrpc, Section::from(String::from("sunrpc")));
        assert_eq!(Section::Unknown, Section::from(String::from("test")));
        assert_eq!(Section::Vm, Section::from(Path::new("/etc/vm.txt")));
        assert_eq!(
            Section::Kernel,
            Section::from(Path::new("/etc/kernel.tar.gz"))
        );
        assert_eq!(Section::Net, Section::from(Path::new("/networking/abc")));
        assert_eq!(Section::Unknown, Section::from(Path::new("test")));
    }
}
