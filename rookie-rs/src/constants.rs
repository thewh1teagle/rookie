use crate::enums::BrowserConfig;

// Initialize the CHROME_CONFIG as a static variable with specific values
#[cfg(target_os = "windows")] 
pub static CHROME_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%LOCALAPPDATA%/Google/Chrome{channel}/User Data", 
        "%APPDATA%/Google/Chrome/User Data"
    ],
    channels: &["", "-Beta", "-Dev", "-Nightly"],
};


#[cfg(target_os = "windows")] 
pub static BRAVE_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%LOCALAPPDATA%/BraveSoftware/Brave-Browser{channel}/User Data", 
        "%APPDATA%/BraveSoftware/Brave-Browser{channel}/User Data", 
    ],
    channels: &["", "-Beta", "-Dev", "-Nightly"],
};

#[cfg(target_os = "windows")] 
pub static EDGE_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%LOCALAPPDATA%/Microsoft/Edge{channel}/User Data", 
        "%APPDATA%/Microsoft/Edge{channel}/User Data", 
    ],
    channels: &["", "-Beta", "-Dev", "-Nightly"],
};

#[cfg(target_os = "windows")] 
pub static FIREFOX_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
    "%APPDATA%/Mozilla/Firefox",
    "%LOCALAPPDATA%/Mozilla/Firefox"
    ],
    channels: &["", "-Beta", "-Dev", "-Nightly"],
};


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


#[cfg(target_os = "linux")] 
pub static FIREFOX_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/snap/firefox/common/.mozilla/firefox",
        "~/.mozilla/firefox"
    ],
    channels: &[],
};


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


