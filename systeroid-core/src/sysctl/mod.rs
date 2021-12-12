/// Sysctl wrapper.
pub mod controller;

/// Sysctl section.
pub mod section;

/// Sysctl display options.
pub mod display;

/// Kernel parameter.
pub mod parameter;

/// Default location to preload values.
pub const DEFAULT_PRELOAD: &str = "/etc/sysctl.conf";

/// Deprecated variables to skip while listing.
/// <https://bugzilla.redhat.com/show_bug.cgi?id=152435>
pub const DEPRECATED_VARIABLES: &[&str] = &["base_reachable_time", "retrans_time"];
