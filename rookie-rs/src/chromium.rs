use std::{path::PathBuf, error::Error};
use serde_json;

#[cfg(target_os = "windows")]
use aes_gcm::{Aes256Gcm, Key,aead::{Aead, KeyInit, generic_array::GenericArray}};

use crate::enums::*;
use crate::utils::*;
use crate::sqlite;

#[cfg(target_os = "windows")]
use base64::{Engine as _, engine::general_purpose};

#[cfg(target_os = "windows")]
use crate::winapi;


#[cfg(target_os = "windows")]
fn get_v10_key(key64: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut keydpapi: Vec<u8> = general_purpose::STANDARD.decode(&key64)?;
    let keydpapi = &mut keydpapi[5..];
    let v10_key = winapi::decrypt(keydpapi)?;
    Ok(v10_key)
}



#[cfg(target_os = "linux")]
fn get_v10_key() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use pbkdf2::pbkdf2_hmac;
    use sha1::Sha1;

    let mut output = [0u8; 16];
    let salt = b"saltysalt";
    let iterations = 1;
    let password = b"peanuts";

    pbkdf2_hmac::<Sha1>(password, salt, iterations, &mut output);
    Ok(output.to_vec())
}



#[cfg(target_os = "macos")]
fn get_v10_key() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use pbkdf2::pbkdf2_hmac;
    use sha1::Sha1;

    let mut output = [0u8; 16];
    let salt = b"saltysalt";
    let iterations = 1;
    let password = b"peanuts";

    pbkdf2_hmac::<Sha1>(password, salt, iterations, &mut output);
    Ok(output.to_vec())
}

#[cfg(target_os = "windows")]
fn decrypt_encrypted_value(value: String, encrypted_value: &[u8], key: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let key_type = &encrypted_value[..3];
    if !value.is_empty() || !(key_type == b"v11" || key_type == b"v10") { // unknown key_type or value isn't encrypted
        return Ok(value);
    }
    let encrypted_value = &encrypted_value[3..];
    let nonce = &encrypted_value[..12];
    let ciphertext = &encrypted_value[12..];

    // Create a new AES block cipher.
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = GenericArray::from_slice(nonce); // 96-bits; unique per message
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).or(Err("cant decrypt"))?;
    let plaintext = String::from_utf8(plaintext).or(Err("cant decode encrypted value"))?;
    Ok(plaintext)
}

#[cfg(unix)]
fn decrypt_encrypted_value(value: String, encrypted_value: &[u8], key: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let key_type = &encrypted_value[..3];
    if !value.is_empty() || !(key_type == b"v11" || key_type == b"v10") { // unknown key_type or value isn't encrypted
        return Ok(value);
    }
    use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
    
    type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;


    // Create an AES-128 cipher with the provided key.


    let encrypted_value = & mut encrypted_value.to_owned()[3..];
    let iv: [u8; 16] = [b' '; 16];

    let mut  key_array: [u8;16] = [0;16];
    key_array.copy_from_slice(&key[..16]);
    let cipher = Aes128CbcDec::new(&key_array.into(), &iv.into());

    let plaintext = cipher.decrypt_padded_mut::<Pkcs7>(encrypted_value).or(Err("cant decrypt value"))?;


    Ok(String::from_utf8(plaintext.to_vec())?)
}


fn query_cookies(v10_key: Vec<u8>, db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {    
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let db_path_str = db_path.to_str().ok_or("Cant convert db path to str")?;
            unsafe {winapi::release_file_lock(db_path_str);}
        }
    }
    let connection = sqlite::connect(db_path)?;
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
        let expires = epoch_to_systemtime_micros(expires_nt_time_epoch);
        let name: String = row.get(4)?;
        
        let value: String = row.get(5)?;
        let encrypted_value: Vec<u8> = row.get(6)?;
        let decrypted_value = decrypt_encrypted_value(value, &encrypted_value, &v10_key)?;
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
    
    let content = std::fs::read_to_string(&key)?;
    let key_dict: serde_json::Value = serde_json::from_str(content.as_str()).or(Err("Cant read json file"))?;

    let os_crypt = key_dict
        .get("os_crypt")
        .ok_or("can't get os crypt")?;

    let key64 = os_crypt.get("encrypted_key")
        .ok_or("cant get encrypted_key")?
        .as_str()
        .ok_or("Cant convert encrypted_key to str")?;

    let v10_key = get_v10_key(key64)?;
    query_cookies(v10_key, db_path, domains)
}


#[cfg(target_os = "linux")]
pub fn chromium_based(_: PathBuf, db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let v10_key = get_v10_key()?;
    query_cookies(v10_key, db_path, domains)
}


#[cfg(target_os = "macos")]
pub fn chromium_based(_: PathBuf, db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let v10_key = get_v10_key()?;
    query_cookies(v10_key, db_path, domains)
}