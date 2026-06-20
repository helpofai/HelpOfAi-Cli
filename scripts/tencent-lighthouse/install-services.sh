#!/usr/bin/env bash
set -euo pipefail

if [[ "${EUID}" -ne 0 ]]; then
  echo "Run as root: sudo bash scripts/tencent-lighthouse/install-services.sh" >&2
  exit 1
fi

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
HELPOFAI_USER="${HELPOFAI_USER:-${DEEPSEEK_USER:-helpofai}}"
HELPOFAI_ROOT="${HELPOFAI_ROOT:-${DEEPSEEK_ROOT:-/opt/helpofai}}"
BRIDGE_KIND="${HELPOFAI_BRIDGE:-${DEEPSEEK_BRIDGE:-feishu}}"

case "${BRIDGE_KIND}" in
  feishu|lark)
    BRIDGE_SRC="integrations/feishu-bridge"
    BRIDGE_DST="${HELPOFAI_ROOT}/bridge"
    BRIDGE_UNIT="helpofai-feishu-bridge.service"
    BRIDGE_ENV="/etc/helpofai/feishu-bridge.env"
    BRIDGE_ENV_EXAMPLE="deploy/tencent-lighthouse/examples/feishu-bridge.env.example"
    BRIDGE_STATE_DIR="/var/lib/helpofai-feishu-bridge"
    VALIDATOR="integrations/feishu-bridge/scripts/validate-config.mjs"
    ;;
  telegram)
    BRIDGE_SRC="integrations/telegram-bridge"
    BRIDGE_DST="${HELPOFAI_ROOT}/telegram-bridge"
    BRIDGE_UNIT="helpofai-telegram-bridge.service"
    BRIDGE_ENV="/etc/helpofai/telegram-bridge.env"
    BRIDGE_ENV_EXAMPLE="deploy/tencent-lighthouse/examples/telegram-bridge.env.example"
    BRIDGE_STATE_DIR="/var/lib/helpofai-telegram-bridge"
    VALIDATOR="integrations/telegram-bridge/scripts/validate-config.mjs"
    ;;
  *)
    echo "Unknown bridge '${BRIDGE_KIND}'. Use HELPOFAI_BRIDGE=feishu or HELPOFAI_BRIDGE=telegram." >&2
    exit 1
    ;;
esac

install -d -m 0750 -o root -g "${HELPOFAI_USER}" /etc/helpofai
install -d -m 0700 -o "${HELPOFAI_USER}" -g "${HELPOFAI_USER}" "${BRIDGE_STATE_DIR}"
install -d -o "${HELPOFAI_USER}" -g "${HELPOFAI_USER}" "${BRIDGE_DST}"

if [[ ! -f /etc/helpofai/runtime.env && -f "${REPO_ROOT}/deploy/tencent-lighthouse/examples/runtime.env.example" ]]; then
  install -m 0640 -o root -g "${HELPOFAI_USER}" \
    "${REPO_ROOT}/deploy/tencent-lighthouse/examples/runtime.env.example" \
    /etc/helpofai/runtime.env
fi

if [[ ! -f "${BRIDGE_ENV}" && -f "${REPO_ROOT}/${BRIDGE_ENV_EXAMPLE}" ]]; then
  install -m 0640 -o root -g "${HELPOFAI_USER}" \
    "${REPO_ROOT}/${BRIDGE_ENV_EXAMPLE}" \
    "${BRIDGE_ENV}"
fi
rsync -a --delete \
  --exclude node_modules \
  "${REPO_ROOT}/${BRIDGE_SRC}/" \
  "${BRIDGE_DST}/"
chown -R "${HELPOFAI_USER}:${HELPOFAI_USER}" "${BRIDGE_DST}"

if [[ -f "${BRIDGE_DST}/package-lock.json" ]]; then
  sudo -u "${HELPOFAI_USER}" npm --prefix "${BRIDGE_DST}" ci --omit=dev
else
  sudo -u "${HELPOFAI_USER}" npm --prefix "${BRIDGE_DST}" install --omit=dev
fi

install -m 0644 "${REPO_ROOT}/deploy/tencent-lighthouse/systemd/helpofai-runtime.service" /etc/systemd/system/helpofai-runtime.service
install -m 0644 "${REPO_ROOT}/deploy/tencent-lighthouse/systemd/${BRIDGE_UNIT}" "/etc/systemd/system/${BRIDGE_UNIT}"

systemctl daemon-reload
systemctl enable helpofai-runtime "${BRIDGE_UNIT}"

cat <<'EOF'
Services installed but not started.

Before starting, verify:
  /etc/helpofai/runtime.env
EOF
cat <<EOF
  ${BRIDGE_ENV}
  sudo -u ${HELPOFAI_USER} node ${REPO_ROOT}/${VALIDATOR} --env ${BRIDGE_ENV} --runtime-env /etc/helpofai/runtime.env --workspace-root /opt/whalebro --check-filesystem
Then run:
  sudo systemctl start helpofai-runtime
  sudo systemctl start ${BRIDGE_UNIT}
  sudo HELPOFAI_BRIDGE=${BRIDGE_KIND} bash /opt/whalebro/helpofai/scripts/tencent-lighthouse/doctor.sh
  sudo journalctl -u ${BRIDGE_UNIT} -f
EOF
