// config.rs — Configuration file support
//
// Priority chain (highest to lowest):
//   CLI flag > env var > config file > hardcoded default
//
// Config file location: ~/.config/learnloop/config.toml
// In Python terms: like a pyproject.toml or .env file
// In TS terms:     like a .rc file or config.json

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub api_url: Option<String>,
    pub license_key: Option<String>,
    pub glow_url: Option<String>,
    pub client_id: Option<String>,
}

impl Config {
    /// Load config from ~/.config/learnloop/config.toml.
    /// Returns default config if file doesn't exist.
    pub fn load() -> Result<Self> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Config::default());
        }
        let contents = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        Self::parse(&contents)
            .with_context(|| format!("Failed to parse {}. Check the TOML syntax.", path.display()))
    }

    /// Parse config from a TOML string (public for testing)
    pub fn parse(toml_str: &str) -> Result<Self> {
        toml::from_str(toml_str).map_err(|e| anyhow::anyhow!("Invalid config: {}", e))
    }

    /// Returns the config file path
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("learnloop")
            .join("config.toml")
    }
}

/// Resolve a value with priority: explicit (CLI/env) > config file > default
pub fn resolve_option(explicit: Option<&str>, config_val: Option<&str>, default: &str) -> String {
    explicit.or(config_val).unwrap_or(default).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_full_config() {
        let toml = r#"
            api_url = "https://custom.api.com"
            license_key = "sk-test-123"
            glow_url = "https://glow.example.com"
        "#;
        let config = Config::parse(toml).unwrap();
        assert_eq!(config.api_url.unwrap(), "https://custom.api.com");
        assert_eq!(config.license_key.unwrap(), "sk-test-123");
        assert_eq!(config.glow_url.unwrap(), "https://glow.example.com");
    }

    #[test]
    fn test_parse_partial_config() {
        let toml = r#"api_url = "https://custom.api.com""#;
        let config = Config::parse(toml).unwrap();
        assert_eq!(config.api_url.unwrap(), "https://custom.api.com");
        assert!(config.license_key.is_none());
        assert!(config.glow_url.is_none());
    }

    #[test]
    fn test_parse_empty_config() {
        let config = Config::parse("").unwrap();
        assert!(config.api_url.is_none());
        assert!(config.license_key.is_none());
        assert!(config.glow_url.is_none());
    }

    #[test]
    fn test_parse_invalid_toml_returns_error() {
        let result = Config::parse("not valid [[[toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_path_ends_with_expected() {
        let path = Config::config_path();
        assert!(path.ends_with("learnloop/config.toml"));
    }

    #[test]
    fn test_resolve_option_explicit_wins() {
        assert_eq!(
            resolve_option(Some("explicit"), Some("config"), "default"),
            "explicit"
        );
    }

    #[test]
    fn test_resolve_option_config_fallback() {
        assert_eq!(resolve_option(None, Some("config"), "default"), "config");
    }

    #[test]
    fn test_resolve_option_default_fallback() {
        assert_eq!(resolve_option(None, None, "default"), "default");
    }
}
