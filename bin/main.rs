use rookie;

fn main() {
    let cookies = rookie::chrome().unwrap();
    println!("{:?}", cookies);
}