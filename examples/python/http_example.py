import requests
import re
from rookiepy import brave, to_cookiejar

def extract_username(html):
    re_pattern = r'dashboard/ajax_context_list\?current_context=(.+)'
    match = re.search(re_pattern, html)
    if match:
        return match.group(1)
    return ""


def main():
    try:
        # Create a custom cookie store
        cookies = brave(["github.com"])
        cj = to_cookiejar(cookies)

        headers = {
            "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36",
        }
        response = requests.get("https://github.com/", cookies=cj, headers=headers)
        content = response.text
        username = extract_username(content)
        if not username:
            print("Not logged in to Github")
        else:
            print(f"Logged in to Github as {username}")

    except Exception as e:
        print(f"An error occurred: {e}")

main()