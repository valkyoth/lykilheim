#!/usr/bin/env sh
set -eu

scripts/validate-release-metadata.sh
scripts/check-doc-links.sh

if [ -f Cargo.toml ]; then
    cargo fmt --all --check
    cargo clippy --all-targets -- -D warnings
    cargo test

    if cargo deny --version >/dev/null 2>&1; then
        cargo deny check
    else
        echo "cargo-deny not installed; skipping cargo deny check for this bootstrap tree."
    fi

    if cargo audit --version >/dev/null 2>&1; then
        cargo audit
    else
        echo "cargo-audit not installed; skipping cargo audit for this bootstrap tree."
    fi
else
    echo "Cargo.toml not scaffolded yet; Rust compile/test gates start in version 0.1.0."
fi

echo "checks: ok"
