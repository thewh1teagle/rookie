use ini::Ini;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::{error::Error, path::PathBuf};
use crate::{enums::*, sqlite, utils, date};
use lz4_flex::block::decompress_size_prepended;

pub fn firefox_based(
    db_path: PathBuf,
    domains: Option<Vec<&str>>,
) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let connection = sqlite::connect(db_path.clone())?;
    let mut query = "
        SELECT host, path, isSecure, expiry, name, value, isHttpOnly, sameSite from moz_cookies 
    "
    .to_string();

    if let Some(domains) = domains.clone() {
        let domain_queries: Vec<String> = domains
            .iter()
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
        let expires: u64 = row.get(3)?;
        let expires = date::mozilla_timestamp(expires);

        let name: String = row.get(4)?;

        let value: String = row.get(5)?;
        let http_only: bool = row.get(6)?;

        let same_site: i64 = row.get(7)?;
        let cookie = Cookie {
            domain: host.to_string(),
            path: path.to_string(),
            secure: is_secure,
            expires,
            name: name.to_string(),
            value,
            http_only,
            same_site,
        };
        cookies.push(cookie);
    }

    let parent_path = db_path.parent().unwrap_or(&PathBuf::from("")).to_path_buf();
    if let Ok(session_cookies) = get_session_cookies_lz4(domains.to_owned(), parent_path.to_owned())
    {
        cookies.extend(session_cookies);
    }

    if let Ok(session_cookies) = get_session_cookies(domains, parent_path) {
        cookies.extend(session_cookies);
    }
    Ok(cookies)
}

pub fn get_session_cookies(
    domains: Option<Vec<&str>>,
    cookies_dir: PathBuf,
) -> Result<Vec<Cookie>, Box<dyn std::error::Error>> {
    let mut cookies: Vec<Cookie> = vec![];
    let session_file = cookies_dir.join("sessionstore.js");
    let plain = fs::read_to_string(session_file)?;
    let json: Value = serde_json::from_str(&plain)?;
    let windows = json
        .get("windows")
        .ok_or("no windows in json")?
        .as_array()
        .ok_or("windows are not array")?;
    for window in windows {
        let may_cookies_json = window.get("cookies");
        if let Some(cookies_json) = may_cookies_json {
            let cookies_json = cookies_json.as_array();
            if let Some(cookies_json) = cookies_json {
                for json_cookie in cookies_json {
                    let domain = json_cookie
                        .get("host")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let should_add = domains.is_none() || // add every domain
                        domains.is_some() && utils::some_domain_in_host(domains.to_owned(), domain); // only if some domain in host
                    if !should_add {
                        continue;
                    }
                    if let Ok(cookie) = create_cookie(json_cookie) {
                        cookies.push(cookie);
                    }
                }
            }
        }
    }
    Ok(cookies)
}

pub fn get_session_cookies_lz4(
    domains: Option<Vec<&str>>,
    cookies_dir: PathBuf,
) -> Result<Vec<Cookie>, Box<dyn std::error::Error>> {
    let mut cookies: Vec<Cookie> = vec![];
    let session_file_lz4 = cookies_dir.join("sessionstore-backups/recovery.jsonlz4");
    let compressed = fs::read(session_file_lz4)?;
    let compressed = compressed[8..].to_vec();
    let decompressed = decompress_size_prepended(&compressed)?;
    let plain = String::from_utf8(decompressed)?;
    let json: Value = serde_json::from_str(&plain)?;
    let cookies_json = json.get("cookies").ok_or("no cookies in json")?;
    let cookies_json = cookies_json.as_array().ok_or("cookies is not list")?;
    for json_cookie in cookies_json {
        let domain = json_cookie
            .get("host")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let should_add = domains.is_none() || // add every domain
                        utils::some_domain_in_host(domains.to_owned(), domain); // only if some domain in host
        if !should_add {
            continue;
        }
        if let Ok(cookie) = create_cookie(json_cookie) {
            cookies.push(cookie);
        }
    }
    Ok(cookies)
}

pub fn create_cookie(json_cookie: &Value) -> Result<Cookie, Box<dyn std::error::Error>> {
    let host = json_cookie
        .get("host")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let path = json_cookie
        .get("path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let secure = json_cookie
        .get("secure")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let name = json_cookie
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let value = json_cookie
        .get("value")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let http_only = json_cookie
        .get("httponly")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let expires = json_cookie
        .get("expiry")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let expires = date::mozilla_timestamp(expires);

    let same_site = json_cookie
        .get("sameSite")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    let cookie = Cookie {
        domain: host.to_string(),
        expires,
        http_only,
        name: name.to_string(),
        value: value.to_string(),
        path: path.to_string(),
        same_site,
        secure,
    };
    Ok(cookie)
}

pub fn get_default_profile(profiles_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let conf = Ini::load_from_file(profiles_path)?;
    for (sec, prop) in conf.iter() {
        let name: &str = sec.unwrap_or("");
        if name.starts_with("Profile0") {
            let path: &str = prop.get("Path").ok_or("Cant get path from profile0")?;
            return Ok(path.to_string());
        }
    }
    Err("Cant find any profile".into())
}
