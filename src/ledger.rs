// ledger.rs — Hash-based ledger service, built into the CLI
//
// Each entry chains to the previous one via cryptographic hash.
// This gives you tamper-evident audit trails — if any entry is
// modified, the chain breaks.

use serde::{Deserialize, Serialize};

/// A single entry in the ledger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub data: String,
    pub hash: String,
    pub prev_hash: String,
}

/// Hash arbitrary data using BLAKE3
pub fn hash(data: &str) -> String {
    blake3::hash(data.as_bytes()).to_hex().to_string()
}

/// Create a new ledger entry, chaining it to the previous hash
pub fn create_entry(data: &str, prev_hash: &str) -> Entry {
    let combined = format!("{}{}", prev_hash, data);
    Entry {
        data: data.to_string(),
        hash: hash(&combined),
        prev_hash: prev_hash.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_deterministic() {
        assert_eq!(hash("hello"), hash("hello"));
    }

    #[test]
    fn test_hash_different_inputs() {
        assert_ne!(hash("hello"), hash("world"));
    }

    #[test]
    fn test_create_entry_chains() {
        let first = create_entry("genesis", "0");
        let second = create_entry("block 2", &first.hash);
        assert_eq!(second.prev_hash, first.hash);
    }
}
