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

// ── List / Search ──────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPersonaResponse {
    pub personas: Vec<ListPersona>,
    pub total_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPersona {
    pub persona_id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_inactive: bool,
}

// ── Get ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPersonaResponse {
    pub group_id: String,
    pub can_edit: bool,
    pub names: PersonaNameSection,
    pub descriptions: PersonaDescriptionSection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonaNameSection {
    pub current_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonaDescriptionSection {
    pub current_description: Option<String>,
}

// ── Create ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePersonaResponse {
    pub results: Vec<PersonaResultItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonaResultItem {
    pub success: bool,
    pub persona_id: String,
    pub message: String,
}

// ── Delete ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePersonaResponse {
    pub success: bool,
}

// ── Uploads ────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadCreateResponse {
    pub upload_id: String,
    pub upload_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadPatchResponse {
    pub offset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadStatusResponse {
    pub offset: u64,
    pub length: Option<u64>,
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
    fn test_deserialize_list_persona_response() {
        let json = r#"{
            "personas": [
                {"persona_id": "1", "name": "Test", "description": null, "is_inactive": false}
            ],
            "total_count": 1
        }"#;
        let resp: ListPersonaResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.personas[0].name, "Test");
        assert!(resp.personas[0].description.is_none());
    }

    #[test]
    fn test_deserialize_ignores_extra_fields() {
        let json = r#"{"personas": [], "total_count": 0, "extra_field": "ignored"}"#;
        let resp: ListPersonaResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.total_count, 0);
    }

    #[test]
    fn test_deserialize_missing_required_field_fails() {
        let json = r#"{"personas": []}"#;
        assert!(serde_json::from_str::<ListPersonaResponse>(json).is_err());
    }

    #[test]
    fn test_deserialize_get_persona_response() {
        let json = r#"{
            "group_id": "grp-1",
            "can_edit": true,
            "names": {"current_name": "Alice"},
            "descriptions": {"current_description": "A persona"}
        }"#;
        let resp: GetPersonaResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.names.current_name, "Alice");
        assert_eq!(
            resp.descriptions.current_description,
            Some("A persona".into())
        );
    }

    #[test]
    fn test_deserialize_create_persona_response() {
        let json = r#"{"results": [{"success": true, "persona_id": "p-1", "message": "OK"}]}"#;
        let resp: CreatePersonaResponse = serde_json::from_str(json).unwrap();
        assert!(resp.results[0].success);
        assert_eq!(resp.results[0].persona_id, "p-1");
    }

    #[test]
    fn test_serialize_roundtrip() {
        let resp = ListPersonaResponse {
            personas: vec![ListPersona {
                persona_id: "p-1".into(),
                name: "Test".into(),
                description: Some("desc".into()),
                is_inactive: false,
            }],
            total_count: 1,
        };
        let json = serde_json::to_string(&resp).unwrap();
        let roundtrip: ListPersonaResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtrip.total_count, 1);
        assert_eq!(roundtrip.personas[0].persona_id, "p-1");
    }

    #[test]
    fn test_optional_description_absent() {
        let json = r#"{
            "group_id": "grp-1",
            "can_edit": false,
            "names": {"current_name": "Bob"},
            "descriptions": {"current_description": null}
        }"#;
        let resp: GetPersonaResponse = serde_json::from_str(json).unwrap();
        assert!(resp.descriptions.current_description.is_none());
    }

    #[test]
    fn test_deserialize_upload_create_response() {
        let json = r#"{"upload_id": "up-1", "upload_url": "/uploads/up-1"}"#;
        let resp: UploadCreateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.upload_id, "up-1");
        assert_eq!(resp.upload_url, "/uploads/up-1");
    }

    #[test]
    fn test_upload_status_response() {
        let resp = UploadStatusResponse {
            offset: 512,
            length: Some(1024),
        };
        let json = serde_json::to_string(&resp).unwrap();
        let roundtrip: UploadStatusResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtrip.offset, 512);
        assert_eq!(roundtrip.length, Some(1024));
    }
}
