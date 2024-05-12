use regex::Regex;
use reqwest::blocking::Client;
use rookie::{common::enums::CookieToString, load};

fn extract_username(html: &str) -> &str {
  let re = Regex::new(r#"dashboard\/ajax_context_list\?current_context=(.+)""#).unwrap();
  if let Some(capture) = re.captures(html) {
    if let Some(content) = capture.get(1) {
      return content.as_str();
    }
  }
  ""
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Create a custom cookie store
  let client = Client::new();
  let cookies = load(Some(vec!["github.com".into()]))?;
  let response = client.get("https://github.com/")
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36")
    .header("Cookie", cookies.to_string()) // <- try to comment
    .send()?;

  let content = response.text()?;
  let username = extract_username(content.as_str());
  match username {
    "" => println!("Not logged in to GitHub"),
    _ => println!("Logged in to GitHub as {username}"),
  };
  Ok(())
}
