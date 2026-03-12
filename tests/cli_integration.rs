// tests/cli_integration.rs — Binary-level integration tests
//
// These run the actual compiled binary and assert on stdout/stderr/exit codes.
// Like subprocess testing in Python (subprocess.run + assert) or execa in Node.
//
// Runs with: cargo test --test cli_integration

use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;

// ── Help & basic CLI structure ────────────────────────────────

#[test]
fn test_help_output() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("LearnLoop CLI"));
}

#[test]
fn test_ll_alias_works() {
    Command::cargo_bin("ll")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("LearnLoop CLI"));
}

#[test]
fn test_unknown_subcommand_shows_error() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .arg("nonsense")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

// ── Subcommand aliases ────────────────────────────────────────

#[test]
fn test_network_alias() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["net", "--help"])
        .assert()
        .success();
}

#[test]
fn test_glow_alias() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["g", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage a Glow instance"));
}

#[test]
fn test_personas_alias() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["g", "p", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage personas"));
}

#[test]
fn test_ledger_alias() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["l", "--help"])
        .assert()
        .success();
}

// ── Status command ────────────────────────────────────────────

#[test]
fn test_status_human_output() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("LearnLoop Status"));
}

#[test]
fn test_status_json_output() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["--json", "--api-url", "http://127.0.0.1:1", "status"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("{"))
        .stdout(predicate::str::contains("\"api_status\""))
        .stdout(predicate::str::contains("\"config_file\""));
}

// ── Network command ───────────────────────────────────────────

#[test]
fn test_network_unreachable_doesnt_crash() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "network"])
        .assert()
        .success()
        .stdout(predicate::str::contains("unreachable"));
}

#[test]
fn test_network_json_output() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["--json", "--api-url", "http://127.0.0.1:1", "network"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"api_status\""))
        .stdout(predicate::str::contains("\"unreachable\""));
}

// ── Delete confirmation ───────────────────────────────────────

#[test]
fn test_delete_without_yes_aborts_in_non_tty() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["glow", "personas", "delete", "some-id"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Aborted"));
}

// ── JSON flag works globally ──────────────────────────────────

#[test]
fn test_login_unreachable_server_fails() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "login"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to reach OIDC discovery"));
}

#[test]
fn test_logout_no_session() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "logout"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No session found"));
}

#[test]
fn test_sessions_empty() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .arg("sessions")
        .assert()
        .success();
}

#[test]
fn test_sessions_json_output() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["--json", "sessions"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("{"))
        .stdout(predicate::str::contains("\"sessions\""));
}

#[test]
fn test_glow_login_unreachable() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["glow", "--instance-url", "http://127.0.0.1:1", "login"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to reach OIDC discovery"));
}

#[test]
fn test_glow_logout() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["glow", "--instance-url", "http://127.0.0.1:1", "logout"])
        .assert()
        .success();
}

// ── Ledger commands ───────────────────────────────────────────

/// Helper: create a temp directory with valid .ledger files for testing
fn create_test_ledger() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let key = "test-key-for-integration";

    // Build a 3-entry chain using the same JSON format the CLI expects
    // We shell out to avoid importing ledger.rs internals in integration tests
    let attempts = vec![
        ("att-001", "2026-01-01T00:00:00Z", true, 90),
        ("att-002", "2026-01-02T00:00:00Z", true, 85),
        ("att-003", "2026-01-03T00:00:00Z", false, 40),
    ];

    // Use blake3 directly to build the chain (matching ledger.rs logic)
    let mut prev_hash = "0".to_string();
    for (i, (id, ts, passed, score)) in attempts.iter().enumerate() {
        let attempt_json = format!(
            r#"{{"attempt_id":"{}","completed_at":"{}","passed":{},"score":{}}}"#,
            id, ts, passed, score
        );
        let data_hash = blake3::hash(attempt_json.as_bytes()).to_hex().to_string();
        let chain_input = format!("{}{}", prev_hash, data_hash);
        let chain_hash = blake3::hash(chain_input.as_bytes()).to_hex().to_string();
        let signing_key = blake3::derive_key("learnloop ledger v1", key.as_bytes());
        let signature = blake3::keyed_hash(&signing_key, chain_hash.as_bytes())
            .to_hex()
            .to_string();

        let entry = format!(
            r#"{{
  "version": 1,
  "seq": {},
  "attempt": {{
    "attempt_id": "{}",
    "completed_at": "{}",
    "passed": {},
    "score": {}
  }},
  "data_hash": "{}",
  "prev_hash": "{}",
  "chain_hash": "{}",
  "signature": "{}"
}}"#,
            i + 1,
            id,
            ts,
            passed,
            score,
            data_hash,
            prev_hash,
            chain_hash,
            signature
        );

        let path = dir.path().join(format!("{}.ledger", id));
        let mut f = std::fs::File::create(path).unwrap();
        f.write_all(entry.as_bytes()).unwrap();
        prev_hash = chain_hash;
    }

    dir
}

