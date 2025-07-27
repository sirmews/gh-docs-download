# GitHub Documentation Download Tool Makefile

# Default target
.PHONY: all
all: build

# Build the project
.PHONY: build
build:
	cargo build

# Build in release mode
.PHONY: release
release:
	cargo build --release

# Run the tool with example repository using tree URL
.PHONY: test
test: build
	./target/debug/gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs/router/eslint" --list-only

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# Install the tool locally
.PHONY: install
install:
	cargo install --path .

# Run clippy for linting with comprehensive coverage and strict warnings
.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Format code
.PHONY: format
format:
	cargo fmt

# Check formatting
.PHONY: check-format
check-format:
	cargo fmt --check

# Run unit tests
.PHONY: test-unit
test-unit:
	cargo test

# Run documentation tests
.PHONY: test-doc
test-doc:
	cargo test --doc

# Generate documentation
.PHONY: docs
docs:
	cargo doc --open

# Run all checks (format, lint, build, tests)
.PHONY: check
check: check-format lint build test-unit test-doc

# Check if ready for publishing (dry run)
.PHONY: publish-check
publish-check: check
	@echo "Running publish dry-run to check for issues..."
	cargo publish --dry-run --allow-dirty

# Show current version
.PHONY: version
version:
	@echo "Current version:"
	@grep "^version" Cargo.toml

# Publish to crates.io (requires confirmation)
.PHONY: publish
publish: publish-check
	@echo "WARNING: This will publish to crates.io!"
	@echo "Current version: $$(grep '^version' Cargo.toml | cut -d'"' -f2)"
	@read -p "Are you sure you want to publish? (y/N): " confirm && [ "$$confirm" = "y" ]
	cargo publish --allow-dirty

# Show help
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build        - Build the project in debug mode"
	@echo "  release      - Build the project in release mode"
	@echo "  test         - Build and run with example repository"
	@echo "  clean        - Clean build artifacts"
	@echo "  install      - Install the tool locally"
	@echo "  lint         - Run clippy for linting with strict warnings"
	@echo "  format       - Format code with rustfmt"
	@echo "  check-format - Check if code is properly formatted"
	@echo "  test-unit    - Run unit tests"
	@echo "  test-doc     - Run documentation tests"
	@echo "  docs         - Generate and open documentation"
	@echo "  check        - Run all checks (format, lint, build, tests)"
	@echo "  publish-check- Check if ready for publishing (dry run)"
	@echo "  version      - Show current version"
	@echo "  publish      - Publish to crates.io (with confirmation)"
	@echo "  help         - Show this help message"