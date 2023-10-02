use std::error::Error;
use std::path::PathBuf;

use crate::Cookie;
use crate::utils::epoch_to_systemtime_micros;

use libesedb::EseDb;

use crate::winapi;

#[cfg(target_os = "windows")]
pub fn internet_explorer_based(db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    unsafe {
        winapi::release_file_lock(db_path.to_str().unwrap());
    }
    let db = EseDb::open(db_path)?;
    let mut cookies: Vec<Cookie> = vec![];

    for table in db.iter_tables()? {
        let table = table?;
        let name: String = table.name()?;
        
        if name.starts_with("CookieEntry") {
            for rec in table.iter_records().unwrap() {
                let rec = rec?;
                let host = rec.value(8)?;
                let host = host.as_str().unwrap();
                let path = rec.value(9)?;
                let path = path.as_str().unwrap();
                let name: Vec<u8> = rec.value(10)?.as_bytes().unwrap().to_vec();
                let name = String::from_utf8(name).unwrap().trim_matches('\0').to_string();
                let value = rec.value(11)?;
                let same_site = 0;
                let value = String::from_utf8(value.as_bytes().unwrap().to_vec()).unwrap_or("".to_string()).trim_matches('\0').to_string();
                let secure = false;
                let expires = rec.value(4)?.to_i64().unwrap();
                let http_only = false;

                let should_append = domains.is_none() || domains.iter().any(|d| d.contains(&host));
                if should_append {
                    cookies.push(Cookie { host: host.to_string(), path: path.to_string(), secure, expires: epoch_to_systemtime_micros(expires), name, value, http_only, same_site })
                }
                
            }
        }
    }
    Ok(cookies)
}