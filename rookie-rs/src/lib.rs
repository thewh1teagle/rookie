use std::error::Error;

#[cfg(target_os = "windows")]
mod winapi;

mod chromium;
mod paths;
mod sqlite;
mod mozilla;
mod utils;
mod enums;
mod constants;
pub use chromium::chromium_based;
pub use mozilla::firefox_based;
pub use enums::*;



pub fn firefox(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let db_path = paths::find_mozilla_based_paths(constants::FIREFOX_PATHS)?;
    firefox_based(db_path, domains)
}

pub fn chrome(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_chrome_based_paths(constants::CHROME_PATHS)?;
    chromium_based(key, db_path, domains)
}


pub fn brave(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_chrome_based_paths(constants::BRAVE_PATHS)?;
    chromium_based(key, db_path, domains)
}

pub fn edge(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_chrome_based_paths(constants::EDGE_PATHS)?;
    chromium_based(key, db_path, domains)
}

