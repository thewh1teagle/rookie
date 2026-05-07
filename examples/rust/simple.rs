fn main() {
  tracing_subscriber::fmt::init();
  let cookies = rookie::chrome(None::<Vec<String>>).unwrap();
  for cookie in cookies {
    println!("{:?}", cookie);
  }
}
