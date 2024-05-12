fn main() {
  env_logger::init();
  let cookies = rookie::brave(None).unwrap();
  println!("Found {} cookies", cookies.len());
}
