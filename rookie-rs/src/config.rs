use crate::enums::BrowserConfig;
/*
  *********** WINDOWS **********
*/
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

#[cfg(target_os = "windows")] 
pub static VIVALDI_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%LOCALAPPDATA%/Vivaldi/User Data/Default/Cookies",
        "%LOCALAPPDATA%/Vivaldi/User Data/Default/Network/Cookies",
        "%LOCALAPPDATA%/Vivaldi/User Data/Profile */Cookies",
        "%LOCALAPPDATA%/Vivaldi/User Data/Profile */Network/Cookies",

        "%APPDATA%/Vivaldi/User Data/Default/Cookies",
        "%APPDATA%/Vivaldi/User Data/Default/Network/Cookies",
        "%APPDATA%/Vivaldi/User Data/Profile */Cookies",
        "%APPDATA%/Vivaldi/User Data/Profile */Network/Cookies"
    ],
    channels: &[""],
};

#[cfg(target_os = "windows")] 
pub static OPERA_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%LOCALAPPDATA%/Opera Software/Opera {channel}/Cookies",
        "%LOCALAPPDATA%/Opera Software/Opera {channel}/Network/Cookies",

        "%APPDATA%/Opera Software/Opera {channel}/Cookies",
        "%APPDATA%/Opera Software/Opera {channel}/Network/Cookies"
    ],
    channels: &["Stable", "Next", "Developer"],
};

#[cfg(target_os = "windows")] 
pub static CHROMIUM_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%LOCALAPPDATA%/Chromium/User Data/Default/Cookies",
        "%LOCALAPPDATA%/Chromium/User Data/Default/Network/Cookies",
        "%LOCALAPPDATA%/Chromium/User Data/Profile */Cookies",
        "%LOCALAPPDATA%/Chromium/User Data/Profile */Network/Cookies",

        "%APPDATA%/Chromium/User Data/Default/Cookies",
        "%APPDATA%/Chromium/User Data/Default/Network/Cookies",
        "%APPDATA%/Chromium/User Data/Profile */Cookies",
        "%APPDATA%/Chromium/User Data/Profile */Network/Cookies"
    ],
    channels: &[""],
};


#[cfg(target_os = "windows")] 
pub static OPERA_GX_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%LOCALAPPDATA%/Opera Software/Opera GX {channel}/Cookies",
        "%LOCALAPPDATA%/Opera Software/Opera GX {channel}/Network/Cookies",

        "%APPDATA%/Opera Software/Opera GX {channel}/Cookies",
        "%APPDATA%/Opera Software/Opera GX {channel}/Network/Cookies"
    ],
    channels: &["Stable", ""],
};

#[cfg(target_os = "windows")] 
pub static LIBRE_WOLF_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%LOCALAPPDATA%/librewolf",
        "%APPDATA%/librewolf",
    ],
    channels: &[""],
};

#[cfg(target_os = "windows")] 
pub static IE_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%APPDATA%/Microsoft/Windows/WebCache/WebCacheV01.dat",
        "%LOCALAPPDATA%/Microsoft/Windows/WebCache/WebCacheV01.dat",
    ],
    channels: &[""],
};


/*
  *********** LINUX **********
*/

// Initialize the CHROME_CONFIG as a static variable with specific values
#[cfg(target_os = "linux")] 
pub static CHROME_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
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
pub static VIVALDI_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/.config/vivaldi/Default/Cookies",
        "~/.config/vivaldi/Profile */Cookies",
        "~/.config/vivaldi-snapshot/Default/Cookies",
        "~/.config/vivaldi-snapshot/Profile */Cookies",
        "~/.var/app/com.vivaldi.Vivaldi/config/vivaldi/Default/Cookies",
        "~/.var/app/com.vivaldi.Vivaldi/config/vivaldi/Profile */Cookies"
    ],
    channels: &[""],
};

#[cfg(target_os = "linux")] 
pub static OPERA_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/snap/opera/*/.config/opera/Cookies",
        "~/.config/opera/Cookies",
        "~/.config/opera-beta/Cookies",
        "~/.config/opera-developer/Cookies",
        "~/.var/app/com.opera.Opera/config/opera/Cookies",
        "~/.var/app/com.opera.Opera/config/opera-beta/Cookies",
        "~/.var/app/com.opera.Opera/config/opera-developer/Cookies"
    ],
    channels: &["Stable", "Next", "Developer"],
};


#[cfg(target_os = "linux")] 
pub static CHROMIUM_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/snap/chromium/common/chromium/Default/Cookies",
        "~/.config/chromium/Default/Cookies",
        "~/.config/chromium/Profile */Cookies",
        "~/.var/app/org.chromium.Chromium/config/chromium/Default/Cookies",
        "~/.var/app/org.chromium.Chromium/config/chromium/Profile */Cookies"
    ],
    channels: &[""],
};



#[cfg(target_os = "linux")] 
pub static FIREFOX_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/snap/firefox/common/.mozilla/firefox",
        "~/.mozilla/firefox"
    ],
    channels: &[""],
};

#[cfg(target_os = "linux")] 
pub static LIBRE_WOLF_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/snap/librewolf/common/.librewolf",
        "~/.librewolf"
    ],
    channels: &[""],
};

#[cfg(target_os = "linux")] 
pub static OPERA_GX_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[], // not available on Linux
    channels: &["", ""],
};



/*
  *********** MACOS **********
*/

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

#[cfg(target_os = "macos")] 
pub static LIBRE_WOLF_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/Library/Application Support/librewolf"
    ],
    channels: &[""],
};


#[cfg(target_os = "macos")] 
pub static VIVALDI_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/Library/Application Support/Vivaldi/Default/Cookies",
        "~/Library/Application Support/Vivaldi/Profile */Cookies"
    ],
    channels: &[""],
};

#[cfg(target_os = "macos")] 
pub static OPERA_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "%LOCALAPPDATA%/Vivaldi/User Data/Default/Cookies",
        "%LOCALAPPDATA%/Vivaldi/User Data/Default/Network/Cookies",
        "%LOCALAPPDATA%/Vivaldi/User Data/Profile */Cookies",
        "%LOCALAPPDATA%/Vivaldi/User Data/Profile */Network/Cookies",

        "%APPDATA%/Vivaldi/User Data/Default/Cookies",
        "%APPDATA%/Vivaldi/User Data/Default/Network/Cookies",
        "%APPDATA%/Vivaldi/User Data/Profile */Cookies",
        "%APPDATA%/Vivaldi/User Data/Profile */Network/Cookies"
    ],
    channels: &["Stable", "Next", "Developer"],
};

#[cfg(target_os = "macos")] 
pub static CHROMIUM_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/Library/Application Support/Chromium/Default/Cookies",
        "~/Library/Application Support/Chromium/Profile */Cookies"
    ],
    channels: &[""],
};

#[cfg(target_os = "macos")] 
pub static OPERA_GX_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &["~/Library/Application Support/com.operasoftware.OperaGX/Cookies"],
    channels: &["Stable", ""],
};


#[cfg(target_os = "macos")] 
pub static SAFARI_CONFIG: BrowserConfig<'static> = BrowserConfig {
    data_paths: &[
        "~/Library/Containers/com.apple.Safari/Data/Library/Cookies/Cookies.binarycookies",
        "~/Library/Cookies/Cookies.binarycookies"
    ],
    channels: &[""],
};
