# Glow CLI — Development Makefile
#
# Quick reference:
#   make          → build debug
#   make run      → build + run (pass ARGS="personas list")
#   make test     → run all tests
#   make check    → fmt + clippy + test (same as CI)
#   make release  → optimized build
#   make install  → install to ~/.cargo/bin

.PHONY: build run test test-unit test-integration fmt clippy check release install clean watch coverage help

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

# Run only unit tests (inline #[cfg(test)] modules)
test-unit:
	cargo test --lib

# Run only integration tests (tests/ directory)
test-integration:
	cargo test --test cli_integration

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
	@ls -lh target/release/glow target/release/glw

# Install both binaries to ~/.cargo/bin
install:
	cargo install --path .

# Remove build artifacts
clean:
	cargo clean

# Auto-rebuild on file changes (requires: cargo install cargo-watch)
watch:
	cargo watch -x run

# Test coverage (requires: cargo install cargo-llvm-cov && rustup component add llvm-tools-preview)
coverage:
	cargo llvm-cov --open

coverage-ci:
	cargo llvm-cov --lcov --output-path lcov.info

help:
	@echo "Available targets:"
	@echo "  build            Build debug binary"
	@echo "  run              Build + run (use ARGS=\"...\" to pass arguments)"
	@echo "  test             Run all tests"
	@echo "  test-unit        Run unit tests only"
	@echo "  test-integration Run integration tests only"
	@echo "  fmt              Auto-format code"
	@echo "  clippy           Run linter"
	@echo "  check            fmt + clippy + test (mirrors CI)"
	@echo "  release          Optimized release build"
	@echo "  install          Install to ~/.cargo/bin"
	@echo "  clean            Remove build artifacts"
	@echo "  watch            Auto-rebuild on file changes (needs cargo-watch)"
	@echo "  coverage         Run tests with coverage report (needs cargo-llvm-cov)"
