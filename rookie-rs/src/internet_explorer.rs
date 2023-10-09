use std::error::Error;
use std::path::PathBuf;
use crate::Cookie;
use crate::date;
use libesedb::EseDb;
use crate::winapi;


pub fn internet_explorer_based(db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    unsafe {
        if let Some(path) = db_path.to_str() {
            winapi::release_file_lock(path);
        }
    }
    let db = EseDb::open(db_path)?;
    let mut cookies: Vec<Cookie> = vec![];

    for table in db.iter_tables()? {
        let table = table?;
        let name: String = table.name()?;
        
        if name.starts_with("CookieEntry") {
            for rec in table.iter_records()? {
                let rec = rec?;
                let host = rec.value(8)?;
                let host = host.as_str().unwrap_or("");
                let path = rec.value(9)?;
                let path = path.as_str().unwrap_or("");
                let name: Vec<u8> = rec.value(10)?.as_bytes().unwrap_or(&[]).to_vec();
                let name = String::from_utf8(name).unwrap_or("".to_string()).trim_matches('\0').to_string();
                let value = rec.value(11)?;
                let same_site = 0;
                let value = String::from_utf8(value.as_bytes().unwrap_or(&[]).to_vec()).unwrap_or("".to_string()).trim_matches('\0').to_string();
                let secure = false;
                let expires = rec.value(4)?.to_u64().unwrap_or(0);
                let expires = date::internet_explorer_timestamp(expires);
                let http_only = false;

                let should_append = domains.is_none() || domains.iter().any(|d| d.contains(&host));
                if should_append {
                    cookies.push(Cookie { domain: host.to_string(), path: path.to_string(), secure, expires, name, value, http_only, same_site })
                }
                
            }
        }
    }
    Ok(cookies)
}