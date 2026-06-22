# Tencent Lighthouse Hong Kong Phone Setup

This runbook sets up a Tencent Cloud Lighthouse instance in Hong Kong as an
always-on helpofai host controlled from Feishu/Lark or Telegram on a phone.

If you are teaching this as the Tencent-native default path, start with
[docs/TENCENT_CLOUD_REMOTE_FIRST.md](TENCENT_CLOUD_REMOTE_FIRST.md). This file
is the implementation runbook for the Lighthouse host itself.

## Target Architecture

```text
CNB mirror or GitHub branch
  -> /opt/whalebro/helpofai

Phone chat app
  -> Feishu/Lark long-connection bot, or Telegram long-polling bot
  -> helpofai-feishu-bridge.service or helpofai-telegram-bridge.service
  -> http://127.0.0.1:7878 helpofai serve --http
  -> /opt/whalebro
       -> helpofai/

Optional public edge:
EdgeOne -> Caddy/Nginx public site on Lighthouse
```

The runtime API must stay on `127.0.0.1`. The bridge is the only phone-facing
control surface. EdgeOne is optional and should only front a deliberate public
HTTP service, not the runtime API.

## Remote Whalebro Workspace

Use `/opt/whalebro` as the VPS workspace root. The first-class checkout is
`/opt/whalebro/helpofai`.

Create these paths first:

- `/opt/whalebro/helpofai`
- `/opt/whalebro/worktrees`

Linux is enough for Rust, Node, and service work. Mac-only release work such
as iOS simulator runs, `.app`/DMG checks, notarization, and Apple signing
still belongs on the Mac.

## Lighthouse Instance

Recommended package for travel:

- Region: Hong Kong (China)
- Image: plain Ubuntu 24.04 LTS or latest Ubuntu LTS
- Size: buy the HK 2 vCPU / 4 GB / 70 GB plan for the first month
- Login: SSH key, not password
- Firewall: SSH open; runtime API on localhost only

Tencent's Lighthouse docs say Linux instances can use SSH keys, and the
Lighthouse firewall opens SSH/HTTP/HTTPS by default.

Use 4 GB RAM for compiling Rust and running the bridge comfortably. A 4 vCPU /
8 GB plan is better for multiple parallel agent workers.

## Phone Bridge Choice

Use Telegram for the simplest MVP: create a bot with `@BotFather`, put the
token in `/etc/helpofai/telegram-bridge.env`, and install services with
`HELPOFAI_BRIDGE=telegram`.

Use Feishu/Lark when you specifically want the Tencent-native path, tenant
controls, or China-enterprise chat integration.

## Feishu / Lark App

Create an enterprise self-built app in:

- Feishu China: `https://open.feishu.cn/app`
- Lark international: `https://open.larksuite.com/app`

Configure:

1. Enable bot capability.
2. Copy App ID and App Secret.
3. Add permissions for message send/receive. The minimum practical set is:
   - `im:message`
   - `im:message:send_as_bot`
   - direct message read permission for your tenant
   - group @message read permission only if you intentionally enable group
     control later
4. Add event subscription `im.message.receive_v1`.
5. Use long connection / WebSocket mode.
6. Publish the app and add the bot to your Feishu/Lark chat.

## Server Bootstrap

SSH into the Lighthouse instance and run:

```bash
sudo apt-get update
sudo apt-get install -y git
export HELPOFAI_BRANCH=main
export HELPOFAI_REPO_URL=https://cnb.cool/helpofai.net/helpofai.git
git clone --branch "$HELPOFAI_BRANCH" "$HELPOFAI_REPO_URL" /tmp/helpofai
cd /tmp/helpofai
sudo HELPOFAI_REPO_URL="$HELPOFAI_REPO_URL" \
  HELPOFAI_REPO_BRANCH="$HELPOFAI_BRANCH" \
  bash scripts/tencent-lighthouse/bootstrap-ubuntu.sh
```

Use an SSH repo URL instead if you want push access from the VPS. If the CNB
mirror is unavailable, fall back to:

```bash
export HELPOFAI_REPO_URL=https://github.com/helpofai/HelpOfAi-Cli.git
```

For stable release docs, confirm the CNB mirror has the branch or tag before
using it:

```bash
export HELPOFAI_REPO_URL=https://cnb.cool/helpofai.net/helpofai.git
git ls-remote "$HELPOFAI_REPO_URL" \
  refs/heads/main \
  refs/tags/v0.8.37
```

The CNB mirror receives `main` and release tags. CNB is the default source for
this Lighthouse path; GitHub is the fallback only when the CNB workflow or
credentials are unhealthy.

If this deployment setup has not been pushed to Git yet, either push the branch
first or copy this checkout to the VPS before running these commands. A fresh
VPS clone cannot see uncommitted local files.

Install Rust 1.88+ for the `helpofai` user, then build both shipped binaries:

```bash
sudo -iu helpofai
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup-init.sh
sed -n '1,120p' /tmp/rustup-init.sh
sh /tmp/rustup-init.sh -y --profile minimal
. "$HOME/.cargo/env"
rustup default stable
cd /opt/whalebro/helpofai
cargo install --path crates/cli --locked --force
cargo install --path crates/tui --locked --force
exit
```

Copy and install the bridge/service files:

```bash
cd /opt/whalebro/helpofai
sudo bash scripts/tencent-lighthouse/install-services.sh
```

For Telegram instead of Feishu/Lark:

