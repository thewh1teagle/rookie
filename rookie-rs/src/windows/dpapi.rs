use std::{ffi::c_void, ptr};

use eyre::{bail, Result};
use windows::Win32::{Foundation, Security::Cryptography};

pub fn decrypt(keydpapi: &[u8]) -> Result<Vec<u8>> {
  // https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptunprotectdata
  // https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree
  // https://docs.rs/winapi/latest/winapi/um/dpapi/index.html
  // https://docs.rs/winapi/latest/winapi/um/winbase/fn.LocalFree.html

  let data_in = Cryptography::CRYPT_INTEGER_BLOB {
    cbData: keydpapi.len() as u32,
    pbData: keydpapi.as_ptr() as *mut u8, // Safety: CryptUnprotectData will not mutate the contents of data_in, a mutable pointer is required due to a typing quirk
  };
  let mut data_out = Cryptography::CRYPT_INTEGER_BLOB {
    cbData: 0,
    pbData: ptr::null_mut(),
  };

  unsafe {
    let _ = Cryptography::CryptUnprotectData(
      &data_in,
      Some(ptr::null_mut()),
      Some(ptr::null_mut()),
      Some(ptr::null_mut()),
      Some(ptr::null_mut()),
      0,
      &mut data_out,
    );
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
