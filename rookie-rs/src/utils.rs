use std::time::{SystemTime, Duration, UNIX_EPOCH};


pub fn epoch_to_systemtime_micros(timestamp: i64) -> SystemTime {
    if timestamp == 0 {
        UNIX_EPOCH
    } else {
        let unix_time = UNIX_EPOCH + Duration::from_micros((timestamp as u64 - 11_644_473_600_000_000) / 1_000);
        unix_time
    }
}

pub fn unix_timestamp_to_system_time(i64_as_duration: Duration) -> SystemTime {
    UNIX_EPOCH + i64_as_duration
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}


pub fn some_domain_in_host(domains: Option<Vec<&str>>, host: &str) -> bool {
    if let Some(strings) = domains {
        for d in strings {
            if host.contains(d) {
                return true;
            }
        }
    }
    false
}