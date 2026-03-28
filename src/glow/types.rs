// glow/types.rs — Re-exports from auto-generated types + hand-written additions
//
// types_gen.rs is auto-generated from the glow-api OpenAPI spec.
// Add aliases or custom types here.

#[allow(unused_imports)]
pub use super::api::latest::*;

// ── CLI-specific types (not in OpenAPI) ──────────────────────────

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub service: Option<String>,
}
