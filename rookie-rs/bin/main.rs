use rookie;

fn main() {
    let cookies = rookie::firefox(None).unwrap();
    println!("{:?}", cookies);
}