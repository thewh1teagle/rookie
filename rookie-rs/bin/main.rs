use rookie;

fn main() {
    let cookies = rookie::brave(Some(vec!["github.com"])).unwrap();
    println!("{:?}", cookies);
}