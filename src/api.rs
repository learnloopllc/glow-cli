// api.rs — HTTP clients for both APIs
//
// Two clients:
//   LearnLoopClient → central platform (licensing, billing, OAuth)
//   GlowClient      → a specific Glow instance (personas, agents, etc.)

use anyhow::{Context, Result};
use reqwest::blocking;
use serde_json::json;

use crate::types;

// ── Shared helper ──────────────────────────────────────────────

fn post_json<T: serde::de::DeserializeOwned>(
    http: &blocking::Client,
    url: &str,
    body: serde_json::Value,
    license_key: Option<&str>,
) -> Result<T> {
    let mut req = http.post(url).json(&body);
    if let Some(key) = license_key {
        req = req.header("X-License-Key", key);
    }

    let response = req.send().context("Failed to connect to API")?;
    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        anyhow::bail!("API error {}: {}", status, text);
    }

    response.json::<T>().context("Failed to parse API response")
}

// ── LearnLoop Client (central API) ────────────────────────────

pub struct LearnLoopClient {
    base_url: String,
    http: blocking::Client,
    license_key: Option<String>,
}

impl LearnLoopClient {
    pub fn new(base_url: &str, license_key: Option<&str>) -> Self {
        LearnLoopClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: blocking::Client::new(),
            license_key: license_key.map(|s| s.to_string()),
        }
    }

    // TODO: login, license validation, billing endpoints
}

// ── Glow Client (instance API) ────────────────────────────────

pub struct GlowClient {
    base_url: String,
    http: blocking::Client,
    license_key: Option<String>,
}

impl GlowClient {
    pub fn new(base_url: &str, license_key: Option<&str>) -> Self {
        GlowClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: blocking::Client::new(),
            license_key: license_key.map(|s| s.to_string()),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn key(&self) -> Option<&str> {
        self.license_key.as_deref()
    }

    // ── Persona endpoints ──────────────────────────────────────

    pub fn persona_search(&self) -> Result<types::ListPersonaResponse> {
        post_json(&self.http, &self.url("/api/v5/artifacts/personas/search"), json!({}), self.key())
    }

    pub fn persona_get(&self, persona_id: &str) -> Result<types::GetPersonaResponse> {
        post_json(&self.http, &self.url("/api/v5/artifacts/personas/get"), json!({ "persona_id": persona_id }), self.key())
    }

    pub fn persona_create(&self, name: &str, description: Option<&str>) -> Result<types::CreatePersonaResponse> {
        let mut item = json!({ "name": name });
        if let Some(desc) = description {
            item["description"] = json!(desc);
        }
        post_json(&self.http, &self.url("/api/v5/artifacts/personas/create"), json!({ "personas": [item] }), self.key())
    }

    pub fn persona_delete(&self, persona_id: &str) -> Result<types::DeletePersonaResponse> {
        post_json(&self.http, &self.url("/api/v5/artifacts/personas/delete"), json!({ "persona_ids": [persona_id] }), self.key())
    }
}
