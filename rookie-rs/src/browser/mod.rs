pub mod chromium;
pub mod mozilla;

#[cfg(windows)]
pub mod internet_explorer;

#[cfg(macos)]
use crate::safari;