#!/usr/bin/env bash
set -euo pipefail

if [[ "${EUID}" -ne 0 ]]; then
  echo "Run as root: sudo bash scripts/tencent-lighthouse/bootstrap-ubuntu.sh" >&2
  exit 1
fi

HELPOFAI_USER="${HELPOFAI_USER:-${DEEPSEEK_USER:-helpofai}}"
HELPOFAI_ROOT="${HELPOFAI_ROOT:-${DEEPSEEK_ROOT:-/opt/helpofai}}"
WHALEBRO_ROOT="${WHALEBRO_ROOT:-/opt/whalebro}"
REPO_URL="${HELPOFAI_REPO_URL:-${DEEPSEEK_REPO_URL:-https://github.com/helpofai/HelpOfAi-Cli.git}}"
WHALEBRO_EXTRA_REPOS="${WHALEBRO_EXTRA_REPOS:-}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
SOURCE_BRANCH="$(git -C "${SOURCE_ROOT}" branch --show-current 2>/dev/null || true)"
REPO_BRANCH="${HELPOFAI_REPO_BRANCH:-${DEEPSEEK_REPO_BRANCH:-${SOURCE_BRANCH:-main}}}"

apt-get update
apt-get install -y \
  ca-certificates \
  curl \
  git \
  iproute2 \
  openssh-client \
  build-essential \
  pkg-config \
  libdbus-1-dev \
  libssl-dev \
  nodejs \
  npm \
  rsync \
  tmux \
  fail2ban \
  ufw

node_major="$(node -p "Number(process.versions.node.split('.')[0])")"
if (( node_major < 18 )); then
  echo "Node.js 18+ is required for the phone bridges; install a newer Node.js before running install-services.sh." >&2
fi

if ! id -u "${HELPOFAI_USER}" >/dev/null 2>&1; then
  useradd --create-home --shell /bin/bash "${HELPOFAI_USER}"
fi

install -d -o "${HELPOFAI_USER}" -g "${HELPOFAI_USER}" "${HELPOFAI_ROOT}"
install -d -o "${HELPOFAI_USER}" -g "${HELPOFAI_USER}" "${HELPOFAI_ROOT}/bridge"
install -d -o "${HELPOFAI_USER}" -g "${HELPOFAI_USER}" "${HELPOFAI_ROOT}/telegram-bridge"
install -d -o "${HELPOFAI_USER}" -g "${HELPOFAI_USER}" "${WHALEBRO_ROOT}"
install -d -o "${HELPOFAI_USER}" -g "${HELPOFAI_USER}" "${WHALEBRO_ROOT}/worktrees"
install -d -m 0750 -o root -g "${HELPOFAI_USER}" /etc/helpofai
install -d -m 0700 -o "${HELPOFAI_USER}" -g "${HELPOFAI_USER}" /var/lib/helpofai-feishu-bridge
install -d -m 0700 -o "${HELPOFAI_USER}" -g "${HELPOFAI_USER}" /var/lib/helpofai-telegram-bridge

if [[ ! -d "${WHALEBRO_ROOT}/helpofai/.git" ]]; then
  sudo -u "${HELPOFAI_USER}" git clone --branch "${REPO_BRANCH}" "${REPO_URL}" "${WHALEBRO_ROOT}/helpofai"
fi

for repo_spec in ${WHALEBRO_EXTRA_REPOS}; do
  repo_name="${repo_spec%%=*}"
  repo_url="${repo_spec#*=}"
  if [[ -z "${repo_name}" || -z "${repo_url}" || "${repo_name}" == "${repo_url}" ]]; then
    echo "Skipping malformed WHALEBRO_EXTRA_REPOS entry: ${repo_spec}" >&2
    continue
  fi
  if [[ ! -d "${WHALEBRO_ROOT}/${repo_name}/.git" ]]; then
    sudo -u "${HELPOFAI_USER}" git clone "${repo_url}" "${WHALEBRO_ROOT}/${repo_name}" || {
      echo "Warning: failed to clone optional repo ${repo_name} from ${repo_url}" >&2
    }
  fi
done

if [[ ! -f /etc/helpofai/runtime.env ]]; then
  cat >/etc/helpofai/runtime.env <<'EOF'
HELPOFAI_RUNTIME_TOKEN=replace-with-long-random-token
HELPOFAI_RUNTIME_PORT=7878
HELPOFAI_RUNTIME_WORKERS=2
HELPOFAI_PROVIDER=deepseek
DEEPSEEK_API_KEY=replace-with-provider-key
RUST_LOG=info
EOF
  chown root:"${HELPOFAI_USER}" /etc/helpofai/runtime.env
  chmod 0640 /etc/helpofai/runtime.env
fi

if [[ ! -f /etc/helpofai/feishu-bridge.env ]]; then
  cat >/etc/helpofai/feishu-bridge.env <<'EOF'
FEISHU_APP_ID=cli_xxxxxxxxxxxxxxxx
FEISHU_APP_SECRET=replace-with-app-secret
FEISHU_DOMAIN=feishu
HELPOFAI_RUNTIME_URL=http://127.0.0.1:7878
HELPOFAI_RUNTIME_TOKEN=replace-with-same-token-as-runtime-env
HELPOFAI_WORKSPACE=/opt/whalebro
HELPOFAI_MODEL=auto
HELPOFAI_MODE=agent
HELPOFAI_ALLOW_SHELL=true
HELPOFAI_TRUST_MODE=false
HELPOFAI_AUTO_APPROVE=false
HELPOFAI_CHAT_ALLOWLIST=
HELPOFAI_ALLOW_UNLISTED=false
FEISHU_THREAD_MAP_PATH=/var/lib/helpofai-feishu-bridge/thread-map.json
FEISHU_ALLOW_GROUPS=false
FEISHU_REQUIRE_PREFIX_IN_GROUP=true
FEISHU_GROUP_PREFIX=/cw
FEISHU_MAX_REPLY_CHARS=3500
HELPOFAI_TURN_TIMEOUT_MS=900000
EOF
  chown root:"${HELPOFAI_USER}" /etc/helpofai/feishu-bridge.env
  chmod 0640 /etc/helpofai/feishu-bridge.env
fi

ufw allow OpenSSH
ufw --force enable

cat <<EOF

Base server setup complete.

Next:
1. Install Rust 1.88+ for ${HELPOFAI_USER}; rustup is the usual path.
2. Build/install both binaries:
   sudo -iu ${HELPOFAI_USER}
   cd ${WHALEBRO_ROOT}/helpofai
   cargo install --path crates/cli --locked --force
   cargo install --path crates/tui --locked --force
3. Copy integrations/feishu-bridge or integrations/telegram-bridge to ${HELPOFAI_ROOT} and run npm install.
4. Edit /etc/helpofai/runtime.env and the selected bridge env file.
5. Install systemd units with scripts/tencent-lighthouse/install-services.sh.
6. After the env files are edited and services are started, run:
   sudo bash scripts/tencent-lighthouse/doctor.sh

EOF
