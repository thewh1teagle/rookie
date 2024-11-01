fn main() {
  tracing_subscriber::fmt::init();
  let cookies = rookie::zen(None).unwrap();
  for cookie in cookies {
    println!("{:?}", cookie);
  }
}
