use std::path::PathBuf;

use rookie;

fn main() {
    let cookies = rookie::safari_based(PathBuf::from("C:/Users/User/Desktop/burnt-cookie/Cookies.binarycookies"), None).unwrap();
    println!("{:?}", cookies);
}