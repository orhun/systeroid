use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};

/// Sections of the sysctl documentation.
#[derive(Clone, Copy, Debug)]
pub enum SysctlSection {
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

impl From<String> for SysctlSection {
    fn from(value: String) -> Self {
        for section in Self::variants() {
            if value.starts_with(&format!("{}.", section)) {
                return *section;
            }
        }
        Self::Unknown
    }
}

impl Display for SysctlSection {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl SysctlSection {
    /// Returns the variants.
    pub fn variants() -> &'static [SysctlSection] {
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

    /// Returns the sysctl section as a file with `.rst` extension.
    pub fn as_file(&self) -> PathBuf {
        Path::new(&self.to_string()).with_extension("rst")
    }
}

/// Documentation of a kernel parameter.
#[derive(Clone, Debug)]
pub struct Documentation {
    /// Name of the kernel parameter.
    pub name: String,
    /// Description of the kernel parameter.
    pub description: String,
    /// Section of the kernel parameter.
    pub section: SysctlSection,
}

impl Documentation {
    /// Constructs a new instance.
    pub fn new(name: String, description: String, section: SysctlSection) -> Self {
        Self {
            name,
            description,
            section,
        }
    }
}
