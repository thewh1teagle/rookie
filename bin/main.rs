use rookie;

fn main() {
    let cookies = rookie::brave(Some(vec!["google.com"])).unwrap();
    println!("{:?}", cookies);
}