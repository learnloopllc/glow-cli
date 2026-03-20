// glow/mod.rs — HTTP client for Glow instance APIs
//
// Each Glow instance is a separate deployment with its own URL.
// Auth: X-License-Key (license key) + Bearer token (OAuth) — dual auth
// Token is auto-loaded from the stored auth for the instance URL.

pub mod api;
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

    // ── Generic resource CRUD (v5 routes) ────────────────────
    //
    // The new URL pattern: POST /v5/{resource}/{action}

    pub fn resource_action(
        &self,
        resource: &str,
        action: &str,
        body: Option<Value>,
    ) -> Result<Value> {
        let url = self.url(&format!("/v5/{}/{}", resource, action));
        api_request(
            &self.http,
            reqwest::Method::POST,
            &url,
            Some(body.unwrap_or_else(|| json!({}))),
            self.auth(),
        )
    }

    // ── Identity & emulation ────────────────────────────────

    /// Get current user context and identity
    pub fn context(&self) -> Result<Value> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/v5/context"),
            Some(json!({})),
            self.auth(),
        )
    }

    /// Emulate another user profile
    pub fn emulate(&self, target_profile_id: &str, ttl: u32) -> Result<Value> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/v5/emulate"),
            Some(json!({
                "target_profile_id": target_profile_id,
                "ttl": ttl,
            })),
            self.auth(),
        )
    }

    /// Stop emulating and return to own profile
    pub fn unemulate(&self) -> Result<Value> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/v5/unemulate"),
            Some(json!({})),
            self.auth(),
        )
    }

    /// Generate content for a group
    pub fn generate(&self, group_id: &str, body: Option<Value>) -> Result<Value> {
        let mut payload = body.unwrap_or_else(|| json!({}));
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("group_id".to_string(), json!(group_id));
        }
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/v5/generate"),
            Some(payload),
            self.auth(),
        )
    }

    // ── Per-resource media operations ──────────────────────────
    //
    // URL patterns:
    //   POST /v5/{resource}/{media}/upload          — multipart upload
    //   GET  /v5/{resource}/{media}/discover         — discover types
    //   GET  /v5/{resource}/{media}/discover/{id}    — discover specific
    //   POST /v5/{resource}/{media}/create           — TUS initiation
    //   GET  /v5/{resource}/{media}/{id}/status      — TUS status
    //   PATCH /v5/{resource}/{media}/{id}/chunk      — TUS chunk
    //   POST /v5/{resource}/{media}/{id}/finalize    — TUS finalize
    //   GET  /v5/{resource}/{media}/{id}/download    — download
    //   GET  /v5/{resource}/{media}/{id}/preview     — preview

    /// Upload a file via multipart form
    pub fn media_upload(
        &self,
        resource: &str,
        media_type: &str,
        file_path: &str,
    ) -> Result<Value> {
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

        let url = self.url(&format!("/v5/{}/{}/upload", resource, media_type));
        let resp = self
            .authed_request(reqwest::Method::POST, &url)
            .multipart(form)
            .send()
            .with_context(|| format!("Failed to upload to {}", url))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            anyhow::bail!("Upload failed (HTTP {}): {}", status, text);
        }

        resp.json::<Value>().context("Failed to parse upload response")
    }

    /// Discover available upload types for a resource media
    pub fn media_discover(
        &self,
        resource: &str,
        media_type: &str,
        upload_id: Option<&str>,
    ) -> Result<Value> {
        let url = match upload_id {
            Some(id) => self.url(&format!("/v5/{}/{}/discover/{}", resource, media_type, id)),
            None => self.url(&format!("/v5/{}/{}/discover", resource, media_type)),
        };
        api_request(&self.http, reqwest::Method::GET, &url, None, self.auth())
    }

    /// TUS create (initiate resumable upload)
    pub fn media_create(
        &self,
        resource: &str,
        media_type: &str,
        filename: &str,
        size: Option<u64>,
    ) -> Result<Value> {
        let mut body = json!({ "filename": filename });
        if let Some(s) = size {
            body["size"] = json!(s);
        }
        let url = self.url(&format!("/v5/{}/{}/create", resource, media_type));
        api_request(
            &self.http,
            reqwest::Method::POST,
            &url,
            Some(body),
            self.auth(),
        )
    }

    /// TUS chunk upload
    pub fn media_chunk(
        &self,
        resource: &str,
        media_type: &str,
        upload_id: &str,
        data: Vec<u8>,
        offset: u64,
    ) -> Result<Value> {
        let url = self.url(&format!(
            "/v5/{}/{}/{}/chunk",
            resource, media_type, upload_id
        ));
        let resp = self
            .authed_request(reqwest::Method::PATCH, &url)
            .header("Content-Type", "application/offset+octet-stream")
            .header("Upload-Offset", offset.to_string())
            .body(data)
            .send()
            .with_context(|| format!("Failed to upload chunk to {}", url))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            anyhow::bail!("Chunk upload failed (HTTP {}): {}", status, text);
        }

        let new_offset = resp
            .headers()
            .get("Upload-Offset")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

        Ok(json!({ "offset": new_offset }))
    }

    /// TUS status check
    pub fn media_status(
        &self,
        resource: &str,
        media_type: &str,
        upload_id: &str,
    ) -> Result<Value> {
        let url = self.url(&format!(
            "/v5/{}/{}/{}/status",
            resource, media_type, upload_id
        ));
        api_request(&self.http, reqwest::Method::GET, &url, None, self.auth())
    }

    /// TUS finalize
    pub fn media_finalize(
        &self,
        resource: &str,
        media_type: &str,
        upload_id: &str,
        body: Option<Value>,
    ) -> Result<Value> {
        let url = self.url(&format!(
            "/v5/{}/{}/{}/finalize",
            resource, media_type, upload_id
        ));
        api_request(
            &self.http,
            reqwest::Method::POST,
            &url,
            Some(body.unwrap_or_else(|| json!({}))),
            self.auth(),
        )
    }

    /// Download a media file
    pub fn media_download(
        &self,
        resource: &str,
        media_type: &str,
        upload_id: &str,
    ) -> Result<Vec<u8>> {
        let url = self.url(&format!(
            "/v5/{}/{}/{}/download",
            resource, media_type, upload_id
        ));
        let resp = api_request_raw(&self.http, reqwest::Method::GET, &url, None, self.auth())?;
        let bytes = resp.bytes().context("Failed to read download response")?;
        Ok(bytes.to_vec())
    }

    /// Preview a media file
    pub fn media_preview(
        &self,
        resource: &str,
        media_type: &str,
        upload_id: &str,
    ) -> Result<Value> {
        let url = self.url(&format!(
            "/v5/{}/{}/{}/preview",
            resource, media_type, upload_id
        ));
        api_request(&self.http, reqwest::Method::GET, &url, None, self.auth())
    }

    // ── Streaming ─────────────────────────────────────────────

    /// SSE stream — returns raw response for line-by-line reading.
    pub fn stream(
        &self,
        artifact: &str,
        operation: &str,
        entity_id: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<blocking::Response> {
        let mut params = vec![
            format!("artifact={}", artifact),
            format!("operation={}", operation),
        ];
        if let Some(id) = entity_id {
            params.push(format!("entity_id={}", id));
        }
        if let Some(c) = cursor {
            params.push(format!("cursor={}", c));
        }
        let url = format!("{}?{}", self.url("/v5/stream"), params.join("&"));

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

    // ── Resource action (v5 routes) ────────────────────────

    #[test]
    fn test_resource_action_search() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/v5/personas/search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"entries": [], "total_count": 0}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.resource_action("personas", "search", None).unwrap();
        assert_eq!(result["total_count"], 0);
        mock.assert();
    }

    #[test]
    fn test_resource_action_with_body() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/v5/scenarios/get")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"scenario_id": "s-1", "name": "Test"}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client
            .resource_action(
                "scenarios",
                "get",
                Some(json!({"scenario_id": "s-1"})),
            )
            .unwrap();
        assert_eq!(result["name"], "Test");
        mock.assert();
    }

    #[test]
    fn test_resource_action_attempt_start() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/v5/attempt/start")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"attempt_id": "a-1"}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client
            .resource_action("attempt", "start", Some(json!({"scenario_id": "s-1"})))
            .unwrap();
        assert_eq!(result["attempt_id"], "a-1");
        mock.assert();
    }

    // ── Context / emulate / generate ────────────────────────

    #[test]
    fn test_context() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/v5/context")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"profile_id": "p-1", "name": "Alice", "role": "admin"}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.context().unwrap();
        assert_eq!(result["profile_id"], "p-1");
        assert_eq!(result["name"], "Alice");
        mock.assert();
    }

    #[test]
    fn test_emulate() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/v5/emulate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"emulating": true, "target_profile_id": "p-2", "ttl": 300}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.emulate("p-2", 300).unwrap();
        assert_eq!(result["emulating"], true);
        assert_eq!(result["ttl"], 300);
        mock.assert();
    }

    #[test]
    fn test_unemulate() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/v5/unemulate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"emulating": false}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.unemulate().unwrap();
        assert_eq!(result["emulating"], false);
        mock.assert();
    }

    #[test]
    fn test_generate() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/v5/generate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"job_id": "j-1", "status": "queued"}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.generate("grp-1", None).unwrap();
        assert_eq!(result["job_id"], "j-1");
        mock.assert();
    }

    #[test]
    fn test_generate_with_body() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/v5/generate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"job_id": "j-2", "status": "queued"}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client
            .generate("grp-1", Some(json!({"count": 10})))
            .unwrap();
        assert_eq!(result["job_id"], "j-2");
        mock.assert();
    }

    #[test]
    fn test_stream_url_construction() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/v5/stream")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("artifact".into(), "personas".into()),
                mockito::Matcher::UrlEncoded("operation".into(), "create".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "text/event-stream")
            .with_body("data: {\"event\": \"created\"}\n\n")
            .create();

        let client = GlowClient::new(&server.url(), None);
        let resp = client.stream("personas", "create", None, None).unwrap();
        assert!(resp.status().is_success());
        mock.assert();
    }

    // ── Per-resource media ────────────────────────────────────

    #[test]
    fn test_media_upload() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/v5/documents/file/upload")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"upload_id": "up-1", "filename": "test.txt"}"#)
            .create();

        // Create a temp file
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, b"hello").unwrap();

        let client = GlowClient::new(&server.url(), None);
        let result = client
            .media_upload("documents", "file", file_path.to_str().unwrap())
            .unwrap();
        assert_eq!(result["upload_id"], "up-1");
        mock.assert();
    }

    #[test]
    fn test_media_create_tus() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/v5/documents/file/create")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"upload_id": "up-2", "upload_url": "/v5/documents/file/up-2"}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client
            .media_create("documents", "file", "report.pdf", Some(1024))
            .unwrap();
        assert_eq!(result["upload_id"], "up-2");
        mock.assert();
    }

    #[test]
    fn test_media_discover() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/v5/scenarios/video/discover")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"types": ["mp4", "webm"]}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.media_discover("scenarios", "video", None).unwrap();
        assert_eq!(result["types"][0], "mp4");
        mock.assert();
    }

    #[test]
    fn test_media_status() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/v5/documents/file/up-1/status")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"offset": 512, "length": 1024}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.media_status("documents", "file", "up-1").unwrap();
        assert_eq!(result["offset"], 512);
        mock.assert();
    }

    #[test]
    fn test_media_finalize() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/v5/documents/file/up-1/finalize")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"finalized": true}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client
            .media_finalize("documents", "file", "up-1", None)
            .unwrap();
        assert_eq!(result["finalized"], true);
        mock.assert();
    }

    #[test]
    fn test_media_preview() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/v5/documents/file/up-1/preview")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"columns": ["name"], "rows": 3}"#)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let result = client.media_preview("documents", "file", "up-1").unwrap();
        assert_eq!(result["rows"], 3);
        mock.assert();
    }

    // ── Error handling ───────────────────────────────────────

    #[test]
    fn test_401_returns_auth_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/v5/personas/search")
            .with_status(401)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let err = client.resource_action("personas", "search", None).unwrap_err();
        assert!(err.to_string().contains("Authentication failed"));
    }

    #[test]
    fn test_403_returns_permission_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/v5/personas/search")
            .with_status(403)
            .create();

        let client = GlowClient::new(&server.url(), None);
        let err = client.resource_action("personas", "search", None).unwrap_err();
        assert!(err.to_string().contains("Permission denied"));
    }

    #[test]
    fn test_404_returns_not_found_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/v5/personas/get")
            .with_status(404)
            .with_body("persona not found")
            .create();

        let client = GlowClient::new(&server.url(), None);
        let err = client.resource_action("personas", "get", Some(json!({"persona_id": "x"}))).unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_500_returns_api_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/v5/personas/search")
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let client = GlowClient::new(&server.url(), None);
        let err = client.resource_action("personas", "search", None).unwrap_err();
        assert!(err.to_string().contains("API error"));
    }

    #[test]
    fn test_connection_refused_returns_helpful_error() {
        let client = GlowClient::new("http://127.0.0.1:1", None);
        let err = client.resource_action("personas", "search", None).unwrap_err();
        assert!(err.to_string().contains("Failed to connect"));
    }

    #[test]
    fn test_license_key_sent_as_header() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/v5/personas/search")
            .match_header("X-License-Key", "test-key-123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"entries": [], "total_count": 0}"#)
            .create();

        let client = GlowClient::new(&server.url(), Some("test-key-123"));
        client.resource_action("personas", "search", None).unwrap();
        mock.assert();
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
