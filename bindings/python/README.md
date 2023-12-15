# rookiepy

Extract cookies from web browsers
Bindings for [rookie](https://github.com/thewh1teagle/rookie)

## Usage

```python
from rookiepy import chrome
cookies = chrome()
for cookie in cookies:
    print(cookie['domain'], cookie['name'], cookie['value'])
```
