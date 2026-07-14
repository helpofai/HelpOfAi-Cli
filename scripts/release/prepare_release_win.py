import os, pathlib, re, sys

if len(sys.argv) < 2:
    print("usage: python prepare_release_win.py <new-version>")
    sys.exit(1)

new = sys.argv[1]
if not re.match(r"^\d+\.\d+\.\d+$", new):
    print(f"error: '{new}' is not a plain X.Y.Z version")
    sys.exit(1)

# read old version from Cargo.toml
p = pathlib.Path("Cargo.toml")
text = p.read_text(encoding='utf-8')
m = re.search(r'^version = "([^"]+)"', text, re.MULTILINE)
if not m:
    print("error: could not find version in Cargo.toml")
    sys.exit(1)
old = m.group(1)

if old == new:
    print(f"workspace is already at {new}; nothing to bump")
    sys.exit(0)

print(f"Bumping {old} -> {new}")
old_re = re.escape(old)

def bump(path, pattern, repl, minimum):
    p = pathlib.Path(path)
    text = p.read_text(encoding='utf-8')
    out, n = re.subn(pattern, repl, text, flags=re.MULTILINE)
    if n < minimum:
        sys.exit(f"error: expected >= {minimum} replacement(s) in {path}, made {n}")
    p.write_text(out, encoding='utf-8')
    print(f"  {path}: {n} replacement(s)")

# 1) Workspace version.
bump("Cargo.toml", rf'^version = "{old_re}"$', f'version = "{new}"', 1)

# 2) Internal helpofai-* dependency pins in every crate manifest.
total = 0
for manifest in sorted(pathlib.Path("crates").glob("*/Cargo.toml")):
    text = manifest.read_text(encoding='utf-8')
    out, n = re.subn(
        rf'(helpofai-[a-z0-9-]+\s*=\s*\{{[^}}]*version = ")[0-9.]+(")',
        rf"\g<1>{new}\g<2>",
        text,
    )
    if n:
        manifest.write_text(out, encoding='utf-8')
        print(f"  {manifest}: {n} pin(s)")
        total += n
if total == 0:
    sys.exit("error: no internal dependency pins were bumped — wrong old version?")

# 3) npm wrappers.
bump(
    "npm/helpofai/package.json",
    rf'("(?:version|helpofaiBinaryVersion)": ")[0-9.]+(")',
    rf"\g<1>{new}\g<2>",
    2,
)
bump(
    "npm/runtime-sdk/package.json",
    rf'("(?:version)": ")[0-9.]+(")',
    rf"\g<1>{new}\g<2>",
    1,
)

# 4) README install-tag examples (all translations).
# Using a generic regex so we match even if the readme version was out of sync (e.g. 0.8.70).
for readme in ["README.md", "README.zh-CN.md", "README.ja-JP.md", "README.vi.md"]:
    if pathlib.Path(readme).exists():
        bump(readme, rf"--tag v[0-9.]+\b", f"--tag v{new}", 1)

print("Bumping complete.")
