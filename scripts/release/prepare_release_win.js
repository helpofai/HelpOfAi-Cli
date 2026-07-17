const fs = require('fs');
const path = require('path');

const args = process.argv.slice(2);
if (args.length < 1) {
    console.error('usage: node prepare_release_win.js <new-version>');
    process.exit(1);
}

const newVersion = args[0];
if (!/^\d+\.\d+\.\d+$/.test(newVersion)) {
    console.error(`error: '${newVersion}' is not a plain X.Y.Z version`);
    process.exit(1);
}

// 1. Cargo.toml workspace
let cargo = fs.readFileSync('Cargo.toml', 'utf8');
const oldVersionMatch = cargo.match(/^version = "([^"]+)"/m);
if (!oldVersionMatch) {
    console.error('error: could not find version in Cargo.toml');
    process.exit(1);
}
const oldVersion = oldVersionMatch[1];
if (oldVersion === newVersion) {
    console.log(`workspace is already at ${newVersion}; nothing to bump`);
    process.exit(0);
}

console.log(`Bumping ${oldVersion} -> ${newVersion}`);

cargo = cargo.replace(/^version = "[^"]+"/m, `version = "${newVersion}"`);
fs.writeFileSync('Cargo.toml', cargo, 'utf8');
console.log('  Cargo.toml: version bumped');

// 2. crates/*/Cargo.toml
const cratesDir = 'crates';
fs.readdirSync(cratesDir).forEach(dir => {
    const manifestPath = path.join(cratesDir, dir, 'Cargo.toml');
    if (fs.existsSync(manifestPath)) {
        let manifest = fs.readFileSync(manifestPath, 'utf8');
        const original = manifest;
        // Bump both workspace dependency pins and version if defined
        manifest = manifest.replace(/(helpofai-[a-z0-9-]+\s*=\s*\{[^}]*version = ")[0-9.]+(",?)/g, `$1${newVersion}$2`);
        manifest = manifest.replace(/(helpflow\s*=\s*\{[^}]*version = ")[0-9.]+(",?)/g, `$1${newVersion}$2`);
        if (manifest.includes('version = "') && !manifest.includes('version = "workspace"')) {
            manifest = manifest.replace(/^version = "[^"]+"/m, `version = "${newVersion}"`);
        }
        if (manifest !== original) {
            fs.writeFileSync(manifestPath, manifest, 'utf8');
            console.log(`  ${manifestPath}: pins bumped`);
        }
    }
});

// 3. npm/helpofai/package.json & npm/runtime-sdk/package.json
const npmPaths = [
    'npm/helpofai/package.json',
    'npm/runtime-sdk/package.json'
];
npmPaths.forEach(p => {
    if (fs.existsSync(p)) {
        let pkg = fs.readFileSync(p, 'utf8');
        pkg = pkg.replace(/"(version|helpofaiBinaryVersion)": "[0-9.]+"/g, `"$1": "${newVersion}"`);
        fs.writeFileSync(p, pkg, 'utf8');
        console.log(`  ${p}: version bumped`);
    }
});

// 4. README files
const readmes = ['README.md', 'README.zh-CN.md', 'README.ja-JP.md', 'README.vi.md'];
readmes.forEach(r => {
    if (fs.existsSync(r)) {
        let text = fs.readFileSync(r, 'utf8');
        text = text.replace(/--tag v[0-9.]+\b/g, `--tag v${newVersion}`);
        fs.writeFileSync(r, text, 'utf8');
        console.log(`  ${r}: readme tags bumped`);
    }
});

console.log('Bumping complete.');
