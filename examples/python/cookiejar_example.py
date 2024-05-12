from rookiepy import brave, to_cookiejar

cookies = brave()
cj = to_cookiejar(cookies)
print(cj)