#!/usr/bin/env node

const notice = [
  "",
  "  ╭───────────────────────────────────────────────────────────────────╮",
  "  │                                                                   │",
  "  │  deepseek-tui has been renamed to `helpofai`.                    │",
  "  │                                                                   │",
  "  │  Please uninstall this package and install helpofai instead:     │",
  "  │                                                                   │",
  "  │    npm uninstall -g deepseek-tui                                  │",
  "  │    npm install -g helpofai                                       │",
  "  │                                                                   │",
  "  │  helpofai ships the same `helpofai` and `helpofai-tui`         │",
  "  │  binaries plus deprecation shims under the old names. See:        │",
  "  │  https://github.com/helpofai/HelpOfAi-Cli/blob/main/docs/REBRAND.md │",
  "  │                                                                   │",
  "  ╰───────────────────────────────────────────────────────────────────╯",
  "",
].join("\n");

process.stderr.write(notice);
