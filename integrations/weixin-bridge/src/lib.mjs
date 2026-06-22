import crypto from "node:crypto";

// ============================================================================
// iLink Bot API 协议层 — 参考 @tencent-weixin/openclaw-weixin
// ============================================================================

const DEFAULT_API_TIMEOUT_MS = 30_000;
const LONGPOLL_DEFAULT_TIMEOUT_MS = 35_000;

// ---------------------------------------------------------------------------
// 通用工具
// ---------------------------------------------------------------------------

export function parseList(raw) {
  return String(raw || "")
    .split(",")
    .map((item) => item.trim())
    .filter(Boolean);
}

export function parseBool(raw, fallback = false) {
  if (raw == null || raw === "") return fallback;
  return ["1", "true", "yes", "on"].includes(String(raw).trim().toLowerCase());
}

export function envFirst(env, ...names) {
  for (const name of names) {
    const value = env?.[name];
    if (value != null && String(value).trim()) return String(value).trim();
  }
  return "";
}

export function randomUin() {
  const uint32 = crypto.randomBytes(4).readUInt32BE(0);
  return Buffer.from(String(uint32), "utf-8").toString("base64");
}

// iLink-App-Id: 从 openclaw-weixin 的 package.json 已知为 "bot"
const ILINK_APP_ID = "bot";

// iLink-App-ClientVersion: 0x00MMNNPP (major<<16 | minor<<8 | patch)
function buildClientVersion(version) {
  const [major = 0, minor = 0, patch = 0] = version.split(".").map(Number);
  return ((major & 0xff) << 16) | ((minor & 0xff) << 8) | (patch & 0xff);
}
const ILINK_APP_CLIENT_VERSION = String(buildClientVersion("2.4.4"));

// ---------------------------------------------------------------------------
// 消息提取
// ---------------------------------------------------------------------------

export const MessageItemType = {
  NONE: 0,
  TEXT: 1,
  IMAGE: 2,
  VOICE: 3,
  FILE: 4,
  VIDEO: 5,
  TOOL_CALL_START: 11,
  TOOL_CALL_RESULT: 12,
};

export function extractText(itemList) {
  if (!Array.isArray(itemList) || !itemList.length) return "";
  for (const item of itemList) {
    if (item.type === MessageItemType.TEXT && item.text_item?.text != null) {
      return String(item.text_item.text);
    }
    if (item.type === MessageItemType.VOICE && item.voice_item?.text) {
      return item.voice_item.text;
    }
  }
  return "";
}

// ---------------------------------------------------------------------------
// 命令解析（与 feishu/telegram/wechat bridge 一致）
// ---------------------------------------------------------------------------

export function parseCommand(text) {
  const trimmed = String(text || "").trim();
  if (!trimmed.startsWith("/")) return { name: "prompt", args: trimmed };
  const [head, ...rest] = trimmed.split(/\s+/);
  return {
    name: head.slice(1).toLowerCase(),
    args: rest.join(" ").trim(),
  };
}

export function parseApprovalDecisionArgs(args) {
  const parts = String(args || "").split(/\s+/).filter(Boolean);
  return {
    approvalId: parts[0] || "",
    remember: parts.slice(1).includes("remember"),
  };
}

export function commandAction(command) {
  switch (command.name) {
    case "help":
      return { kind: "help" };
    case "status":
      return { kind: "status" };
    case "threads":
      return { kind: "threads" };
    case "new":
      return { kind: "new_thread" };
    case "resume":
      return { kind: "resume", threadId: command.args };
    case "model":
      return { kind: "set_model", modelName: command.args };
    case "interrupt":
      return { kind: "interrupt" };
    case "compact":
      return { kind: "compact" };
    case "allow":
      return {
        kind: "approval",
        decision: "allow",
        ...parseApprovalDecisionArgs(command.args),
      };
    case "deny":
      return {
        kind: "approval",
        decision: "deny",
        ...parseApprovalDecisionArgs(command.args),
      };
    case "prompt":
      return { kind: "prompt", prompt: command.args };
    default:
      return {
        kind: "prompt",
        prompt: `/${command.name}${command.args ? ` ${command.args}` : ""}`,
      };
  }
}

export function preservedChatStateFields(state = {}) {
  const preserved = {};
  if (Object.prototype.hasOwnProperty.call(state || {}, "model")) {
    preserved.model = state.model || null;
  }
  return preserved;
}

// ---------------------------------------------------------------------------
// 消息拆分（微信 iLink 单条消息无明确上限，保守 3500 字符）
// ---------------------------------------------------------------------------

