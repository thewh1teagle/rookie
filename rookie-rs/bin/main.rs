use rookie;

fn main() {
    let cookies = rookie::firefox(Some(vec!["microsoft.com"])).unwrap();
    println!("{:?}", cookies);
}