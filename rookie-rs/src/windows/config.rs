use crate::common::enums::BrowserConfig;

// Initialize the CHROME_CONFIG as a static variable with specific values
pub static CHROME_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "%LOCALAPPDATA%/Google/Chrome{channel}/User Data/Default/Cookies",
    "%LOCALAPPDATA%/Google/Chrome{channel}/User Data/Default/Network/Cookies",
    "%LOCALAPPDATA%/Google/Chrome{channel}/User Data/Profile */Cookies",
    "%LOCALAPPDATA%/Google/Chrome{channel}/User Data/Profile */Network/Cookies",
    "%APPDATA%/Google/Chrome{channel}/User Data/Default/Cookies",
    "%APPDATA%/Google/Chrome{channel}/User Data/Default/Network/Cookies",
    "%APPDATA%/Google/Chrome{channel}/User Data/Profile */Cookies",
    "%APPDATA%/Google/Chrome{channel}/User Data/Profile */Network/Cookies",
  ],
  channels: Some(&["", "-beta", "-dev", "-nightly"]),
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static BRAVE_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "%LOCALAPPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Default/Cookies",
    "%LOCALAPPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Default/Network/Cookies",
    "%LOCALAPPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Profile */Cookies",
    "%LOCALAPPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Profile */Network/Cookies",
    "%APPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Default/Cookies",
    "%APPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Default/Network/Cookies",
    "%APPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Profile */Cookies",
    "%APPDATA%/BraveSoftware/Brave-Browser{channel}/User Data/Profile */Network/Cookies",
  ],
  channels: Some(&["", "-beta", "-dev", "-nightly"]),
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static EDGE_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "%LOCALAPPDATA%/Microsoft/Edge{channel}/User Data/Default/Cookies",
    "%LOCALAPPDATA%/Microsoft/Edge{channel}/User Data/Default/Network/Cookies",
    "%LOCALAPPDATA%/Microsoft/Edge{channel}/User Data/Profile */Cookies",
    "%LOCALAPPDATA%/Microsoft/Edge{channel}/User Data/Profile */Network/Cookies",
    "%APPDATA%/Microsoft/Edge{channel}/User Data/Default/Cookies",
    "%APPDATA%/Microsoft/Edge{channel}/User Data/Default/Network/Cookies",
    "%APPDATA%/Microsoft/Edge{channel}/User Data/Profile */Cookies",
    "%APPDATA%/Microsoft/Edge{channel}/User Data/Profile */Network/Cookies",
  ],
  channels: Some(&["", "-beta", "-dev", "-nightly"]),
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static FIREFOX_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "%APPDATA%/Mozilla/Firefox",
    "%LOCALAPPDATA%/Mozilla/Firefox",
  ],
  channels: Some(&["", "-beta", "-dev", "-nightly"]),
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static VIVALDI_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "%LOCALAPPDATA%/Vivaldi/User Data/Default/Cookies",
    "%LOCALAPPDATA%/Vivaldi/User Data/Default/Network/Cookies",
    "%LOCALAPPDATA%/Vivaldi/User Data/Profile */Cookies",
    "%LOCALAPPDATA%/Vivaldi/User Data/Profile */Network/Cookies",
    "%APPDATA%/Vivaldi/User Data/Default/Cookies",
    "%APPDATA%/Vivaldi/User Data/Default/Network/Cookies",
    "%APPDATA%/Vivaldi/User Data/Profile */Cookies",
    "%APPDATA%/Vivaldi/User Data/Profile */Network/Cookies",
  ],
  channels: None,
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static OPERA_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "%LOCALAPPDATA%/Opera Software/Opera {channel}/Cookies",
    "%LOCALAPPDATA%/Opera Software/Opera {channel}/Network/Cookies",
    "%APPDATA%/Opera Software/Opera {channel}/Cookies",
    "%APPDATA%/Opera Software/Opera {channel}/Network/Cookies",
  ],
  channels: Some(&["Stable", "Next", "Developer"]),
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static CHROMIUM_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "%LOCALAPPDATA%/Chromium/User Data/Default/Cookies",
    "%LOCALAPPDATA%/Chromium/User Data/Default/Network/Cookies",
    "%LOCALAPPDATA%/Chromium/User Data/Profile */Cookies",
    "%LOCALAPPDATA%/Chromium/User Data/Profile */Network/Cookies",
    "%APPDATA%/Chromium/User Data/Default/Cookies",
    "%APPDATA%/Chromium/User Data/Default/Network/Cookies",
    "%APPDATA%/Chromium/User Data/Profile */Cookies",
    "%APPDATA%/Chromium/User Data/Profile */Network/Cookies",
  ],
  channels: None,
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static ARC_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "%LOCALAPPDATA%/Packages/TheBrowserCompany.Arc*/LocalCache/Local/Arc/User Data/Default/Network/Cookies",
    "%LOCALAPPDATA%/Packages/TheBrowserCompany.Arc*/LocalCache/Local/Arc/User Data/Profile */Network/Cookies",
  ],
  channels: None,
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static OPERA_GX_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "%LOCALAPPDATA%/Opera Software/Opera GX {channel}/Cookies",
    "%LOCALAPPDATA%/Opera Software/Opera GX {channel}/Network/Cookies",
    "%APPDATA%/Opera Software/Opera GX {channel}/Cookies",
    "%APPDATA%/Opera Software/Opera GX {channel}/Network/Cookies",
  ],
  channels: Some(&["Stable", ""]),
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static OCTO_BROWSER_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "%LOCALAPPDATA%/Octo Browser/tmp/*/Default/Network/Cookies",
    "%APPDATA%/Octo Browser/tmp/*/Default/Network/Cookies",
  ],
  channels: Some(&["Stable", ""]),
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static LIBREWOLF_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &["%LOCALAPPDATA%/librewolf", "%APPDATA%/librewolf"],
  channels: None,
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static IE_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "%APPDATA%/Microsoft/Windows/WebCache/WebCacheV01.dat",
    "%LOCALAPPDATA%/Microsoft/Windows/WebCache/WebCacheV01.dat",
  ],
  channels: None,
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};
