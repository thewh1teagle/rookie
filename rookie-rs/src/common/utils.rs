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
