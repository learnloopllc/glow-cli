// types.rs — Response types from the Glow API
//
// These mirror the Pydantic models on the server side.
// In Python:   class ListPersonaResponse(BaseModel): ...
// In TS:       interface ListPersonaResponse { ... }
// In Rust:     struct ListPersonaResponse { ... } with Serialize + Deserialize
//
// We only define the fields we actually USE in the CLI.
// serde will silently ignore any extra fields in the JSON response.
// This means we don't have to mirror every single field from the API.

use serde::{Deserialize, Serialize};

// ── List / Search ──────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPersonaResponse {
    pub personas: Vec<ListPersona>, // Vec<T> = like list[T] in Python, T[] in TS
    pub total_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPersona {
    pub persona_id: String,
    pub name: String,
    pub description: Option<String>, // Option<T> = like T | None in Python, T | null in TS
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

// ── LearnLoop API: License ─────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateLicenseResponse {
    pub valid: bool,
    pub license: Option<LicenseInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub id: String,
}

// ── LearnLoop API: Usage ──────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageReportResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageSummaryResponse {
    pub total_simulations: i64,
    pub unreported_simulations: i64,
}

// ── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

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
}
