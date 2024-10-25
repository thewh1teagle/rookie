use crate::common::{date, enums::*, sqlite};
use eyre::{bail, Result};
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use crate::windows;

#[cfg(not(target_os = "linux"))]
#[allow(unused)]
use eyre::ContextCompat;

#[cfg(target_os = "macos")]
use crate::macos::secrets;

#[cfg(target_os = "windows")]
use aes_gcm::{
  aead::{generic_array::GenericArray, Aead, KeyInit},
  Aes256Gcm, Key,
};
#[cfg(target_os = "windows")]
use base64::{engine::general_purpose, Engine as _};
#[cfg(target_os = "windows")]
use eyre::Context;

/// Returns cookies from chromium based browser
#[cfg(target_os = "windows")]
pub fn chromium_based(
  key: PathBuf,
  db_path: PathBuf,
  domains: Option<Vec<String>>,
) -> Result<Vec<Cookie>> {
  let content = std::fs::read_to_string(key)?;
  let key_dict: serde_json::Value =
    serde_json::from_str(content.as_str()).context("Can't read json file")?;

  let legacy_key = key_dict["os_crypt"]["encrypted_key"]
    .as_str()
    .unwrap_or_default();

  #[allow(unused)]
  let appbound_key = key_dict["os_crypt"]["app_bound_encrypted_key"]
    .as_str()
    .unwrap_or_default();

  #[cfg(feature = "appbound")]
  {
    let keys = if !appbound_key.is_empty() {
      if !privilege::user::privileged() {
        bail!("Chrome cookies from version v130 can be decrypted only when running as admin due to appbound encryption!")
      }
      let key = crate::windows::appbound::get_keys(appbound_key)?;
      vec![key]
    } else {
      get_keys(legacy_key)?
    };
    query_cookies(keys, db_path, domains)
  }

  #[cfg(not(feature = "appbound"))]
  {
    let keys = get_keys(legacy_key)?;
    query_cookies(keys, db_path, domains)
  }
}

/// Returns cookies from chromium based browser
#[cfg(unix)]
pub fn chromium_based(
  config: &BrowserConfig,
  db_path: PathBuf,
  domains: Option<Vec<String>>,
) -> Result<Vec<Cookie>> {
  // Simple AES
  let keys = get_keys(config)?;
  query_cookies(keys, db_path, domains)
}

#[cfg(unix)]
fn create_pbkdf2_key(password: &str, salt: &[u8; 9], iterations: u32) -> Vec<u8> {
  use pbkdf2::pbkdf2_hmac;
  use sha1::Sha1;
  let mut output = [0u8; 16];
  pbkdf2_hmac::<Sha1>(password.as_bytes(), salt, iterations, &mut output);
  output.to_vec()
}

#[cfg(target_os = "windows")]
fn get_keys(key64: &str) -> Result<Vec<Vec<u8>>> {
  let mut keydpapi: Vec<u8> = general_purpose::STANDARD.decode(key64)?;
  let keydpapi = &mut keydpapi[5..];
  let v10_key = crate::windows::dpapi::decrypt(keydpapi)?;
  let keys: Vec<Vec<u8>> = vec![v10_key];
  Ok(keys)
}

