use std::error::Error;
mod chromium;
mod paths;
mod sqlite;
mod mozilla;
mod utils;
mod enums;
mod config;
mod date;

pub use chromium::chromium_based;
pub use mozilla::firefox_based;
pub use enums::*;


cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod winapi;        
        mod internet_explorer;
        use std::path::PathBuf;
    }
    else if #[cfg(target_os = "macos")] {
        mod safari;
        pub use safari::safari_based;
        mod secrets;
    }
    else if #[cfg(target_os = "linux")] {
        mod secrets;
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
pub fn firefox(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
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
pub fn libre_wolf(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
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
pub fn chrome(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
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
pub fn chromium(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
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
pub fn brave(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
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
pub fn edge(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
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
pub fn vivaldi(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
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
pub fn opera(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
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
pub fn opera_gx(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
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
pub fn safari(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
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
pub fn internet_explorer(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    pub use internet_explorer::internet_explorer_based;

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
pub fn load(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
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


