use rookie;

fn main() {
    let cookies = rookie::firefox().unwrap();
    println!("{:?}", cookies);
}