use std::time::{SystemTime, Duration, UNIX_EPOCH};

pub fn epoch_to_systemtime_micros(timestamp: i64) -> SystemTime {
    if timestamp == 0 {
        UNIX_EPOCH
    } else {
        let unix_time = UNIX_EPOCH + Duration::from_micros((timestamp as u64 - 11_644_473_600_000_000) / 1_000);
        unix_time
    }
}

#[cfg(target_os = "macos")]
pub fn unix_timestamp_to_system_time(timestamp: i64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(timestamp as u64)
}