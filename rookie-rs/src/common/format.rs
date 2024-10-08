use crate::enums::Cookie;

pub fn json(cookies: Vec<Cookie>) -> String {
  serde_json::to_string_pretty(&cookies).expect("cannot convert cookies to json")
}

pub fn netscape(cookies: Vec<Cookie>) -> String {
  let mut data = indoc::formatdoc! {"
    # Netscape HTTP Cookie File
    # Generated by Rookie {}
    # Edit at your own risk.\n
  ", crate::version()}
  .to_string();
  for cookie in cookies {
    let domain = if cookie.http_only {
      format!("#HttpOnly_{}", cookie.domain)
    } else {
      cookie.domain
    };
    let subdomain = domain.starts_with('.').to_string().to_uppercase();
    let https_only = cookie.secure.to_string().to_uppercase();
    data.push_str(&format!(
      "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
      domain,
      subdomain,
      cookie.path,
      https_only,
      cookie.expires.unwrap_or(0),
      cookie.name,
      cookie.value
    ));
  }
  data
}
