from rookiepy import load

cookies = load()
for cookie in cookies:
    print(f'host: {cookie.host} name: {cookie.name}, value: {cookie.value}')