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

#[cfg(target_os = "linux")]
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}