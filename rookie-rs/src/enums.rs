use std::{fmt::{self}, time::SystemTime};

#[derive(Debug)]
pub struct Cookie {
    pub host: String,
    pub path:     String,
	pub secure:   bool,
	pub expires:  SystemTime,
	pub name:     String,
	pub value:    String,
	pub http_only: bool,
	pub same_site: i64
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cookie:\n\
             - Host: {}\n\
             - Path: {}\n\
             - Secure: {}\n\
             - Expires: {:?}\n\
             - Name: {}\n\
             - Value: {}\n\
             - Http Only: {}\n\
             - Same Site: {}",
            self.host, self.path, self.secure, self.expires, self.name, self.value, self.http_only, self.same_site
        )
    }
}


pub trait CookieToString {
    fn to_string(&self) -> String;
}

impl CookieToString for Vec<Cookie> {
    fn to_string(&self) -> String {
        self.iter()
            .map(|cookie| format!("{}={}", cookie.name, cookie.value))
            .collect::<Vec<String>>()
            .join(";")
    }
}