#!/usr/bin/env bash
# Source into an interactive agent shell (tmux, ssh) to export the provider
# key and set defaults that systemd normally handles via EnvironmentFile=.
#
# Usage (as the helpofai user):
#   . /opt/whalebro/helpofai/scripts/remote-smoke/agent-session.sh
#   helpofai models           # should list deepseek-v4-pro
#   gh auth status             # should show the fine-grained PAT
#
# The runtime.env file is 0640 root:helpofai, readable by the helpofai user.
set -a
# shellcheck disable=SC1091
. /etc/helpofai/runtime.env
set +a
export HELPOFAI_MODEL="${HELPOFAI_MODEL:-deepseek-v4-pro}"
