#[macro_use]
extern crate napi_derive;

use napi::{Result, Status};
use rookie::enums::Cookie;
use std::path::PathBuf;

#[napi(object)]
pub struct CookieObject {
  pub domain: String,
  pub path: String,
  pub secure: bool,
  pub expires: Option<i64>,
  pub name: String,
  pub value: String,
  pub http_only: bool,
  pub same_site: i64,
}

#[napi]
pub fn version() -> Result<String> {
  Ok(rookie::version())
}

fn cookies_to_js(cookies: Vec<Cookie>) -> Result<Vec<CookieObject>> {
  let mut js_cookies: Vec<CookieObject> = vec![];
  for cookie in cookies {
    js_cookies.push(CookieObject {
      domain: cookie.domain,
      path: cookie.path,
      secure: cookie.secure,
      http_only: cookie.http_only,
      same_site: cookie.same_site,
      expires: cookie.expires.map(|v| v as i64),
      name: cookie.name,
      value: cookie.value,
    });
  }

  Ok(js_cookies)
}

#[napi]
pub fn any_browser(
  db_path: String,
  domains: Option<Vec<String>>,
  key_path: Option<&str>,
) -> Result<Vec<CookieObject>> {
  let cookies = rookie::any_browser(&db_path, domains, key_path)
    .map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;
  cookies_to_js(cookies)
}

/// Common browsers

#[napi]
pub fn firefox(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies =
    rookie::firefox(domains).map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;
  cookies_to_js(cookies)
}

#[napi]
pub fn librewolf(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies =
    rookie::librewolf(domains).map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;
  cookies_to_js(cookies)
}

#[napi]
pub fn chrome(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies =
    rookie::chrome(domains).map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;
  cookies_to_js(cookies)
}

#[napi]
pub fn brave(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies =
    rookie::brave(domains).map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;

  cookies_to_js(cookies)
}

#[napi]
pub fn arc(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies =
    rookie::arc(domains).map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;

  cookies_to_js(cookies)
}

#[napi]
pub fn edge(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies =
    rookie::edge(domains).map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;
  cookies_to_js(cookies)
}

#[napi]
pub fn opera(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies =
    rookie::opera(domains).map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;

  cookies_to_js(cookies)
}

#[napi]
pub fn opera_gx(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies =
    rookie::opera_gx(domains).map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;

  cookies_to_js(cookies)
}

#[napi]
pub fn chromium(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies =
    rookie::chromium(domains).map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;
  cookies_to_js(cookies)
}

#[napi]
pub fn vivaldi(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies =
    rookie::vivaldi(domains).map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;

  cookies_to_js(cookies)
}

#[napi]
pub fn firefox_based(db_path: String, domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies = rookie::firefox_based(PathBuf::from(db_path), domains)
    .map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;
  cookies_to_js(cookies)
}

#[napi]
pub fn load(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies =
    rookie::load(domains).map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;
  cookies_to_js(cookies)
}

/// Windows only browsers

#[napi]
#[cfg(target_os = "windows")]
pub fn octo_browser(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies = rookie::octo_browser(domains)
    .map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;

  cookies_to_js(cookies)
}

#[napi]
#[cfg(target_os = "windows")]
pub fn internet_explorer(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies = rookie::internet_explorer(domains)
    .map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;
  cookies_to_js(cookies)
}
#[napi]
#[cfg(target_os = "windows")]
pub fn chromium_based(
  key_path: String,
  db_path: String,
  domains: Option<Vec<String>>,
) -> Result<Vec<CookieObject>> {
  let cookies = rookie::chromium_based(PathBuf::from(key_path), PathBuf::from(db_path), domains)
    .map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;
  cookies_to_js(cookies)
}

/// MacOS browsers

#[napi]
#[cfg(target_os = "macos")]
pub fn safari(domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  let cookies =
    rookie::safari(domains).map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;
  cookies_to_js(cookies)
}

/// Unix browsers

#[napi]
#[cfg(unix)]
pub fn chromium_based(db_path: String, domains: Option<Vec<String>>) -> Result<Vec<CookieObject>> {
  use rookie::common::enums::BrowserConfig;

  let db_path = db_path.as_str();
  let config = BrowserConfig {
    channels: None,
    data_paths: &[db_path],
    os_crypt_name: Some("chrome"),
    osx_key_service: None,
    osx_key_user: None,
  };
  let cookies = rookie::chromium_based(&config, PathBuf::from(db_path), domains)
    .map_err(|e| napi::Error::new(Status::Unknown, format!("{e:?}")))?;
  cookies_to_js(cookies)
}
