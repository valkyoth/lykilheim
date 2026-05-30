#!/usr/bin/env sh
set -eu

scripts/validate-release-metadata.sh
scripts/check-doc-links.sh

if [ -f Cargo.toml ]; then
    cargo fmt --all --check
    cargo clippy --all-targets -- -D warnings
    cargo test

    if ! cargo deny --help >/dev/null 2>&1; then
        echo "ERROR: cargo-deny is required. Install with: cargo install --locked cargo-deny" >&2
        exit 1
    fi
    cargo deny check bans licenses sources

    # RustSec advisories are checked by cargo-audit so its database can stay under target/.
    if ! cargo audit --help >/dev/null 2>&1; then
        echo "ERROR: cargo-audit is required. Install with: cargo install --locked cargo-audit" >&2
        exit 1
    fi
    cargo audit --db "${CARGO_AUDIT_DB:-target/advisory-db}"
else
    echo "Cargo.toml not scaffolded yet; Rust compile/test gates start in version 0.1.0."
fi

echo "checks: ok"