#[test]
fn test_ledger_verify_valid_chain() {
    let dir = create_test_ledger();
    Command::cargo_bin("learnloop")
        .unwrap()
        .args([
            "--license-key",
            "test-key-for-integration",
            "ledger",
            "verify",
            "--path",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("3 entries, all valid"));
}

#[test]
fn test_ledger_verify_json_output() {
    let dir = create_test_ledger();
    Command::cargo_bin("learnloop")
        .unwrap()
        .args([
            "--json",
            "--license-key",
            "test-key-for-integration",
            "ledger",
            "verify",
            "--path",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"valid\": true"))
        .stdout(predicate::str::contains("\"total_entries\": 3"));
}

#[test]
fn test_ledger_status_shows_info() {
    let dir = create_test_ledger();
    Command::cargo_bin("learnloop")
        .unwrap()
        .args([
            "--license-key",
            "test-key-for-integration",
            "ledger",
            "status",
            "--path",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Total Entries:  3"))
        .stdout(predicate::str::contains("Chain Valid:"));
}

#[test]
fn test_ledger_verify_wrong_key_fails() {
    let dir = create_test_ledger();
    Command::cargo_bin("learnloop")
        .unwrap()
        .args([
            "--license-key",
            "wrong-key",
            "ledger",
            "verify",
            "--path",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .failure(); // exits non-zero because chain is invalid
}

#[test]
fn test_ledger_verify_nonexistent_dir_fails() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args([
            "--license-key",
            "any-key",
            "ledger",
            "verify",
            "--path",
            "/tmp/nonexistent-ledger-dir-12345",
        ])
        .assert()
        .failure();
}

#[test]
fn test_ledger_requires_license_key() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["ledger", "status", "--path", "/tmp"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("License key required"));
}

// ── New commands: whoami, licenses, orgs, usage, billing ─────

#[test]
fn test_whoami_help() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["whoami", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("authenticated user"));
}

#[test]
fn test_licenses_list_help() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["licenses", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("List all licenses"));
}

#[test]
fn test_licenses_alias() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["lic", "--help"])
        .assert()
        .success();
}

#[test]
fn test_orgs_list_help() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["orgs", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("organizations"));
}

#[test]
fn test_orgs_alias() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["org", "--help"])
        .assert()
        .success();
}

#[test]
fn test_orgs_members_help() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["orgs", "members", "org-1", "list", "--help"])
        .assert()
        .success();
}

#[test]
fn test_billing_plans_help() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["billing", "plans", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("plans"));
}

#[test]
fn test_usage_requires_license_id() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["usage"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("LICENSE_ID"));
}

#[test]
fn test_license_delete_without_yes_aborts() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "licenses", "delete", "lic-1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Aborted"));
}

#[test]
fn test_org_delete_without_yes_aborts() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "orgs", "delete", "org-1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Aborted"));
}

#[test]
fn test_deploy_destroy_without_yes_aborts() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "deploy", "destroy", "d-1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Aborted"));
}

#[test]
fn test_deploy_list_help() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["deploy", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("deployments"));
}

#[test]
fn test_deploy_destroy_alias() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["deploy", "rm", "d-1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Aborted"));
}

#[test]
fn test_deploy_list_alias() {
    Command::cargo_bin("learnloop")
        .unwrap()
        .args(["deploy", "ls", "--help"])
        .assert()
        .success();
}
