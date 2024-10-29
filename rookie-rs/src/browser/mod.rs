use crate::enums::Cookie;
use eyre::Result;
use std::path::Path;

pub(crate) mod chromium;
pub(crate) mod mozilla;

#[cfg(target_os = "windows")]
pub(crate) mod internet_explorer;

#[cfg(target_os = "macos")]
pub(crate) mod safari;

#[cfg(target_os = "linux")]
pub fn w3m(db_path: &Path, domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  Ok(Vec::new())
}

#[cfg(any(windows, target_os = "linux"))]
pub fn lynx(db_path: &Path, domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  Ok(Vec::new())
}
