use eyre::{anyhow, bail, Result};
use std::{collections::HashMap, sync::Arc};
use zbus::{blocking::Connection, zvariant::ObjectPath, zvariant::Value, Message};

use super::APP_ID;

/// Get password from either kdewallet or libsecret (ubuntu)
pub fn get_passwords(os_crypt_name: &str) -> Result<Vec<String>> {
  // Attempt to get the password from libsecret
  let mut passwords: Vec<String> = vec![];
  for schema in [
    "chrome_libsecret_os_crypt_password_v2",
    "chrome_libsecret_os_crypt_password_v1",
  ] {
    if let Ok(libsecret_pass) = get_password_libsecret(schema, os_crypt_name) {
      passwords.push(libsecret_pass);
    }
  }
  // Attempt to get the password from kdewallet
  if let Ok(password) = get_password_kdewallet(os_crypt_name) {
    passwords.push(password);
  }

  Ok(passwords)
}

fn libsecret_call<T>(connection: &Connection, method: &str, args: T) -> zbus::Result<Arc<Message>>
where
  T: serde::ser::Serialize + zvariant::DynamicType,
{
  connection.call_method(
    Some("org.freedesktop.secrets"),
    "/org/freedesktop/secrets",
    Some("org.freedesktop.Secret.Service"),
    method,
    &args,
  )
}

fn kwallet_call<T>(connection: &Connection, method: &str, args: T) -> zbus::Result<Arc<Message>>
where
  T: serde::ser::Serialize + zvariant::DynamicType,
{
  connection.call_method(
    Some("org.kde.kwalletd5"),
    "/modules/kwalletd5",
    Some("org.kde.KWallet"),
    method,
    &args,
  )
}

fn get_password_libsecret(schema: &str, crypt_name: &str) -> Result<String> {
  let connection = Connection::session()?;
  let mut content = HashMap::<&str, &str>::new();
  content.insert("xdg:schema", schema);
  content.insert("application", crypt_name);
  let m = libsecret_call(&connection, "SearchItems", &content)?;
  let (reply_paths, _): (Vec<ObjectPath>, Vec<ObjectPath>) = m.body()?;
  let path = reply_paths.first().ok_or(anyhow!("search items empty"))?;

  let m = libsecret_call(&connection, "Unlock", vec![path])?;
  let reply: (Vec<ObjectPath>, ObjectPath) = m.body()?;
  let object_path = reply.0.first().ok_or(anyhow!("Can't unlock"))?;

  let mut content = HashMap::<&str, &str>::new();
  content.insert("plain", "");
  let m = libsecret_call(&connection, "OpenSession", &("plain", Value::new("")))?;

  let reply: (Value, ObjectPath) = m.body()?;
  let session = reply.1;

  let m = libsecret_call(&connection, "GetSecrets", &(vec![object_path], session))?;
  type Response<'a> = (ObjectPath<'a>, Vec<u8>, Vec<u8>, String);
  let reply: HashMap<ObjectPath, Response> = m.body()?;
  let inner = reply.get(object_path).ok_or(anyhow!("Can't get secrets"))?;
  let secret = &inner.2;

  Ok(String::from_utf8(secret.clone())?)
}

fn get_password_kdewallet(crypt_name: &str) -> Result<String> {
  let connection = Connection::session()?;
  let folder = format!("{} Keys", capitalize(crypt_name));
  let key = format!("{} Safe Storage", capitalize(crypt_name));

  let m = kwallet_call(&connection, "networkWallet", ())?;
  let network_wallet: String = m.body()?;

  let m = kwallet_call(&connection, "open", (network_wallet.clone(), 0_i64, APP_ID))?;
  let handle: i32 = m.body()?;
  let m = kwallet_call(&connection, "readPassword", (handle, folder, key, APP_ID))?;
  let password: String = m.body()?;
  let m = kwallet_call(&connection, "close", (network_wallet, false))?;
  let close_ok: i32 = m.body()?;
  if close_ok != 1 {
    bail!("Close failed");
  }

  Ok(password)
}

pub fn capitalize(s: &str) -> String {
  let mut c = s.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  }
}
