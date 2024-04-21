# Gotchas

## Password prompt

This library may trigger a password prompt with kde-wallet on linux / macOS with chromium based browsers when accessing browser cookies.

## Session Cookies Retrieval

Chrome-based browsers have a security feature that prevents external access to their cookies file. To bypass this security measure, we restart the browser seamlessly. As a result, session cookies are retrieved and can be used, but they will expire once the browser is closed again.

## MacOS Safari permission denied

On recent versions of macOS the path of the cookies at `/Users/user/Library/Containers/com.apple.Safari/Data/Library/Cookies/Cookies.binarycookies` has restricted access.
You can grant full disk access through `System Settings` -> Search for `Full disk access` -> And enable it for terminal / the app you running from.

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
