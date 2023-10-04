use std::{env, path::PathBuf, error::Error};
use crate::{mozilla::get_default_profile, BrowserConfig};
use glob;


fn expand_glob_paths(path: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut data_paths: Vec<PathBuf> = vec![];
    if let Some(path_str) = path.to_str() {
        for entry in glob::glob(path_str)? {
            if entry.is_ok() {
                data_paths.push(entry?);
            }
        }
    } 
    Ok(data_paths)
}

#[cfg(target_os = "windows")]
pub fn expand_path(path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    use regex::Regex;
    // Define a regex pattern to match placeholders like %SOMETHING%
    let re = Regex::new(r"%([^%]+)%")?;

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

    Ok(path_buf)
}

#[cfg(unix)]
pub fn expand_path(path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        // Get the value of the HOME environment variable
    let home = env::var("HOME")?;

    // Replace ~ or $HOME with the actual home directory path
    let expanded_path = path
        .replace("~", &home)
        .replace("$HOME", &home);

    // Convert the expanded path to a PathBuf
    Ok(PathBuf::from(expanded_path))
}


pub fn find_chrome_based_paths(browser_config: &BrowserConfig) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    for path in browser_config.data_paths { // base paths
        let channels: &[&str] = &browser_config.channels.as_deref().unwrap_or(&[""]);
        for channel in channels { // channels
            let path = path.replace("{channel}", channel);
            let db_path = expand_path(path.as_str())?;
            let glob_db_paths = expand_glob_paths(db_path)?;
            for db_path in glob_db_paths { // glob expanded paths
                if db_path.exists() {
                    if let Some(parent) = db_path.parent() {
                    let key_path = ["../../Local State", "../Local State", "Local State"]
                        .iter()
                        .map(|p| parent.join(p))
                        .find(|p| p.exists())
                        .unwrap_or_else(|| parent.join("Local State"));
                    return Ok((key_path, db_path));
                }
            }
            }
            
        }
    }
    Err(("can't find any cookies file").into())
}



pub fn find_mozilla_based_paths(browser_config: &BrowserConfig) -> Result<PathBuf, Box<dyn std::error::Error>> {
    for path in browser_config.data_paths { // base paths
        let channels: &[&str] = &browser_config.channels.as_deref().unwrap_or(&[""]);
        for channel in channels { // channels
            let path = path.replace("{channel}", &channel);
            let firefox_path = expand_path(path.as_str())?;
            let glob_paths = expand_glob_paths(firefox_path)?;
            for path in glob_paths { // expanded glob paths
                let profiles_path = path.join("profiles.ini");
                let default_profile = get_default_profile(profiles_path.as_path()).unwrap_or("".to_string());
                let db_path = path.join(default_profile).join("cookies.sqlite");    
                if db_path.exists() {
                    return Ok(db_path);
                }
            }
        }
    }
    

    Err(("cant find any brave cookies file").into())
}


#[cfg(target_os = "macos")]
pub fn find_safari_based_paths(browser_config: &BrowserConfig) -> Result<PathBuf, Box<dyn std::error::Error>> {
    for path in browser_config.data_paths { // base paths
        let channels: &[&str] = &browser_config.channels.as_deref().unwrap_or(&[""]);
        for channel in channels { // channels
            let path = path.replace("{channel}", &channel);
            let safari_path = expand_path(path.as_str())?;
            let glob_paths = expand_glob_paths(safari_path)?;
            for path in glob_paths { // expanded glob paths
                if path.exists() {
                    return Ok(path);
                }
            }
        }
    }
    

    Err(("cant find any brave cookies file").into())
}

#[cfg(target_os = "windows")]
pub fn find_ie_based_paths(browser_config: &BrowserConfig) -> Result<PathBuf, Box<dyn std::error::Error>> {
    for path in browser_config.data_paths { // base paths
        let channels: &[&str] = &browser_config.channels.as_deref().unwrap_or(&[""]);
        for channel in channels { // channels
            
            let path = path.replace("{channel}", &channel);
            let path = expand_path(path.as_str())?;
            let glob_paths = expand_glob_paths(path)?;
            for path in glob_paths { // expanded glob paths
                if path.exists() {
                    return Ok(path);
                }
            }
        }
    }
    

    Err(("cant find any IE cookies file").into())
}