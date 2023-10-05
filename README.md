# rookie
Load cookies from any browser on any platform

## Install
```shell
cargo add rookie
```

## Usage
```rust
use rookie::{self, Cookie};

fn main() {
    let domains = Some(vec!["google.com"]); // set to None to get all
    let cookies: Vec<Cookie> = rookie::chrome(domains).unwrap();
    println!("{:?}", cookies);
}
```


## Bindings
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

## Examples
For [Python](https://github.com/thewh1teagle/rookie/tree/main/examples)

For [Rust](https://github.com/thewh1teagle/rookie/tree/main/rookie-rs/examples)

## Contribute
So far the following platforms are supported:

* **Chrome:** `Linux`, `MacOS`, `Windows`
* **Firefox:** `Linux`, `MacOS`, `Windows`
* **LibreWolf:** `Linux`, `MacOS`, `Windows`
* **Opera:** `Linux`, `MacOS`, `Windows`
* **Opera GX:** `MacOS`, `Windows`
* **Edge:** Linux, `MacOS`, `Windows`
* **Internet Explorer:** `Windows`
* **Chromium:** `Linux`, `MacOS`, `Windows`
* **Brave:** `Linux`, `MacOS`, `Windows`
* **Vivaldi:** `Linux`, `MacOS`, `Windows`
* **Safari:** `MacOS`

You are welcome to contribute support for other browsers, or other platforms.

## Support new browsers
If you have a browser with which the library isn't working with, it may not have been added to the list of supported browsers configs. You can create a pull request (PR) or an issue with the path to the cookies file on your computer, and I will add it.

look at [config.rs](https://github.com/thewh1teagle/rookie/blob/main/rookie-rs/src/config.rs) to see what configurations is needed.

## Gotchas

This library may trigger a password prompt on Linux systems when accessing browser cookies. 

Be prepared to enter your administrator password when prompted.

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
