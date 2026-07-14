import pathlib, re, sys, subprocess, json

fail = 0

# 1) Get Cargo.toml workspace version
root_cargo = pathlib.Path("Cargo.toml")
if not root_cargo.exists():
    print("error: Cargo.toml not found")
    sys.exit(1)
text = root_cargo.read_text(encoding='utf-8')
m = re.search(r'^version = "([^"]+)"', text, re.MULTILINE)
if not m:
    print("error: could not find version in Cargo.toml")
    sys.exit(1)
workspace_version = m.group(1)

# 2) Get npm version
npm_pkg = pathlib.Path("npm/helpofai/package.json")
if not npm_pkg.exists():
    print("error: npm/helpofai/package.json not found")
    sys.exit(1)
try:
    pkg = json.loads(npm_pkg.read_text(encoding='utf-8'))
    npm_version = pkg.get("version")
except Exception as e:
    print(f"error parsing package.json: {e}")
    sys.exit(1)

if workspace_version != npm_version:
    print(f"error: npm version ({npm_version}) does not match workspace version ({workspace_version})")
    fail = 1

# 3) Get facts version
facts_file = pathlib.Path("web/lib/facts.generated.ts")
if not facts_file.exists():
    print("error: web/lib/facts.generated.ts not found")
    sys.exit(1)
facts_text = facts_file.read_text(encoding='utf-8')
m_facts = re.search(r'"version":\s*"([^"]+)"', facts_text)
if not m_facts:
    print("error: could not find version in facts.generated.ts")
    sys.exit(1)
facts_version = m_facts.group(1)

if workspace_version != facts_version:
    print(f"error: facts version ({facts_version}) does not match workspace version ({workspace_version})")
    fail = 1

# 4) Verify Cargo.lock sync
try:
    res = subprocess.run(["cargo", "metadata", "--locked", "--format-version", "1", "--no-deps"], capture_output=True, text=True)
    if res.returncode != 0:
        print("error: Cargo.lock is out of sync with manifests.")
        fail = 1
except Exception as e:
    print(f"error running cargo metadata: {e}")
    fail = 1

if fail == 0:
    print(f"Version state OK: workspace={workspace_version}, npm={npm_version}, facts={facts_version}, lockfile in sync.")
else:
    print("Version check failed.")
sys.exit(fail)
