use rookie::chrome;
fn main() {
    pretty_env_logger::init();
    let cookies = chrome(None).unwrap();
    println!("Found {} cookies", cookies.len());
}
