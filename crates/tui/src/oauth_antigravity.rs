use std::time::Duration;

use crate::logging;
use anyhow::{Context, Result, bail};
use helpofai_config::{
    AntigravityAccountStore, antigravity_oauth_client_id, antigravity_oauth_client_secret,
    now_epoch_secs,
};

const GOOGLE_TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";

/// Default quota exhaustion cooldown period (5 minutes).
pub const DEFAULT_QUOTA_COOLDOWN_SECS: u64 = 300;

/// Get the valid Google OAuth access token for Antigravity, automatically rotating to the next
/// standby account if the active account is currently quota-exhausted, and refreshing expired tokens.
pub fn get_valid_antigravity_access_token() -> Result<Option<String>> {
    let file_path = AntigravityAccountStore::file_path();
    if !file_path.exists() {
        return Ok(None);
    }

    let mut store = AntigravityAccountStore::load();
    if store.accounts.is_empty() {
        return Ok(None);
    }

    let now_secs = now_epoch_secs();
    let account = match store.get_available_account_mut() {
        Some(acc) => acc,
        None => return Ok(None),
    };

    if account.is_token_expired(now_secs) {
        if let Some(refresh_token) = account.refresh_token.clone() {
            match refresh_token_blocking(&refresh_token) {
                Ok((new_access, expires_in)) => {
                    account.access_token = new_access;
                    account.expires_at_epoch_secs = now_secs + expires_in;
                    let _ = store.save();
                }
                Err(err) => {
                    tracing::warn!(
                        "Failed to refresh Antigravity Google OAuth token for {}: {err}",
                        account.email
                    );
                }
            }
        }
    }

    let token = store.active_account().map(|a| a.access_token.clone());
    Ok(token)
}

/// Mark the currently active account as quota-exhausted (e.g. on HTTP 429 / RESOURCE_EXHAUSTED)
/// and failover to the next available Google account in the pool.
/// Returns `Some(new_email)` if failed over to an available account, or `None` if all accounts are exhausted.
pub fn mark_active_exhausted_and_failover() -> Result<Option<String>> {
    let mut store = AntigravityAccountStore::load();
    if store.accounts.is_empty() {
        return Ok(None);
    }

    let old_email = store
        .active_account()
        .map(|a| a.email.clone())
        .unwrap_or_default();
    let switched = store.mark_active_exhausted_and_failover(DEFAULT_QUOTA_COOLDOWN_SECS);

    if let Some(ref new_email) = switched {
        logging::warn(format!(
            "[Antigravity] Model quota exhausted for Google account ({old_email}). Automatically failing over to: {new_email}"
        ));
    } else {
        logging::warn(format!(
            "[Antigravity] All {} Google accounts have exhausted their model quotas. Cooldown in progress.",
            store.accounts.len()
        ));
    }

    Ok(switched)
}

/// Synchronously refresh a Google OAuth access token using its refresh token.
fn refresh_token_blocking(refresh_token: &str) -> Result<(String, u64)> {
    let client_id = antigravity_oauth_client_id();
    let client_secret = antigravity_oauth_client_secret();

    let client = crate::tls::reqwest_blocking_client_builder()
        .timeout(Duration::from_secs(30))
        .build()
        .context("building Google OAuth refresh client")?;

    let response = client
        .post(GOOGLE_TOKEN_ENDPOINT)
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ])
        .send()
        .context("sending Google OAuth refresh request")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        bail!("Google token refresh failed (HTTP {status}): {body}");
    }

    let json: serde_json::Value = response.json().context("parsing token refresh response")?;
    let access_token = json["access_token"]
        .as_str()
        .context("missing access_token in Google OAuth refresh response")?
        .to_string();
    let expires_in = json["expires_in"].as_u64().unwrap_or(3600);

    Ok((access_token, expires_in))
}
