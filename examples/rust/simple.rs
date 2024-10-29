fn main() {
  tracing_subscriber::fmt::init();
  let cookies = rookie::w3m(None).unwrap();
  for cookie in cookies {
    println!("{:?}", cookie);
  }
}
