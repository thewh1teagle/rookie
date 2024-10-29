/*
See https://github.com/runassu/chrome_v20_decryption/blob/main/decrypt_chrome_v20_cookie.py
cargo build --release --features appbound
*/
use base64::{prelude::BASE64_STANDARD, Engine};
use eyre::{bail, eyre, Result};

use aes_gcm::{
  aead::{generic_array::GenericArray, Aead, KeyInit},
  Aes256Gcm, Key,
};

mod impersonate;

fn decrypt(key: &mut [u8], as_system: bool) -> Result<Vec<u8>> {
  let mut handle = None;
  if as_system {
    handle = Some(impersonate::start_impersonate()?);
  }
  let result = crate::windows::dpapi::decrypt(key)?;
  if let Some(handle) = handle {
    impersonate::stop_impersonate(handle)?;
  }
  Ok(result)
}

pub fn get_keys(key64: &str) -> Result<Vec<Vec<u8>>> {
  let mut keys: Vec<Vec<u8>> = Vec::new();

  let key_u8 = BASE64_STANDARD.decode(key64)?;
  if !key_u8.starts_with(b"APPB") {
    bail!("key not starts with APPB")
  }
  let key_u8 = &key_u8[4..];
  let key64 = BASE64_STANDARD.encode(key_u8);
  let mut system_decrypted = decrypt(&mut BASE64_STANDARD.decode(key64)?, true)?;
  let user_decrypted = decrypt(&mut system_decrypted, false)?;
  let key = &user_decrypted[user_decrypted.len() - 61..];

  // Most chrome browsers can use the system->user decrypted key directly (last 32 bytes)
  keys.push(key[key.len() - 32..].to_vec());

  // Chrome also decrypt the decrypted key with hardcoded AES key from elevation_service.exe
  let decrypted_key = &key[key.len() - 61..];
  let aes_key = BASE64_STANDARD.decode("sxxuJBrIRnKNqcH6xJNmUc/7lE0UOrgWJ2vMbaAoR4c=")?;
  let iv = &decrypted_key[1..1 + 12];
  let mut ciphertext = decrypted_key[1 + 12..1 + 12 + 32].to_vec();
  let tag = &decrypted_key[1 + 12 + 32..];
  ciphertext.extend(tag);
  let aes_key = Key::<Aes256Gcm>::from_slice(&aes_key);
  let cipher = Aes256Gcm::new(aes_key);
  let nonce = GenericArray::from_slice(iv); // 96-bits; unique per message
  if let Ok(plain) = cipher
    .decrypt(nonce, ciphertext.as_slice())
    .map_err(|e| eyre!("{:?}", e))
  {
    keys.push(plain)
  }

  Ok(keys)
}
