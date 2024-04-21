pub(crate) mod chromium;
pub(crate) mod mozilla;

#[cfg(target_os = "windows")]
pub(crate) mod internet_explorer;

#[cfg(target_os = "macos")]
pub(crate) mod safari;
