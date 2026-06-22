const fs = require("fs");
const { execFileSync } = require("child_process");

const GLIBC_VERSION_RE = /GLIBC_(\d+)\.(\d+)(?:\.(\d+))?/g;

function isLinux() {
  return process.platform === "linux";
}

function parseVersion(text) {
  const match = String(text || "").match(/(\d+)\.(\d+)(?:\.(\d+))?/);
  if (!match) return null;
  return [Number(match[1]), Number(match[2]), Number(match[3] || 0)];
}

function compareVersion(a, b) {
  for (let i = 0; i < 3; i += 1) {
    if (a[i] !== b[i]) return a[i] - b[i];
  }
  return 0;
}

function formatVersion(version) {
  return version[2] ? `${version[0]}.${version[1]}.${version[2]}` : `${version[0]}.${version[1]}`;
}

function detectHostGlibc() {
  try {
    const out = execFileSync("getconf", ["GNU_LIBC_VERSION"], {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    });
    const version = parseVersion(out);
    if (version) return version;
  } catch {
    // fall through to ldd
  }
  try {
    const out = execFileSync("ldd", ["--version"], {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    });
    const firstLine = out.split("\n", 1)[0];
    const version = parseVersion(firstLine);
    if (version) return version;
  } catch {
    // glibc not present (e.g. musl / Alpine)
  }
  return null;
}

function detectBinaryRequiredGlibc(filePath) {
  const buf = fs.readFileSync(filePath);
  const text = buf.toString("latin1");
  let highest = null;
  GLIBC_VERSION_RE.lastIndex = 0;
  let match;
  while ((match = GLIBC_VERSION_RE.exec(text)) !== null) {
    const version = [Number(match[1]), Number(match[2]), Number(match[3] || 0)];
    if (!highest || compareVersion(version, highest) > 0) {
      highest = version;
    }
  }
  return highest;
}

function buildFromSourceHint() {
  return [
    "You can still run helpofai by building from source with Cargo:",
    "",
    "  # Requires Rust 1.88+ (https://rustup.rs)",
    "  cargo install helpofai-cli --locked   # provides `helpofai`",
    "  cargo install helpofai-tui --locked   # provides `helpofai-tui`",
    "",
    "Or build from a checkout:",
    "",
    "  git clone https://github.com/helpofai/HelpOfAi-Cli.git",
    "  cd HelpOfAi",
    "  cargo install --path crates/cli --locked",
    "  cargo install --path crates/tui --locked",
    "",
    "See https://github.com/helpofai/HelpOfAi-Cli/blob/main/docs/INSTALL.md",
  ].join("\n");
}

function skipGlibcCheck() {
  return (
    process.env.HELPOFAI_SKIP_GLIBC_CHECK === "1" ||
    process.env.DEEPSEEK_TUI_SKIP_GLIBC_CHECK === "1" ||
    process.env.DEEPSEEK_SKIP_GLIBC_CHECK === "1"
  );
}

function glibcCompatibilityMessage(required, host) {
  const hostLine = host
    ? `this system has glibc ${formatVersion(host)}, which is too old for that asset.`
    : "this system does not appear to provide GNU libc.";
  return [
    `Prebuilt HelpOfAi Linux binaries require GLIBC_${formatVersion(required)}, but ${hostLine}`,
    "",
    "Official Linux release binaries are GNU libc builds. Ubuntu 22.04 ships glibc 2.35,",
    "so it cannot run a binary built against Ubuntu 24.04/glibc 2.39.",
    "",
    buildFromSourceHint(),
    "",
    "Release engineering follow-up: build Linux GNU assets against an older glibc",
    "baseline, or add a musl/static Linux asset.",
    "",
    "Set HELPOFAI_SKIP_GLIBC_CHECK=1 to bypass this check at your own risk.",
  ].join("\n");
}

function preflightGlibc(filePath) {
  if (!isLinux()) return;
  if (skipGlibcCheck()) {
    return;
  }

  const required = detectBinaryRequiredGlibc(filePath);
  if (!required) {
    // Statically linked / musl binary, or no GLIBC_* version dependencies present.
    return;
  }

  const host = detectHostGlibc();
  if (!host) {
    throw new Error(glibcCompatibilityMessage(required, null));
  }

  if (compareVersion(host, required) < 0) {
    throw new Error(glibcCompatibilityMessage(required, host));
  }
}

module.exports = {
  preflightGlibc,
  detectHostGlibc,
  detectBinaryRequiredGlibc,
  // exported for tests
  _internal: {
    parseVersion,
    compareVersion,
    formatVersion,
    glibcCompatibilityMessage,
    skipGlibcCheck,
  },
};
