# LearnLoop CLI

Rust-based CLI for the LearnLoop platform. Talks to both the central LearnLoop API and individual Glow instances.

## Install

```bash
cargo build --release
```

Produces two binaries: `learnloop` (full) and `ll` (alias).

## Usage

```bash
# Configure
export LEARNLOOP_LICENSE_KEY=your-key
export GLOW_API_URL=https://your-instance.example.com

# Commands
learnloop login                     # OAuth login
learnloop network                   # Connectivity check
learnloop ledger status             # Audit ledger
learnloop glow personas list        # Glow instance commands
ll glow personas list               # Short form
```
