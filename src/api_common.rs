// api_common.rs — Shared HTTP helpers for both API clients
//
// Both AdminClient and GlowClient use these.

use anyhow::{Context, Result};
use reqwest::blocking;

// ── Auth modes for the shared helper ─────────────────────────

pub(crate) enum Auth<'a> {
    None,
    LicenseKey(&'a str),
    Bearer(&'a str),
    /// Dual auth: license key (X-License-Key) + OAuth bearer token
    Dual {
        license_key: &'a str,
        token: &'a str,
    },
}

// ── Shared request helpers ───────────────────────────────────

fn apply_auth(req: blocking::RequestBuilder, auth: Auth) -> blocking::RequestBuilder {
    match auth {
        Auth::None => req,
        Auth::LicenseKey(key) => req.header("X-License-Key", key),
        Auth::Bearer(token) => req.header("Authorization", format!("Bearer {}", token)),
        Auth::Dual { license_key, token } => req
            .header("X-License-Key", license_key)
            .header("Authorization", format!("Bearer {}", token)),
    }
}

fn handle_error_status(status: reqwest::StatusCode, text: &str) -> anyhow::Error {
    match status.as_u16() {
        401 => anyhow::anyhow!(
            "Authentication failed (HTTP 401). Run 'glow login' to authenticate."
        ),
        403 => anyhow::anyhow!(
            "Permission denied (HTTP 403). Your account may not have access to this resource."
        ),
        404 => anyhow::anyhow!("Resource not found (HTTP 404): {}", text),
        _ => anyhow::anyhow!("API error (HTTP {}): {}", status, text),
    }
}

pub(crate) fn api_request<T: serde::de::DeserializeOwned>(
    http: &blocking::Client,
    method: reqwest::Method,
    url: &str,
    body: Option<serde_json::Value>,
    auth: Auth,
) -> Result<T> {
    let mut req = apply_auth(http.request(method, url), auth);

    if let Some(body) = body {
        req = req.json(&body);
    }

    let response = req.send().with_context(|| {
        format!(
            "Failed to connect to {}. Check your network or run 'glow network' to diagnose.",
            url
        )
    })?;

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        return Err(handle_error_status(status, &text));
    }

    response.json::<T>().context(
        "Failed to parse API response. This might indicate a version mismatch between the CLI and server.",
    )
}

/// Like api_request but returns the raw Response (for SSE streaming, binary downloads, etc.)
pub(crate) fn api_request_raw(
    http: &blocking::Client,
    method: reqwest::Method,
    url: &str,
    body: Option<serde_json::Value>,
    auth: Auth,
) -> Result<blocking::Response> {
    let mut req = apply_auth(http.request(method, url), auth);

    if let Some(body) = body {
        req = req.json(&body);
    }

    let response = req.send().with_context(|| {
        format!(
            "Failed to connect to {}. Check your network or run 'glow network' to diagnose.",
            url
        )
    })?;

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        return Err(handle_error_status(status, &text));
    }

    Ok(response)
}
