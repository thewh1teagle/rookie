mod enums;
mod winapi;
mod paths;
mod chromium;
use std::error::Error;

use chromium::chromium_based;
use enums::*;



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
    println!("{} {}", key.as_os_str().to_str().unwrap(), db_path.as_os_str().to_str().unwrap());
    chromium_based(key, db_path)
}

