from rookiepy import chrome, to_cookiejar

cookies = chrome()
cj = to_cookiejar(cookies)
print(cj)