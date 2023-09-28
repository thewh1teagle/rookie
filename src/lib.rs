mod enums;
mod winapi;
mod paths;
mod chromium;
mod mozilla;
mod utils;
use std::error::Error;

use chromium::chromium_based;
use mozilla::firefox_based;
use enums::*;



pub fn firefox() -> Result<Vec<Cookie>, Box<dyn Error>> {
    let db_path = paths::find_firefox_paths();
    firefox_based(db_path)
}

pub fn chrome() -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_chrome_paths();
    chromium_based(key, db_path)
}


pub fn brave() -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_brave_paths();
    chromium_based(key, db_path)
}

pub fn edge() -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_edge_paths();
    chromium_based(key, db_path)
}

