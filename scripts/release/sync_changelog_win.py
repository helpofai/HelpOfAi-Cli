import pathlib, sys

keep = 15
if len(sys.argv) > 1:
    keep = int(sys.argv[1])

root_path = pathlib.Path(".")
changelog_path = root_path.joinpath("CHANGELOG.md") if root_path.joinpath("CHANGELOG.md").exists() else pathlib.Path(__file__).parent.parent.parent.joinpath("CHANGELOG.md")

if not changelog_path.exists():
    print("error: CHANGELOG.md not found")
    sys.exit(1)

content = changelog_path.read_text(encoding='utf-8')
lines = content.splitlines()

output_lines = []
count = 0
for line in lines:
    if line.startswith("[") and "]: http" in line:
        break
    if line.startswith("## [") and "[Unreleased]" not in line:
        count += 1
    if count > keep:
        break
    output_lines.append(line)

output_text = "\n".join(output_lines) + "\n"
output_text += "---\n\n"
output_text += "Older releases: [CHANGELOG.md](https://github.com/helpofai/HelpOfAi-Cli/blob/main/CHANGELOG.md) and [docs/CHANGELOG_ARCHIVE.md](https://github.com/helpofai/HelpOfAi-Cli/blob/main/docs/CHANGELOG_ARCHIVE.md).\n"

tui_changelog_path = pathlib.Path("crates/tui/CHANGELOG.md")
tui_changelog_path.write_text(output_text, encoding='utf-8')
print(f"wrote crates/tui/CHANGELOG.md ({len(output_lines)} lines, {keep} sections kept)")
