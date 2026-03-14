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
    Command::cargo_bin("glow")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Glow CLI"));
}

#[test]
fn test_glw_alias_works() {
    Command::cargo_bin("glw")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Glow CLI"));
}

#[test]
fn test_unknown_subcommand_shows_error() {
    Command::cargo_bin("glow")
        .unwrap()
        .arg("nonsense")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown resource 'nonsense'"))
        .stderr(predicate::str::contains("personas"));
}

// ── Top-level Glow instance commands ─────────────────────────

#[test]
fn test_health_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["health", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("health"));
}

#[test]
fn test_login_targets_glow_instance() {
    // Top-level login now targets the glow instance
    Command::cargo_bin("glow")
        .unwrap()
        .args(["--instance-url", "http://127.0.0.1:1", "login"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to reach OIDC discovery"));
}

#[test]
fn test_logout_glow_instance() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["--instance-url", "http://127.0.0.1:1", "logout"])
        .assert()
        .success();
}

// ── Top-level identity & streaming commands ─────────────────

#[test]
fn test_context_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["context", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("context"));
}

#[test]
fn test_emulate_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["emulate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TARGET_PROFILE_ID"));
}

#[test]
fn test_unemulate_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["unemulate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("emulat"));
}

#[test]
fn test_generate_requires_group_id() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["generate"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("GROUP_ID"));
}

#[test]
fn test_stream_requires_artifact_and_operation() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["stream"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--artifact"))
        .stderr(predicate::str::contains("--operation"));
}

// ── Per-resource media operations ────────────────────────────

#[test]
fn test_media_requires_action() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["documents", "file"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Expected a media action"));
}

#[test]
fn test_media_upload_requires_file() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["documents", "file", "upload"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--file"));
}

#[test]
fn test_media_download_requires_id() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["documents", "image", "download"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--id"));
}

#[test]
fn test_media_unknown_action_shows_error() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["documents", "file", "nonsense"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown media action"));
}

// ── Instance management ─────────────────────────────────────

#[test]
fn test_instances_list_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["instances", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("configured"));
}

#[test]
fn test_instances_alias() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["inst", "--help"])
        .assert()
        .success();
}

#[test]
fn test_instances_add_requires_url() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["instances", "add", "test-inst"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--url"));
}

#[test]
fn test_use_unknown_instance_fails() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["use", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown instance"));
}

// ── Admin commands ───────────────────────────────────────────

#[test]
fn test_admin_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Platform administration"));
}

#[test]
fn test_admin_login_unreachable() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "admin", "login"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to reach OIDC discovery"));
}

#[test]
fn test_admin_logout_no_session() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "admin", "logout"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No session found"));
}

#[test]
fn test_admin_sessions_empty() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "sessions"])
        .assert()
        .success();
}

#[test]
fn test_admin_sessions_json_output() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["--json", "admin", "sessions"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("{"))
        .stdout(predicate::str::contains("\"sessions\""));
}

#[test]
fn test_admin_whoami_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "whoami", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("authenticated user"));
}

#[test]
fn test_admin_network_alias() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "net", "--help"])
        .assert()
        .success();
}

#[test]
fn test_admin_network_unreachable_doesnt_crash() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "admin", "network"])
        .assert()
        .success()
        .stdout(predicate::str::contains("unreachable"));
}

#[test]
fn test_admin_network_json_output() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["--json", "--api-url", "http://127.0.0.1:1", "admin", "network"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"api_status\""))
        .stdout(predicate::str::contains("\"unreachable\""));
}

#[test]
fn test_admin_status_human_output() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "admin", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Glow Status"));
}

#[test]
fn test_admin_status_json_output() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["--json", "--api-url", "http://127.0.0.1:1", "admin", "status"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("{"))
        .stdout(predicate::str::contains("\"api_status\""))
        .stdout(predicate::str::contains("\"config_file\""));
}

#[test]
fn test_admin_licenses_list_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "licenses", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("List all licenses"));
}

#[test]
fn test_admin_licenses_alias() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "lic", "--help"])
        .assert()
        .success();
}

#[test]
fn test_admin_orgs_list_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "orgs", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("organizations"));
}

#[test]
fn test_admin_orgs_alias() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "org", "--help"])
        .assert()
        .success();
}

#[test]
fn test_admin_orgs_members_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "orgs", "members", "org-1", "list", "--help"])
        .assert()
        .success();
}

#[test]
fn test_admin_billing_plans_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "billing", "plans", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("plans"));
}

#[test]
fn test_admin_usage_requires_license_id() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "usage"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("LICENSE_ID"));
}

#[test]
fn test_admin_license_delete_without_yes_aborts() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "admin", "licenses", "delete", "lic-1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Aborted"));
}

#[test]
fn test_admin_org_delete_without_yes_aborts() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "admin", "orgs", "delete", "org-1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Aborted"));
}

#[test]
fn test_admin_deploy_destroy_without_yes_aborts() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["--api-url", "http://127.0.0.1:1", "admin", "deploy", "destroy", "d-1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Aborted"));
}

#[test]
fn test_admin_deploy_list_help() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "deploy", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("deployments"));
}

#[test]
fn test_admin_deploy_destroy_alias() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "deploy", "rm", "d-1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Aborted"));
}

#[test]
fn test_admin_deploy_list_alias() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["admin", "deploy", "ls", "--help"])
        .assert()
        .success();
}

// ── Ledger commands (top-level) ──────────────────────────────

#[test]
fn test_ledger_alias() {
    Command::cargo_bin("glow")
        .unwrap()
        .args(["l", "--help"])
        .assert()
        .success();
}

/// Helper: create a temp directory with valid .ledger files for testing
fn create_test_ledger() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let key = "test-key-for-integration";

    let attempts = vec![
        ("att-001", "2026-01-01T00:00:00Z", true, 90),
        ("att-002", "2026-01-02T00:00:00Z", true, 85),
        ("att-003", "2026-01-03T00:00:00Z", false, 40),
    ];

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
    Command::cargo_bin("glow")
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
    Command::cargo_bin("glow")
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
    Command::cargo_bin("glow")
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
    Command::cargo_bin("glow")
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
        .failure();
}

#[test]
fn test_ledger_verify_nonexistent_dir_fails() {
    Command::cargo_bin("glow")
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
    Command::cargo_bin("glow")
        .unwrap()
        .args(["ledger", "status", "--path", "/tmp"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("License key required"));
}
