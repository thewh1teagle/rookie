pub mod chromium;
pub mod mozilla;

#[cfg(target_os = "windows")]
pub mod internet_explorer;

#[cfg(target_os = "macos")]
pub mod safari;