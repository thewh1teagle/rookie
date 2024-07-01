use lazy_static::lazy_static;
use rookie::{self, enums::Cookie, Result};
use std::collections::HashMap;

type BrowserFn = fn(Option<Vec<String>>) -> Result<Vec<Cookie>>;

lazy_static! {
  pub static ref BROWSERS_MAP: HashMap<String, BrowserFn> = {
    let mut map: HashMap<String, BrowserFn> = HashMap::default();

    map.insert("brave".into(), rookie::brave);

    #[cfg(target_os = "linux")]
    map.insert("cachy".into(), rookie::cachy);

    map.insert("chromium".into(), rookie::chromium);
    map.insert("chrome".into(), rookie::chrome);
    map.insert("edge".into(), rookie::edge);
    map.insert("firefox".into(), rookie::firefox);

    #[cfg(target_os = "windows")]
    map.insert("internet_explorer".into(), rookie::internet_explorer);
    map.insert("librewolf".into(), rookie::librewolf);
    map.insert("opera".into(), rookie::opera);
    map.insert("opera gx".into(), rookie::opera_gx);

    #[cfg(target_os = "macos")]
    map.insert("safari".into(), rookie::safari);

    map.insert("vivaldi".into(), rookie::vivaldi);

    map.insert("arc".into(), rookie::arc);

    map
  };
}

lazy_static! {
  pub static ref BROWSERS_MAP_KEYS: Vec<&'static str> = {
    let keys = BROWSERS_MAP.keys();
    keys.map(|s| s.as_str()).collect()
  };
}
