// glow/mod.rs — HTTP client for Glow instance APIs
//
// Each Glow instance is a separate deployment with its own URL.
// Auth: X-License-Key (license key) + Bearer token (OAuth) — dual auth
// Token is auto-loaded from the stored auth for the instance URL.

pub mod types;

use anyhow::{Context, Result};
use reqwest::blocking;
use serde_json::{json, Value};
use std::io::BufRead;

use crate::api_common::{api_request, api_request_raw, Auth};

pub struct GlowClient {
    base_url: String,
    http: blocking::Client,
    license_key: Option<String>,
    token: Option<String>,
}

impl GlowClient {
    pub fn new(base_url: &str, license_key: Option<&str>) -> Self {
        // Auto-load OAuth token from stored auth for this Glow instance
        let token = crate::auth::get_token(base_url)
            .ok()
            .map(|t| t.access_token);

        GlowClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: blocking::Client::new(),
            license_key: license_key.map(|s| s.to_string()),
            token,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn auth(&self) -> Auth<'_> {
        match (&self.license_key, &self.token) {
            (Some(k), Some(t)) => Auth::Dual {
                license_key: k,
                token: t,
            },
            (Some(k), None) => Auth::LicenseKey(k),
            (None, Some(t)) => Auth::Bearer(t),
            (None, None) => Auth::None,
        }
    }

    /// Build an authenticated request (for custom requests like uploads)
    fn authed_request(
        &self,
        method: reqwest::Method,
        url: &str,
    ) -> blocking::RequestBuilder {
        let mut req = self.http.request(method, url);
        if let Some(ref k) = self.license_key {
            req = req.header("X-License-Key", k);
        }
        if let Some(ref t) = self.token {
            req = req.header("Authorization", format!("Bearer {}", t));
        }
        req
    }

    // ── Health ────────────────────────────────────────────────