```bash
cd /opt/whalebro/helpofai
sudo HELPOFAI_BRIDGE=telegram bash scripts/tencent-lighthouse/install-services.sh
```

After editing both env files, validate the bridge/runtime pairing:

```bash
sudo -u helpofai node /opt/helpofai/bridge/scripts/validate-config.mjs \
  --env /etc/helpofai/feishu-bridge.env \
  --runtime-env /etc/helpofai/runtime.env \
  --workspace-root /opt/whalebro \
  --check-filesystem
```

## Secrets

Generate one runtime token and put the same value in both env files:

```bash
openssl rand -hex 32
sudoedit /etc/helpofai/runtime.env
sudoedit /etc/helpofai/feishu-bridge.env
```

Required values:

- `/etc/helpofai/runtime.env`
  - `HELPOFAI_PROVIDER=deepseek`
  - `HELPOFAI_RUNTIME_TOKEN`
  - `DEEPSEEK_API_KEY`
- `/etc/helpofai/feishu-bridge.env`
  - `FEISHU_APP_ID`
  - `FEISHU_APP_SECRET`
  - `FEISHU_DOMAIN=feishu` for Feishu, `lark` for Lark
  - `HELPOFAI_RUNTIME_TOKEN`
  - `FEISHU_ALLOW_GROUPS=false` for the first deployment

For first pairing, either:

1. Temporarily set `HELPOFAI_ALLOW_UNLISTED=true`, message the bot, copy the
   returned `chat_id`, then set `HELPOFAI_CHAT_ALLOWLIST=<chat_id>` and turn
   unlisted access back off.
2. Or obtain the chat ID from Feishu/Lark event logs and set the allowlist
   before first start.

## Start Services

```bash
sudo systemctl start helpofai-runtime
sudo systemctl status helpofai-runtime --no-pager
curl -s http://127.0.0.1:7878/health

sudo systemctl start helpofai-feishu-bridge
sudo journalctl -u helpofai-feishu-bridge -f
```

For Telegram, use `helpofai-telegram-bridge` for the bridge service name.

Run the Lighthouse doctor after both services are configured:

```bash
cd /opt/whalebro/helpofai
sudo bash scripts/tencent-lighthouse/doctor.sh
```

For Telegram, run:

```bash
sudo HELPOFAI_BRIDGE=telegram bash scripts/tencent-lighthouse/doctor.sh
```

Enable on boot is done by `install-services.sh`; if needed:

```bash
sudo systemctl enable helpofai-runtime helpofai-feishu-bridge
```

For Telegram, enable `helpofai-telegram-bridge` instead of
`helpofai-feishu-bridge`.

## Phone Commands

DMs can be plain text and are the intended first control path:

```text
check git status and summarize what needs attention
```

Group chats are disabled by default. If you later set
`FEISHU_ALLOW_GROUPS=true`, group prompts must start with `/cw`.

Useful commands:

- `/status`
- `/threads`
- `/new`
- `/resume <thread_id>`
- `/interrupt`
- `/compact`
- `/allow <approval_id>`
- `/deny <approval_id>`
- `/allow <approval_id> remember`

Use `remember` only when you intentionally want the runtime thread to flip
toward auto-approval for future tools.

## CNB Deploy Button

After the manual Lighthouse setup passes, CNB can become the repeatable deploy
button:

1. Copy `deploy/tencent-lighthouse/cnb/cnb.yml.example` to `.cnb.yml` in the
   CNB repo.
2. Copy `deploy/tencent-lighthouse/cnb/tag_deploy.yml.example` to
   `.cnb/tag_deploy.yml`.
3. Configure the CNB deploy secrets documented in
   `deploy/tencent-lighthouse/cnb/README.md`.
4. Trigger the `lighthouse-hk` deployment environment.

Keep this manual until the server is boring. Automatic deploys on every push
are convenient later, but they can consume CNB quota and restart the bridge
while a phone turn is active.

## EdgeOne

EdgeOne is not required for the first Feishu/Lark long-connection setup. Add it
only when you need a public HTTPS domain in front of a deliberate public
service on the Lighthouse host.

Good EdgeOne uses:

- public docs or tutorial site
- tiny operator status page
- future webhook-mode bridge endpoint
- demo web app hosted on the same Lighthouse instance

Do not use EdgeOne to expose:

- `http://127.0.0.1:7878`
- `/v1/*` runtime endpoints
- any endpoint that accepts `HELPOFAI_RUNTIME_TOKEN`

## End-to-End Validation

From a phone DM to the bot:

1. Send `/status` and confirm runtime version, localhost bind, auth state,
   workspace, git repo, branch, and dirty counts.
2. Send a harmless prompt such as `summarize git status`.
3. Send `/interrupt` while a turn is active and confirm the turn stops.
4. Send `/threads`, then `/resume <thread_id>` for one listed thread.
5. Trigger a tool approval and verify both `/allow <approval_id>` and
   `/deny <approval_id>` paths.
6. Restart both services and re-run `/status`.
7. Reboot the instance, then confirm `systemctl status helpofai-runtime` and
   `systemctl status helpofai-feishu-bridge` return to active.

## Operational Notes

- Bind `helpofai serve --http` to `127.0.0.1`.
- Keep the Lighthouse firewall focused on SSH for this setup.
- Use SSH key auth.
- Use `tmux` for emergency terminal work from Blink/Termius.
- Keep `/opt/whalebro/helpofai` on a personal branch while working from the
  phone.
