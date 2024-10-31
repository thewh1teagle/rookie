use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Browser {
  pub paths: Vec<String>,
  pub channels: Option<Vec<String>>,
  pub unix_crypt_name: Option<String>,
  pub osx_key_service: Option<String>,
  pub osx_key_user: Option<String>,
}

pub type BrowsersMap = HashMap<String, Browser>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub version: String,
  pub platforms: HashMap<String, BrowsersMap>,
}

pub static CONFIG: Lazy<Config> =
  Lazy::new(|| serde_json::from_str(include_str!("../config.json")).unwrap());

#[allow(clippy::unnecessary_to_owned)]
pub fn get_browser_config(name: &str) -> &Browser {
  let platform = if cfg!(windows) {
    "windows"
  } else if cfg!(target_os = "macos") {
    "macos"
  } else {
    "linux"
  };
  let config = CONFIG
    .platforms
    .get(platform)
    .unwrap()
    .get(name)
    .unwrap()
    .to_owned();
  config
}
