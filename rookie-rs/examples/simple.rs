fn main() {
  let cookies = rookie::brave(None).unwrap();
  println!("Found {} cookies!", cookies.len());
}
