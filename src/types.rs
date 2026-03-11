// types.rs — Response types from the Glow API
//
// These mirror the Pydantic models on the server side.
// In Python:   class ListPersonaResponse(BaseModel): ...
// In TS:       interface ListPersonaResponse { ... }
// In Rust:     struct ListPersonaResponse { ... } with Deserialize
//
// We only define the fields we actually USE in the CLI.
// serde will silently ignore any extra fields in the JSON response.
// This means we don't have to mirror every single field from the API.

use serde::Deserialize;

// ── List / Search ──────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ListPersonaResponse {
    pub personas: Vec<ListPersona>,   // Vec<T> = like list[T] in Python, T[] in TS
    pub total_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListPersona {
    pub persona_id: String,
    pub name: String,
    pub description: Option<String>,  // Option<T> = like T | None in Python, T | null in TS
    pub is_inactive: bool,
}

// ── Get ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct GetPersonaResponse {
    pub group_id: String,
    pub can_edit: bool,
    pub names: PersonaNameSection,
    pub descriptions: PersonaDescriptionSection,
}

#[derive(Debug, Deserialize)]
pub struct PersonaNameSection {
    pub current_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PersonaDescriptionSection {
    pub current_description: Option<String>,
}

// ── Create ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreatePersonaResponse {
    pub results: Vec<PersonaResultItem>,
}

#[derive(Debug, Deserialize)]
pub struct PersonaResultItem {
    pub success: bool,
    pub persona_id: String,
    pub message: String,
}

// ── Delete ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct DeletePersonaResponse {
    pub success: bool,
}
