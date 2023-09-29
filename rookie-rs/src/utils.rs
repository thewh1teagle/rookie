use std::time::{SystemTime, Duration, UNIX_EPOCH};

pub fn epoch_to_systemtime(timestamp: i64) -> SystemTime {
    if timestamp == 0 {
        UNIX_EPOCH
    } else {
        let unix_time = UNIX_EPOCH + Duration::from_micros((timestamp as u64 - 11_644_473_600_000_000) / 1_000);
        unix_time
    }
}