    pub fn health(&self) -> Result<types::HealthResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url("/health"),
            None,
            Auth::None,
        )
    }

    // ── Persona endpoints (typed shortcuts) ──────────────────

    pub fn persona_search(&self) -> Result<types::ListPersonaResponse> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/api/v5/artifacts/personas/search"),
            Some(json!({})),
            self.auth(),
        )
    }

    pub fn persona_get(&self, persona_id: &str) -> Result<types::GetPersonaResponse> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/api/v5/artifacts/personas/get"),
            Some(json!({ "persona_id": persona_id })),
            self.auth(),
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
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/api/v5/artifacts/personas/create"),
            Some(json!({ "personas": [item] })),
            self.auth(),
        )
    }

    pub fn persona_delete(&self, persona_id: &str) -> Result<types::DeletePersonaResponse> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/api/v5/artifacts/personas/delete"),
            Some(json!({ "persona_ids": [persona_id] })),
            self.auth(),
        )
    }

    // ── Generic artifact CRUD ────────────────────────────────
    //
    // Handles all 25+ artifact types with a single method.
    // All artifact endpoints are POST at /api/v5/artifacts/{type}/{action}.

    pub fn artifact_action(
        &self,
        artifact_type: &str,
        action: &str,
        body: Option<Value>,
    ) -> Result<Value> {
        let url = self.url(&format!(
            "/api/v5/artifacts/{}/{}",
            artifact_type, action
        ));
        api_request(
            &self.http,
            reqwest::Method::POST,
            &url,
            Some(body.unwrap_or_else(|| json!({}))),
            self.auth(),
        )
    }

    // ── Events ───────────────────────────────────────────────

    /// SSE stream — returns raw response for line-by-line reading.
    /// Caller should use BufReader to read SSE events.
    pub fn events_stream(
        &self,
        artifact_type: Option<&str>,
        artifact_id: Option<&str>,
        operation: Option<&str>,
    ) -> Result<blocking::Response> {
        let mut url = self.url("/api/v5/events/stream");
        let mut params = vec![];
        if let Some(t) = artifact_type {
            params.push(format!("artifact_type={}", t));
        }
        if let Some(id) = artifact_id {
            params.push(format!("artifact_id={}", id));
        }
        if let Some(op) = operation {
            params.push(format!("operation={}", op));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let resp = self
            .authed_request(reqwest::Method::GET, &url)
            .header("Accept", "text/event-stream")
            .send()
            .with_context(|| format!("Failed to connect to SSE stream at {}", url))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            anyhow::bail!("SSE stream error (HTTP {}): {}", status, text);
        }

        Ok(resp)
    }

    /// Poll for events
    pub fn events_poll(&self, body: Value) -> Result<Value> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/api/v5/events/poll"),
            Some(body),
            self.auth(),
        )
    }

    /// Dispatch a webhook event
    pub fn events_webhook_dispatch(&self, body: Value) -> Result<Value> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/api/v5/events/webhooks/dispatch"),
            Some(body),
            self.auth(),
        )
    }

    // ── Uploads (TUS 1.0 protocol) ──────────────────────────

    /// Create a new upload (TUS creation)
    pub fn upload_create(
        &self,
        filename: &str,
        content_type: Option<&str>,
        size: Option<u64>,
    ) -> Result<types::UploadCreateResponse> {
        let mut body = json!({ "filename": filename });
        if let Some(ct) = content_type {
            body["content_type"] = json!(ct);
        }
        if let Some(s) = size {
            body["size"] = json!(s);
        }
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/uploads/create"),
            Some(body),
            self.auth(),
        )
    }

    /// Upload file data (TUS PATCH — sends raw bytes)
    pub fn upload_patch(
        &self,
        upload_id: &str,
        data: Vec<u8>,
        offset: u64,
    ) -> Result<types::UploadPatchResponse> {
        let url = self.url(&format!("/uploads/{}", upload_id));
        let resp = self
            .authed_request(reqwest::Method::PATCH, &url)
            .header("Content-Type", "application/offset+octet-stream")
            .header("Upload-Offset", offset.to_string())
            .body(data)
            .send()
            .with_context(|| format!("Failed to upload data to {}", url))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            anyhow::bail!("Upload failed (HTTP {}): {}", status, text);
        }

        let new_offset = resp
            .headers()
            .get("Upload-Offset")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

        Ok(types::UploadPatchResponse {
            offset: new_offset,
        })
    }

    /// Check upload status (TUS HEAD)
    pub fn upload_status(&self, upload_id: &str) -> Result<types::UploadStatusResponse> {
        let url = self.url(&format!("/uploads/{}/status", upload_id));
        let resp = self
            .authed_request(reqwest::Method::HEAD, &url)
            .send()
            .with_context(|| format!("Failed to check upload status at {}", url))?;

        if !resp.status().is_success() {
            anyhow::bail!("Upload status check failed (HTTP {})", resp.status());
        }

        let offset = resp
            .headers()
            .get("Upload-Offset")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

        let length = resp
            .headers()
            .get("Upload-Length")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        Ok(types::UploadStatusResponse { offset, length })
    }

    /// Download an uploaded file
    pub fn upload_download(&self, upload_id: &str) -> Result<Vec<u8>> {
        let url = self.url(&format!("/uploads/{}/download", upload_id));
        let resp = api_request_raw(
            &self.http,
            reqwest::Method::GET,
            &url,
            None,
            self.auth(),
        )?;
        let bytes = resp.bytes().context("Failed to read download response")?;
        Ok(bytes.to_vec())
    }

    /// Multipart file upload
    pub fn upload_multipart(&self, file_path: &str) -> Result<Value> {
        let path = std::path::Path::new(file_path);
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();
        let data = std::fs::read(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path))?;

        let part = blocking::multipart::Part::bytes(data).file_name(filename);
        let form = blocking::multipart::Form::new().part("file", part);

        let url = self.url("/uploads/multipart");
        let resp = self
            .authed_request(reqwest::Method::POST, &url)
            .multipart(form)
            .send()
            .with_context(|| "Failed to upload file")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            anyhow::bail!("Multipart upload failed (HTTP {}): {}", status, text);
        }

        resp.json::<Value>().context("Failed to parse upload response")
    }

    /// Raw binary upload
    pub fn upload_raw(&self, file_path: &str, content_type: Option<&str>) -> Result<Value> {
        let data = std::fs::read(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path))?;
        let ct = content_type.unwrap_or("application/octet-stream");

        let url = self.url("/uploads/raw");
        let resp = self
            .authed_request(reqwest::Method::POST, &url)
            .header("Content-Type", ct)
            .body(data)
            .send()
            .with_context(|| "Failed to upload file")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            anyhow::bail!("Raw upload failed (HTTP {}): {}", status, text);
        }

        resp.json::<Value>().context("Failed to parse upload response")
    }

    /// Parse a CSV file on the server
    pub fn upload_csv_parse(&self, file_path: &str) -> Result<Value> {
        let data = std::fs::read(file_path)
            .with_context(|| format!("Failed to read CSV file: {}", file_path))?;
        let filename = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("data.csv")
            .to_string();

        let part = blocking::multipart::Part::bytes(data).file_name(filename);
        let form = blocking::multipart::Form::new().part("file", part);

        let url = self.url("/uploads/csv");
        let resp = self
            .authed_request(reqwest::Method::POST, &url)
            .multipart(form)
            .send()
            .with_context(|| "Failed to parse CSV")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            anyhow::bail!("CSV parse failed (HTTP {}): {}", status, text);
        }

        resp.json::<Value>().context("Failed to parse CSV response")
    }

    /// Get upload preview
    pub fn upload_preview(&self) -> Result<Value> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url("/uploads/preview"),
            None,
            self.auth(),
        )
    }

    /// Download upload template
    pub fn upload_template(&self) -> Result<Vec<u8>> {
        let resp = api_request_raw(
            &self.http,
            reqwest::Method::GET,
            &self.url("/uploads/template"),
            None,
            self.auth(),
        )?;
        let bytes = resp.bytes().context("Failed to read template response")?;
        Ok(bytes.to_vec())
    }

    /// Discover available upload types
    pub fn upload_discover(&self) -> Result<Value> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url("/uploads/discover"),
            None,
            self.auth(),
        )
    }

    /// Finalize an upload
    pub fn upload_finalize(&self, body: Value) -> Result<Value> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/uploads/finalize"),
            Some(body),
            self.auth(),
        )
    }
}