export function splitMessage(text, maxChars = 3500) {
  const value = String(text || "");
  const chars = Array.from(value);
  if (chars.length <= maxChars) return value ? [value] : [];
  const chunks = [];
  let cursor = 0;
  while (cursor < chars.length) {
    chunks.push(chars.slice(cursor, cursor + maxChars).join(""));
    cursor += maxChars;
  }
  return chunks;
}

// ---------------------------------------------------------------------------
// Runtime 工具
// ---------------------------------------------------------------------------

export function compactRuntimeError(status, body) {
  const message =
    body?.error?.message ||
    body?.message ||
    (typeof body === "string" ? body : JSON.stringify(body));
  return `Runtime API request failed (${status}): ${message}`;
}

export function latestRunningTurn(detail) {
  const turns = Array.isArray(detail?.turns) ? detail.turns : [];
  for (let index = turns.length - 1; index >= 0; index -= 1) {
    const turn = turns[index];
    if (["queued", "in_progress"].includes(turn?.status)) return turn;
  }
  return null;
}

export function activeTurnBlock(detail, state = {}) {
  const runningTurn = latestRunningTurn(detail);
  if (!runningTurn) return null;
  return {
    turnId: runningTurn.id || state.activeTurnId || "",
    message: `Thread already has active turn ${
      runningTurn.id || state.activeTurnId || "(unknown)"
    }. Wait for it to finish or send /interrupt.`,
  };
}

// ---------------------------------------------------------------------------
// 帮助文本
// ---------------------------------------------------------------------------

export function helpText() {
  return [
    "HelpOfAi Weixin Bot 命令:",
    "/help - 显示此帮助",
    "/status - 运行时和工作区状态",
    "/threads - 最近运行时线程",
    "/new - 为此聊天创建新线程",
    "/resume <thread_id> - 绑定到已有线程",
    "/model <name|default> - 设置或重置此聊天的模型",
    "/interrupt - 中断活跃 turn",
    "/compact - 压缩当前线程",
    "/allow <approval_id> [remember] - 批准工具调用",
    "/deny <approval_id> - 拒绝工具调用",
    "",
    "其他所有内容均作为 HelpOfAi 提示发送。",
  ].join("\n");
}

// ============================================================================
// iLink Bot HTTP API 调用
// ============================================================================

export const ILinkLoginBase = "https://ilinkai.weixin.qq.com";

function authHeaders({ token } = {}) {
  const headers = {
    "Content-Type": "application/json",
    AuthorizationType: "ilink_bot_token",
    "X-WECHAT-UIN": randomUin(),
    "iLink-App-Id": ILINK_APP_ID,
    "iLink-App-ClientVersion": ILINK_APP_CLIENT_VERSION,
  };
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }
  return headers;
}

/**
 * 通用 POST 到 iLink API。
 */
export async function apiPost({ baseUrl, endpoint, body, token, timeoutMs, signal }) {
  const url = `${baseUrl.replace(/\/+$/, "")}/${endpoint}`;
  const ms = timeoutMs || DEFAULT_API_TIMEOUT_MS;
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), ms);
  signal?.addEventListener("abort", () => controller.abort(), { once: true });

  try {
    const response = await fetch(url, {
      method: "POST",
      headers: authHeaders({ token }),
      body,
      signal: controller.signal,
    });
    const text = await response.text();
    if (!response.ok) {
      throw new Error(
        `iLink API ${endpoint} failed: HTTP ${response.status} — ${text.slice(0, 200)}`
      );
    }
    return text;
  } finally {
    clearTimeout(timer);
  }
}

/**
 * 通用 GET 到 iLink API（用于轮询扫码状态等）。
 */
export async function apiGet({ baseUrl, endpoint, token, timeoutMs, signal }) {
  const url = `${baseUrl.replace(/\/+$/, "")}/${endpoint}`;
  const ms = timeoutMs || DEFAULT_API_TIMEOUT_MS;
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), ms);
  signal?.addEventListener("abort", () => controller.abort(), { once: true });

  try {
    const response = await fetch(url, {
      method: "GET",
      headers: authHeaders({ token }),
      signal: controller.signal,
    });
    const text = await response.text();
    if (!response.ok) {
      // 长轮询超时是正常的，不抛错
      if (response.status === 524 || text.includes("timeout")) {
        return text;
      }
      throw new Error(
        `iLink API ${endpoint} failed: HTTP ${response.status} — ${text.slice(0, 200)}`
      );
    }
    return text;
  } finally {
    clearTimeout(timer);
  }
}

// ============================================================================
// 扫码登录
// ============================================================================

/**
 * 获取登录二维码。
 * 参考 login-qr.ts:79-90 — 端点 get_bot_qrcode，bot_type 是 query 参数。
 * @returns {Promise<{qrcode: string, qrcodeUrl: string, sessionKey: string}>}
 */
