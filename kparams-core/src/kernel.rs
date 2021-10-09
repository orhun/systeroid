use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};

/// Sections of the sysctl documentation.
#[derive(Clone, Debug)]
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
}

impl Display for SysctlSection {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl SysctlSection {
    /// Returns the variants.
    pub fn iter() -> &'static [&'static SysctlSection] {
        &[
            &Self::Abi,
            &Self::Fs,
            &Self::Kernel,
            &Self::Net,
            &Self::Sunrpc,
            &Self::User,
            &Self::Vm,
        ]
    }

    /// Returns the sysctl section as a file with `.rst` extension.
    pub fn as_file(&self) -> PathBuf {
        Path::new(&self.to_string()).with_extension("rst")
    }
}

/// Representation of a kernel parameter.
#[derive(Clone, Debug)]
pub struct Parameter<'a> {
    /// Name of the kernel parameter.
    pub name: &'a str,
    /// Description of the kernel parameter.
    pub description: &'a str,
    /// Section of the kernel parameter.
    pub section: &'a SysctlSection,
}

impl<'a> Parameter<'a> {
    /// Constructs a new instance.
    pub fn new(name: &'a str, description: &'a str, section: &'a SysctlSection) -> Self {
        Self {
            name,
            description,
            section,
        }
    }
}
