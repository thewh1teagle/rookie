use rookie;

fn main() {
    let cookies = rookie::firefox(Some(vec!["google.com"])).unwrap();
    println!("{:?}", cookies);
}