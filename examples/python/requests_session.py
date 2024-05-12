import rookiepy
import requests

def create_session() -> requests.Session:
    """
    Create requests session with cookiejar that contains web browsers cookies
    """
    # Load cookies from browser
    cookies = rookiepy.load()
    # Create Cookiejar from cookies
    cj = rookiepy.to_cookiejar(cookies)
    # Create session
    session = requests.session()
    # Set session cookiejar
    session.cookies = cj
    return session

session = create_session()
session.get('https://google.com/')