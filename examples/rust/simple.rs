fn main() {
  tracing_subscriber::fmt::init();
  let cookies = rookie::chrome::<String>(None).unwrap();
  for cookie in cookies {
    println!("{:?}", cookie);
  }
}
