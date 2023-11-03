#[cfg(unix)]
use anyhow::{Result, anyhow, bail};

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        use crate::common::utils;
        use crate::config;
        use std::{collections::HashMap, sync::Arc};
        use zbus::{blocking::Connection, zvariant::Value, zvariant::ObjectPath, Message};


        fn libsecret_call<'a, T>(
            connection: &Connection,
            method: &str,
            args: T,
        ) -> zbus::Result<Arc<Message>>
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
        

        fn kwallet_call<'a, T>(
            connection: &Connection,
            method: &str,
            args: T,
        ) -> zbus::Result<Arc<Message>>
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
        

        pub fn get_passwords(os_crypt_name: &str) -> Result<Vec<String>> {
            // Attempt to get the password from libsecret
            let mut passwords: Vec<String> = vec![];
            for schema in ["chrome_libsecret_os_crypt_password_v2", "chrome_libsecret_os_crypt_password_v1"] {
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
        
        
        fn get_password_libsecret(schema: &str, crypt_name: &str) -> Result<String>  {
            let connection = Connection::session()?;
            let mut content = HashMap::<&str, &str>::new();
            content.insert("xdg:schema", schema);
            content.insert("application", crypt_name);
            let m = libsecret_call(&connection, "SearchItems", &content)?;
            let (reply_paths, _) : (Vec<ObjectPath>, Vec<ObjectPath>) = m.body()?;
            let path = reply_paths.first().ok_or(anyhow!("search items empty"))?;
        
        
            let m = libsecret_call(&connection, "Unlock", vec![path])?;
            let reply: (Vec<ObjectPath>, ObjectPath)  = m.body()?;
            let object_path = reply.0.first().ok_or(anyhow!("Cant unlock"))?;
        
        
            let mut content = HashMap::<&str, &str>::new();
            content.insert("plain", "");
            let m = libsecret_call(&connection, "OpenSession", &("plain", Value::new("")))?;
        
        
            let reply: (Value, ObjectPath) = m.body()?;
            let session = reply.1;
        
            let m = libsecret_call(&connection, "GetSecrets", &(vec![object_path], session))?;
            type Response<'a> = (ObjectPath<'a>, Vec<u8>, Vec<u8>, String);
            let reply: HashMap::<ObjectPath, Response>  = m.body()?;
            let inner = reply.get(object_path).ok_or(anyhow!("Cant get secrets"))?;
            let secret = &inner.2;
            
            Ok(String::from_utf8(secret.clone())?)
        }
  
        
        
        fn get_password_kdewallet(crypt_name: &str)-> Result<String> {
            let connection = Connection::session()?;
            let folder = format!("{} Keys", utils::capitalize(crypt_name));
            let key = format!("{} Safe Storage", utils::capitalize(crypt_name));

            let m = kwallet_call(&connection, "networkWallet", ())?;
            let network_wallet: String = m.body()?;

            let m = kwallet_call(&connection, "open", (network_wallet.clone(), 0 as i64, config::APP_ID))?;
            let handle: i32 = m.body()?;
            let m = kwallet_call(&connection, "readPassword", (handle, folder, key, config::APP_ID))?;
            let password: String = m.body()?;
            let m = kwallet_call(&connection, "close", (network_wallet, false))?;
            let close_ok: i32 = m.body()?;
            if close_ok != 1 {
                bail!("Close failed");
            }
            
            Ok(password)
        }


    }
    else if #[cfg(target_os = "macos")] {
        use std::process::Command;
        pub fn get_osx_keychain_password(osx_key_service: &str, osx_key_user: &str) -> Result<String> {
            let cmd = Command::new("/usr/bin/security")
                .args(&["-q", "find-generic-password", "-w", "-a", osx_key_user, "-s", osx_key_service])
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
    }
}

