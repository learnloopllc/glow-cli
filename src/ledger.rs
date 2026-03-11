// ledger.rs — File-based cryptographic ledger for attempt tracking
//
// Each completed attempt in a Glow instance produces a sealed .ledger file.
// Files are chained cryptographically — modifying any file breaks the chain.
//
// Glow writes the files. The CLI reads, verifies, and syncs them to the
// LearnLoop API for billing and compliance.
//
// See docs/ledger.md for the full design.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

// ── Types ─────────────────────────────────────────────────────

/// Data from a completed attempt (written by Glow)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptData {
    pub attempt_id: String,
    pub completed_at: String,
    pub passed: bool,
    pub score: Option<i32>,
}

/// A sealed ledger entry — one .ledger file on disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub version: u32,
    pub seq: u64,
    pub attempt: AttemptData,
    pub data_hash: String,
    pub prev_hash: String,
    pub chain_hash: String,
    pub signature: String,
}

/// Result of verifying a chain
#[derive(Debug, Serialize)]
pub struct ChainStatus {
    pub total_entries: usize,
    pub valid: bool,
    pub latest_hash: String,
    pub errors: Vec<String>,
}

// ── Core crypto ───────────────────────────────────────────────

/// Hash arbitrary data with BLAKE3, returning a 64-char hex string
pub fn hash(data: &str) -> String {
    blake3::hash(data.as_bytes()).to_hex().to_string()
}

/// Derive a 32-byte signing key from a license key string.
/// Uses BLAKE3's key derivation with a fixed context so the same
/// license key always produces the same signing key.
pub fn derive_signing_key(license_key: &str) -> [u8; 32] {
    blake3::derive_key("learnloop ledger v1", license_key.as_bytes())
}

/// Compute the canonical data hash for an attempt.
/// The canonical form is serde_json's deterministic struct serialization.
/// Glow (Python) must match this with json.dumps(data, sort_keys=True, separators=(",",":"))
pub fn compute_data_hash(attempt: &AttemptData) -> String {
    let canonical = serde_json::to_string(attempt).expect("AttemptData is always serializable");
    hash(&canonical)
}

/// Sign a chain_hash using a license key (BLAKE3 keyed hash / MAC)
pub fn sign(chain_hash: &str, license_key: &str) -> String {
    let signing_key = derive_signing_key(license_key);
    blake3::keyed_hash(&signing_key, chain_hash.as_bytes())
        .to_hex()
        .to_string()
}

// ── Seal + Verify ─────────────────────────────────────────────

/// Create a sealed ledger entry from attempt data.
/// This is what Glow calls after grading an attempt.
/// Also used by tests to generate valid chains.
#[allow(dead_code)] // used in tests + will be used by Glow integration
pub fn seal_entry(
    attempt: &AttemptData,
    seq: u64,
    prev_hash: &str,
    license_key: &str,
) -> LedgerEntry {
    let data_hash = compute_data_hash(attempt);
    let chain_hash = hash(&format!("{}{}", prev_hash, data_hash));
    let signature = sign(&chain_hash, license_key);

    LedgerEntry {
        version: 1,
        seq,
        attempt: attempt.clone(),
        data_hash,
        prev_hash: prev_hash.to_string(),
        chain_hash,
        signature,
    }
}

/// Verify a single entry's hashes and signature.
pub fn verify_entry(
    entry: &LedgerEntry,
    expected_prev_hash: &str,
    license_key: &str,
) -> Result<()> {
    // 1. Verify data_hash matches attempt data
    let expected_data_hash = compute_data_hash(&entry.attempt);
    if entry.data_hash != expected_data_hash {
        anyhow::bail!(
            "seq={}: data_hash mismatch — attempt data may be tampered",
            entry.seq
        );
    }

    // 2. Verify prev_hash links to the expected previous entry
    if entry.prev_hash != expected_prev_hash {
        anyhow::bail!(
            "seq={}: prev_hash mismatch — chain ordering may be corrupted",
            entry.seq
        );
    }

    // 3. Verify chain_hash
    let expected_chain = hash(&format!("{}{}", entry.prev_hash, entry.data_hash));
    if entry.chain_hash != expected_chain {
        anyhow::bail!(
            "seq={}: chain_hash mismatch — entry may be tampered",
            entry.seq
        );
    }

    // 4. Verify signature (proves license key holder created this)
    let expected_sig = sign(&entry.chain_hash, license_key);
    if entry.signature != expected_sig {
        anyhow::bail!(
            "seq={}: signature mismatch — wrong license key or tampered data",
            entry.seq
        );
    }

    Ok(())
}

