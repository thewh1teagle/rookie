use rookie;

fn main() {
    let cookies = rookie::chromium(None).unwrap();
    println!("{:?}", cookies);
}