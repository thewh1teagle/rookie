use std::{env, path::PathBuf};
use crate::{mozilla::get_default_profile, BrowserConfig};


#[cfg(target_os = "windows")]
pub fn expand_path(path: &str) -> PathBuf {
    use regex::Regex;
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

#[cfg(unix)]
pub fn expand_path(path: &str) -> PathBuf {
        // Get the value of the HOME environment variable
    let home = env::var("HOME").unwrap();

    // Replace ~ or $HOME with the actual home directory path
    let expanded_path = path
        .replace("~", &home)
        .replace("$HOME", &home);

    // Convert the expanded path to a PathBuf
    PathBuf::from(expanded_path)
}


pub fn find_chrome_based_paths(browser_config: &BrowserConfig) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    for path in browser_config.data_paths {
        for channel in browser_config.channels {
            let path = path.replace("{channel}", &channel);
            let user_data_path = expand_path(path.as_str());
            let key_path = user_data_path.join("Local State");
            let db_path = user_data_path.join("Default/Network/Cookies");
            if db_path.exists() {
                return Ok((key_path, db_path));
            }
        }
    }
    
    
    Err(("can't find any cookies file").into())
}

pub fn find_mozilla_based_paths(browser_config: &BrowserConfig) -> Result<PathBuf, Box<dyn std::error::Error>> {
    for path in browser_config.data_paths {
        for channel in browser_config.channels {
            let path = path.replace("{channel}", &channel);
            let firefox_path = expand_path(path.as_str());
            let profiles_path = firefox_path.join("profiles.ini");
            let default_profile = get_default_profile(profiles_path.as_path()).unwrap();
            let db_path = firefox_path.join(default_profile).join("cookies.sqlite");    
            if db_path.exists() {
                return Ok(db_path);
            }
        }
    }
    

    Err(("cant find any brave cookies file").into())
}