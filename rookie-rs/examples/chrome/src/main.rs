use rookie::{self, Cookie};

fn main() {
    let cookies: Vec<Cookie> = rookie::chrome(None).unwrap();
    println!("{:?}", cookies);
}