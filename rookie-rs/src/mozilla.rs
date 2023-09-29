use std::path::Path;
use std::time::UNIX_EPOCH;
use ini::Ini;
use std::{path::PathBuf, error::Error};

use crate::sqlite;
use crate::enums::*;

pub fn firefox_based(db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let connection = sqlite::connect(db_path);
    let mut query = "
        SELECT host, path, isSecure, expiry, name, value, isHttpOnly, sameSite from moz_cookies 
    ".to_string();

    if let Some(domains) = domains {
        let domain_queries: Vec<String> = domains.iter()
            .map(|domain| format!("host LIKE '%{}%'", domain))
            .collect();
        
        if !domain_queries.is_empty() {
            let joined_queries = domain_queries.join(" OR ");
            query += &format!("WHERE ({})", joined_queries);
        }
    }
    
    query += ";";

    let mut cookies: Vec<Cookie> = vec![];
    let mut stmt = connection.prepare(query.as_str())?;
    let mut rows = stmt.query([])?;


    while let Some(row) = rows.next()? {
        let host: String = row.get(0)?;
        let path: String = row.get(1)?;
        let is_secure: bool = row.get(2)?;
        let expires_nt_time_epoch: i64 = row.get(3)?;
        let duration = std::time::Duration::from_secs(expires_nt_time_epoch as u64);
        let expires = UNIX_EPOCH + duration;

        let name: String = row.get(4)?;
        

        let value: String = row.get(5)?;
        let http_only: bool = row.get(6)?;
        
        let same_site: i64 = row.get(7)?;
        let cookie = Cookie {
            host: host.to_string(),
            path: path.to_string(),
            secure: is_secure,
            expires,
            name: name.to_string(),
            value,
            http_only,
            same_site
        };
        cookies.push(cookie);
    }
    Ok(cookies)
}


pub fn get_default_profile(profiles_path: &Path) -> Option<String> {
    let conf = Ini::load_from_file(profiles_path).unwrap();  
    for (sec, prop) in conf.iter() {
        let name: &str = sec.unwrap_or("");
        if name.starts_with("Profile0") {
            let path: &str = prop.get("Path").unwrap();
            return Some(path.to_string());
        }
    }
    None
}
