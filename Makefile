.PHONY: help version current patch minor major release test build clean

# Default target
help:
	@echo "Available commands:"
	@echo ""
	@echo "Version Management:"
	@echo "  version          Show current version"
	@echo "  tags             List available git tags"
	@echo "  patch            Bump patch version (0.0.x)"
	@echo "  minor            Bump minor version (0.x.0)"
	@echo "  major            Bump major version (x.0.0)"
	@echo "  release          Create a release with current version"
	@echo ""
	@echo "Development:"
	@echo "  test             Run all tests"
	@echo "  test-wasm        Run tests for wasm target"
	@echo "  build            Build all crates"
	@echo "  build-wasm       Build for wasm target"
	@echo "  check            Run cargo check"
	@echo "  clippy           Run clippy checks"
	@echo "  fmt              Format code"
	@echo "  clean            Clean build artifacts"
	@echo ""
	@echo "Utilities:"
	@echo "  check-versioning Check versioning system setup"
	@echo "  git-versions     Check available git dependency versions"
	@echo "  security-check   Check tag immutability and version integrity"
	@echo ""
	@echo "Examples:"
	@echo "  make patch       # Bump patch version"
	@echo "  make test        # Run tests"
	@echo "  make build       # Build project"

# Version management
version:
	@./scripts/app/version.sh current

current:
	@./scripts/app/version.sh current

tags:
	@./scripts/app/version.sh tags

patch:
	@./scripts/app/version.sh patch

minor:
	@./scripts/app/version.sh minor

major:
	@./scripts/app/version.sh major

release:
	@./scripts/app/version.sh release

# Development commands
test:
	cargo test --workspace

test-wasm:
	cargo test --workspace --target wasm32-unknown-unknown

build:
	cargo build --release --workspace

build-wasm:
	cargo build --release --workspace --target wasm32-unknown-unknown

check:
	cargo check --workspace

clippy:
	cargo clippy --workspace -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clean:
	cargo clean
	rm -rf target/

# Install development dependencies
install-dev:
	rustup target add wasm32-unknown-unknown
	cargo install cargo-watch

# Run tests in watch mode
test-watch:
	cargo watch -x test

# Check versioning system
check-versioning:
	@./scripts/app/check-versioning.sh

# Check available git versions
git-versions:
	@./scripts/app/check-git-versions.sh

# Security check for tag immutability
security-check:
	@./scripts/app/security-check.sh

# Build and test everything
all: clean check fmt-check clippy test test-wasm build build-wasm 