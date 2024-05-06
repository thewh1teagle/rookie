pub fn some_domain_in_host(domains: Option<Vec<String>>, host: &str) -> bool {
  if let Some(strings) = domains {
    for d in strings {
      if host.contains(&d) {
        return true;
      }
    }
  }
  false
}
