use eyre::{bail, Context, Result};
use privilege::user::privileged;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::path::PathBuf;

/// Create temp folder and return path
pub fn temp_folder(prefix: &str, suffix: &str, rand_len: usize) -> Result<PathBuf> {
  let random_string: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(rand_len)
    .map(char::from) // From link above, this is needed in later versions
    .collect();
  let name = format!("{}{}{}", prefix, random_string, suffix);
  let tmp = std::env::temp_dir();
  let temp_path = tmp.join(name);
  std::fs::create_dir(temp_path.clone())?;
  log::trace!("created dir {}", temp_path.clone().display());
  Ok(temp_path)
}

/// dst should be directory
pub fn shadow_copy(src: PathBuf, dst: PathBuf) -> Result<()> {
  if !src.exists() {
    bail!("Source file not exists: {}", src.clone().display())
  }
  if !privileged() {
    bail!("No admin rights")
  }
  log::info!(
    "Creating shadow copy to cookies file from {} to {}",
    src.display(),
    dst.display()
  );
  rawcopy_rs_next::rawcopy(src.clone().to_str().unwrap(), dst.to_str().unwrap())
    .map_err(|err| eyre::eyre!(Box::new(err)))
    .context(format!(
      "Can't shadow copy from {} to {}",
      src.display(),
      dst.display(),
    ))?;

  Ok(())
}
