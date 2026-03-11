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

    let response = req.send().with_context(|| {
        format!(
            "Failed to connect to {}. Check your network or run 'learnloop network' to diagnose.",
            url
        )
    })?;

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        match status.as_u16() {
            401 => anyhow::bail!(
                "Authentication failed (HTTP 401). Run 'learnloop login' to authenticate, or check your LEARNLOOP_LICENSE_KEY."
            ),
            403 => anyhow::bail!(
                "Permission denied (HTTP 403). Your account may not have access to this resource."
            ),
            404 => anyhow::bail!("Resource not found (HTTP 404): {}", text),
            _ => anyhow::bail!("API error (HTTP {}): {}", status, text),
        }
    }

    response.json::<T>().context(
        "Failed to parse API response. This might indicate a version mismatch between the CLI and server.",
    )
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

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn key(&self) -> Option<&str> {
        self.license_key.as_deref()
    }

    /// Validate the license key, returns the license_id if valid
    pub fn validate_license(&self) -> Result<types::ValidateLicenseResponse> {
        let key = self.key().ok_or_else(|| {
            anyhow::anyhow!("No license key set. Use --license-key or set LEARNLOOP_LICENSE_KEY.")
        })?;
        post_json(
            &self.http,
            &self.url("/licenses/validate"),
            json!({ "key": key }),
            self.key(),
        )
    }

    /// Report a ledger entry to the usage tracking system
    pub fn usage_report(
        &self,
        license_id: &str,
        entry_hash: &str,
        simulation_count: i64,
    ) -> Result<types::UsageReportResponse> {
        post_json(
            &self.http,
            &self.url("/usage/report"),
            json!({
                "license_id": license_id,
                "entry_hash": entry_hash,
                "simulation_count": simulation_count
            }),
            self.key(),
        )
    }

    /// Get usage summary for a license
    #[allow(dead_code)] // will be used by ledger status with API integration
    pub fn usage_summary(&self, license_id: &str) -> Result<types::UsageSummaryResponse> {
        let resp = self
            .http
            .get(self.url(&format!("/usage/summary/{}", license_id)))
            .send()
            .context("Failed to connect to LearnLoop API.")?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().unwrap_or_default();
            anyhow::bail!("API error (HTTP {}): {}", status, text);
        }
        resp.json().context("Failed to parse usage summary.")
    }
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
        post_json(
            &self.http,
            &self.url("/api/v5/artifacts/personas/search"),
            json!({}),
            self.key(),
        )
    }

    pub fn persona_get(&self, persona_id: &str) -> Result<types::GetPersonaResponse> {
        post_json(
            &self.http,
            &self.url("/api/v5/artifacts/personas/get"),
            json!({ "persona_id": persona_id }),
            self.key(),
        )
    }

    pub fn persona_create(
        &self,
        name: &str,
        description: Option<&str>,
    ) -> Result<types::CreatePersonaResponse> {
        let mut item = json!({ "name": name });
        if let Some(desc) = description {
            item["description"] = json!(desc);
        }
        post_json(
            &self.http,
            &self.url("/api/v5/artifacts/personas/create"),
            json!({ "personas": [item] }),
            self.key(),
        )
    }

    pub fn persona_delete(&self, persona_id: &str) -> Result<types::DeletePersonaResponse> {
        post_json(
            &self.http,
            &self.url("/api/v5/artifacts/personas/delete"),
            json!({ "persona_ids": [persona_id] }),
            self.key(),
        )
    }
}

// ── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persona_search_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/v5/artifacts/personas/search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"personas": [], "total_count": 0}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.persona_search().unwrap();

        assert_eq!(result.total_count, 0);
        assert!(result.personas.is_empty());
        mock.assert();
    }

    #[test]
    fn test_persona_get_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/v5/artifacts/personas/get")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "group_id": "grp-123",
                    "can_edit": true,
                    "names": {"current_name": "Dr. Smith"},
                    "descriptions": {"current_description": "A test persona"}
                }"#,
            )
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.persona_get("p-123").unwrap();

        assert_eq!(result.names.current_name, "Dr. Smith");
        assert_eq!(result.group_id, "grp-123");
        assert!(result.can_edit);
        mock.assert();
    }

    #[test]
    fn test_persona_create_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/v5/artifacts/personas/create")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"results": [{"success": true, "persona_id": "p-new", "message": "Created"}]}"#,
            )
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client
            .persona_create("Test Persona", Some("A description"))
            .unwrap();

        assert!(result.results[0].success);
        assert_eq!(result.results[0].persona_id, "p-new");
        mock.assert();
    }

    #[test]
    fn test_persona_delete_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/v5/artifacts/personas/delete")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"success": true}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.persona_delete("p-123").unwrap();
        assert!(result.success);
        mock.assert();
    }

    #[test]
    fn test_license_key_sent_as_header() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/v5/artifacts/personas/search")
            .match_header("X-License-Key", "test-key-123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"personas": [], "total_count": 0}"#)
            .create();

        let client = GlowClient::new(&server.url(), Some("test-key-123"));
        client.persona_search().unwrap();
        mock.assert();
    }

    #[test]
    fn test_401_returns_auth_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/api/v5/artifacts/personas/search")
            .with_status(401)
            .with_body("Unauthorized")
            .create();

        let client = GlowClient::new(&server.url(), None);
        let err = client.persona_search().unwrap_err();
        assert!(err.to_string().contains("Authentication failed"));
    }

    #[test]
    fn test_403_returns_permission_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/api/v5/artifacts/personas/search")
            .with_status(403)
            .with_body("Forbidden")
            .create();

        let client = GlowClient::new(&server.url(), None);
        let err = client.persona_search().unwrap_err();
        assert!(err.to_string().contains("Permission denied"));
    }

    #[test]
    fn test_404_returns_not_found_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/api/v5/artifacts/personas/get")
            .with_status(404)
            .with_body("persona not found")
            .create();

        let client = GlowClient::new(&server.url(), None);
        let err = client.persona_get("nonexistent").unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_500_returns_api_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/api/v5/artifacts/personas/search")
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let client = GlowClient::new(&server.url(), None);
        let err = client.persona_search().unwrap_err();
        assert!(err.to_string().contains("API error"));
    }

    #[test]
    fn test_connection_refused_returns_helpful_error() {
        let client = GlowClient::new("http://127.0.0.1:1", None);
        let err = client.persona_search().unwrap_err();
        assert!(err.to_string().contains("Failed to connect"));
    }
}
