// schema_cache.rs — Fetch and cache OpenAPI specs from Glow instances
//
// On `--help`, the CLI fetches /openapi.json from the active instance,
// caches it locally, and uses it to show parameter docs.
// Cache TTL: 24 hours. One file per instance URL.

use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::time::Duration;

const CACHE_TTL: Duration = Duration::from_secs(24 * 60 * 60);
const FETCH_TIMEOUT: Duration = Duration::from_secs(3);

/// Directory for cached schemas: ~/.config/glow/schema-cache/
fn cache_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("glow")
        .join("schema-cache")
}

/// Hash an instance URL to a cache filename
fn cache_key(instance_url: &str) -> String {
    let mut hasher = DefaultHasher::new();
    instance_url.hash(&mut hasher);
    format!("{:016x}.json", hasher.finish())
}

/// Load a cached spec if it exists and is fresh
fn load_cached(instance_url: &str) -> Option<Value> {
    let path = cache_dir().join(cache_key(instance_url));
    let metadata = std::fs::metadata(&path).ok()?;
    let age = metadata.modified().ok()?.elapsed().ok()?;
    if age > CACHE_TTL {
        return None;
    }
    let contents = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&contents).ok()
}

/// Save a spec to the cache
fn save_cache(instance_url: &str, spec: &Value) {
    let dir = cache_dir();
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(cache_key(instance_url));
    if let Ok(json) = serde_json::to_string(spec) {
        let _ = std::fs::write(path, json);
    }
}

/// Fetch /openapi.json from an instance (with short timeout)
fn fetch_spec(instance_url: &str) -> Result<Value> {
    let url = format!("{}/openapi.json", instance_url.trim_end_matches('/'));
    let client = reqwest::blocking::Client::builder()
        .timeout(FETCH_TIMEOUT)
        .build()?;
    let resp = client
        .get(&url)
        .send()
        .with_context(|| format!("Failed to fetch schema from {}", url))?;
    if !resp.status().is_success() {
        anyhow::bail!("Schema fetch returned HTTP {}", resp.status());
    }
    resp.json::<Value>()
        .context("Failed to parse OpenAPI spec")
}

/// Get the OpenAPI spec for an instance (cached or fresh)
pub fn get_spec(instance_url: &str) -> Option<Value> {
    if let Some(cached) = load_cached(instance_url) {
        return Some(cached);
    }
    if let Ok(spec) = fetch_spec(instance_url) {
        save_cache(instance_url, &spec);
        Some(spec)
    } else {
        None
    }
}

/// A field extracted from an OpenAPI request schema
pub struct FieldInfo {
    pub name: String,
    pub field_type: String,
    pub required: bool,
    pub description: Option<String>,
}

/// Look up the request body fields for POST /{resource}/{action}
pub fn lookup_fields(spec: &Value, resource: &str, action: &str) -> Vec<FieldInfo> {
    let path = format!("/{}/{}", resource, action);
    let schema = resolve_request_schema(spec, &path);
    let schema = match schema {
        Some(s) => s,
        None => return Vec::new(),
    };

    let properties = match schema.get("properties") {
        Some(Value::Object(p)) => p,
        _ => return Vec::new(),
    };

    let required_fields: Vec<&str> = schema
        .get("required")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();

    properties
        .iter()
        .map(|(name, prop)| FieldInfo {
            name: name.clone(),
            field_type: extract_type(prop),
            required: required_fields.contains(&name.as_str()),
            description: prop
                .get("description")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        })
        .collect()
}

/// Resolve the request body schema for a POST path, following $refs
fn resolve_request_schema<'a>(spec: &'a Value, path: &str) -> Option<&'a Value> {
    let path_item = spec.get("paths")?.get(path)?;
    let operation = path_item.get("post")?;
    let body = operation.get("requestBody")?;
    let content = body.get("content")?.get("application/json")?;
    let schema = content.get("schema")?;

    // Follow $ref if present
    if let Some(ref_str) = schema.get("$ref").and_then(|v| v.as_str()) {
        resolve_ref(spec, ref_str)
    } else {
        Some(schema)
    }
}

/// Resolve a $ref like "#/components/schemas/FooRequest"
fn resolve_ref<'a>(spec: &'a Value, ref_path: &str) -> Option<&'a Value> {
    let parts: Vec<&str> = ref_path.trim_start_matches("#/").split('/').collect();
    let mut current = spec;
    for part in parts {
        current = current.get(part)?;
    }
    Some(current)
}

/// Extract a human-readable type string from a property schema
fn extract_type(prop: &Value) -> String {
    if let Some(r) = prop.get("$ref").and_then(|v| v.as_str()) {
        return r.rsplit('/').next().unwrap_or("object").to_string();
    }
    if let Some(any_of) = prop.get("anyOf").and_then(|v| v.as_array()) {
        let types: Vec<String> = any_of
            .iter()
            .filter(|t| t.get("type").and_then(|v| v.as_str()) != Some("null"))
            .map(extract_type)
            .collect();
        if types.len() == 1 {
            return types[0].clone();
        }
        return types.join(" | ");
    }
    match prop.get("type").and_then(|v| v.as_str()) {
        Some("array") => {
            let item_type = prop
                .get("items")
                .map(extract_type)
                .unwrap_or_else(|| "any".to_string());
            format!("{}[]", item_type)
        }
        Some(t) => t.to_string(),
        None => "any".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_cache_key_deterministic() {
        let a = cache_key("http://localhost:8000");
        let b = cache_key("http://localhost:8000");
        assert_eq!(a, b);
    }

    #[test]
    fn test_cache_key_different_urls() {
        let a = cache_key("http://localhost:8000");
        let b = cache_key("http://localhost:9000");
        assert_ne!(a, b);
    }

    #[test]
    fn test_extract_type_string() {
        assert_eq!(extract_type(&json!({"type": "string"})), "string");
    }

    #[test]
    fn test_extract_type_array() {
        assert_eq!(
            extract_type(&json!({"type": "array", "items": {"type": "string"}})),
            "string[]"
        );
    }

    #[test]
    fn test_extract_type_nullable() {
        let prop = json!({"anyOf": [{"type": "string"}, {"type": "null"}]});
        assert_eq!(extract_type(&prop), "string");
    }

    #[test]
    fn test_lookup_fields_from_spec() {
        let spec = json!({
            "paths": {
                "/personas/search": {
                    "post": {
                        "requestBody": {
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "properties": {
                                            "query": {"type": "string", "description": "Search query"},
                                            "limit": {"type": "integer", "description": "Max results"}
                                        },
                                        "required": ["query"]
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
        let fields = lookup_fields(&spec, "personas", "search");
        assert_eq!(fields.len(), 2);
        assert!(fields.iter().any(|f| f.name == "query" && f.required));
        assert!(fields.iter().any(|f| f.name == "limit" && !f.required));
    }

    #[test]
    fn test_lookup_fields_with_ref() {
        let spec = json!({
            "paths": {
                "/personas/search": {
                    "post": {
                        "requestBody": {
                            "content": {
                                "application/json": {
                                    "schema": {"$ref": "#/components/schemas/SearchRequest"}
                                }
                            }
                        }
                    }
                }
            },
            "components": {
                "schemas": {
                    "SearchRequest": {
                        "properties": {
                            "query": {"type": "string"},
                            "limit": {"type": "integer"}
                        },
                        "required": ["query"]
                    }
                }
            }
        });
        let fields = lookup_fields(&spec, "personas", "search");
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_lookup_fields_missing_path() {
        let spec = json!({"paths": {}});
        let fields = lookup_fields(&spec, "foo", "bar");
        assert!(fields.is_empty());
    }
}
