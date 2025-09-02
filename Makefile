.PHONY: help version current patch minor major release test build clean plan

# Default target
help:
	@echo "Available commands:"
	@echo ""
	@echo "Version Management:"
	@echo "  version          Show current version"
	@echo "  tags             List available git tags"
	@echo "  patch            Bump patch version (0.0.x)"
	@echo "  next             Bump to next patch (no tag)"
	@echo "  minor            Bump minor version (0.x.0)"
	@echo "  major            Bump major version (x.0.0)"
	@echo "  release          Create a release with current version"
	@echo ""
	@echo "Development:"
	@echo "  test             Run all tests"
	@echo "  build            Build all crates"
	@echo "  check            Run cargo check"
	@echo "  clippy           Run clippy checks"
	@echo "  fmt              Format code"
	@echo "  clean            Clean build artifacts"
	@echo "  plan             Show the current project plan"
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
	@scripts/app/version.sh current

current:
	@scripts/app/version.sh current

tags:
	@git tag --sort=-version:refname | head -10

patch:
	@scripts/app/version.sh patch

minor:
	@scripts/app/version.sh minor

major:
	@scripts/app/version.sh major

release:
	@scripts/app/version.sh release

# Development commands
test:
	cargo test --workspace
	@# Optionally run canister tests if dfx and script are available
	@if [ -x scripts/app/test.sh ] && command -v dfx >/dev/null 2>&1; then \
		echo "Running canister tests via scripts/app/test.sh"; \
		bash scripts/app/test.sh; \
	else \
		echo "Skipping canister tests (dfx not installed or script missing)"; \
	fi

build:
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

# Check versioning system
check-versioning:
	@./scripts/app/check-versioning.sh

# Check available git versions
git-versions:
	@./scripts/app/check-git-versions.sh

# Build and test everything
all: clean check fmt-check clippy test build
