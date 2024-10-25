fn main() {
  let cookies = rookie::chrome(None).unwrap();
  for cookie in cookies {
    println!("{:?}", cookie);
  }
}
