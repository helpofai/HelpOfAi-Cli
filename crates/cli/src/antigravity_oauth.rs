use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::Command;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use helpofai_config::{
    AntigravityAccount, AntigravityAccountStore, ConfigStore, ProviderKind,
    antigravity_oauth_client_id, antigravity_oauth_client_secret, now_epoch_secs,
};

const GOOGLE_AUTH_ENDPOINT: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USERINFO_ENDPOINT: &str = "https://www.googleapis.com/oauth2/v2/userinfo";
const GOOGLE_SCOPES: &str = "https://www.googleapis.com/auth/generative-language https://www.googleapis.com/auth/userinfo.email openid";

/// Launch interactive Google OAuth browser login for Google Antigravity.
/// Stores the account into the multi-account pool and sets Antigravity as active provider.
pub fn run_antigravity_oauth_login(store: &mut ConfigStore) -> Result<()> {
    println!("Initializing Google Antigravity OAuth browser login...");

    let client_id = antigravity_oauth_client_id();
    let client_secret = antigravity_oauth_client_secret();

    // Bind to an ephemeral loopback port
    let listener = TcpListener::bind("127.0.0.1:0")
        .context("Failed to bind local loopback listener for OAuth callback")?;
    let port = listener.local_addr()?.port();
    let redirect_uri = format!("http://127.0.0.1:{port}/oauth2callback");

    // Random state string for CSRF mitigation
    let state = format!("{:x}", now_epoch_secs() ^ 0x5a5a5a5a5a5a5a5a);

    let auth_url = format!(
        "{GOOGLE_AUTH_ENDPOINT}?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent%20select_account&state={state}",
        urlencoding(&client_id),
        urlencoding(&redirect_uri),
        urlencoding(GOOGLE_SCOPES),
    );

    println!();
    println!("Opening your web browser for Google Account authentication...");
    println!("If your browser did not open automatically, visit:");
    println!("  {auth_url}");
    println!();
    println!("Waiting for authorization callback on port {port}...");

    open_browser_url(&auth_url);

    // Accept single incoming HTTP callback
    listener.set_nonblocking(false)?;
    let (mut stream, _) = listener
        .accept()
        .context("Failed to accept OAuth callback connection")?;
    stream.set_read_timeout(Some(Duration::from_secs(120)))?;

    let mut buf = [0u8; 4096];
    let bytes_read = stream
        .read(&mut buf)
        .context("Reading OAuth callback request")?;
    let request_text = String::from_utf8_lossy(&buf[..bytes_read]);

    let (code, callback_state) = parse_oauth_callback(&request_text)?;

    if callback_state != state {
        let err_response = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nOAuth state mismatch.";
        let _ = stream.write_all(err_response.as_bytes());
        bail!("OAuth state verification failed. Possible CSRF attempt.");
    }

    // Return friendly HTML confirmation page to browser
    let html_body = r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>HelpOfAi - Google Account Connected</title>
<style>
  body { font-family: system-ui, -apple-system, sans-serif; display: flex; align-items: center; justify-content: center; height: 100vh; margin: 0; background: #0d1117; color: #c9d1d9; text-align: center; }
  .card { background: #161b22; padding: 40px; border-radius: 12px; border: 1px solid #30363d; max-width: 440px; box-shadow: 0 8px 24px rgba(0,0,0,0.5); }
  h2 { color: #58a6ff; margin-top: 0; }
  p { color: #8b949e; line-height: 1.5; }
  .badge { display: inline-block; padding: 4px 12px; background: #238636; color: #fff; border-radius: 20px; font-size: 14px; margin-bottom: 12px; }
</style>
</head>
<body>
<div class="card">
  <div class="badge">&#10003; Connected</div>
  <h2>Google Antigravity Linked</h2>
  <p>Your Google account has been successfully authenticated in HelpOfAi CLI.</p>
  <p>You can close this window and return to your terminal.</p>
</div>
</body>
</html>"#;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        html_body.len(),
        html_body
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();

    // Exchange authorization code for tokens
    println!("Exchanging authorization code for OAuth tokens...");
    let http_client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let form_body = format!(
        "client_id={}&client_secret={}&code={}&grant_type=authorization_code&redirect_uri={}",
        urlencoding(&client_id),
        urlencoding(&client_secret),
        urlencoding(&code),
        urlencoding(&redirect_uri)
    );

    let token_resp = http_client
        .post(GOOGLE_TOKEN_ENDPOINT)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(form_body)
        .send()
        .context("Failed to contact Google OAuth token endpoint")?;

    if !token_resp.status().is_success() {
        let err_body = token_resp.text().unwrap_or_default();
        bail!("Failed to obtain tokens from Google OAuth: {err_body}");
    }

    let token_json: serde_json::Value = token_resp.json()?;
    let access_token = token_json["access_token"]
        .as_str()
        .context("Missing access_token in Google OAuth response")?
        .to_string();
    let refresh_token = token_json["refresh_token"].as_str().map(ToOwned::to_owned);
    let expires_in = token_json["expires_in"].as_u64().unwrap_or(3600);

    // Fetch authenticated user's email address
    let userinfo_resp = http_client
        .get(GOOGLE_USERINFO_ENDPOINT)
        .bearer_auth(&access_token)
        .send()
        .context("Failed to fetch Google userinfo")?;

    let email = if userinfo_resp.status().is_success() {
        let userinfo_json: serde_json::Value = userinfo_resp.json().unwrap_or_default();
        userinfo_json["email"]
            .as_str()
            .unwrap_or("unknown@google.com")
            .to_string()
    } else {
        "google-user".to_string()
    };

    // Store in multi-account store
    let mut account_store = AntigravityAccountStore::load();
    let now_secs = now_epoch_secs();

    let account = AntigravityAccount {
        email: email.clone(),
        access_token,
        refresh_token,
        expires_at_epoch_secs: now_secs + expires_in,
        quota_exhausted_until_epoch_secs: 0,
        added_at: format!("{} UTC", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")),
        tier: helpofai_config::AntigravityTier::Auto,
    };

    let pos = account_store.add_or_update(account);
    account_store.active_index = pos;
    account_store.save()?;

    // Update config store
    store.config.provider = ProviderKind::Antigravity;
    let provider_cfg = store
        .config
        .providers
        .for_provider_mut(ProviderKind::Antigravity);
    provider_cfg.auth_mode = Some("oauth".to_string());
    store.save()?;

    println!();
    println!("------------------------------------------------------------");
    println!("✓ Successfully authenticated with Google Antigravity!");
    println!("  Account : {email}");
    println!(
        "  Pool    : {} account(s) configured",
        account_store.accounts.len()
    );
    println!("  Status  : Active");
    println!("  Failover: Automatic multi-account rotation enabled on quota exhaustion");
    println!("------------------------------------------------------------");
    println!(
        "Run `helpofai` to start coding, or `helpofai auth login --provider antigravity` again to add another account."
    );

    Ok(())
}

/// Print all configured Antigravity accounts, their active status, and quota countdowns.
pub fn list_antigravity_accounts() -> Result<()> {
    let store = AntigravityAccountStore::load();
    let now_secs = now_epoch_secs();

    if store.accounts.is_empty() {
        println!("No Google Antigravity accounts configured.");
        println!(
            "Run `helpofai auth login --provider antigravity` to connect your Google account."
        );
        return Ok(());
    }

    println!(
        "Google Antigravity Multi-Account Pool ({} total):",
        store.accounts.len()
    );
    println!(
        "{:<3} {:<30} {:<10} {:<10} {:<18} {:<15}",
        "#", "Email", "Role", "Tier", "Auto Model", "Quota State"
    );
    println!("{}", "-".repeat(90));

    for (i, account) in store.accounts.iter().enumerate() {
        let is_active = i == store.active_index;
        let role = if is_active { "Active *" } else { "Standby" };
        let tier_name = account.tier.display_name();
        let auto_model = account.tier.auto_model();
        let quota_state = if account.is_quota_exhausted(now_secs) {
            let remaining = account.quota_cooldown_remaining_secs(now_secs);
            format!("Exhausted ({remaining}s left)")
        } else {
            "Ready".to_string()
        };

        println!(
            "{:<3} {:<30} {:<10} {:<10} {:<18} {:<15}",
            i, account.email, role, tier_name, auto_model, quota_state
        );
    }

    println!();
    println!("Automatic failover is active: if the active account exhausts its model quota (429),");
    println!("HelpOfAi will automatically failover and retry using the next available account.");
    Ok(())
}

/// Set the tier (free, pro, paid, enterprise, auto) for an Antigravity account.
pub fn set_antigravity_account_tier(identifier: &str, tier_str: &str) -> Result<()> {
    let tier = helpofai_config::AntigravityTier::parse(tier_str)
        .context("Invalid tier. Choose one of: free, pro, paid, enterprise, auto")?;
    let mut store = AntigravityAccountStore::load();
    let email = store.set_account_tier(identifier, tier)?;
    println!(
        "Updated Google Antigravity tier for {} to: {} (auto model: {})",
        email,
        tier.display_name(),
        tier.auto_model()
    );
    Ok(())
}

/// Switch active account by email or 0-based index.
pub fn switch_antigravity_account(identifier: &str) -> Result<()> {
    let mut store = AntigravityAccountStore::load();
    let switched_email = store.switch_account(identifier)?;
    store.save()?;
    println!("Switched active Google Antigravity account to: {switched_email}");
    Ok(())
}

/// Remove an account by email or 0-based index.
pub fn remove_antigravity_account(identifier: &str) -> Result<()> {
    let mut store = AntigravityAccountStore::load();
    let removed_email = store.remove_account(identifier)?;
    store.save()?;
    println!("Removed Google Antigravity account: {removed_email}");
    if let Some(active) = store.active_account() {
        println!("Active account is now: {}", active.email);
    } else {
        println!("No accounts remaining in pool.");
    }
    Ok(())
}

fn parse_oauth_callback(request: &str) -> Result<(String, String)> {
    let first_line = request
        .lines()
        .next()
        .context("Empty HTTP callback request")?;
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 2 || parts[0] != "GET" {
        bail!("Invalid HTTP method in callback");
    }

    let url_path = parts[1];
    let query = url_path
        .split('?')
        .nth(1)
        .context("Callback URL missing query parameters")?;

    let mut code = None;
    let mut state = None;

    for param in query.split('&') {
        let mut kv = param.splitn(2, '=');
        let k = kv.next().unwrap_or_default();
        let v = kv.next().unwrap_or_default();
        if k == "code" {
            code = Some(urldecode(v));
        } else if k == "state" {
            state = Some(urldecode(v));
        }
    }

    let code = code.context("Missing 'code' query parameter in OAuth callback")?;
    let state = state.context("Missing 'state' query parameter in OAuth callback")?;

    Ok((code, state))
}

fn open_browser_url(url: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("cmd").args(["/c", "start", url]).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(url).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(url).spawn();
    }
}

fn urlencoding(input: &str) -> String {
    let mut encoded = String::new();
    for b in input.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            _ => {
                encoded.push_str(&format!("%{b:02X}"));
            }
        }
    }
    encoded
}

fn urldecode(input: &str) -> String {
    let mut res = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(val) = u8::from_str_radix(&input[i + 1..i + 3], 16) {
                res.push(val);
                i += 3;
                continue;
            }
        } else if bytes[i] == b'+' {
            res.push(b' ');
            i += 1;
            continue;
        }
        res.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&res).to_string()
}
