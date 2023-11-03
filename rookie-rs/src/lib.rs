pub mod config;
pub mod common;
pub mod browser;

use anyhow::{Result, bail};
use common::enums::Cookie;
use common::paths;
use std::path::PathBuf;
use browser::mozilla::firefox_based;
use browser::chromium::chromium_based;

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        use common::winapi;
        use browser::internet_explorer;
        pub use internet_explorer::internet_explorer_based;
    }
    else if #[cfg(target_os = "macos")] {
        use browser::safari::safari_based;
        use common::secrets;
    }
    else if #[cfg(target_os = "linux")] {
        use common::secrets;
    }
}

/// Returns cookies from firefox
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies = rookie::firefox(Some(domains));
/// }
/// ```
pub fn firefox(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    let db_path = paths::find_mozilla_based_paths(&config::FIREFOX_CONFIG)?;
    firefox_based(db_path, domains)
}

/// Returns cookies from libre wolf
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies = rookie::libre_wolf(Some(domains));
/// }
/// ```
pub fn libre_wolf(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    let db_path = paths::find_mozilla_based_paths(&config::LIBRE_WOLF_CONFIG)?;
    firefox_based(db_path, domains)
}

/// Returns cookies from chrome
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies = rookie::chrome(Some(domains));
/// }
/// ```
pub fn chrome(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let (key, db_path) = paths::find_chrome_based_paths(&config::CHROME_CONFIG)?;
            chromium_based(PathBuf::from(key), db_path, domains)
        } else {
            let (_, db_path) = paths::find_chrome_based_paths(&config::CHROME_CONFIG)?;
            chromium_based(&config::CHROME_CONFIG, db_path, domains)
        }
    }
}

/// Returns cookies from chromium
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies = rookie::chromium(Some(domains));
/// }
/// ```
pub fn chromium(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let (key, db_path) = paths::find_chrome_based_paths(&config::CHROMIUM_CONFIG)?;
            chromium_based(PathBuf::from(key), db_path, domains)
        } else {
            let (_, db_path) = paths::find_chrome_based_paths(&config::CHROMIUM_CONFIG)?;
            chromium_based(&config::CHROMIUM_CONFIG, db_path, domains)
        }
    }
}

/// Returns cookies from brave
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies = rookie::brave(Some(domains));
/// }
/// ```
pub fn brave(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let (key, db_path) = paths::find_chrome_based_paths(&config::BRAVE_CONFIG)?;
            chromium_based(PathBuf::from(key), db_path, domains)
        } else {
            let (_, db_path) = paths::find_chrome_based_paths(&config::BRAVE_CONFIG)?;
            chromium_based(&config::BRAVE_CONFIG, db_path, domains)
        }
    }
}

/// Returns cookies from edge
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies = rookie::edge(Some(domains));
/// }
/// ```
pub fn edge(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let (key, db_path) = paths::find_chrome_based_paths(&config::EDGE_CONFIG)?;
            chromium_based(PathBuf::from(key), db_path, domains)
        } else {
            let (_, db_path) = paths::find_chrome_based_paths(&config::EDGE_CONFIG)?;
            chromium_based(&config::EDGE_CONFIG, db_path, domains)
        }
    }
}

/// Returns cookies from vivaldi
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies = rookie::vivaldi(Some(domains));
/// }
/// ```
pub fn vivaldi(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let (key, db_path) = paths::find_chrome_based_paths(&config::VIVALDI_CONFIG)?;
            chromium_based(PathBuf::from(key), db_path, domains)
        } else {
            let (_, db_path) = paths::find_chrome_based_paths(&config::VIVALDI_CONFIG)?;
            chromium_based(&config::VIVALDI_CONFIG, db_path, domains)
        }
    }
}

