#[macro_use]
extern crate napi_derive;

use napi::{Env, JsObject, Result};
use rookie::enums::Cookie;
use std::path::PathBuf;

fn cookies_to_js(env: &Env, cookies: Vec<Cookie>) -> Result<Vec<JsObject>> {
  let mut js_cookies: Vec<napi::JsObject> = vec![];
  for cookie in cookies {
    let mut obj = env.create_object().unwrap();
    obj.set("domain", cookie.domain)?;
    obj.set("path", cookie.path)?;
    obj.set("secure", cookie.secure)?;
    obj.set("http_only", cookie.http_only)?;
    obj.set("same_site", cookie.same_site)?;
    obj.set("expires", cookie.expires.map(|e| e as i32))?;
    obj.set("name", cookie.name)?;
    obj.set("value", cookie.value)?;
    js_cookies.push(obj);
  }

  Ok(js_cookies)
}

#[napi]
pub fn any_browser(
  env: Env,
  db_path: String,
  domains: Option<Vec<&str>>,
  key_path: Option<&str>,
) -> Result<Vec<JsObject>> {
  let cookies = rookie::any_browser(&db_path, domains, key_path).unwrap();
  cookies_to_js(&env, cookies)
}

/// Common browsers

#[napi]
pub fn firefox(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::firefox(domains).unwrap();
  cookies_to_js(&env, cookies)
}

#[napi]
pub fn librewolf(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::librewolf(domains).unwrap();
  cookies_to_js(&env, cookies)
}

#[napi]
pub fn chrome(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::chrome(domains).unwrap();
  cookies_to_js(&env, cookies)
}

#[napi]
pub fn brave(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::brave(domains).unwrap();

  cookies_to_js(&env, cookies)
}

#[napi]
pub fn edge(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::edge(domains).unwrap();
  cookies_to_js(&env, cookies)
}

#[napi]
pub fn opera(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::opera(domains).unwrap();

  cookies_to_js(&env, cookies)
}

#[napi]
pub fn opera_gx(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::opera_gx(domains).unwrap();

  cookies_to_js(&env, cookies)
}

#[napi]
pub fn chromium(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::chromium(domains).unwrap();
  cookies_to_js(&env, cookies)
}

#[napi]
pub fn vivaldi(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::vivaldi(domains).unwrap();

  cookies_to_js(&env, cookies)
}

#[napi]
pub fn firefox_based(
  env: Env,
  db_path: String,
  domains: Option<Vec<&str>>,
) -> Result<Vec<JsObject>> {
  let cookies = rookie::firefox_based(PathBuf::from(db_path), domains).unwrap();
  cookies_to_js(&env, cookies)
}

#[napi]
pub fn load(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::load(domains).unwrap();
  cookies_to_js(&env, cookies)
}

/// Windows only browsers

#[napi]
#[cfg(target_os = "windows")]
pub fn octo_browser(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::octo_browser(domains).unwrap();

  cookies_to_js(&env, cookies)
}

#[napi]
#[cfg(target_os = "windows")]
pub fn internet_explorer(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::internet_explorer(domains).unwrap();
  cookies_to_js(&env, cookies)
}
#[napi]
#[cfg(target_os = "windows")]
pub fn chromium_based(
  env: Env,
  key_path: String,
  db_path: String,
  domains: Option<Vec<&str>>,
) -> Result<Vec<JsObject>> {
  let cookies =
    rookie::chromium_based(PathBuf::from(key_path), PathBuf::from(db_path), domains).unwrap();
  cookies_to_js(&env, cookies)
}

/// MacOS browsers

#[napi]
#[cfg(target_os = "macos")]
pub fn safari(env: Env, domains: Option<Vec<&str>>) -> Result<Vec<JsObject>> {
  let cookies = rookie::safari(domains).unwrap();
  cookies_to_js(&env, cookies)
}

/// Unix browsers

#[napi]
#[cfg(unix)]
pub fn chromium_based(
  env: Env,
  db_path: String,
  domains: Option<Vec<&str>>,
) -> Result<Vec<JsObject>> {
  use rookie::common::enums::BrowserConfig;

  let db_path = db_path.as_str();
  let config = BrowserConfig {
    channels: None,
    data_paths: &[db_path],
    os_crypt_name: Some("chrome"),
    osx_key_service: None,
    osx_key_user: None,
  };
  let cookies = rookie::chromium_based(&config, PathBuf::from(db_path), domains).unwrap();
  cookies_to_js(&env, cookies)
}
