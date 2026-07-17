const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

let fail = 0;

// 1) Get Cargo.toml workspace version
const rootCargo = 'Cargo.toml';
if (!fs.existsSync(rootCargo)) {
    console.error('error: Cargo.toml not found');
    process.exit(1);
}
const cargoText = fs.readFileSync(rootCargo, 'utf8');
const m = cargoText.match(/^version = "([^"]+)"/m);
if (!m) {
    console.error('error: could not find version in Cargo.toml');
    process.exit(1);
}
const workspaceVersion = m[1];

// 2) Get npm version
const npmPkgPath = 'npm/helpofai/package.json';
if (!fs.existsSync(npmPkgPath)) {
    console.error('error: npm/helpofai/package.json not found');
    process.exit(1);
}
let npmVersion;
try {
    const pkg = JSON.parse(fs.readFileSync(npmPkgPath, 'utf8'));
    npmVersion = pkg.version;
} catch (e) {
    console.error(`error parsing package.json: ${e}`);
    process.exit(1);
}

if (workspaceVersion !== npmVersion) {
    console.error(`error: npm version (${npmVersion}) does not match workspace version (${workspaceVersion})`);
    fail = 1;
}

// 3) Get facts version
const factsFile = 'web/lib/facts.generated.ts';
if (!fs.existsSync(factsFile)) {
    console.error('error: web/lib/facts.generated.ts not found');
    process.exit(1);
}
const factsText = fs.readFileSync(factsFile, 'utf8');
const mFacts = factsText.match(/"version":\s*"([^"]+)"/);
if (!mFacts) {
    console.error('error: could not find version in facts.generated.ts');
    process.exit(1);
}
const factsVersion = mFacts[1];

if (workspaceVersion !== factsVersion) {
    console.error(`error: facts version (${factsVersion}) does not match workspace version (${workspaceVersion})`);
    fail = 1;
}

// 4) Verify Cargo.lock sync
try {
    execSync('cargo metadata --locked --format-version 1 --no-deps', { stdio: 'ignore' });
} catch (e) {
    console.error('error: Cargo.lock is out of sync with manifests.');
    fail = 1;
}

if (fail === 0) {
    console.log(`Version state OK: workspace=${workspaceVersion}, npm=${npmVersion}, facts=${factsVersion}, lockfile in sync.`);
} else {
    console.error('Version check failed.');
}

process.exit(fail);
