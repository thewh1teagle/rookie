from pathlib import Path
import re


def increment_ver(version):
    version = version.split('.')
    version[2] = str(int(version[2]) + 1)
    return '.'.join(version)

cur = Path(__file__).parent
cargo_paths = [cur / '../rookie-rs', cur / '../bindings/python']


for path in cargo_paths:
    toml = path / 'cargo.toml'
    content = toml.open('r').read()
    pattern = 'version = "(.+)"[\S\s]edition = ".+"'
    ver = re.search(pattern, content)
    if ver:
        new_ver = increment_ver(ver.group(1))
        new_content = re.sub(pattern, f'version = "{new_ver}"\nedition = "2021"', content)
        with toml.open('w') as f:
            f.write(new_content)

# cd rookie-rs && cargo publish