use crate::common::enums::BrowserConfig;

pub static CHROME_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/.config/google-chrome{channel}/Default/Cookies",
    "~/.config/google-chrome{channel}/Profile */Cookies",
    "~/.var/app/com.google.Chrome/config/google-chrome{channel}/Default/Cookies",
    "~/.var/app/com.google.Chrome/config/google-chrome{channel}/Profile */Cookies",
  ],
  channels: Some(&["", "-beta", "-dev", "-nightly"]),
  os_crypt_name: Some("chrome"),
  osx_key_service: None,
  osx_key_user: None,
};

pub static BRAVE_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/snap/brave/*/.config/BraveSoftware/Brave-Browser/Default/Cookies",
    "~/.config/BraveSoftware/Brave-Browser{channel}/Default/Cookies",
    "~/.config/BraveSoftware/Brave-Browser{channel}/Profile */Cookies",
    "~/.var/app/com.brave.Browser/config/BraveSoftware/Brave-Browser{channel}/Default/Cookies",
    "~/.var/app/com.brave.Browser/config/BraveSoftware/Brave-Browser{channel}/Profile */Cookies",
  ],
  channels: Some(&["", "-beta", "-dev", "-nightly"]),
  os_crypt_name: Some("brave"),
  osx_key_service: None,
  osx_key_user: None,
};

pub static EDGE_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/.config/microsoft-edge{channel}/Default/Cookies",
    "~/.config/microsoft-edge{channel}/Profile */Cookies",
    "~/.var/app/com.microsoft.Edge/config/microsoft-edge{channel}/Default/Cookies",
    "~/.var/app/com.microsoft.Edge/config/microsoft-edge{channel}/Profile */Cookies",
  ],
  channels: Some(&["", "-beta", "-dev", "-nightly"]),
  os_crypt_name: Some("chromium"),
  osx_key_service: None,
  osx_key_user: None,
};

pub static VIVALDI_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/.config/vivaldi/Default/Cookies",
    "~/.config/vivaldi/Profile */Cookies",
    "~/.config/vivaldi-snapshot/Default/Cookies",
    "~/.config/vivaldi-snapshot/Profile */Cookies",
    "~/.var/app/com.vivaldi.Vivaldi/config/vivaldi/Default/Cookies",
    "~/.var/app/com.vivaldi.Vivaldi/config/vivaldi/Profile */Cookies",
  ],
  channels: None,
  os_crypt_name: Some("chrome"),
  osx_key_service: None,
  osx_key_user: None,
};

pub static OPERA_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/snap/opera/*/.config/opera/Default/Cookies",
    "~/snap/opera/*/.config/opera/Cookies",
    "~/.config/opera/Default/Cookies",
    "~/.config/opera/Cookies",
    "~/.var/app/com.opera.Opera/config/opera/Default/Cookies",
    "~/.var/app/com.opera.Opera/config/opera/Cookies",
    "~/snap/opera-beta/*/.config/opera/Default/Cookies",
    "~/snap/opera-beta/*/.config/opera/Cookies",
    "~/.config/opera-beta/Default/Cookies",
    "~/.config/opera-beta/Cookies",
    "~/.var/app/com.opera.Opera/config/opera-beta/Default/Cookies",
    "~/.var/app/com.opera.Opera/config/opera-beta/Cookies",
    "~/snap/opera-developer/*/.config/opera/Default/Cookies",
    "~/snap/opera-developer/*/.config/opera/Cookies",
    "~/.config/opera-developer/Default/Cookies",
    "~/.config/opera-developer/Cookies",
    "~/.var/app/com.opera.Opera/config/opera-developer/Default/Cookies",
    "~/.var/app/com.opera.Opera/config/opera-developer/Cookies",
  ],
  channels: Some(&["Stable", "Next", "Developer"]),
  os_crypt_name: Some("chromium"),
  osx_key_service: None,
  osx_key_user: None,
};

pub static CHROMIUM_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/snap/chromium/common/chromium/Default/Cookies",
    "~/.config/chromium/Default/Cookies",
    "~/.config/chromium/Profile */Cookies",
    "~/.var/app/org.chromium.Chromium/config/chromium/Default/Cookies",
    "~/.var/app/org.chromium.Chromium/config/chromium/Profile */Cookies",
  ],
  channels: None,
  os_crypt_name: Some("chromium"),
  osx_key_service: None,
  osx_key_user: None,
};

pub static ARC_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/snap/arc/common/arc/Default/Cookies",
    "~/.config/arc/Default/Cookies",
    "~/.config/arc/Profile */Cookies",
    "~/.var/app/org.arc.Arc/config/arc/Default/Cookies",
    "~/.var/app/org.arc.Arc/config/arc/Profile */Cookies",
  ],
  channels: None,
  os_crypt_name: Some("arc"),
  osx_key_service: None,
  osx_key_user: None,
};

pub static FIREFOX_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[
    "~/snap/firefox/common/.mozilla/firefox",
    "~/.mozilla/firefox",
    "~/.var/app/org.mozilla.firefox/.mozilla/firefox",
  ],
  channels: None,
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static LIBREWOLF_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &["~/snap/librewolf/common/.librewolf", "~/.librewolf"],
  channels: None,
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static CACHY_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &["~/.cachy"],
  channels: None,
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};

pub static OPERA_GX_CONFIG: BrowserConfig<'static> = BrowserConfig {
  data_paths: &[],
  channels: Some(&["", ""]),
  os_crypt_name: None,
  osx_key_service: None,
  osx_key_user: None,
};