/// Verify an entire chain of entries (must be sorted by seq)
pub fn verify_chain(entries: &[LedgerEntry], license_key: &str) -> ChainStatus {
    let mut errors = Vec::new();
    let mut prev_hash = "0".to_string();

    for entry in entries {
        if let Err(e) = verify_entry(entry, &prev_hash, license_key) {
            errors.push(e.to_string());
        }
        prev_hash = entry.chain_hash.clone();
    }

    ChainStatus {
        total_entries: entries.len(),
        valid: errors.is_empty(),
        latest_hash: prev_hash,
        errors,
    }
}

// ── File I/O ──────────────────────────────────────────────────

/// Read a single ledger entry from a file
pub fn read_entry(path: &Path) -> Result<LedgerEntry> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read ledger file: {}", path.display()))?;
    serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse ledger file: {}", path.display()))
}

/// Read all .ledger files from a directory, sorted by seq
pub fn read_chain(dir: &Path) -> Result<Vec<LedgerEntry>> {
    if !dir.exists() {
        anyhow::bail!(
            "Ledger directory not found: {}. Is this a Glow instance with ledger enabled?",
            dir.display()
        );
    }

    let mut entries = Vec::new();
    for file in
        std::fs::read_dir(dir).with_context(|| format!("Failed to read: {}", dir.display()))?
    {
        let path = file?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("ledger") {
            entries.push(read_entry(&path)?);
        }
    }

    entries.sort_by_key(|e| e.seq);
    Ok(entries)
}

/// Write a ledger entry as {attempt_id}.ledger
#[allow(dead_code)] // used in tests + will be used by Glow integration
pub fn write_entry(dir: &Path, entry: &LedgerEntry) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)
        .with_context(|| format!("Failed to create ledger directory: {}", dir.display()))?;

    let path = dir.join(format!("{}.ledger", entry.attempt.attempt_id));
    let contents =
        serde_json::to_string_pretty(entry).context("Failed to serialize ledger entry")?;
    std::fs::write(&path, contents)
        .with_context(|| format!("Failed to write: {}", path.display()))?;
    Ok(path)
}

// ── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_attempt(id: &str) -> AttemptData {
        AttemptData {
            attempt_id: id.to_string(),
            completed_at: "2026-03-11T10:00:00Z".to_string(),
            passed: true,
            score: Some(85),
        }
    }

    const KEY: &str = "test-license-key-123";

    // ── Primitive: hashing ────────────────────────────────

    #[test]
    fn test_hash_deterministic() {
        assert_eq!(hash("hello"), hash("hello"));
    }

    #[test]
    fn test_hash_different_inputs() {
        assert_ne!(hash("hello"), hash("world"));
    }

    #[test]
    fn test_hash_returns_64_hex_chars() {
        let h = hash("test");
        assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(h.len(), 64);
    }

    // ── Primitive: key derivation ─────────────────────────

    #[test]
    fn test_derive_key_deterministic() {
        assert_eq!(derive_signing_key("k"), derive_signing_key("k"));
    }

    #[test]
    fn test_derive_key_different_inputs() {
        assert_ne!(derive_signing_key("a"), derive_signing_key("b"));
    }

    // ── Primitive: signing ────────────────────────────────

    #[test]
    fn test_sign_deterministic() {
        assert_eq!(sign("data", "key"), sign("data", "key"));
    }

    #[test]
    fn test_sign_different_keys() {
        assert_ne!(sign("data", "key-a"), sign("data", "key-b"));
    }

    // ── Composed: seal + verify ───────────────────────────

    #[test]
    fn test_seal_and_verify_genesis() {
        let entry = seal_entry(&sample_attempt("att-001"), 1, "0", KEY);
        assert_eq!(entry.version, 1);
        assert_eq!(entry.seq, 1);
        assert_eq!(entry.prev_hash, "0");
        verify_entry(&entry, "0", KEY).unwrap();
    }

    #[test]
    fn test_seal_is_deterministic() {
        let e1 = seal_entry(&sample_attempt("att-001"), 1, "0", KEY);
        let e2 = seal_entry(&sample_attempt("att-001"), 1, "0", KEY);
        assert_eq!(e1.chain_hash, e2.chain_hash);
        assert_eq!(e1.signature, e2.signature);
    }

    #[test]
    fn test_chain_of_three() {
        let e1 = seal_entry(&sample_attempt("att-001"), 1, "0", KEY);
        let e2 = seal_entry(&sample_attempt("att-002"), 2, &e1.chain_hash, KEY);
        let e3 = seal_entry(&sample_attempt("att-003"), 3, &e2.chain_hash, KEY);

        verify_entry(&e1, "0", KEY).unwrap();
        verify_entry(&e2, &e1.chain_hash, KEY).unwrap();
        verify_entry(&e3, &e2.chain_hash, KEY).unwrap();
    }

    // ── Tamper detection ──────────────────────────────────

    #[test]
    fn test_tampered_score_fails() {
        let mut entry = seal_entry(&sample_attempt("att-001"), 1, "0", KEY);
        entry.attempt.score = Some(100);
        let err = verify_entry(&entry, "0", KEY).unwrap_err();
        assert!(err.to_string().contains("data_hash mismatch"));
    }

    #[test]
    fn test_wrong_license_key_fails() {
        let entry = seal_entry(&sample_attempt("att-001"), 1, "0", KEY);
        let err = verify_entry(&entry, "0", "wrong-key").unwrap_err();
        assert!(err.to_string().contains("signature mismatch"));
    }

    #[test]
    fn test_broken_chain_link_fails() {
        let e1 = seal_entry(&sample_attempt("att-001"), 1, "0", KEY);
        let e2 = seal_entry(&sample_attempt("att-002"), 2, &e1.chain_hash, KEY);
        let err = verify_entry(&e2, "wrong_prev", KEY).unwrap_err();
        assert!(err.to_string().contains("prev_hash mismatch"));
    }

    // ── Chain verification ────────────────────────────────

    #[test]
    fn test_verify_valid_chain() {
        let e1 = seal_entry(&sample_attempt("att-001"), 1, "0", KEY);
        let e2 = seal_entry(&sample_attempt("att-002"), 2, &e1.chain_hash, KEY);
        let e3 = seal_entry(&sample_attempt("att-003"), 3, &e2.chain_hash, KEY);

        let status = verify_chain(&[e1, e2, e3], KEY);
        assert!(status.valid);
        assert_eq!(status.total_entries, 3);
        assert!(status.errors.is_empty());
    }

    #[test]
    fn test_verify_empty_chain() {
        let status = verify_chain(&[], KEY);
        assert!(status.valid);
        assert_eq!(status.total_entries, 0);
        assert_eq!(status.latest_hash, "0");
    }

    #[test]
    fn test_verify_chain_catches_tamper() {
        let e1 = seal_entry(&sample_attempt("att-001"), 1, "0", KEY);
        let mut e2 = seal_entry(&sample_attempt("att-002"), 2, &e1.chain_hash, KEY);
        e2.attempt.score = Some(999);
        let e3 = seal_entry(&sample_attempt("att-003"), 3, &e2.chain_hash, KEY);

        let status = verify_chain(&[e1, e2, e3], KEY);
        assert!(!status.valid);
        assert!(!status.errors.is_empty());
    }

    // ── File I/O ──────────────────────────────────────────

    #[test]
    fn test_write_and_read_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let entry = seal_entry(&sample_attempt("att-001"), 1, "0", KEY);

        let path = write_entry(dir.path(), &entry).unwrap();
        assert!(path.exists());
        assert!(path.to_str().unwrap().contains("att-001.ledger"));

        let read_back = read_entry(&path).unwrap();
        assert_eq!(read_back.chain_hash, entry.chain_hash);
        assert_eq!(read_back.signature, entry.signature);
    }

    #[test]
    fn test_read_chain_sorted_by_seq() {
        let dir = tempfile::tempdir().unwrap();
        let e1 = seal_entry(&sample_attempt("att-001"), 1, "0", KEY);
        let e2 = seal_entry(&sample_attempt("att-002"), 2, &e1.chain_hash, KEY);
        let e3 = seal_entry(&sample_attempt("att-003"), 3, &e2.chain_hash, KEY);

        // Write in reverse order — read_chain should still sort by seq
        write_entry(dir.path(), &e3).unwrap();
        write_entry(dir.path(), &e1).unwrap();
        write_entry(dir.path(), &e2).unwrap();

        let chain = read_chain(dir.path()).unwrap();
        assert_eq!(chain.len(), 3);
        assert_eq!(chain[0].seq, 1);
        assert_eq!(chain[1].seq, 2);
        assert_eq!(chain[2].seq, 3);
    }

    #[test]
    fn test_read_chain_nonexistent_dir_fails() {
        let err = read_chain(Path::new("/nonexistent/path")).unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    // ── End-to-end: write → read → verify ─────────────────

    #[test]
    fn test_end_to_end() {
        let dir = tempfile::tempdir().unwrap();

        let e1 = seal_entry(&sample_attempt("att-001"), 1, "0", KEY);
        write_entry(dir.path(), &e1).unwrap();

        let e2 = seal_entry(&sample_attempt("att-002"), 2, &e1.chain_hash, KEY);
        write_entry(dir.path(), &e2).unwrap();

        let e3 = seal_entry(&sample_attempt("att-003"), 3, &e2.chain_hash, KEY);
        write_entry(dir.path(), &e3).unwrap();

        // Read back and verify
        let chain = read_chain(dir.path()).unwrap();
        let status = verify_chain(&chain, KEY);
        assert!(status.valid);
        assert_eq!(status.total_entries, 3);
    }
}
