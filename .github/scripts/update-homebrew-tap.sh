#!/usr/bin/env bash
# Update the Homebrew tap at helpofai/homebrew-deepseek-tui after a release.
#
# Expected environment:
#   TAG       – git tag, e.g. "v0.8.31"
#   MANIFEST  – path to helpofai-artifacts-sha256.txt
#   TAP_REPO  – owner/repo of the Homebrew tap
#   TOKEN     – PAT with contents:write on TAP_REPO (optional; skips if unset)

set -euo pipefail

: "${TAG:?}"
: "${MANIFEST:?}"
: "${TAP_REPO:?}"

if [ -z "${TOKEN:-}" ]; then
  echo "No Homebrew tap token configured; skipping."
  exit 0
fi

VERSION="${TAG#v}"

die() { echo "::error::${1}" >&2; exit 1; }

sha() {
  local file="${1:?}"
  local val
  val="$(awk -v f="${file}" '$2 == f {print $1; exit}' "${MANIFEST}")"
  if [ -z "${val}" ]; then
    die "Missing binary in checksum manifest: ${file}"
  fi
  echo "${val}"
}

# --- read checksums ---------------------------------------------------

# Canonical dispatcher and TUI
readonly SHA_COD_MACOS_ARM="$(sha helpofai-macos-arm64)"
readonly SHA_TUI_MACOS_ARM="$(sha helpofai-tui-macos-arm64)"
readonly SHA_COD_MACOS_X64="$(sha helpofai-macos-x64)"
readonly SHA_TUI_MACOS_X64="$(sha helpofai-tui-macos-x64)"
readonly SHA_COD_LINUX_ARM="$(sha helpofai-linux-arm64)"
readonly SHA_TUI_LINUX_ARM="$(sha helpofai-tui-linux-arm64)"
readonly SHA_COD_LINUX_X64="$(sha helpofai-linux-x64)"
readonly SHA_TUI_LINUX_X64="$(sha helpofai-tui-linux-x64)"

# --- temp dirs --------------------------------------------------------

FORMULA_FILE="$(mktemp)"
TAP_DIR="$(mktemp -d)"
trap 'rm -rf "${TAP_DIR}" "${FORMULA_FILE}"' EXIT

# --- generate formula --------------------------------------------------

readonly BASE_URL="https://github.com/helpofai/HelpOfAi-Cli/releases/download/${TAG}"

cat > "${FORMULA_FILE}" << EOF
class DeepseekTui < Formula
  desc "Terminal-native coding agent for DeepSeek V4"
  homepage "https://github.com/helpofai/HelpOfAi-Cli"
  version "${VERSION}"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "${BASE_URL}/helpofai-macos-arm64", using: :nounzip
      sha256 "${SHA_COD_MACOS_ARM}"
      resource "tui" do
        url "${BASE_URL}/helpofai-tui-macos-arm64", using: :nounzip
        sha256 "${SHA_TUI_MACOS_ARM}"
      end
    else
      url "${BASE_URL}/helpofai-macos-x64", using: :nounzip
      sha256 "${SHA_COD_MACOS_X64}"
      resource "tui" do
        url "${BASE_URL}/helpofai-tui-macos-x64", using: :nounzip
        sha256 "${SHA_TUI_MACOS_X64}"
      end
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "${BASE_URL}/helpofai-linux-arm64", using: :nounzip
      sha256 "${SHA_COD_LINUX_ARM}"
      resource "tui" do
        url "${BASE_URL}/helpofai-tui-linux-arm64", using: :nounzip
        sha256 "${SHA_TUI_LINUX_ARM}"
      end
    else
      url "${BASE_URL}/helpofai-linux-x64", using: :nounzip
      sha256 "${SHA_COD_LINUX_X64}"
      resource "tui" do
        url "${BASE_URL}/helpofai-tui-linux-x64", using: :nounzip
        sha256 "${SHA_TUI_LINUX_X64}"
      end
    end
  end

  def install
    bin.install Dir["*"].first => "helpofai"
    resource("tui").stage { bin.install Dir["*"].first => "helpofai-tui" }
  end

  test do
    system "#{bin}/helpofai", "--version"
  end
end
EOF

# --- push to tap repo --------------------------------------------------

ENCODED_TOKEN="$(printf '%s' "${TOKEN}" | python3 -c 'import sys,urllib.parse;print(urllib.parse.quote(sys.stdin.read(),safe=""))')"
TAP_URL="https://x-access-token:${ENCODED_TOKEN}@github.com/${TAP_REPO}.git"

git clone --depth 1 "${TAP_URL}" "${TAP_DIR}"

mkdir -p "${TAP_DIR}/Formula"
cp "${FORMULA_FILE}" "${TAP_DIR}/Formula/deepseek-tui.rb"

cd "${TAP_DIR}"
git config user.name  "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"

git add Formula/deepseek-tui.rb

if git diff --cached --quiet; then
  echo "Formula unchanged (already at ${VERSION}); nothing to push."
  exit 0
fi

git commit -m "chore: bump formula to ${VERSION}

Automated update from the release workflow."

git push origin HEAD:main
echo "Pushed formula update to ${TAP_REPO} (v${VERSION})"
