from .rookiepy import (
    firefox, 
    brave, 
    edge, 
    chrome, 
    chromium_based, 
    chromium, 
    opera, 
    vivaldi
)
from typing import List, Any
import http.cookiejar


__all__ = [
    "firefox",
    "brave",
    "edge",
    "chrome",
    "chromium",
    "opera",
    "vivaldi"
    "chromium_based",
    "firefox_based"
    "to_dict",
    "to_cookiejar",
    "create_cookie",
    "load"
]

def create_cookie(host, path, secure, expires, name, value, http_only):
    """Shortcut function to create a cookie"""
    # HTTPOnly flag goes in _rest, if present (see https://github.com/python/cpython/pull/17471/files#r511187060)
    return http.cookiejar.Cookie(0, name, value, None, False, host, host.startswith('.'), host.startswith('.'), path,
                                 True, secure, expires, False, None, None,
                                 {'HTTPOnly': ''} if http_only else {})


def to_dict(cookies: List[Any]):
    return [
        {'name': c.name,
             'host': c.host,
             'path': c.path,
             'secure': c.secure,
             'expires': c.expires,
             'value': c.value,
             'http_only': c.http_only,
             'same_site': c.same_site
        } 
        for c in cookies
    ]

def to_cookiejar(cookies: List[Any]):
    cj = http.cookiejar.CookieJar()
    
    for cookie_obj in cookies:
        c = create_cookie(
            cookie_obj.host,
            cookie_obj.path,
            cookie_obj.secure,
            cookie_obj.expires,
            cookie_obj.name,
            cookie_obj.value,
            cookie_obj.http_only,

        )
        cj.set_cookie(c)
    return cj