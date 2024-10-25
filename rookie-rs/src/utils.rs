use std::fs;
use std::path::PathBuf;

use eyre::Result;
use rand::distributions::Alphanumeric;
use rand::Rng;

#[allow(unused)]
pub fn random_string(length: usize, prefix: &str, suffix: &str) -> String {
  let random_part: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(length)
    .map(char::from)
    .collect();

  format!("{}{}{}", prefix, random_part, suffix)
}

#[allow(unused)]
pub fn temp_dir() -> Result<PathBuf> {
  let tmp_path = std::env::temp_dir();
  let tmp_path = tmp_path.join(random_string(6, "rookie", ""));
  fs::create_dir_all(&tmp_path)?;
  Ok(tmp_path)
}
