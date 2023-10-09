from rookiepy import load

cookies = load()
for cookie in cookies:
    print(f'domain: {cookie["domain"]} name: {cookie["name"]}, value: {cookie["value"]}')