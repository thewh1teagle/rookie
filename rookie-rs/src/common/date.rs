pub fn chromium_timestamp(timestamp: u64) -> Option<u64> {
  if timestamp == 0 {
    return None;
  }
  let mut timestamp = timestamp - 11_644_473_600_000_000;
  timestamp /= 1000000; // milliseconds to seconds
  unix_timestamp(timestamp)
}

pub fn mozilla_timestamp(timestamp: u64) -> Option<u64> {
  unix_timestamp(timestamp)
}

fn unix_timestamp(timestamp: u64) -> Option<u64> {
  if timestamp == 0 {
    return None;
  }
  Some(timestamp)
}

#[cfg(target_os = "macos")]
pub fn safari_timestamp(timestamp: u64) -> Option<u64> {
  if timestamp == 0 {
    return None;
  }
  let unix_timestamp = timestamp + 978_307_200;
  // nanoseconds to seconds
  let unix_timestamp = unix_timestamp / 1_000_000_000;
  Some(unix_timestamp)
}
#[cfg(target_os = "windows")]
pub fn internet_explorer_timestamp(timestamp: u64) -> Option<u64> {
  if timestamp == 0 {
    return None;
  }
  let mut timestamp = timestamp - 116_444_736_000_000_000;
  timestamp /= 10_000_000;
  unix_timestamp(timestamp)
}
