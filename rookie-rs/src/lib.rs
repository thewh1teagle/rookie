// Public

// Common
pub mod common;
mod utils;
pub use common::enums;

#[cfg(feature = "appbound")]
#[allow(unused)]
static PAEXEC_BYTES: &[u8] = include_bytes!("paexec.bin"); // wget.exe https://github.com/thewh1teagle/rookie/releases/download/appbound-binaries/paexec.exe -O rookie-rs\src\paexec.bin

#[cfg(feature = "appbound")]
#[allow(unused)]
static CRYPT_UNPROTECT_BYTES: &[u8] = include_bytes!("unprotect.bin"); // cargo build --release -p cryptunprotect OR wget.exe https://github.com/thewh1teagle/rookie/releases/download/appbound-binaries/paexec.exe -O rookie-rs\src\unprotect.bin

// Browser
#[cfg(target_os = "windows")]
pub use browser::internet_explorer::internet_explorer_based;
#[cfg(target_os = "macos")]
pub use browser::safari::safari_based;
pub use browser::{chromium::chromium_based, mozilla::firefox_based};

// Config
#[cfg(target_os = "linux")]
pub use linux::config;
#[cfg(target_os = "macos")]
pub use macos::config;
#[cfg(target_os = "windows")]
pub use windows::config;

// Private
mod browser;
use common::paths;
use enums::Cookie;
use eyre::bail;
pub use eyre::Result;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
use std::path::PathBuf;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

/// Returns rookie version
/// Format: <semver>(<commit>)
///
/// # Examples
///
/// ```
/// let version = rookie::version();
/// println!("{}", version);
/// ```
pub fn version() -> String {
  format!("{} ({})", env!("CARGO_PKG_VERSION"), env!("COMMIT_HASH"))
}

/// Returns cookies from Firefox
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::firefox(Some(domains));
/// ```
pub fn firefox(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  let db_path = paths::find_mozilla_based_paths(&config::FIREFOX_CONFIG)?;
  firefox_based(db_path, domains)
}

/// Returns cookies from LibreWolf
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::librewolf(Some(domains));
/// ```
pub fn librewolf(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  let db_path = paths::find_mozilla_based_paths(&config::LIBREWOLF_CONFIG)?;
  firefox_based(db_path, domains)
}

/// Returns cookies from Cachy Browser (Linux only)
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::cachy(Some(domains));
/// ```
#[cfg(target_os = "linux")]
pub fn cachy(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  let db_path = paths::find_mozilla_based_paths(&config::CACHY_CONFIG)?;
  firefox_based(db_path, domains)
}

/// Returns cookies from Chrome
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::chrome(Some(domains));
/// ```
pub fn chrome(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  #[cfg(target_os = "windows")]
  {
    let (key, db_path) = paths::find_chrome_based_paths(&config::CHROME_CONFIG)?;
    chromium_based(key, db_path, domains)
  }
  #[cfg(unix)]
  {
    let (_, db_path) = paths::find_chrome_based_paths(&config::CHROME_CONFIG)?;
    chromium_based(&config::CHROME_CONFIG, db_path, domains)
  }
}

/// Returns cookies from Chromium
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::chromium(Some(domains));
/// ```
pub fn chromium(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  #[cfg(target_os = "windows")]
  {
    let (key, db_path) = paths::find_chrome_based_paths(&config::CHROMIUM_CONFIG)?;
    chromium_based(key, db_path, domains)
  }
  #[cfg(unix)]
  {
    let (_, db_path) = paths::find_chrome_based_paths(&config::CHROMIUM_CONFIG)?;
    chromium_based(&config::CHROMIUM_CONFIG, db_path, domains)
  }
}

/// Returns cookies from Brave
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::brave(Some(domains));
/// ```
pub fn brave(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  #[cfg(target_os = "windows")]
  {
    let (key, db_path) = paths::find_chrome_based_paths(&config::BRAVE_CONFIG)?;
    chromium_based(key, db_path, domains)
  }
  #[cfg(unix)]
  {
    let (_, db_path) = paths::find_chrome_based_paths(&config::BRAVE_CONFIG)?;
    chromium_based(&config::BRAVE_CONFIG, db_path, domains)
  }
}

/// Returns cookies from Arc
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::brave(Some(domains));
/// ```
pub fn arc(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  #[cfg(target_os = "windows")]
  {
    let (key, db_path) = paths::find_chrome_based_paths(&config::ARC_CONFIG)?;
    chromium_based(key, db_path, domains)
  }
  #[cfg(unix)]
  {
    let (_, db_path) = paths::find_chrome_based_paths(&config::ARC_CONFIG)?;
    chromium_based(&config::ARC_CONFIG, db_path, domains)
  }
}

/// Returns cookies from Edge
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::edge(Some(domains));
/// ```
pub fn edge(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  #[cfg(target_os = "windows")]
  {
    let (key, db_path) = paths::find_chrome_based_paths(&config::EDGE_CONFIG)?;
    chromium_based(key, db_path, domains)
  }
  #[cfg(unix)]
  {
    let (_, db_path) = paths::find_chrome_based_paths(&config::EDGE_CONFIG)?;
    chromium_based(&config::EDGE_CONFIG, db_path, domains)
  }
}

/// Returns cookies from Vivaldi
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::vivaldi(Some(domains));
/// ```
pub fn vivaldi(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  #[cfg(target_os = "windows")]
  {
    let (key, db_path) = paths::find_chrome_based_paths(&config::VIVALDI_CONFIG)?;
    chromium_based(key, db_path, domains)
  }
  #[cfg(unix)]
  {
    let (_, db_path) = paths::find_chrome_based_paths(&config::VIVALDI_CONFIG)?;
    chromium_based(&config::VIVALDI_CONFIG, db_path, domains)
  }
}

