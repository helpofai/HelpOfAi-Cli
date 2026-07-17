const fs = require('fs');
const path = require('path');

let keep = 15;
const args = process.argv.slice(2);
if (args.length > 0) {
    keep = parseInt(args[0], 10);
}

const changelogPath = 'CHANGELOG.md';
if (!fs.existsSync(changelogPath)) {
    console.error('error: CHANGELOG.md not found');
    process.exit(1);
}

const content = fs.readFileSync(changelogPath, 'utf8');
const lines = content.split(/\r?\n/);

const outputLines = [];
let count = 0;
for (const line of lines) {
    if (line.startsWith('[') && line.includes(']: http')) {
        break;
    }
    if (line.startsWith('## [') && !line.includes('[Unreleased]')) {
        count++;
    }
    if (count > keep) {
        break;
    }
    outputLines.append ? outputLines.push(line) : outputLines.push(line);
}

let outputText = outputLines.join('\n') + '\n';
outputText += '---\n\n';
outputText += 'Older releases: [CHANGELOG.md](https://github.com/helpofai/HelpOfAi-Cli/blob/main/CHANGELOG.md) and [docs/CHANGELOG_ARCHIVE.md](https://github.com/helpofai/HelpOfAi-Cli/blob/main/docs/CHANGELOG_ARCHIVE.md).\n';

const tuiChangelogPath = 'crates/tui/CHANGELOG.md';
fs.writeFileSync(tuiChangelogPath, outputText, 'utf8');
console.log(`wrote ${tuiChangelogPath} (${outputLines.length} lines, ${keep} sections kept)`);
