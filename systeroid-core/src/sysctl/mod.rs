/// Sysctl wrapper.
pub mod controller;

/// Sysctl section.
pub mod section;

/// Sysctl display options.
pub mod display;

/// Kernel parameter.
pub mod parameter;

/// Default configuration file to preload values from.
pub const DEFAULT_PRELOAD: &str = "/etc/sysctl.conf";

/// Default system configuration files to preload values from.
pub const SYSTEM_PRELOAD: &[&str] = &[
    "/etc/sysctl.d",
    "/run/sysctl.d",
    "/usr/local/lib/sysctl.d",
    "/usr/lib/sysctl.d",
    "/lib/sysctl.d",
    DEFAULT_PRELOAD,
];

/// Deprecated variables to skip while listing.
/// <https://bugzilla.redhat.com/show_bug.cgi?id=152435>
pub const DEPRECATED_VARIABLES: &[&str] = &["base_reachable_time", "retrans_time"];