#[cfg(target_os = "linux")]
fn get_keys(config: &BrowserConfig) -> Result<Vec<Vec<u8>>> {
  // AES CBC key

  let salt = b"saltysalt";

  let iterations = 1;

  let mut keys: Vec<Vec<u8>> = vec![];
  if let Ok(passwords) = crate::linux::secrets::get_passwords(config.os_crypt_name.unwrap_or("")) {
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

  Ok(keys)
}

#[cfg(target_os = "macos")]
fn get_keys(config: &BrowserConfig) -> Result<Vec<Vec<u8>>> {
  let salt = b"saltysalt";

  let iterations = 1003;

  let mut keys: Vec<Vec<u8>> = vec![];

  let key_service = config.osx_key_service.context("missing osx_key_service")?;
  let key_user = config.osx_key_user.context("missing osx_key_user")?;
  let password =
    secrets::get_osx_keychain_password(key_service, key_user).unwrap_or("peanuts".to_string());

  let key = create_pbkdf2_key(password.as_str(), salt, iterations);
  keys.push(key);

  let key = create_pbkdf2_key("peanuts", salt, iterations);
  keys.push(key);
  let key = create_pbkdf2_key("", salt, iterations);
  keys.push(key);

  Ok(keys)
}

/// Decrypt cookie value using aes GCM
#[cfg(windows)]
fn decrypt_encrypted_value(
  value: String,
  encrypted_value: &[u8],
  keys: Vec<Vec<u8>>,
) -> Result<String> {
  let key_type = &encrypted_value[..3];
  if !value.is_empty() || !(key_type == b"v11" || key_type == b"v10" || key_type == b"v20") {
    // unknown key_type or value isn't encrypted
    log::warn!("Unknown key type: {:?}", key_type);
    return Ok(value);
  }
  log::debug!("key type: {:?}", key_type);

  let encrypted_value = &encrypted_value[3..];
  let nonce = &encrypted_value[..12]; // iv
  let ciphertext = &encrypted_value[12..];

  // Create a new AES block cipher.
  if let Some(key) = keys.into_iter().next() {
    let key = Key::<Aes256Gcm>::from_slice(key.as_slice());
    let cipher = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(nonce); // 96-bits; unique per message
    let plaintext = cipher
      .decrypt(nonce, ciphertext.as_ref())
      .map_err(eyre::Error::msg)
      .context("Can't decrypt using key")?;

    let plaintext = if key_type == b"v20" {
      String::from_utf8(plaintext[32..].to_vec()).context("Can't decode encrypted value")
    } else {
      String::from_utf8(plaintext).context("Can't decode encrypted value")
    }?;
    return Ok(plaintext);
  }
  bail!("decrypt_encrypted_value failed")
}

/// Decrypt cookie value using aes cbc
#[cfg(unix)]
fn decrypt_encrypted_value(
  value: String,
  encrypted_value: &[u8],
  keys: Vec<Vec<u8>>,
) -> Result<String> {
  // cbc
  if !value.is_empty() {
    // unknown key_type or value isn't encrypted
    return Ok(value);
  }
  if encrypted_value.is_empty() {
    return Ok("".into());
  }
  let key_type = &encrypted_value[..3];

  if !(key_type == b"v11" || key_type == b"v10" || key_type == b"v20") {
    return Ok(value);
  }
  log::debug!("key type: {:?}", key_type);

  use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};

  type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

  // Create an AES-128 cipher with the provided key.

  let encrypted_value = &mut encrypted_value.to_owned()[3..];
  let iv: [u8; 16] = [b' '; 16];

  for key in &keys {
    let mut key_array: [u8; 16] = [0; 16];
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
          log::debug!("Error in decode decrypt value with utf8. trying from index 32");

          let decoded = String::from_utf8(plaintext[32..].to_vec()).unwrap_or_else(|_| {
            log::warn!("Error decoding from index 32 with UTF-8");
            "".into()
          });
          return Ok(decoded);
        }
      }
    }
  }
  bail!("decrypt_encrypted_value failed")
}

#[cfg(target_os = "windows")]
fn unlock_file(mut path: PathBuf) -> Result<PathBuf> {
  let mut shadow_copy_success = false;
  // Shadow copy cookies file so we can read session cookies
  // Admin rights required
  if privilege::user::privileged() {
    log::debug!("Admin rights detected");
    if let Ok(temp_dir) = windows::shadow_copy::temp_folder(".tmp", "", 10) {
      let result = windows::shadow_copy::shadow_copy(path.clone(), temp_dir.clone().to_path_buf());
      log::debug!("shadow copy result: {:?}", result);
      if result.is_ok() {
        shadow_copy_success = true;
        path = temp_dir.join(path.file_name().unwrap());
      }
    }
  }

  // Elegantly restart the process which lock the cookies file (And unlock it) using restart manager API
  if !shadow_copy_success {
    log::warn!("Unlocking Chrome database... This may take a while (sometimes up to a minute)");
    unsafe {
      crate::windows::restart_manager::release_file_lock(path.to_str().unwrap());
    }
  }
  Ok(path)
}

#[allow(unused_mut)]
fn query_cookies(
  keys: Vec<Vec<u8>>,
  mut db_path: PathBuf,
  domains: Option<Vec<String>>,
) -> Result<Vec<Cookie>> {
  // In windows unlock file locking
  #[cfg(target_os = "windows")]
  {
    db_path = unlock_file(db_path)?;
  }

  log::info!(
    "Creating SQLite connection to {}",
    db_path.to_str().unwrap_or("")
  );
  let connection = sqlite::connect(db_path)?;
  let mut query =
        "SELECT host_key, path, is_secure, expires_utc, name, value, CAST(encrypted_value AS BLOB), is_httponly, samesite FROM cookies ".to_string();

  if let Some(domains) = domains {
    let domain_queries: Vec<String> = domains
      .iter()
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
    if encrypted_value.is_empty() {
      continue;
    }
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
      same_site,
    };
    cookies.push(cookie);
  }
  Ok(cookies)
}
