[![PyPi Downloads][PyPi-downloads]][PyPi-url]
[![PyPi Version][PyPi-version]][PyPi-url]
[![Crates][Crates-badge]][Crates-url]
[![License][License-shield]][License-url]


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
    println!("{cookies:?}");
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
    print(cookie['domain'], cookie['value'])
```

## Examples
For [Python](https://github.com/thewh1teagle/rookie/tree/main/examples)

For [Rust](https://github.com/thewh1teagle/rookie/tree/main/rookie-rs/examples)

## Docs
Docs currenly available for `Rust` at [docs.rs/rookie](https://docs.rs/rookie)  
For `Python` you can see examples folder, it has very simple usage.

## CLI
You can use rookie as a `CLI` tool which will decrypt the cookies and print it as `JSON`  
See [cli](https://github.com/thewh1teagle/rookie/tree/main/cli) folder

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
### Password prompt with kde-wallet on linux
This library may trigger a password prompt on Linux systems when accessing browser cookies. 
### Session Cookies Retrieval
Chrome-based browsers have a security feature that prevents external access to their cookies file. To bypass this security measure, we restart the browser seamlessly. As a result, session cookies are retrieved and can be used, but they will expire once the browser is closed again.



## Using on Unsupported platforms
To use rookie on other platforms for instance on `Android`,  
Copy the Cookies file from the mobile phone, you can search using
```shell
find /data/data -type f -name Cookies
```
And pull the Cookies file you want and then execute `CLI` on that file
```shell
./cli --path <Cookies path>
```

## Manually import website cookies
To import cookies from rookiepy into the browser,

you can execute short javascript code in the browser console 

and construct the cookies manually,

but you must execute it while the specific domain it opened.
```python
import rookiepy
from datetime import datetime, timezone, timedelta

def create_js_code(cookies):
    expires = datetime.now(timezone.utc)
    expires += timedelta(days=365) # one year expires
    expires = expires.strftime('%a, %d %b %Y %H:%M:%S %Z')
    js_code = ''

    for cookie in cookies:
        name = cookie.get("name", "")
        value = cookie.get("value", "")
        if name and value:
            js_code += f'document.cookie = "{name}={value};expires={expires};"\n'
    js_code += 'location.reload()\n'
    return js_code

cookies = rookiepy.brave(["github.com"])
print(create_js_code(cookies))
```
In this example, I extracted the cookies from the `Brave` browser from the domain 'github.com.'

I cleared all of my browser cookies, executed the code, copied the output. 

Then, I opened `github.com` in the browser and pasted it into the console. As a result, I was logged in into my account.

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



[PyPi-downloads]: https://img.shields.io/pypi/dm/rookiepy
[PyPi-version]: https://img.shields.io/pypi/v/rookiepy?color=00aa00
[PyPi-url]: https://pypi.org/project/rookiepy/
[Crates-badge]: https://img.shields.io/crates/v/rookie
[Crates-url]: https://crates.io/crates/rookie/
[License-shield]: https://img.shields.io/github/license/thewh1teagle/rookie?color=00aaaa
[License-url]: https://github.com/thewh1teagle/rookie/blob/main/rookie-rs/MIT-LICENSE.txt
