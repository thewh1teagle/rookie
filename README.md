# rookie

[![PyPi Downloads](https://img.shields.io/pypi/dm/rookiepy?logo=python)](https://pypi.org/project/rookiepy/)
[![PyPi Version](https://img.shields.io/pypi/v/rookiepy?color=00aa00&logo=python)](https://pypi.org/project/rookiepy/)
[![NPM Version](https://img.shields.io/npm/v/@rookie-rs/api?logo=npm&color=0076CE)](https://www.npmjs.com/package/@rookie-rs/api)
[![Crates](https://img.shields.io/crates/v/rookie?logo=rust)](https://crates.io/crates/rookie/)
[![License](https://img.shields.io/github/license/thewh1teagle/rookie?color=00aaaa&logo=license)](https://github.com/thewh1teagle/rookie/blob/main/rookie-rs/MIT-LICENSE.txt)

Load cookies from any browser on any platform

## Features 🚀

- Available for `Rust`, `Python`, and `JavaScript`
- Ensures type safety (e.g., `TypeScript`, `Python` with type hints)
- Super Fast, Built with `Rust`
- Bypass `Chrome` restriction of file locking and appbound encryption (requires admin rights on `Windows` from v130.x)
- Read session cookies from `Chrome` based browsers! (requires admin rights on `Windows`)
- Wide browsers support
- Cross-platform support for `Windows`, `Linux`, and `macOS`

## Usage ⚙️

## Rust

```shell
cargo add rookie
```

Create `main.rs` with the following

```rust
use rookie::brave;

fn main() {
    let domains = vec!["google.com"];
    let cookies = brave(Some(domains)).unwrap();
    for cookie in cookies {
        println!("{:?}", cookie);
    }
}
```

## Python

```shell
pip install rookiepy
```

And the usage it similar to Rust

```python
import rookiepy
cookies = rookiepy.firefox(["google.com"])
for cookie in cookies:
    print(cookie['domain'], cookie['value'])
```

## JavaScript

```console
npm install @rookie-rs/api
```

```js
import { brave } from "@rookie-rs/api";
const cookies = brave();
for (const cookie of cookies) {
  console.log(cookie);
}
```

## Examples 📋

`Rust` [examples/rust](examples/rust)

`Python` [examples/python](examples/python)

`JavaScript` [examples/javascript](examples/javascript)

## Docs 📘

`Rust`

- [docs/Rust.md](docs/Rust.md)
- [docs.rs/rookie](https://docs.rs/rookie)

`Python`

- [docs/Python.md](docs/Python.md)

`JavaScript`

- [docs/JavaScript.md](docs/JavaScript.md)

## CLI 💻

You can use rookie as a `CLI` tool which will decrypt the cookies and print it as `JSON`  
See [cli](https://github.com/thewh1teagle/rookie/tree/main/cli) folder

## Contribute 🤝

So far the following platforms are supported:

- **Arc:** `Linux`, `macOS`, `Windows`
- **Brave:** `Linux`, `macOS`, `Windows`
- **Cachy:** `Linux`
- **Chrome:** `Linux`, `macOS`, `Windows`
- **Chromium:** `Linux`, `macOS`, `Windows`
- **Edge:** `Linux`, `macOS`, `Windows`
- **Firefox:** `Linux`, `macOS`, `Windows`
- **Internet Explorer:** `Windows`
- **LibreWolf:** `Linux`, `macOS`, `Windows`
- **Opera:** `Linux`, `macOS`, `Windows`
- **Opera GX:** `macOS`, `Windows`
- **Safari:** `macOS`
- **Vivaldi:** `Linux`, `macOS`, `Windows`

You are welcome to contribute support for other browsers, or other platforms.

## Support new browsers 🌐

If you have a browser with which the library isn't working with, it may not have been added to the list of supported browsers configs. You can create a pull request (PR) or an issue with the path to the cookies file on your computer, and I will add it.

look at [src/windows/config.rs](https://github.com/thewh1teagle/rookie/blob/main/rookie-rs/src/windows/config.rs) to see what configurations is needed.

## Testing Dates (DD/MM/YY) 📅

| Browser   |   Linux    |   macOS    |  Windows   |
| :-------- | :--------: | :--------: | :--------: |
| Arc       | 2024/08/07 | 2024/08/07 | 2024/08/07 |
| Brave     | 2024/10/26 | 2024/10/26 | 2024/10/26 |
| Cachy     | 2024/06/04 |    N/A     |    N/A     |
| Chromium  | 2024/10/26 | 2024/10/26 | 2023/10/01 |
| Chrome    | 2024/10/26 | 2024/10/26 | 2024/03/16 |
| Edge      | 2023/10/01 |     -      | 2023/10/01 |
| Firefox   | 2024/10/26 | 2023/11/25 | 2024/03/16 |
| IE        |    N/A     |    N/A     |  23/10/01  |
| LibreWolf | 2023/10/01 | 2023/11/25 |  23/10/01  |
| Opera     | 2023/10/01 |     -      |  23/10/01  |
| Opera GX  |    N/A     |     -      |  23/10/01  |
| Safari    |    N/A     | 2023/11/25 |    N/A     |
| Vivaldi   | 2023/10/01 | 2023/11/25 | 2023/10/01 |

## Credits 🙌

[github.com/borisbabic/browser_cookie3](https://github.com/borisbabic/browser_cookie3)
