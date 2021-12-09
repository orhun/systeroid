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