/// Returns cookies from opera
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies = rookie::opera(Some(domains));
/// }
/// ```
pub fn opera(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let (key, db_path) = paths::find_chrome_based_paths(&config::OPERA_CONFIG)?;
            chromium_based(PathBuf::from(key), db_path, domains)
        } else {
            let (_, db_path) = paths::find_chrome_based_paths(&config::OPERA_CONFIG)?;
            chromium_based(&config::OPERA_CONFIG, db_path, domains)
        }
    }
}

/// Returns cookies from opera gx
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies = rookie::opera_gx(Some(domains));
/// }
/// ```
pub fn opera_gx(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let (key, db_path) = paths::find_chrome_based_paths(&config::OPERA_GX_CONFIG)?;
            chromium_based(PathBuf::from(key), db_path, domains)
        } else {
            let (_, db_path) = paths::find_chrome_based_paths(&config::OPERA_GX_CONFIG)?;
            chromium_based(&config::OPERA_GX_CONFIG, db_path, domains)
        }
    }
}

/// Returns cookies from safari (MacOS only)
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies = rookie::safari(Some(domains));
/// }
/// ```
#[cfg(target_os = "macos")]
pub fn safari(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    let db_path = paths::find_safari_based_paths(&config::SAFARI_CONFIG)?;
    safari_based(db_path, domains)
}

/// Returns cookies from internet explorer (Windows only)
///
/// # Arguments
///
/// * `domains` - A optional list that for getting specific domains only
///
/// # Examples
///
/// ```
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies = rookie::internet_explorer(Some(domains));
/// }
/// ```
#[cfg(target_os = "windows")]
pub fn internet_explorer(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
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
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies = rookie::load(Some(domains));
/// }
/// ```
pub fn load(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    let mut cookies = Vec::new();

    let mut browser_types = vec![firefox, libre_wolf, opera, edge, chromium, brave, vivaldi];
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            browser_types.push(chrome);
            browser_types.push(opera_gx);
            browser_types.push(internet_explorer);
        }

        else if #[cfg(target_os = "linux")] {
            browser_types.push(chrome)
        }

        else if #[cfg(target_os = "macos")] {
            browser_types.push(opera_gx);
            browser_types.push(chrome);
            browser_types.push(safari);
        }
    }

    for browser_fn in browser_types.iter() {
        let browser_cookies = browser_fn(domains.clone()).unwrap_or(vec![]);
        cookies.extend(browser_cookies);
    }

    Ok(cookies)
}

/// Returns cookies from specific browser
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
///
/// fn main() {
///     let domains = vec!["google.com"];
///     let cookies_path = "C:\\Users\\User\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\default\\network\\Cookies";
///     let key_path = "C:\\Users\\User\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\Local State";
///     let cookies = rookie::any_browser(cookies_path, None, Some(key_path)).unwrap();
/// }
/// ```
pub fn any_browser(
    cookies_path: &str,
    domains: Option<Vec<&str>>,
    key_path: Option<&str>,
) -> Result<Vec<Cookie>> {

    // chromium based
    cfg_if::cfg_if! {
        // Linux Chromium
        if #[cfg(unix)] {
            use crate::config;
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
                if let Ok(cookies) = chromium_based(&browser_config, cookies_path.into(), domains.clone()) {
                    return Ok(cookies);
                }
            }
        } 
        // Windows chromium
        else {
            if let Some(key_path) = key_path {
                if let Ok(cookies) = chromium_based(PathBuf::from(key_path), cookies_path.into(), domains.clone()) {
                    return Ok(cookies);
                }
            }
        }
    }

    // Firefox
    if let Ok(cookies) = firefox_based(cookies_path.into(), domains.clone()) {
        return Ok(cookies);
    }

    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            // Internet Explorer
            if let Ok(cookies) = internet_explorer_based(cookies_path.into(), domains.clone()) {
                return Ok(cookies);
            }
        }
        // Safari
        else if #[cfg(target_os = "macos")] {
            // try safari based
            if let Ok(cookies) = safari_based(cookies_path.into(), domains) {
                return Ok(cookies);
            }
        }
    }
    bail!("cant find any cookies");
}
