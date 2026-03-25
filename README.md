# Glow CLI

Rust-based CLI for the Glow platform. Talks to both the central LearnLoop API and individual Glow instances.

## Install

```bash
cargo build --release
```

Produces two binaries: `glow` (full) and `glw` (alias).

## Usage

```bash
# Configure
export GLOW_API_URL=https://your-instance.example.com

# Commands
glow login                     # OAuth login
glow admin network             # Connectivity check
glow personas list             # Glow instance commands
glw personas list              # Short form
```
