import rookiepy
from pathlib import Path
from os import getenv

# Using pathlib and cross platform paths is always recommended!
localappdata = getenv('LOCALAPPDATA')
db_path = Path(localappdata) / 'BraveSoftware/Brave-Browser/User Data/default/network/Cookies'
key_path = Path(localappdata) / 'BraveSoftware/Brave-Browser/User Data/Local State' # Optional
cookies = rookiepy.any_browser(db_path=str(db_path), key_path=str(key_path), domains=None)
print(cookies)