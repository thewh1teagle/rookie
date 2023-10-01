use crate::enums::BrowserConfig;

// Initialize the CHROME_CONFIG as a static variable with specific values
#[cfg(target_os = "windows")] 
pub static CHROME_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%LOCALAPPDATA%/Google/Chrome{channel}/User Data/Default/Cookies",
        "%LOCALAPPDATA%/Google/Chrome{channel}/User Data/Default/Network/Cookies",
        "%LOCALAPPDATA%/Google/Chrome{channel}/User Data/Profile */Cookies",
        "%LOCALAPPDATA%/Google/Chrome{channel}/User Data/Profile */Network/Cookie",

        "%APPDATA%/Google/Chrome{channel}/User Data/Default/Cookies",
        "%APPDATA%/Google/Chrome{channel}/User Data/Default/Network/Cookies",
        "%APPDATA%/Google/Chrome{channel}/User Data/Profile */Cookies",
        "%APPDATA%/Google/Chrome{channel}/User Data/Profile */Network/Cookie"
    ],
    channels: &["", "-Beta", "-Dev", "-Nightly"],
};


#[cfg(target_os = "windows")] 
pub static BRAVE_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%LOCALAPPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Default/Cookies",
        "%LOCALAPPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Default/Network/Cookies",
        "%LOCALAPPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Profile */Cookies",
        "%LOCALAPPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Profile */Network/Cookies",

        "%APPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Default/Cookies",
        "%APPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Default/Network/Cookies",
        "%APPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Profile */Cookies",
        "%APPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Profile */Network/Cookies"
    ],
    channels: &["", "-Beta", "-Dev", "-Nightly"],
};

#[cfg(target_os = "windows")] 
pub static EDGE_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%LOCALAPPDATA%/Microsoft/Edge{channel}/User Data/Default/Cookies",
        "%LOCALAPPDATA%/Microsoft/Edge{channel}/User Data/Default/Network/Cookies",
        "%LOCALAPPDATA%/Microsoft/Edge{channel}/User Data/Profile */Cookies",
        "%LOCALAPPDATA%/Microsoft/Edge{channel}/User Data/Profile */Network/Cookies",

        "%APPDATA%/Microsoft/Edge{channel}/User Data/Default/Cookies",
        "%APPDATA%/Microsoft/Edge{channel}/User Data/Default/Network/Cookies",
        "%APPDATA%/Microsoft/Edge{channel}/User Data/Profile */Cookies",
        "%APPDATA%/Microsoft/Edge{channel}/User Data/Profile */Network/Cookies"
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


// Initialize the CHROME_CONFIG as a static variable with specific values
#[cfg(target_os = "linux")] 
pub static CHROME_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/snap/chromium/common/chromium/Default/Cookies",
        "~/.config/google-chrome{channel}/Default/Cookies",
        "~/.config/google-chrome{channel}/Profile */Cookies",
        "~/.var/app/com.google.Chrome/config/google-chrome{channel}/Default/Cookies",
        "~/.var/app/com.google.Chrome/config/google-chrome{channel}/Profile */Cookies"
    ],
    channels: &["", "-Beta", "-Dev", "-Nightly"],
};


#[cfg(target_os = "linux")] 
pub static BRAVE_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/snap/brave/*/.config/BraveSoftware/Brave-Browser/Default/Cookies",
        "~/.config/BraveSoftware/Brave-Browser{channel}/Default/Cookies",
        "~/.config/BraveSoftware/Brave-Browser{channel}/Profile */Cookies",
        "~/.var/app/com.brave.Browser/config/BraveSoftware/Brave-Browser{channel}/Default/Cookies",
        "~/.var/app/com.brave.Browser/config/BraveSoftware/Brave-Browser{channel}/Profile */Cookies"
    ],
    channels: &["", "-Beta", "-Dev", "-Nightly"],
};

#[cfg(target_os = "linux")] 
pub static EDGE_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/.config/microsoft-edge{channel}/Default/Cookies",
        "~/.config/microsoft-edge{channel}/Profile */Cookies",
        "~/.var/app/com.microsoft.Edge/config/microsoft-edge{channel}/Default/Cookies",
        "~/.var/app/com.microsoft.Edge/config/microsoft-edge{channel}/Profile */Cookies",
    ],
    channels: &["", "-Beta", "-Dev", "-Nightly"],
};

#[cfg(target_os = "linux")] 
pub static FIREFOX_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/snap/firefox/common/.mozilla/firefox",
        "~/.mozilla/firefox"
    ],
    channels: &[""],
};


#[cfg(target_os = "macos")] 
pub static CHROME_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/Library/Application Support/Google/Chrome{channel}/Default/Cookies",
        "~/Library/Application Support/Google/Chrome{channel}/Profile */Cookies"
    ],
    channels: &["", "-Beta", "-Dev", "-Nightly"],
};


#[cfg(target_os = "macos")] 
pub static BRAVE_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/Library/Application Support/BraveSoftware/Brave-Browser{channel}/Default/Cookies",
        "~/Library/Application Support/BraveSoftware/Brave-Browser{channel}/Profile */Cookies"
    ],
    channels: &["", "-Beta", "-Dev", "-Nightly"],
};

#[cfg(target_os = "macos")] 
pub static EDGE_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/Library/Application Support/Microsoft Edge{channel}/Default/Cookies",
        "~/Library/Application Support/Microsoft Edge{channel}/Profile */Cookies"
    ],
    channels: &["", " Beta", " Dev", " Canary"],
};

#[cfg(target_os = "macos")] 
pub static FIREFOX_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/Library/Application Support/Firefox"
    ],
    channels: &[""],
};


