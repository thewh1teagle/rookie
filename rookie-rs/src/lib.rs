use std::error::Error;

#[cfg(target_os = "windows")]
mod winapi;

mod chromium;
mod paths;
mod sqlite;
mod mozilla;
mod utils;
mod enums;
mod config;
pub use chromium::chromium_based;
pub use mozilla::firefox_based;
pub use enums::*;



pub fn load(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let mut cookies = Vec::new();

    let firefox_cookies = firefox(domains.clone()).unwrap_or(vec![]);
    cookies.extend(firefox_cookies);


    let chrome_cookies = chrome(domains.clone()).unwrap_or(vec![]);
    cookies.extend(chrome_cookies);


    let brave_cookies = brave(domains.clone()).unwrap_or(vec![]);
    cookies.extend(brave_cookies);

    let edge_cookies = edge(domains).unwrap_or(vec![]);
    cookies.extend(edge_cookies);

    Ok(cookies)
}

pub fn firefox(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let db_path = paths::find_mozilla_based_paths(&config::FIREFOX_CONFIG)?;
    firefox_based(db_path, domains)
}

pub fn chrome(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_chrome_based_paths(&config::CHROME_CONFIG)?;
    chromium_based(key, db_path, domains)
}

pub fn chromium(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_chrome_based_paths(&config::CHROMIUM_CONFIG)?;
    chromium_based(key, db_path, domains)
}


pub fn brave(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_chrome_based_paths(&config::BRAVE_CONFIG)?;
    chromium_based(key, db_path, domains)
}

pub fn edge(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_chrome_based_paths(&config::EDGE_CONFIG)?;
    chromium_based(key, db_path, domains)
}


pub fn vivaldi(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_chrome_based_paths(&config::VIVALDI_CONFIG)?;
    chromium_based(key, db_path, domains)
}

pub fn opera(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_chrome_based_paths(&config::OPERA_CONFIG)?;
    chromium_based(key, db_path, domains)
}