with open(r'Cargo.toml', 'r', encoding='utf-8') as f:
    text = f.read()
text = text.replace('version = "0.8.76"', 'version = "0.9.1"')
with open(r'Cargo.toml', 'w', encoding='utf-8') as f:
    f.write(text)

with open(r'npm/runtime-sdk/package.json', 'r', encoding='utf-8') as f:
    import json
    data = json.load(f)
    data['version'] = '0.9.1'
with open(r'npm/runtime-sdk/package.json', 'w', encoding='utf-8') as f:
    json.dump(data, f, indent=2)
print('Done')
