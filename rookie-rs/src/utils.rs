use std::time::{SystemTime, Duration, UNIX_EPOCH};

const TEN_YEARS_SECS: Duration = Duration::from_secs(10 * 365 * 24 * 60 * 60); // default expires



pub fn unix_timestamp_to_system_time(i64_as_duration: Duration) -> SystemTime {
    if i64_as_duration == Duration::from_secs(0) {
        // If the input duration is 0, treat it as infinite
        let current_time = SystemTime::now();
        current_time + TEN_YEARS_SECS
    } else {
        UNIX_EPOCH + i64_as_duration
    }
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