/// Returns cookies from Opera
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::opera(Some(domains));
/// ```
pub fn opera(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  #[cfg(target_os = "windows")]
  {
    let (key, db_path) = paths::find_chrome_based_paths(&config::OPERA_CONFIG)?;
    chromium_based(key, db_path, domains)
  }
  #[cfg(unix)]
  {
    let (_, db_path) = paths::find_chrome_based_paths(&config::OPERA_CONFIG)?;
    chromium_based(&config::OPERA_CONFIG, db_path, domains)
  }
}

/// Returns cookies from Opera GX
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::opera_gx(Some(domains));
/// ```
pub fn opera_gx(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  #[cfg(target_os = "windows")]
  {
    let (key, db_path) = paths::find_chrome_based_paths(&config::OPERA_GX_CONFIG)?;
    chromium_based(key, db_path, domains)
  }
  #[cfg(unix)]
  {
    let (_, db_path) = paths::find_chrome_based_paths(&config::OPERA_GX_CONFIG)?;
    chromium_based(&config::OPERA_GX_CONFIG, db_path, domains)
  }
}

/// Returns cookies from Octo Browser
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::octo_browser(Some(domains));
/// ```
#[cfg(target_os = "windows")]
pub fn octo_browser(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  let (key, db_path) = paths::find_chrome_based_paths(&config::OCTO_BROWSER_CONFIG)?;
  chromium_based(key, db_path, domains)
}

/// Returns cookies from Safari (macOS only)
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::safari(Some(domains));
/// ```
#[cfg(target_os = "macos")]
pub fn safari(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  let db_path = paths::find_safari_based_paths(&config::SAFARI_CONFIG)?;
  safari_based(db_path, domains)
}

/// Returns cookies from Internet Explorer (Windows only)
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::internet_explorer(Some(domains));
/// ```
#[cfg(target_os = "windows")]
pub fn internet_explorer(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  let db_path = paths::find_ie_based_paths(&config::IE_CONFIG)?;
  internet_explorer_based(db_path, domains)
}

/// Returns cookies from all browsers
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies = rookie::load(Some(domains));
/// ```
pub fn load(domains: Option<Vec<String>>) -> Result<Vec<Cookie>> {
  let mut cookies = Vec::new();

  let mut browser_types = vec![
    firefox, librewolf, opera, edge, chromium, brave, vivaldi, arc,
  ];

  #[cfg(target_os = "windows")]
  {
    browser_types.push(chrome);
    browser_types.push(internet_explorer);
    browser_types.push(opera_gx);
  }
  #[cfg(target_os = "linux")]
  {
    browser_types.push(chrome);
    browser_types.push(cachy);
  }
  #[cfg(target_os = "macos")]
  {
    browser_types.push(chrome);
    browser_types.push(opera_gx);
    browser_types.push(safari);
  }

  for browser_fn in browser_types.iter() {
    let browser_cookies = browser_fn(domains.clone()).unwrap_or(vec![]);
    cookies.extend(browser_cookies);
  }

  Ok(cookies)
}

/// Returns cookies from specific browser
/// Useful for CLI apps
///
/// # Arguments
///
/// * `cookies_path` - Absolute path for cookies file
/// * `domains` - Optional list that for getting specific domains only
/// * `key_path` - Optional absolute path for key required to decrypt the cookies (required for chrome)
///
/// # Examples
///
/// ```
/// let domains = vec!["google.com"];
/// let cookies_path = "C:\\Users\\User\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\default\\network\\Cookies";
/// let key_path = "C:\\Users\\User\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\Local State";
/// let cookies = rookie::any_browser(cookies_path, None, Some(key_path)).unwrap();
/// ```
#[allow(unused_variables)]
pub fn any_browser(
  cookies_path: &str,
  domains: Option<Vec<String>>,
  key_path: Option<&str>,
) -> Result<Vec<Cookie>> {
  // chromium based
  #[cfg(unix)]
  {
    let chrome_configs = &[
      &config::CHROME_CONFIG,
      &config::BRAVE_CONFIG,
      &config::CHROMIUM_CONFIG,
      &config::EDGE_CONFIG,
      &config::OPERA_CONFIG,
      &config::OPERA_GX_CONFIG,
      &config::VIVALDI_CONFIG,
    ];
    for browser_config in chrome_configs {
      if let Ok(cookies) = chromium_based(browser_config, cookies_path.into(), domains.clone()) {
        return Ok(cookies);
      }
    }
  }
  #[cfg(target_os = "windows")]
  {
    if let Some(key_path) = key_path {
      if let Ok(cookies) = chromium_based(
        PathBuf::from(key_path),
        cookies_path.into(),
        domains.clone(),
      ) {
        return Ok(cookies);
      }
    }
  }
  // Windows chromium

  // Firefox
  if let Ok(cookies) = firefox_based(cookies_path.into(), domains.clone()) {
    return Ok(cookies);
  }

  #[cfg(target_os = "windows")]
  {
    // Internet Explorer
    if let Ok(cookies) = internet_explorer_based(cookies_path.into(), domains.clone()) {
      return Ok(cookies);
    }
  }
  #[cfg(target_os = "macos")]
  {
    if let Ok(cookies) = safari_based(cookies_path.into(), domains) {
      return Ok(cookies);
    }
  }
  bail!("Can't find any cookies");
}
