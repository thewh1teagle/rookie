use rookie;

fn main() {
    let cookies = rookie::load(None).unwrap();
    println!("{:?}", cookies);
}