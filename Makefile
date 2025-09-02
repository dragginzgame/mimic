.PHONY: help version current tags patch minor major release test build check clippy fmt fmt-check clean plan install-dev test-watch all ensure-clean

# Check for clean git state
ensure-clean:
	@if ! git diff-index --quiet HEAD --; then \
		echo "🚨 Working directory not clean! Please commit or stash your changes."; \
		exit 1; \
	fi

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
	@echo "  release          CI-driven release (local target is no-op)"
	@echo ""
	@echo "Development:"
	@echo "  test             Run all tests"
	@echo "  build            Build all crates"
	@echo "  check            Run cargo check"
	@echo "  clippy           Run clippy checks"
	@echo "  fmt              Format code"
	@echo "  fmt-check        Check formatting"
	@echo "  clean            Clean build artifacts"
	@echo "  plan             Show the current project plan"
	@echo ""
	@echo "Utilities:"
	@echo "  install-dev      Install development dependencies"
	@echo "  test-watch       Run tests in watch mode"
	@echo "  all              Run all checks, tests, and build"
	@echo ""
	@echo "Examples:"
	@echo "  make patch       # Bump patch version"
	@echo "  make test        # Run tests"
	@echo "  make build       # Build project"

# Version management
version:
	@scripts/app/version.sh current

current:
	@scripts/app/version.sh current

tags:
	@git tag --sort=-version:refname | head -10

patch: ensure-clean
	@scripts/app/version.sh patch

minor: ensure-clean
	@scripts/app/version.sh minor

major: ensure-clean
	@scripts/app/version.sh major

release: ensure-clean
	@echo "Release handled by CI on tag push"

# Development commands
test: ensure-clean
	cargo test --workspace
	@if [ -x scripts/app/test.sh ] && command -v dfx >/dev/null 2>&1; then \
		echo "Running canister tests via scripts/app/test.sh"; \
		bash scripts/app/test.sh; \
	else \
		echo "Skipping canister tests (dfx not installed or script missing)"; \
	fi

build: ensure-clean
	cargo build --release --workspace

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

# Planning summary
plan:
	@echo "=== PLAN.md ==="
	@{ [ -f PLAN.md ] && sed -n '1,200p' PLAN.md; } || echo "No PLAN.md found."
	@echo
	@echo "=== .codex/plan.json ==="
	@{ [ -f .codex/plan.json ] && sed -n '1,200p' .codex/plan.json; } || echo "No .codex/plan.json found."

# Install development dependencies
install-dev:
	cargo install cargo-watch

# Run tests in watch mode
test-watch:
	cargo watch -x test

# Build and test everything
all: ensure-clean clean check fmt-check clippy test build