export async function getLoginQR({ botType = "3" } = {}) {
  const raw = await apiPost({
    baseUrl: ILinkLoginBase,
    endpoint: `ilink/bot/get_bot_qrcode?bot_type=${encodeURIComponent(botType)}`,
    body: JSON.stringify({ local_token_list: [] }),
  });
  const data = JSON.parse(raw);
  const qrcodeUrl = data.qrcode_img_content || "";
  const sessionKey = data.qrcode || crypto.randomUUID();
  return { qrcode: data.qrcode, qrcodeUrl, sessionKey };
}

/**
 * 轮询扫码状态直到确认或超时。
 * 参考 login-qr.ts:112-136 — 端点 get_qrcode_status，GET 方法。
 * @returns {Promise<{connected: boolean, botToken?: string, accountId?: string, baseUrl?: string, userId?: string, message: string}>}
 */
export async function waitForLogin({ sessionKey, timeoutMs = 300_000 } = {}) {
  const deadline = Date.now() + timeoutMs;

  while (Date.now() < deadline) {
    let raw;
    try {
      raw = await apiGet({
        baseUrl: ILinkLoginBase,
        endpoint: `ilink/bot/get_qrcode_status?qrcode=${encodeURIComponent(sessionKey)}`,
        timeoutMs: 35_000, // 长轮询超时
      });
    } catch {
      // 超时等网络错误 → 视为 wait，继续轮询
      await new Promise((r) => setTimeout(r, 1000));
      continue;
    }

    const data = JSON.parse(raw);
    const status = data.status;

    if (status === "confirmed") {
      return {
        connected: true,
        botToken: data.bot_token,
        accountId: data.ilink_bot_id,
        baseUrl: data.baseurl || ILinkLoginBase,
        userId: data.ilink_user_id,
        message: "已连接微信。",
      };
    }

    if (status === "expired" || status === "binded_redirect") {
      return { connected: false, message: status === "expired" ? "二维码已过期，请重试。" : "已连接过此桥接，无需重复连接。" };
    }

    // 等 1 秒再轮询
    await new Promise((r) => setTimeout(r, 1000));
  }

  return { connected: false, message: "登录超时，请重试。" };
}

// ============================================================================
// 消息 API
// ============================================================================

/**
 * 长轮询获取新消息。
 */
export async function getUpdates({ baseUrl, token, get_updates_buf = "", timeoutMs = LONGPOLL_DEFAULT_TIMEOUT_MS, signal }) {
  const raw = await apiPost({
    baseUrl,
    endpoint: "ilink/bot/getupdates",
    body: JSON.stringify({
      get_updates_buf,
      base_info: { bot_agent: "HelpOfAi/1.0" },
    }),
    token,
    timeoutMs,
    signal,
  });
  return JSON.parse(raw);
}

/**
 * 发送消息。
 */
export async function sendMessage({ baseUrl, token, body, timeoutMs }) {
  await apiPost({
    baseUrl,
    endpoint: "ilink/bot/sendmessage",
    body: JSON.stringify({
      ...body,
      base_info: { bot_agent: "HelpOfAi/1.0" },
    }),
    token,
    timeoutMs,
  });
}

/**
 * 发送/取消输入状态。
 */
export async function sendTyping({ baseUrl, token, ilinkUserId, typingTicket, status = 1 }) {
  await apiPost({
    baseUrl,
    endpoint: "ilink/bot/sendtyping",
    body: JSON.stringify({
      ilink_user_id: ilinkUserId,
      typing_ticket: typingTicket,
      status,
      base_info: { bot_agent: "HelpOfAi/1.0" },
    }),
    token,
  });
}

/**
 * 获取账号配置（含 typing_ticket）。
 */
export async function getConfig({ baseUrl, token, ilinkUserId, contextToken }) {
  const raw = await apiPost({
    baseUrl,
    endpoint: "ilink/bot/getconfig",
    body: JSON.stringify({
      ilink_user_id: ilinkUserId,
      context_token: contextToken,
      base_info: { bot_agent: "HelpOfAi/1.0" },
    }),
    token,
  });
  return JSON.parse(raw);
}

/**
 * 通知上线。
 */
export async function notifyStart({ baseUrl, token }) {
  const raw = await apiPost({
    baseUrl,
    endpoint: "ilink/bot/msg/notifystart",
    body: JSON.stringify({ base_info: { bot_agent: "HelpOfAi/1.0" } }),
    token,
  });
  return JSON.parse(raw);
}

/**
 * 通知下线。
 */
export async function notifyStop({ baseUrl, token }) {
  const raw = await apiPost({
    baseUrl,
    endpoint: "ilink/bot/msg/notifystop",
    body: JSON.stringify({ base_info: { bot_agent: "HelpOfAi/1.0" } }),
    token,
  });
  return JSON.parse(raw);
}
