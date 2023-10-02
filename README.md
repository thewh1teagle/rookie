# rookie
Load cookies from any browser on any platform

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

# Examples
For [Python](https://github.com/thewh1teagle/rookie/tree/main/examples)

For [Rust](https://github.com/thewh1teagle/rookie/tree/main/rookie-rs/examples)

## Testing Dates  (dd/mm/yy)

Browser  |  Linux   |  MacOS   | Windows  |
:------  | :------: | :------: | :------: |
Chrome   | 1/10/23  |    -     |  1/10/23 |
Firefox  | 1/10/23  |    -     |  1/10/23 |
LibreWolf| 1/10/23  |    -     |  1/10/23 |
Opera    | 1/10/23  |    -     |  1/10/23 |
Opera GX |   N/A    |    -     |  1/10/23 |
Edge     | 1/10/23  |    -     |  1/10/23 |
IE       |   N/A    |   N/A    |  1/10/23 |
Chromium | 1/10/23  |    -     |  1/10/23 |
Brave    | 1/10/23  |    -     |  1/10/23 |
Vivaldi  | 1/10/23  |    -     |  1/10/23 |
Safari   |   N/A    |  2/10/23 |    N/A   |
