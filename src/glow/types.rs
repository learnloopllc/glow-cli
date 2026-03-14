// glow/types.rs — Response types from the Glow instance API
//
// These mirror the Pydantic models on the Glow server side.
// We only define the fields we actually USE in the CLI.
// serde will silently ignore any extra fields in the JSON response.

use serde::{Deserialize, Serialize};

// ── Health ────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: Option<String>,
}

// ── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_health_response() {
        let json = r#"{"status": "healthy", "version": "2.0.0"}"#;
        let resp: HealthResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "healthy");
        assert_eq!(resp.version, Some("2.0.0".into()));
    }

    #[test]
    fn test_deserialize_health_without_version() {
        let json = r#"{"status": "ok"}"#;
        let resp: HealthResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
        assert!(resp.version.is_none());
    }

    #[test]
    fn test_deserialize_ignores_extra_fields() {
        let json = r#"{"status": "ok", "extra_field": "ignored"}"#;
        let resp: HealthResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }
}
