# LearnLoop CLI — Development Makefile
#
# Quick reference:
#   make          → build debug
#   make run      → build + run (pass ARGS="glow personas list")
#   make test     → run tests
#   make check    → fmt + clippy + test (same as CI)
#   make release  → optimized build
#   make install  → install to ~/.cargo/bin

.PHONY: build run test fmt clippy check release install clean watch help

# Default target
build:
	cargo build

# Run with arguments: make run ARGS="glow personas list"
ARGS ?= --help
run:
	cargo run -- $(ARGS)

# Run all tests
test:
	cargo test

# Auto-format code
fmt:
	cargo fmt

# Lint
clippy:
	cargo clippy -- -D warnings

# Run everything CI runs (format check + lint + test)
check: fmt clippy test

# Optimized release build
release:
	cargo build --release
	@echo "\nBinaries:"
	@ls -lh target/release/learnloop target/release/ll

# Install both binaries to ~/.cargo/bin
install:
	cargo install --path .

# Remove build artifacts
clean:
	cargo clean

# Auto-rebuild on file changes (requires: cargo install cargo-watch)
watch:
	cargo watch -x run

help:
	@echo "Available targets:"
	@echo "  build    Build debug binary"
	@echo "  run      Build + run (use ARGS=\"...\" to pass arguments)"
	@echo "  test     Run tests"
	@echo "  fmt      Auto-format code"
	@echo "  clippy   Run linter"
	@echo "  check    fmt + clippy + test (mirrors CI)"
	@echo "  release  Optimized release build"
	@echo "  install  Install to ~/.cargo/bin"
	@echo "  clean    Remove build artifacts"
	@echo "  watch    Auto-rebuild on file changes (needs cargo-watch)"
