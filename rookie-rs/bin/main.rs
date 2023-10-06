use rookie;

fn main() {
    let cookies = rookie::load(Some(vec!["microsoft.com"])).unwrap();
    println!("{:?}", cookies);
}