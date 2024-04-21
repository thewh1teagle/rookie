#[cfg(unix)]
use eyre::{anyhow, bail, Result};

use std::process::Command;
pub fn get_osx_keychain_password(osx_key_service: &str, osx_key_user: &str) -> Result<String> {
  let cmd = Command::new("/usr/bin/security")
    .args([
      "-q",
      "find-generic-password",
      "-w",
      "-a",
      osx_key_user,
      "-s",
      osx_key_service,
    ])
    .output();

  match cmd {
    Ok(output) => {
      if output.status.success() {
        let password = String::from_utf8(output.stdout)?;
        Ok(password.trim().to_string())
      } else {
        bail!("Failed to retrieve password from OSX Keychain")
      }
    }
    Err(e) => Err(anyhow!("Error executing security command: {}", e)),
  }
}
