# Ledger System Design

## Overview

The ledger is a **file-based, cryptographically chained audit trail** for tracking completed simulation attempts. Each completed attempt in a Glow instance produces a sealed `.ledger` file. These files are tamper-evident — modifying any file breaks the chain.

```
Glow Instance                    CLI                      LearnLoop API
(writes files)              (reads + verifies)          (stores + bills)

attempt graded ──► .ledger file ──► learnloop ledger sync ──► simulation_ledger table
                                                                      │
                                                                      ▼
                                                               Stripe metering
```

## File Format

Each `.ledger` file is JSON. One file per completed attempt.

**Location on Glow instance:** configurable, default `./data/ledger/`

**Filename:** `{attempt_id}.ledger`

```json
{
  "version": 1,
  "seq": 1,
  "attempt": {
    "attempt_id": "550e8400-e29b-41d4-a716-446655440000",
    "score": 85,
    "passed": true,
    "completed_at": "2026-03-11T10:30:00Z"
  },
  "data_hash": "a3f8b2...64 hex chars",
  "prev_hash": "0",
  "chain_hash": "b7c9d1...64 hex chars",
  "signature": "e2f4a8...64 hex chars"
}
```

### Field definitions

| Field | Description |
|---|---|
| `version` | Schema version (currently `1`) |
| `seq` | Sequence number (1, 2, 3...). Determines chain order. |
| `attempt` | The attempt data being recorded |
| `data_hash` | `BLAKE3(canonical_json(attempt))` — integrity of the attempt data |
| `prev_hash` | `chain_hash` from the previous entry, or `"0"` for genesis |
| `chain_hash` | `BLAKE3(prev_hash + data_hash)` — links this entry to the chain |
| `signature` | `BLAKE3_keyed(chain_hash, derived_key)` — proves the license key holder created this |

## Cryptographic Design

### Hash algorithm: BLAKE3

All hashing uses BLAKE3 (256-bit output, 64 hex chars). Chosen because:
- Faster than SHA-256 (important for large ledgers)
- Built-in keyed hash mode (no need for separate HMAC)
- Built-in key derivation (no need for HKDF/PBKDF2)

### Three layers of protection

```
Layer 1: data_hash      ← proves attempt data hasn't been modified
Layer 2: chain_hash     ← proves ordering hasn't been changed (links to previous)
Layer 3: signature       ← proves the entry was created by the license key holder
```

### Key derivation

The license key is a human-readable string (e.g., `glw_sk_abc123`). We derive a 32-byte signing key using BLAKE3's key derivation:

```
signing_key = BLAKE3_derive_key("learnloop ledger v1", license_key_bytes)
```

The context string `"learnloop ledger v1"` ensures the derived key is domain-separated — even if the same license key is used elsewhere, the derived keys will differ.

### Signature

```
signature = BLAKE3_keyed(signing_key, chain_hash_bytes)
```

This is a MAC (Message Authentication Code), not a digital signature. It proves:
- The entry was created by someone who knows the license key
- The chain_hash hasn't been tampered with

It does NOT prove:
- The identity of the signer (any holder of the key could sign)
- Non-repudiation (the signer can't deny they signed, but we can't prove WHO signed)

For our use case (billing/compliance), this is sufficient. The license key is a shared secret between the customer and LearnLoop.

## Chain Verification

To verify a chain:

```
1. Sort entries by seq
2. Set expected_prev = "0"
3. For each entry:
   a. Recompute data_hash from attempt data → must match
   b. Check prev_hash == expected_prev → must match
   c. Recompute chain_hash from prev_hash + data_hash → must match
   d. Recompute signature using license key → must match
   e. Set expected_prev = chain_hash
4. All entries pass → chain is valid
```

If ANY check fails, the chain is broken at that point. This means:
- Modifying attempt data → data_hash mismatch
- Reordering entries → prev_hash mismatch
- Inserting/removing entries → chain breaks
- Using wrong license key → signature mismatch

## Sync Flow

```
CLI reads local .ledger files
        │
        ▼
CLI verifies chain locally (catches corruption early)
        │
        ▼
CLI asks LearnLoop API for latest known seq
        │
        ▼
CLI uploads entries with seq > latest known
        │
        ▼
LearnLoop API stores in simulation_ledger table
        │
        ▼
Background job reports to Stripe for billing
```

The API's `simulation_ledger` table mirrors the file-based chain:

```sql
simulation_ledger (
    license_id         UUID REFERENCES licenses(id),
    entry_hash         VARCHAR(64),        -- maps to chain_hash
    previous_hash      VARCHAR(64),        -- maps to prev_hash
    simulation_count   INTEGER DEFAULT 1,  -- 1 per attempt
    reported_to_stripe BOOLEAN DEFAULT FALSE,
    seq                BIGSERIAL
)
```

## Canonical JSON

When computing `data_hash`, the attempt data must be serialized deterministically. Both Glow (Python) and the CLI (Rust) must produce the same bytes for the same data.

**Canonical form:** compact JSON with sorted keys, no trailing whitespace.

**Python (Glow):**
```python
import json
canonical = json.dumps(attempt_data, sort_keys=True, separators=(",", ":"))
```

**Rust (CLI):**
```rust
let canonical = serde_json::to_string(&attempt)?;
// serde_json preserves struct field order, which is stable
```

**Important:** Both implementations must use the SAME field order. The Rust struct field order defines the canonical order. Glow's Python code must use `sort_keys=True` or explicitly match the Rust field order:

```
attempt_id → completed_at → passed → score
```

(Alphabetical via `sort_keys=True` matches the Rust struct field order if fields are defined alphabetically.)

## Implementation Checklist

### Glow (Python) — writes ledger files

- [ ] After grading an attempt, create a `.ledger` file
- [ ] Read the latest existing `.ledger` file to get `prev_hash`
- [ ] Compute `data_hash` from canonical JSON of attempt data
- [ ] Compute `chain_hash = BLAKE3(prev_hash + data_hash)`
- [ ] Derive signing key from license key: `blake3.derive_key("learnloop ledger v1", key)`
- [ ] Compute `signature = blake3.keyed_hash(signing_key, chain_hash)`
- [ ] Write `{attempt_id}.ledger` file
- [ ] Store ledger files in a configurable directory (default `./data/ledger/`)

**Python BLAKE3:** `pip install blake3`
```python
import blake3

def derive_signing_key(license_key: str) -> bytes:
    return blake3.blake3(license_key.encode(), derive_key_context="learnloop ledger v1").digest()

def hash_data(data: str) -> str:
    return blake3.blake3(data.encode()).hexdigest()

def keyed_hash(key: bytes, data: str) -> str:
    return blake3.blake3(data.encode(), key=key).hexdigest()
```

### CLI (Rust) — reads, verifies, syncs

- [x] `ledger.rs` — seal, verify, read/write chain logic
- [x] `learnloop ledger verify --path <dir>` — verify local chain
- [x] `learnloop ledger status --path <dir>` — show chain statistics
- [x] `learnloop ledger sync --path <dir>` — upload to LearnLoop API
- [x] Unit tests for all crypto primitives
- [x] End-to-end tests (write chain → read → verify)

### LearnLoop API (Python) — stores, bills

- [x] `simulation_ledger` table (exists)
- [x] `POST /usage/report` endpoint (exists)
- [x] `GET /usage/summary/{license_id}` endpoint (exists)
- [x] Stripe meter event reporting (exists)
- [ ] `POST /usage/latest` endpoint — return latest entry hash + seq for sync
- [ ] Verify chain_hash + signature server-side on report (optional, defense-in-depth)
