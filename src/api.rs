// api.rs — HTTP client that talks to the Glow API
//
// Think of this like a Python class:
//
//   class Client:
//       def __init__(self, base_url, license_key):
//           self.base_url = base_url
//           self.session = requests.Session()
//           self.session.headers["X-License-Key"] = license_key
//
//       def persona_search(self):
//           return self.session.post(f"{self.base_url}/api/v5/artifacts/personas/search").json()

use anyhow::{Context, Result};
use reqwest::blocking;
use serde_json::json;

use crate::types; // Import our response types

/// API client — holds the base URL and license key
pub struct Client {
    base_url: String,
    http: blocking::Client,           // reqwest's HTTP client (like requests.Session)
    license_key: Option<String>,
}

// impl = methods on a struct (like class methods in Python)
impl Client {
    /// Create a new API client
    /// &str = borrowed string (we .to_string() to make owned copies)
    pub fn new(base_url: &str, license_key: Option<&str>) -> Self {
        Client {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: blocking::Client::new(),
            license_key: license_key.map(|s| s.to_string()),
        }
    }

    // ── Private helpers ────────────────────────────────────────

    /// Build a POST request with the license key header
    /// &self = like `self` in Python — borrows the struct (read-only access)
    fn post(&self, path: &str) -> blocking::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.post(&url);

        // if let = like "if x is not None" in Python
        if let Some(key) = &self.license_key {
            req = req.header("X-License-Key", key);
        }

        req
    }

    /// Send a POST request with a JSON body and parse the response
    /// T: serde::de::DeserializeOwned = "T must be deserializable from JSON"
    /// This is like TypeScript generics: function post<T>(path: string, body: object): T
    fn post_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<T> {
        let response = self
            .post(path)
            .json(&body)            // Serialize body to JSON (like json=body in requests)
            .send()                  // Send the HTTP request
            .context("Failed to connect to API")?;  // ? = propagate error, context = add message

        // Check for HTTP errors (4xx, 5xx)
        let status = response.status();
        if !status.is_success() {
            let text = response.text().unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, text);
            // bail! = like `raise Exception(...)` in Python
        }

        // Parse JSON response into our type T
        // Like: data = response.json() in Python, but with type validation
        let data = response.json::<T>().context("Failed to parse API response")?;
        Ok(data)
    }

    // ── Persona endpoints ──────────────────────────────────────

    /// POST /api/v5/artifacts/personas/search
    pub fn persona_search(&self) -> Result<types::ListPersonaResponse> {
        // json!({}) = like json.dumps({}) — empty body for "list all"
        self.post_json("/api/v5/artifacts/personas/search", json!({}))
    }

    /// POST /api/v5/artifacts/personas/get
    pub fn persona_get(&self, persona_id: &str) -> Result<types::GetPersonaResponse> {
        self.post_json(
            "/api/v5/artifacts/personas/get",
            json!({ "persona_id": persona_id }),
        )
    }

    /// POST /api/v5/artifacts/personas/create
    pub fn persona_create(
        &self,
        name: &str,
        description: Option<&str>,
    ) -> Result<types::CreatePersonaResponse> {
        // Build the persona item — only include description if provided
        let mut item = json!({ "name": name });
        if let Some(desc) = description {
            item["description"] = json!(desc);
        }

        self.post_json(
            "/api/v5/artifacts/personas/create",
            json!({ "personas": [item] }),
        )
    }

    /// POST /api/v5/artifacts/personas/delete
    pub fn persona_delete(&self, persona_id: &str) -> Result<types::DeletePersonaResponse> {
        self.post_json(
            "/api/v5/artifacts/personas/delete",
            json!({ "persona_ids": [persona_id] }),
        )
    }
}
