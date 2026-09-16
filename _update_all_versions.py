import os

files_to_update = [
    'crates/agent/Cargo.toml',
    'crates/app-server/Cargo.toml',
    'crates/cli/Cargo.toml',
    'crates/config/Cargo.toml',
    'crates/core/Cargo.toml',
    'crates/execpolicy/Cargo.toml',
    'crates/hooks/Cargo.toml',
    'crates/tools/Cargo.toml',
    'crates/tui/Cargo.toml',
    'npm/helpofai/package.json',
    'README.md',
    'README.zh-CN.md',
    'README.ja-JP.md',
    'README.vi.md',
    'CHANGELOG.md',
]

for path in files_to_update:
    if os.path.exists(path):
        with open(path, 'r', encoding='utf-8') as f:
            content = f.read()
        new_content = content.replace('0.8.99', '0.9.1')
        with open(path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f'Updated {path}')
