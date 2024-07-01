use crate::common::enums::BrowserConfig;

pub static CHROME_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/Library/Application Support/Google/Chrome{channel}/Default/Cookies",
    "~/Library/Application Support/Google/Chrome{channel}/Profile */Cookies",
  ],
  channels: Some(&["", "-beta", "-dev", "-nightly"]),
  os_crypt_name: Some("chrome"),
  osx_key_service: Some("Chrome Safe Storage"),
  osx_key_user: Some("Chrome"),
};

pub static BRAVE_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/Library/Application Support/BraveSoftware/Brave-Browser{channel}/Default/Cookies",
    "~/Library/Application Support/BraveSoftware/Brave-Browser{channel}/Profile */Cookies",
  ],
  channels: Some(&["", "-beta", "-dev", "-nightly"]),
  os_crypt_name: Some("brave"),
  osx_key_service: Some("Brave Safe Storage"),
  osx_key_user: Some("Brave"),
};

pub static EDGE_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/Library/Application Support/Microsoft Edge{channel}/Default/Cookies",
    "~/Library/Application Support/Microsoft Edge{channel}/Profile */Cookies",
  ],
  channels: Some(&["", " Beta", " Dev", " Canary"]),
  os_crypt_name: Some("chromium"),
  osx_key_service: Some("Microsoft Edge Safe Storage"),
  osx_key_user: Some("Microsoft Edge"),
};

pub static FIREFOX_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &["~/Library/Application Support/Firefox"],
  channels: None,
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static LIBREWOLF_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &["~/Library/Application Support/librewolf"],
  channels: None,
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static VIVALDI_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/Library/Application Support/Vivaldi/Default/Cookies",
    "~/Library/Application Support/Vivaldi/Profile */Cookies",
  ],
  channels: None,
  os_crypt_name: Some("chrome"),
  osx_key_service: Some("Vivaldi Safe Storage"),
  osx_key_user: Some("Vivaldi"),
};

pub static OPERA_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/Library/Application Support/com.operasoftware.Opera/Cookies",
    "~/Library/Application Support/com.operasoftware.OperaNext/Cookies",
    "~/Library/Application Support/com.operasoftware.OperaDeveloper/Cookies",
  ],
  channels: Some(&["Stable", "Next", "Developer"]),
  os_crypt_name: Some("chromium"),
  osx_key_service: Some("Opera Safe Storage"),
  osx_key_user: Some("Opera"),
};

pub static CHROMIUM_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/Library/Application Support/Chromium/Default/Cookies",
    "~/Library/Application Support/Chromium/Profile */Cookies",
  ],
  channels: None,
  os_crypt_name: Some("chromium"),
  osx_key_service: Some("Chromium Safe Storage"),
  osx_key_user: Some("Chromium"),
};

pub static ARC_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/Library/Application Support/Arc/User Data/Default/Cookies",
    "~/Library/Application Support/Arc/User Data/Profile */Cookies",
  ],
  channels: Some(&["", "-beta", "-dev", "-nightly"]),
  os_crypt_name: Some("arc"),
  osx_key_service: Some("Arc Safe Storage"),
  osx_key_user: Some("Arc"),
};

pub static OPERA_GX_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &["~/Library/Application Support/com.operasoftware.OperaGX/Cookies"],
  channels: Some(&["Stable", ""]),
  os_crypt_name: Some("chromium"),
  osx_key_service: Some("Opera Safe Storage"),
  osx_key_user: Some("Opera"),
};

pub static SAFARI_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/Library/Containers/com.apple.Safari/Data/Library/Cookies/Cookies.binarycookies",
    "~/Library/Cookies/Cookies.binarycookies",
  ],
  channels: None,
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};
