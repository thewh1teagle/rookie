import platform
from rookiepy import arc, brave, chromium, chrome, edge, firefox, librewolf, vivaldi

# On all platforms
browsers_fn = [arc, brave, chromium, chrome, edge, firefox, librewolf, vivaldi]

# Linux
if platform.system() == "Linux":
    from rookiepy import cachy, opera

    browsers_fn.append(cachy)

# Windows
elif platform.system() == "Windows":
    from rookiepy import internet_explorer, opera, opera_gx

    browsers_fn.extend([internet_explorer, opera, opera_gx])

# macOS
elif platform.system() == "Darwin":
    from rookiepy import safari

    browsers_fn.extend([safari])

for fn in browsers_fn:
    cookies = fn()
    print(f"Found {len(cookies)} cookies!")
