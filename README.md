# rookie

[![PyPi Downloads][PyPi-downloads]][PyPi-url]
[![PyPi Version][PyPi-version]][PyPi-url]
[![NPM Version][NPM-version]][NPM-url]
[![Crates][Crates-badge]][Crates-url]
[![License][License-shield]][License-url]

Load cookies from any browser on any platform

## Install ⚙️

```shell
cargo add rookie
```

## Usage

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

## Bindings 📚

There's `Python` and `JavaScript` bindings for rookie!

## Python

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

`Python` [examples/python](examples/python)

`JavaScript` [examples/javascript](examples/javascript)

`Rust` [examples/rust](examples/rust)

## Docs 📘

`Python`

- [docs/Python.md](docs/Python.md)

`JavaScript`

- [docs/JavaScript.md](docs/JavaScript.md)

`Rust`

- [docs/Rust.md](docs/Rust.md)
- [docs.rs/rookie](https://docs.rs/rookie)

## Features 🚀

- Fast, written in `Rust`
- Bypass `Chrome` restriction like file locking
- Read session cookies from `Chrome` based browsers! (`Windows`)
- Wide browsers support
- Cross-platform support for `Windows`, `Linux`, and `macOS`
- Available for `Rust`, `Python`, and `JavaScript`
- Ensures type safety (e.g., `TypeScript`, `Python` with type hints)

## CLI 💻

You can use rookie as a `CLI` tool which will decrypt the cookies and print it as `JSON`  
See [cli](https://github.com/thewh1teagle/rookie/tree/main/cli) folder

## Contribute 🤝

So far the following platforms are supported:

- **Chrome:** `Linux`, `macOS`, `Windows`
- **Firefox:** `Linux`, `macOS`, `Windows`
- **LibreWolf:** `Linux`, `macOS`, `Windows`
- **Cachy:** `Linux`
- **Opera:** `Linux`, `macOS`, `Windows`
- **Opera GX:** `macOS`, `Windows`
- **Edge:** `Linux`, `macOS`, `Windows`
- **Internet Explorer:** `Windows`
- **Chromium:** `Linux`, `macOS`, `Windows`
- **Brave:** `Linux`, `macOS`, `Windows`
- **Vivaldi:** `Linux`, `macOS`, `Windows`
- **Safari:** `macOS`

You are welcome to contribute support for other browsers, or other platforms.

## Support new browsers 🌐

If you have a browser with which the library isn't working with, it may not have been added to the list of supported browsers configs. You can create a pull request (PR) or an issue with the path to the cookies file on your computer, and I will add it.

look at [config.rs](https://github.com/thewh1teagle/rookie/blob/main/rookie-rs/src/config.rs) to see what configurations is needed.

## Testing Dates (DD/MM/YY) 📅

| Browser   |  Linux   |  macOS   | Windows  |
| :-------- | :------: | :------: | :------: |
| Chrome    | 01/10/23 | 25/11/23 | 16/03/24 |
| Firefox   | 01/10/23 | 25/11/23 | 16/03/24 |
| LibreWolf | 01/10/23 | 25/11/23 | 01/10/23 |
| Cachy     | 04/06/24 |   N/A    |   N/A    |
| Opera     | 01/10/23 |    -     | 01/10/23 |
| Opera GX  |   N/A    |    -     | 01/10/23 |
| Edge      | 01/10/23 |    -     | 01/10/23 |
| IE        |   N/A    |   N/A    | 01/10/23 |
| Chromium  | 01/10/23 | 25/11/23 | 01/10/23 |
| Brave     | 01/10/23 | 25/11/23 | 01/10/23 |
| Vivaldi   | 01/10/23 | 25/11/23 | 01/10/23 |
| Safari    |   N/A    | 02/10/23 |   N/A    |

## Credits 🙌

[github.com/borisbabic/browser_cookie3](https://github.com/borisbabic/browser_cookie3)

[PyPi-downloads]: https://img.shields.io/pypi/dm/rookiepy?logo=python
[PyPi-version]: https://img.shields.io/pypi/v/rookiepy?color=00aa00&logo=python
[PyPi-url]: https://pypi.org/project/rookiepy/
[NPM-version]: https://img.shields.io/npm/v/@rookie-rs/api?logo=npm&color=0076CE

[NPM-url]: [https://npm.com](https://www.npmjs.com/package/@thewh1teagle/rookie)
[Crates-badge]: https://img.shields.io/crates/v/rookie?logo=rust
[Crates-url]: https://crates.io/crates/rookie/
[License-shield]: https://img.shields.io/github/license/thewh1teagle/rookie?color=00aaaa&logo=license
[License-url]: https://github.com/thewh1teagle/rookie/blob/main/rookie-rs/MIT-LICENSE.txt
