use eyre::{bail, Context, Result};
use windows::{core::w, Win32::Security::Cryptography};

pub fn decrypt(keydpapi: &[u8]) -> Result<Vec<u8>> {
  let mut provider_handle = Cryptography::NCRYPT_PROV_HANDLE::default();
  unsafe {
    Cryptography::NCryptOpenStorageProvider(
      &mut provider_handle,
      w!("Microsoft Software Key Storage Provider"),
      0,
    )
    .context("NCryptOpenStorageProvider failed")?;
  }

  let mut key_handle = Cryptography::NCRYPT_KEY_HANDLE::default();
  unsafe {
    Cryptography::NCryptOpenKey(
      provider_handle,
      &mut key_handle,
      w!("Google Chromekey1"),
      Cryptography::CERT_KEY_SPEC::default(),
      Cryptography::NCRYPT_FLAGS::default(),
    )
    .context("NCryptOpenKey failed")?;
  }

  let mut output_buffer = vec![0u8; keydpapi.len()];
  let mut output_length = 0u32;
  unsafe {
    Cryptography::NCryptDecrypt(
      key_handle,
      Some(keydpapi),
      None,
      Some(&mut output_buffer),
      &mut output_length,
      Cryptography::NCRYPT_SILENT_FLAG,
    )
    .context("NCryptDecrypt failed")?;
  }
  if output_length as usize > output_buffer.len() {
    bail!("NCryptDecrypt output longer than input");
  }
  output_buffer.truncate(output_length as usize);

  unsafe {
    Cryptography::NCryptFreeObject(key_handle)?;
    Cryptography::NCryptFreeObject(provider_handle)?;
  }

  Ok(output_buffer)
}
