// const APP_ID: &str = "rookie";


cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        // use dbus::arg::messageitem::MessageItem;
        // use dbus::blocking::Connection;
        // use std::{time::Duration};
        
        pub fn get_password(os_crypt_name: &str) -> Result<String, Box<dyn std::error::Error>> {
            // Attempt to get the password from libsecret
            if let Ok(libsecret_pass) = get_password_libsecret(os_crypt_name) {
                return Ok(libsecret_pass.to_string());
            }
            
            // Attempt to get the password from kdewallet
            if let Ok(kdewallet_pass) = get_password_kdewallet(os_crypt_name) {
                return Ok(kdewallet_pass.to_string());
            }
        
            // Both methods failed, return an error
            Err("Password retrieval failed".into())
        }
        
        
        fn get_password_libsecret(_crypt_name: &str) -> Result<&str, Box<dyn std::error::Error>> {
            // let conn = Connection::new_session()?;
            // let schemas = ["chrome_libsecret_os_crypt_password_v2", "chrome_libsecret_os_crypt_password_v1"];
            // for schema in schemas {
            //     let proxy = conn.with_proxy("org.freedesktop.secrets", "/org/freedesktop/secrets", Duration::from_millis(5000));
            //     let items: (String,) = proxy
            //         .method_call(
            //             "org.freedesktop.Secret.Service", 
            //             "SearchItems", 
            //             // ("dummy.name.without.owner",)
            //             (("xdg:schema",schema), ("application", crypt_name))
            //         )?;
            //     // let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;
            //     // for name in names { println!("{}", name); }

            // }
            Err("not Implemented".into())
        }
        
        fn get_password_kdewallet(_crypt_name: &str)-> Result<&str, Box<dyn std::error::Error>> {
            Err("not Implemented".into())
        }


    }
    else if #[cfg(target_os = "macos")] {
        use std::process::Command;
        pub fn get_osx_keychain_password(osx_key_service: &str, osx_key_user: &str) -> Result<String, Box<dyn std::error::Error>> {
            let cmd = Command::new("/usr/bin/security")
                .args(&["-q", "find-generic-password", "-w", "-a", osx_key_user, "-s", osx_key_service])
                .output();
        
            match cmd {
                Ok(output) => {
                    if output.status.success() {
                        let password = String::from_utf8(output.stdout)?;
                        Ok(password.trim().to_string())
                    } else {
                        Err("Failed to retrieve password from OSX Keychain".into())
                    }
                }
                Err(e) => Err(format!("Error executing security command: {}", e).into()),
            }
        }
    }
}