// ── SSE helper ────────────────────────────────────────────────

/// Read SSE events from a response and call the handler for each data line.
pub fn read_sse_events(
    response: blocking::Response,
    mut handler: impl FnMut(&str),
) -> Result<()> {
    let reader = std::io::BufReader::new(response);
    for line in reader.lines() {
        let line = line.context("Error reading SSE stream")?;
        if let Some(data) = line.strip_prefix("data: ") {
            handler(data);
        }
        // Silently skip event:, id:, retry:, and blank lines
    }
    Ok(())
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
        let result = client.persona_create("Test", Some("A description")).unwrap();
        assert!(result.results[0].success);
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

    // ── Health ───────────────────────────────────────────────

    #[test]
    fn test_health_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/health")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status": "healthy", "version": "1.2.3"}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.health().unwrap();
        assert_eq!(result.status, "healthy");
        assert_eq!(result.version, Some("1.2.3".into()));
        mock.assert();
    }

    // ── Generic artifact ─────────────────────────────────────

    #[test]
    fn test_artifact_action_search() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/v5/artifacts/agents/search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"agents": [], "total_count": 0}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.artifact_action("agents", "search", None).unwrap();
        assert_eq!(result["total_count"], 0);
        mock.assert();
    }

    #[test]
    fn test_artifact_action_with_body() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/v5/artifacts/scenarios/get")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"scenario_id": "s-1", "name": "Test"}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client
            .artifact_action(
                "scenarios",
                "get",
                Some(json!({"scenario_id": "s-1"})),
            )
            .unwrap();
        assert_eq!(result["name"], "Test");
        mock.assert();
    }

    // ── Events ───────────────────────────────────────────────

    #[test]
    fn test_events_poll() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/v5/events/poll")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"events": [], "cursor": "abc"}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.events_poll(json!({})).unwrap();
        assert_eq!(result["cursor"], "abc");
        mock.assert();
    }

    #[test]
    fn test_events_webhook_dispatch() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/v5/events/webhooks/dispatch")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"dispatched": true}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client
            .events_webhook_dispatch(json!({"artifact_type": "personas"}))
            .unwrap();
        assert_eq!(result["dispatched"], true);
        mock.assert();
    }

    // ── Uploads ──────────────────────────────────────────────

    #[test]
    fn test_upload_create() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/uploads/create")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"upload_id": "up-123", "upload_url": "/uploads/up-123"}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client
            .upload_create("test.csv", Some("text/csv"), Some(1024))
            .unwrap();
        assert_eq!(result.upload_id, "up-123");
        mock.assert();
    }

    #[test]
    fn test_upload_preview() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/uploads/preview")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"columns": ["name", "email"], "rows": 5}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.upload_preview().unwrap();
        assert_eq!(result["rows"], 5);
        mock.assert();
    }

    #[test]
    fn test_upload_discover() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/uploads/discover")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"types": ["csv", "json", "xlsx"]}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.upload_discover().unwrap();
        assert_eq!(result["types"][0], "csv");
        mock.assert();
    }

    #[test]
    fn test_upload_finalize() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/uploads/finalize")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"finalized": true, "records": 42}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client
            .upload_finalize(json!({"upload_id": "up-123"}))
            .unwrap();
        assert_eq!(result["records"], 42);
        mock.assert();
    }

    // ── Error handling ───────────────────────────────────────

    #[test]
    fn test_401_returns_auth_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/api/v5/artifacts/personas/search")
            .with_status(401)
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

    // ── SSE helper ───────────────────────────────────────────

    #[test]
    fn test_read_sse_events() {
        let sse_data = "event: message\ndata: hello\n\nevent: message\ndata: world\n\n";
        let cursor = std::io::Cursor::new(sse_data);

        // Simulate a Response-like reader
        let mut events = vec![];
        let reader = std::io::BufReader::new(cursor);
        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("data: ") {
                events.push(line[6..].to_string());
            }
        }
        assert_eq!(events, vec!["hello", "world"]);
    }

    // ── Dual auth ────────────────────────────────────────────

    #[test]
    fn test_auth_none_when_no_credentials() {
        let client = GlowClient {
            base_url: "http://localhost".into(),
            http: blocking::Client::new(),
            license_key: None,
            token: None,
        };
        matches!(client.auth(), Auth::None);
    }

    #[test]
    fn test_auth_license_key_only() {
        let client = GlowClient {
            base_url: "http://localhost".into(),
            http: blocking::Client::new(),
            license_key: Some("key".into()),
            token: None,
        };
        matches!(client.auth(), Auth::LicenseKey(_));
    }

    #[test]
    fn test_auth_bearer_only() {
        let client = GlowClient {
            base_url: "http://localhost".into(),
            http: blocking::Client::new(),
            license_key: None,
            token: Some("tok".into()),
        };
        matches!(client.auth(), Auth::Bearer(_));
    }

    #[test]
    fn test_auth_dual() {
        let client = GlowClient {
            base_url: "http://localhost".into(),
            http: blocking::Client::new(),
            license_key: Some("key".into()),
            token: Some("tok".into()),
        };
        matches!(client.auth(), Auth::Dual { .. });
    }
}
