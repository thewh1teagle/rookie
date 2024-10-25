#[cfg(all(not(windows), feature = "appbound"))]
compile_error!("The `appbound` feature is supported only on Windows");

fn commit_hash() -> String {
  let output = std::process::Command::new("git")
    .args(["rev-parse", "--short", "HEAD"])
    .output()
    .unwrap();
  String::from_utf8(output.stdout).unwrap()
}
fn main() {
  let hash = commit_hash();
  println!("cargo:rustc-env=COMMIT_HASH={}", hash);
}
