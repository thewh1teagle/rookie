use std::time::{SystemTime, Duration};

pub fn chromium_timestamp(timestamp: u64) -> SystemTime {
    let mut timestamp = timestamp - 11644473600000000;
    timestamp /= 1000000; // milliseconds to seconds
    return unix_timestamp(timestamp)
}

pub fn mozilla_timestamp(timestamp: u64) -> SystemTime {
    return unix_timestamp(timestamp);
}

#[cfg(target_os = "windows")]
pub fn internet_explorer_timestamp(timestamp: u64) -> SystemTime {
    let mut timestamp = timestamp - 116444736000000000;
    timestamp /= 10000000;
    return  unix_timestamp(timestamp);
}


#[cfg(target_os = "macos")]
pub fn safari_timestamp(timestamp: u64) -> SystemTime {
    let timestamp = timestamp + 978307200f64 as u64;
    return  unix_timestamp(timestamp);
}

fn unix_timestamp(timestamp: u64) -> SystemTime {
    return SystemTime::UNIX_EPOCH + Duration::from_secs(timestamp as u64);
}