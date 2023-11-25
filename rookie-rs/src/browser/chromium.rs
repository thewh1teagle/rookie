use std::path::PathBuf;
use crate::common::{enums::*, date, sqlite};
use log::{warn, info};
use anyhow::{Result, anyhow, bail};

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        use aes_gcm::{Aes256Gcm, Key,aead::{Aead, KeyInit, generic_array::GenericArray}};
        use serde_json;
        use base64::{Engine as _, engine::general_purpose};
        use crate::winapi;
    }
    else if #[cfg(unix)] {
        use crate::common::secrets;
    }
}


#[cfg(target_os = "windows")]
fn get_keys(key64: &str) -> Result<Vec<Vec<u8>>> {
    let mut keydpapi: Vec<u8> = general_purpose::STANDARD.decode(&key64)?;
    let keydpapi = &mut keydpapi[5..];
    let v10_key = winapi::decrypt(keydpapi)?;
    let mut keys: Vec<Vec<u8>> = vec![];
    keys.push(v10_key);
    Ok(keys)
}


#[cfg(unix)]
fn create_pbkdf2_key(password: &str, salt: &[u8; 9], iterations: u32) -> Vec<u8> {
    use pbkdf2::pbkdf2_hmac;
    use sha1::Sha1;
    let mut output = [0u8; 16];
    pbkdf2_hmac::<Sha1>(password.as_bytes(), salt, iterations, &mut output);
    return output.to_vec();
}

#[cfg(unix)]
fn get_keys(config: &BrowserConfig) -> Result<Vec<Vec<u8>>> {
    // AES CBC key

    let salt = b"saltysalt";
    let iterations: u32;
    #[cfg(target_os = "linux")]
    {
        iterations = 1;
    }

    #[cfg(target_os = "macos")]
    {
        iterations = 1003;
    }

    let mut keys: Vec<Vec<u8>> = vec![];
    

    cfg_if::cfg_if! {
        if #[cfg(target_os = "linux")] {
            if let Ok(passwords) = secrets::get_passwords(config.os_crypt_name.unwrap_or("")) {
                for password in passwords {
                    let key = create_pbkdf2_key(password.as_str(), salt, iterations);
                    keys.push(key);
                }
            }
            // default keys
            let key = create_pbkdf2_key("peanuts", salt, iterations);
            keys.push(key);
            let key = create_pbkdf2_key("", salt, iterations);
            keys.push(key);
        }
        else if #[cfg(target_os = "macos")] {
            let key_service = config.osx_key_service.ok_or(anyhow!("missing osx_key_service"))?;
            let key_user = config.osx_key_user.ok_or(anyhow!("missing osx_key_user"))?;
            let password = secrets::get_osx_keychain_password(key_service, key_user).unwrap_or("peanuts".to_string());

            // keychain key
            let key = create_pbkdf2_key(password.as_str(), salt, iterations);
            keys.push(key);

            // default keys
            let key = create_pbkdf2_key("peanuts", salt, iterations);
            keys.push(key);
            let key = create_pbkdf2_key("", salt, iterations);
            keys.push(key);

            
        }
    }
    Ok(keys)
    
}


#[cfg(target_os = "windows")]
fn decrypt_encrypted_value(value: String, encrypted_value: &[u8], keys: Vec<Vec<u8>>) -> Result<String> {
    // gcm
    let key_type = &encrypted_value[..3];
    if !value.is_empty() || !(key_type == b"v11" || key_type == b"v10") { // unknown key_type or value isn't encrypted
        return Ok(value);
    }
    let encrypted_value = &encrypted_value[3..];
    let nonce = &encrypted_value[..12];
    let ciphertext = &encrypted_value[12..];

    // Create a new AES block cipher.
    for key in keys {
        let key = Key::<Aes256Gcm>::from_slice(key.as_slice());
        let cipher = Aes256Gcm::new(&key);
        let nonce = GenericArray::from_slice(nonce); // 96-bits; unique per message
        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).or(Err(anyhow!("cant decrypt using key")))?;
        let plaintext = String::from_utf8(plaintext).or(Err(anyhow!("cant decode encrypted value")))?;
        return Ok(plaintext);
    }
    bail!("decrypt_encrypted_value failed")

}

#[cfg(unix)]
fn decrypt_encrypted_value(value: String, encrypted_value: &[u8], keys: Vec<Vec<u8>>) -> Result<String> {
    // cbc
    if !value.is_empty() { // unknown key_type or value isn't encrypted
        return Ok(value);
    }
    if encrypted_value.len() <= 0 {
        return Ok("".into());
    }
    let key_type = &encrypted_value[..3];
    if !(key_type == b"v11" || key_type == b"v10") {
        return Ok(value);
    }
    use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
    
    type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;


    // Create an AES-128 cipher with the provided key.


    let encrypted_value = & mut encrypted_value.to_owned()[3..];
    let iv: [u8; 16] = [b' '; 16];


    for key in &keys {
        let mut  key_array: [u8;16] = [0;16];
        key_array.copy_from_slice(&key[..16]);
        let cipher = Aes128CbcDec::new(&key_array.into(), &iv.into());
        let mut cloned_encrypted_value: Vec<u8> = encrypted_value.to_vec();
        
        if let Ok(plaintext) = cipher.decrypt_padded_mut::<Pkcs7>(&mut cloned_encrypted_value) {
            
            let decoded = String::from_utf8(plaintext.to_vec());
            match decoded {
                Ok(decoded) => {
                    return Ok(decoded);
                }
                Err(_) => {
                    warn!("Error in decode decrypt value with utf8");
                    return Ok("".into());
                }
            }
        }
    }
    bail!("decrypt_encrypted_value failed")

}


fn query_cookies(keys: Vec<Vec<u8>>, db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {    
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let db_path_str = db_path.to_str().ok_or(anyhow!("Cant convert db path to str"))?;
            warn!("Unlocking chrome database, it may take a while (sometimes up to minute)");
            unsafe {winapi::release_file_lock(db_path_str);}
        }
    }

    
    info!("Creating sqlite connection to {}", db_path.to_str().unwrap_or(""));
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
        let expires: u64 = row.get(3)?;
        let expires = date::chromium_timestamp(expires);
        let name: String = row.get(4)?;
        
        let value: String = row.get(5)?;
        let encrypted_value: Vec<u8> = row.get(6)?;
        let decrypted_value = decrypt_encrypted_value(value, &encrypted_value, keys.to_owned())?;
        let http_only: bool = row.get(7)?;
        
        let same_site: i64 = row.get(8)?;
        let cookie = Cookie {
            domain: host_key.to_string(),
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
pub fn chromium_based(key: PathBuf, db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    // Use DPAPI
    let content = std::fs::read_to_string(&key)?;
    let key_dict: serde_json::Value = serde_json::from_str(content.as_str()).or(Err(anyhow!("Cant read json file")))?;

    let os_crypt = key_dict
        .get("os_crypt")
        .ok_or(anyhow!("can't get os crypt"))?;

    let key64 = os_crypt.get("encrypted_key")
        .ok_or(anyhow!("cant get encrypted_key"))?
        .as_str()
        .ok_or(anyhow!("Cant convert encrypted_key to str"))?;

    let keys = get_keys(key64)?;
    query_cookies(keys, db_path, domains)
}


#[cfg(unix)]
pub fn chromium_based(config: &BrowserConfig, db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>> {
    // Simple AES
    let keys = get_keys(&config)?;
    query_cookies(keys, db_path, domains)
}