use std::path::PathBuf;

use rookie;

fn main() {
    let cookies = rookie::chromium_based(PathBuf::from("/home/user/snap/brave/285/.config/BraveSoftware/Brave-Browser/Local State"), PathBuf::from("/home/user/snap/brave/285/.config/BraveSoftware/Brave-Browser/Default/Cookies"), None).unwrap();
    println!("{:?}", cookies);
}