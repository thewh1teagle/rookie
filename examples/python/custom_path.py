import rookiepy
from pathlib import Path

# Using pathlib and cross platform paths is always recommended!
db_path = Path("C:/Users/User/AppData/Local/BraveSoftware/Brave-Browser/User Data/default/network/Cookies").resolve().absolute()
key_path = Path("C:/Users/User/AppData/Local/BraveSoftware/Brave-Browser/User Data/Local State").resolve().absolute() # optional
db_path, key_path = str(db_path), str(key_path)
cookies = rookiepy.any_browser(db_path=db_path, key_path=key_path, domains=None)
print(cookies)