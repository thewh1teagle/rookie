from browser_cookie3 import brave

key = "/home/user/snap/brave/285/.config/BraveSoftware/Brave-Browser/Local State"
db = "/home/user/snap/brave/285/.config/BraveSoftware/Brave-Browser/Default/Cookies"
cookies = brave(key_file=key, cookie_file=db)