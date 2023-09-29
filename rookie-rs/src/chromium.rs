use std::{path::PathBuf, fs, error::Error};
use serde_json;
use aes_gcm::{Aes256Gcm, Key,aead::{Aead, KeyInit, generic_array::GenericArray}};

use crate::enums::*;
use crate::utils::*;
use crate::sqlite;

#[cfg(target_os = "windows")]
use base64::{Engine as _, engine::general_purpose};

#[cfg(target_os = "linux")]
use bcrypt_pbkdf;



#[cfg(target_os = "windows")]
fn get_v10_key(key64: &str) -> Vec<u8> {
    use crate::winapi;
    let mut keydpapi: Vec<u8> = general_purpose::STANDARD.decode(&key64).unwrap();
    let keydpapi = &mut keydpapi[5..];
    let v10_key = winapi::decrypt(keydpapi).unwrap();
    v10_key
}



#[cfg(target_os = "linux")]
fn get_v10_key() -> Result<Vec<u8>, bcrypt_pbkdf::Error> {
    let mut output = [0u8; 64];
    bcrypt_pbkdf::bcrypt_pbkdf(b"peanuts", b"saltysalt", 1, output.as_mut())?;
    Ok(output.to_vec())
}

#[cfg(target_os = "macos")]
fn get_v10_key() -> Result<Vec<u8>, bcrypt_pbkdf::Error> {
    let mut output = [0u8; 64];
    bcrypt_pbkdf::bcrypt_pbkdf(b"peanuts", b"saltysalt", 1, output.as_mut())?;
    Ok(output.to_vec())
}

fn decrypt_encrypted_value(value: String, encrypted_value: &[u8], key: &[u8]) -> String {
    let key_type = &encrypted_value[..3];
    if value.is_empty() && (key_type == b"v11" || key_type == b"v10") {
        return value;
    }
    let encrypted_value = &encrypted_value[3..];
    let nonce = &encrypted_value[..12];
    let ciphertext = &encrypted_value[12..];

    // Create a new AES block cipher.
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = GenericArray::from_slice(nonce); // 96-bits; unique per message
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    let plaintext = String::from_utf8(plaintext).unwrap();
    plaintext
}


fn query_cookies(v10_key: Vec<u8>, db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let connection = sqlite::connect(db_path);
    let mut query = "SELECT host_key, path, is_secure, expires_utc, name, value, encrypted_value, is_httponly, samesite FROM cookies ".to_string();

    if let Some(domains) = domains {
        let domain_queries: Vec<String> = domains.iter()
            .map(|domain| format!("host_key LIKE '%{}%'", domain))
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
        let host_key: String = row.get(0)?;
        let path: String = row.get(1)?;
        let is_secure: bool = row.get(2)?;
        let expires_nt_time_epoch: i64 = row.get(3)?;
        let expires = epoch_to_systemtime(expires_nt_time_epoch);
        let name: String = row.get(4)?;
        
        let value: String = row.get(5)?;
        let encrypted_value: Vec<u8> = row.get(6)?;
        let decrypted_value = decrypt_encrypted_value(value, &encrypted_value, &v10_key);
        let http_only: bool = row.get(7)?;
        
        let same_site: i64 = row.get(8)?;
        let cookie = Cookie {
            host: host_key.to_string(),
            path: path.to_string(),
            secure: is_secure,
            expires,
            name: name.to_string(),
            value: decrypted_value,
            http_only,
            same_site
        };
        cookies.push(cookie);
    }
    Ok(cookies)
}




#[cfg(target_os = "windows")]
pub fn chromium_based(key: PathBuf, db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let content = fs::read_to_string(&key).unwrap();
    let key_dict: serde_json::Value = serde_json::from_str(content.as_str()).expect("Cant read json file");
    let key64 = key_dict.get("os_crypt").unwrap().get("encrypted_key").unwrap().as_str().unwrap();
    let v10_key = get_v10_key(key64);
    query_cookies(v10_key, db_path, domains)
}


#[cfg(target_os = "linux")]
pub fn chromium_based(key: PathBuf, db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let content = fs::read_to_string(&key).unwrap();
    let _key_dict: serde_json::Value = serde_json::from_str(content.as_str()).expect("Cant read json file");
    let v10_key = get_v10_key();
    query_cookies(v10_key.unwrap(), db_path, domains)
}


#[cfg(target_os = "macos")]
pub fn chromium_based(key: PathBuf, db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let content = fs::read_to_string(&key).unwrap();
    let _key_dict: serde_json::Value = serde_json::from_str(content.as_str()).expect("Cant read json file");
    let v10_key = get_v10_key();
    query_cookies(v10_key.unwrap(), db_path, domains)
}