use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cookie {
  pub domain: String,
  pub path: String,
  pub secure: bool,
  pub expires: Option<u64>,
  pub name: String,
  pub value: String,
  pub http_only: bool,
  pub same_site: i64,
}

pub trait CookieToString {
  fn to_string(&self) -> String;
}

impl CookieToString for Vec<Cookie> {
  fn to_string(&self) -> String {
    self
      .iter()
      .map(|cookie| format!("{}={}", cookie.name, cookie.value))
      .collect::<Vec<String>>()
      .join(";")
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrowserConfig {
  pub data_paths: Vec<String>,
  pub channels: Option<Vec<String>>,
  pub os_crypt_name: Option<String>,
  pub osx_key_service: Option<String>,
  pub osx_key_user: Option<String>,
}
