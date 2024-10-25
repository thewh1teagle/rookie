fn main() {
  tracing_subscriber::fmt::init();
  let cookies = rookie::brave(None).unwrap();
  println!("Found {} cookies", cookies.len());
}
