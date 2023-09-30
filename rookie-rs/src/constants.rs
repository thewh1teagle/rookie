#[cfg(target_os = "windows")] 
pub const CHROME_PATHS: &'static [&'static str] = &[
    "%LOCALAPPDATA%/Google/Chrome/User Data"
];

#[cfg(target_os = "windows")] 
pub const BRAVE_PATHS: &'static [&'static str] = &[
    "%LOCALAPPDATA%/BraveSoftware/Brave-Browser/User Data"
];

#[cfg(target_os = "windows")] 
pub const EDGE_PATHS: &'static [&'static str] = &[
    "%LOCALAPPDATA%/Microsoft/Edge/User Data"
];

#[cfg(target_os = "windows")] 
pub const FIREFOX_PATHS: &'static [&'static str] = &[
    "%APPDATA%/Mozilla/Firefox"
];



#[cfg(target_os = "linux")] 
pub const CHROME_PATHS: &'static [&'static str] = &[
    "%LOCALAPPDATA%/Google/Chrome/User Data"
];

#[cfg(target_os = "linux")] 
pub const BRAVE_PATHS: &'static [&'static str] = &[
    "%LOCALAPPDATA%/BraveSoftware/Brave-Browser/User Data"
];

#[cfg(target_os = "linux")] 
pub const EDGE_PATHS: &'static [&'static str] = &[
    "%LOCALAPPDATA%/Microsoft/Edge/User Data"
];

#[cfg(target_os = "linux")] 
pub const FIREFOX_PATHS: &'static [&'static str] = &[
    "~/snap/firefox/common/.mozilla/firefox",
    "~/.mozilla/firefox"
];



#[cfg(target_os = "macos")] 
pub const CHROME_PATHS: &'static [&'static str] = &[
    "%LOCALAPPDATA%/Google/Chrome/User Data"
];

#[cfg(target_os = "macos")] 
pub const BRAVE_PATHS: &'static [&'static str] = &[
    "%LOCALAPPDATA%/BraveSoftware/Brave-Browser/User Data"
];

#[cfg(target_os = "macos")] 
pub const EDGE_PATHS: &'static [&'static str] = &[
    "%LOCALAPPDATA%/Microsoft/Edge/User Data"
];

#[cfg(target_os = "macos")] 
pub const FIREFOX_PATHS: &'static [&'static str] = &[
    "~/Library/Application Support/Firefox"
];


