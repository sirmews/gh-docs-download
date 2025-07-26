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

# Run the tool with example repository
.PHONY: test
test: build
	./target/debug/gh-docs-download --repo rust-lang/rust --list-only

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# Install the tool locally
.PHONY: install
install:
	cargo install --path .

# Run clippy for linting with strict warnings
.PHONY: lint
lint:
	cargo clippy -- -D warnings

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
	@echo "  help         - Show this help message"