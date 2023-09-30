use std::{env, path::PathBuf};
use crate::mozilla::get_default_profile;
use regex::Regex;


#[cfg(target_os = "windows")] 
const CHROME_PATHS: &'static [&'static str] = &[
    "%LOCALAPPDATA%/Google/Chrome/User Data"
];

#[cfg(target_os = "windows")] 
const BRAVE_PATHS: &'static [&'static str] = &[
    "%LOCALAPPDATA%/BraveSoftware/Brave-Browser/User Data"
];

#[cfg(target_os = "windows")] 
const EDGE_PATHS: &'static [&'static str] = &[
    "%LOCALAPPDATA%/Microsoft/Edge/User Data"
];

#[cfg(target_os = "windows")] 
const FIREFOX_PATHS: &'static [&'static str] = &[
    "%APPDATA%/Mozilla/Firefox"
];




#[cfg(target_os = "windows")]
pub fn expand_path(path: &str) -> PathBuf {
    // Define a regex pattern to match placeholders like %SOMETHING%
    let re = Regex::new(r"%([^%]+)%").unwrap();

    // Clone the input path for modification
    let mut expanded_path = path.to_owned();

    // Iterate over all matches of the regex pattern in the input path
    for capture in re.captures_iter(&path) {
        // Get the matched placeholder (e.g., "APPDATA" from "%APPDATA%")
        let placeholder = &capture[1];

        // Try to get the corresponding environment variable value
        if let Ok(var_value) = env::var(placeholder) {
            // Replace the placeholder with the environment variable value
            expanded_path = expanded_path.replace(&capture[0], &var_value);
        }
    }

    // Convert the expanded path to a PathBuf
    let path_buf = PathBuf::from(expanded_path);

    path_buf
}

#[cfg(target_os = "windows")]
pub fn find_chrome_paths() -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    for path in CHROME_PATHS {
        let user_data_path = expand_path(path);
        let key_path = user_data_path.join("Local State");
        let db_path = user_data_path.join("Default/Network/Cookies");
        if db_path.exists() {
            return Ok((key_path, db_path));
        }
    }
    Err(("cant find any chrome cookies file").into())
}

#[cfg(target_os = "windows")]
pub fn find_brave_paths() -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    for path in BRAVE_PATHS {
        let user_data_path = expand_path(path);
        let key_path = user_data_path.join("Local State");
        let db_path = user_data_path.join("Default/Network/Cookies");
        if db_path.exists() {
            return Ok((key_path, db_path));
        }
    }
    Err(("cant find any brave cookies file").into())
}

#[cfg(target_os = "windows")]
pub fn find_edge_paths() -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    for path in EDGE_PATHS {
        let user_data_path = expand_path(path);
        let key_path = user_data_path.join("Local State");
        let db_path = user_data_path.join("Default/Network/Cookies");
        if db_path.exists() {
            return Ok((key_path, db_path));
        }
    }
    Err(("cant find any brave cookies file").into())
}


#[cfg(target_os = "windows")]
pub fn find_firefox_paths() -> Result<PathBuf, Box<dyn std::error::Error>> {
    for path in FIREFOX_PATHS {
        let firefox_path = expand_path(path);
        let profiles_path = firefox_path.join("profiles.ini");
        let default_profile = get_default_profile(profiles_path.as_path()).unwrap();
        let db_path = firefox_path.join(default_profile).join("cookies.sqlite");    
        if db_path.exists() {
            return Ok(db_path);
        }
    }
    Err(("cant find any brave cookies file").into())
}

#[cfg(target_os = "linux")]
pub fn find_firefox_paths() -> PathBuf {
    let username = env::var("USER").unwrap();
    let user_home_str = format!("/home/{}", username.as_str());
    let user_home_path = path::Path::new(&user_home_str);

    let firefox_path = user_home_path.join("snap/firefox/common/.mozilla/firefox");
    
    let profiles_path = firefox_path.join("profiles.ini");
    let default_profile = get_default_profile(&profiles_path).unwrap();
    let db_path = firefox_path.join(default_profile).join("cookies.sqlite");    
    db_path
}


#[cfg(target_os = "linux")]
pub fn find_chrome_paths() -> (PathBuf, PathBuf) {
    let appdata_path = env::var("APPDATA").unwrap();
    let appdata_path = path::Path::new(appdata_path.as_str());
    let user_data_path = appdata_path.join("../local/Google/Chrome/User Data");
    let key_path = user_data_path.join("Local State");
    let db_path = user_data_path.join("Default/Network/Cookies");
    (key_path, db_path)
}


#[cfg(target_os = "linux")]
pub fn find_brave_paths() -> (PathBuf, PathBuf) {
    let appdata_path = env::var("APPDATA").unwrap();
    let appdata_path = path::Path::new(appdata_path.as_str());
    
    let user_data_path = appdata_path.join("../local/BraveSoftware/Brave-Browser/User Data");
    let key_path = user_data_path.join("Local State");
    let db_path = user_data_path.join("Default/Network/Cookies");
    (key_path, db_path)
}

#[cfg(target_os = "linux")]
pub fn find_edge_paths() -> (PathBuf, PathBuf) {
    let appdata_path = env::var("APPDATA").unwrap();
    let appdata_path = path::Path::new(appdata_path.as_str());
    
    let user_data_path = appdata_path.join("../local/Microsoft/Edge/User Data");
    let key_path = user_data_path.join("Local State");
    let db_path = user_data_path.join("Default/Network/Cookies");
    (key_path, db_path)
}





#[cfg(target_os = "macos")]
pub fn find_firefox_paths() -> PathBuf {
    let username = env::var("USER").unwrap();
    let user_home_str = format!("/home/{}", username.as_str());
    let user_home_path = path::Path::new(&user_home_str);

    let firefox_path = user_home_path.join("snap/firefox/common/.mozilla/firefox");
    
    let profiles_path = firefox_path.join("profiles.ini");
    let default_profile = get_default_profile(&profiles_path).unwrap();
    let db_path = firefox_path.join(default_profile).join("cookies.sqlite");    
    db_path
}


#[cfg(target_os = "macos")]
pub fn find_chrome_paths() -> (PathBuf, PathBuf) {
    let appdata_path = env::var("APPDATA").unwrap();
    let appdata_path = path::Path::new(appdata_path.as_str());
    let user_data_path = appdata_path.join("../local/Google/Chrome/User Data");
    let key_path = user_data_path.join("Local State");
    let db_path = user_data_path.join("Default/Network/Cookies");
    (key_path, db_path)
}


#[cfg(target_os = "macos")]
pub fn find_brave_paths() -> (PathBuf, PathBuf) {
    let appdata_path = env::var("APPDATA").unwrap();
    let appdata_path = path::Path::new(appdata_path.as_str());
    
    let user_data_path = appdata_path.join("../local/BraveSoftware/Brave-Browser/User Data");
    let key_path = user_data_path.join("Local State");
    let db_path = user_data_path.join("Default/Network/Cookies");
    (key_path, db_path)
}

#[cfg(target_os = "macos")]
pub fn find_edge_paths() -> (PathBuf, PathBuf) {
    let appdata_path = env::var("APPDATA").unwrap();
    let appdata_path = path::Path::new(appdata_path.as_str());
    
    let user_data_path = appdata_path.join("../local/Microsoft/Edge/User Data");
    let key_path = user_data_path.join("Local State");
    let db_path = user_data_path.join("Default/Network/Cookies");
    (key_path, db_path)
}

