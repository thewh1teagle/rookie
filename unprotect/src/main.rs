/*
Simple program that receive base64 encoded secret and decrypt with cryptUnprotect from WindowsAPI.
Used for decrypt appbound encrypted key of Chrome (v20 key)

See [decrypt.py](https://gist.github.com/thewh1teagle/d0bbc6bc678812e39cba74e1d407e5c7)

cargo build --bin unprotect --release
*/
use base64::{prelude::BASE64_STANDARD, Engine};
use eyre::{bail, eyre, Result};
use std::env;

fn dpapi_decrypt(keydpapi: &mut [u8]) -> Result<Vec<u8>> {
  // https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptunprotectdata
  // https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree
  // https://docs.rs/winapi/latest/winapi/um/dpapi/index.html
  // https://docs.rs/winapi/latest/winapi/um/winbase/fn.LocalFree.html1

  use std::ffi::c_void;
  use std::ptr;
  use windows::Win32::{Foundation, Security::Cryptography};

  let data_in = Cryptography::CRYPT_INTEGER_BLOB {
    cbData: keydpapi.len() as u32,
    pbData: keydpapi.as_mut_ptr(),
  };
  let mut data_out = Cryptography::CRYPT_INTEGER_BLOB {
    cbData: 0,
    pbData: ptr::null_mut(),
  };

  unsafe {
    let _ = match Cryptography::CryptUnprotectData(
      &data_in,
      Some(ptr::null_mut()),
      Some(ptr::null_mut()),
      Some(ptr::null_mut()),
      Some(ptr::null_mut()),
      0,
      &mut data_out,
    ) {
      Ok(_) => Ok(()),
      Err(_) => Err(eyre!("CryptUnprotectData failed")),
    };
  }
  if data_out.pbData.is_null() {
    bail!("CryptUnprotectData returned a null pointer");
  }

  let decrypted_data =
    unsafe { std::slice::from_raw_parts(data_out.pbData, data_out.cbData as usize).to_vec() };
  let pbdata_hlocal = Foundation::HLOCAL(data_out.pbData as *mut c_void);
  unsafe {
    let _ = Foundation::LocalFree(pbdata_hlocal);
  };
  Ok(decrypted_data)
}

fn main() {
  let encrypted_base64 = env::args()
    .nth(1)
    .expect("Please specify encrypted key as base64");
  let mut decoded = BASE64_STANDARD.decode(encrypted_base64.trim()).unwrap();
  let decrypted = dpapi_decrypt(&mut decoded).unwrap();
  println!("{}", BASE64_STANDARD.encode(decrypted));
}
