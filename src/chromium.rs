use std::{path::PathBuf, fs, error::Error};
use serde_json;
use rusqlite::{self, OpenFlags};


use base64::{Engine as _, engine::general_purpose};
use aes_gcm::{Aes256Gcm, Key,aead::{Aead, KeyInit, generic_array::GenericArray}};


use crate::winapi;
use crate::enums::*;
use crate::utils::*;


fn get_v10_key(key64: &str) -> Vec<u8> {
    let mut keydpapi: Vec<u8> = general_purpose::STANDARD.decode(&key64).unwrap();
    let keydpapi = &mut keydpapi[5..];
    let v10_key = winapi::decrypt(keydpapi).unwrap();
    v10_key
}


fn decrypt_encrypted_value(value: &[u8], key: &[u8]) -> String {
    let value = &value[3..];
    let nonce = &value[..12];
    let ciphertext = &value[12..];

    // Create a new AES block cipher.
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = GenericArray::from_slice(nonce); // 96-bits; unique per message
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    let plaintext = String::from_utf8(plaintext).unwrap();
    plaintext
}


fn query_cookies(v10_key: Vec<u8>, db_path: PathBuf) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI;
    let conn_str = format!("{}", db_path.canonicalize().unwrap().to_str().unwrap());
    let connection = rusqlite::Connection::open_with_flags(conn_str, flags).unwrap();
    let query = "
        SELECT host_key, path, is_secure, expires_utc, name, value, encrypted_value, is_httponly, samesite
        FROM cookies;
    ";

    let mut cookies: Vec<Cookie> = vec![];
    let mut stmt = connection.prepare(query)?;
    let mut rows = stmt.query([])?;


    while let Some(row) = rows.next()? {
        let host_key: String = row.get(0)?;
        let path: String = row.get(1)?;
        let is_secure: bool = row.get(2)?;
        let expires_nt_time_epoch: i64 = row.get(3)?;
        let expires = epoch_to_systemtime(expires_nt_time_epoch);
        let name: String = row.get(4)?;
        

        let encrypted_value: Vec<u8> = row.get(6)?;
        let decrypted_value = decrypt_encrypted_value(&encrypted_value, &v10_key);
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


pub fn chromium_based(key: PathBuf, db_path: PathBuf) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let content = fs::read_to_string(&key).unwrap();
    let key_dict: serde_json::Value = serde_json::from_str(content.as_str()).expect("Cant read json file");
    let key64 = key_dict.get("os_crypt").unwrap().get("encrypted_key").unwrap().as_str().unwrap();
    let v10_key = get_v10_key(key64);
    query_cookies(v10_key, db_path)
}
