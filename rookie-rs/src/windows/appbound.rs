/*
See https://gist.github.com/thewh1teagle/d0bbc6bc678812e39cba74e1d407e5c7 and https://github.com/runassu/chrome_v20_decryption/blob/main/decrypt_chrome_v20_cookie.py

wget.exe https://github.com/thewh1teagle/rookie/releases/download/appbound-binaries/paexec.exe
wget.exe https://github.com/thewh1teagle/rookie/releases/download/appbound-binaries/unprotect.exe
cargo build --release --features appbound
*/
use base64::{prelude::BASE64_STANDARD, Engine};
use eyre::{bail, Context, Result};
use std::fs;
use std::process::{Command, Stdio};

use crate::utils::temp_dir;

use aes_gcm::{
  aead::{generic_array::GenericArray, Aead, KeyInit},
  Aes256Gcm, Key,
};

static UNPROTECT_NAME: &str = "unprotect.exe";
static PAEXEC_NAME: &str = "paexec.exe";

static PAEXEC_BYTES: &[u8] = include_bytes!("../../../paexec.exe"); // Place it in workspace root: wget.exe https://github.com/thewh1teagle/rookie/releases/download/appbound-binaries/paexec.exe
static CRYPT_UNPROTECT_BYTES: &[u8] = include_bytes!("../../../unprotect.exe"); // cargo build --release -p cryptunprotect

fn decrypt(key_b64: &str, as_system: bool) -> Result<String> {
  let dir = temp_dir()?;
  let unprotect_path = dir.join(UNPROTECT_NAME);

  fs::write(&unprotect_path, CRYPT_UNPROTECT_BYTES).expect("Failed to write unprotect.exe");

  let output = if as_system {
    let paexec_path = dir.join(PAEXEC_NAME);
    fs::write(&paexec_path, PAEXEC_BYTES).expect("Failed to write paexec.exe");
    let args = ["-s", unprotect_path.to_str().unwrap(), key_b64];
    Command::new(paexec_path)
      .args(args)
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .output()
      .context("Failed to execute paexec")?
  } else {
    Command::new(unprotect_path)
      .arg(key_b64)
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .output()
      .context("Failed to execute unprotect")?
  };
  // fs::remove_dir_all(dir)?;

  let result = String::from_utf8(output.stdout).context("utf8")?;
  Ok(result.trim().to_string())
}

pub fn get_keys(key64: &str) -> Result<Vec<u8>> {
  let key_u8 = BASE64_STANDARD.decode(key64)?;
  if !key_u8.starts_with(b"APPB") {
    bail!("key not starts with APPB")
  }
  let key_u8 = &key_u8[4..];
  let key64 = BASE64_STANDARD.encode(key_u8);
  let system_decrypted = decrypt(&key64, true)?;
  let user_decrypted = decrypt(system_decrypted.trim(), false)?;
  let key = BASE64_STANDARD.decode(user_decrypted)?;
  let decrypted_key = &key[key.len() - 61..];
  if decrypted_key[0] != 1 {
    bail!("key[0] != 1")
  }

  let aes_key = BASE64_STANDARD.decode("sxxuJBrIRnKNqcH6xJNmUc/7lE0UOrgWJ2vMbaAoR4c=")?;
  let iv = &decrypted_key[1..1 + 12];
  let mut ciphertext = decrypted_key[1 + 12..1 + 12 + 32].to_vec();
  let tag = &decrypted_key[1 + 12 + 32..];
  ciphertext.extend(tag);

  let aes_key = Key::<Aes256Gcm>::from_slice(&aes_key);
  let cipher = Aes256Gcm::new(aes_key);
  let nonce = GenericArray::from_slice(iv); // 96-bits; unique per message
  let plain = cipher.decrypt(nonce, ciphertext.as_slice()).unwrap();
  Ok(plain)
}
