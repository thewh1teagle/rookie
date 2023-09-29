# rookie
Load cookies from your web browsers

# Install
```shell
cargo add rookie
```

# Usage
```rust
use rookie::{self, Cookie};

fn main() {
    let domains = Some(vec!["google.com"]); // set to None to get all
    let cookies: Vec<Cookie> = rookie::chrome(domains).unwrap();
    println!("{:?}", cookies);
}
```


# Bindings
There's Python bindings to `rookie`
```shell
pip install rookiepy
```
And the usage it similar to original
```python
import rookiepy
cookies = rookiepy.firefox(["google.com"])
for cookie in cookies:
    print(cookie.domain, cookie.value)
```