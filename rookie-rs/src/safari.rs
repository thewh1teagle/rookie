use std::path::Path;
use std::time::UNIX_EPOCH;
use ini::Ini;
use std::{path::PathBuf, error::Error};

use crate::sqlite;
use crate::enums::*;

static APPLE_TO_UNIX_TIME: i32 = 978307200;

pub fn safari_based(db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    // 1. open cookies file
    // 2. parse headers
    // 3. parse pages (total from headers)
    // 4. get N cookies from each page, iterate
    // 5. parse each cookie
    // 6. add each cookie based on domain filter
    let cookies: Vec<Cookies> = vec![];
    Ok(cookies)
}



