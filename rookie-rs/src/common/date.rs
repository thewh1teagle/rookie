pub fn chromium_timestamp(timestamp: u64) -> Option<u64> {
    if timestamp <= 0 {
        return None;
    }
    let mut timestamp = timestamp - 11644473600000000;
    timestamp /= 1000000; // milliseconds to seconds
    unix_timestamp(timestamp)
}

pub fn mozilla_timestamp(timestamp: u64) -> Option<u64> {
    unix_timestamp(timestamp)
}

#[cfg(target_os = "windows")]
pub fn internet_explorer_timestamp(timestamp: u64) -> Option<u64> {
    if timestamp <= 0 {
        return None;
    }
    let mut timestamp = timestamp - 116444736000000000;
    timestamp /= 10000000;
    unix_timestamp(timestamp)
}


#[cfg(target_os = "macos")]
pub fn safari_timestamp(timestamp: u64) -> Option<u64> {
    if timestamp <= 0 {
        return None;
    }
    let timestamp = timestamp + 978307200f64 as u64;
    unix_timestamp(timestamp)
}

fn unix_timestamp(timestamp: u64) -> Option<u64> {
    if timestamp <= 0 {
        return None;
    }
    return Some(timestamp);
}