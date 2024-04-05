fn main() {
    let cookies = rookie::chrome(None).unwrap();
    println!("{cookies:?}");
}
