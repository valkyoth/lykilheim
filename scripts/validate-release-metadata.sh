#!/usr/bin/env sh
set -eu

if ! grep -q '^# Lykilheim$' README.md; then
    echo "release metadata: README.md must identify Lykilheim" >&2
    exit 1
fi

if ! grep -q 'EUROPEAN UNION PUBLIC LICENCE v. 1.2' LICENSE; then
    echo "release metadata: LICENSE does not look like EUPL 1.2" >&2
    exit 1
fi

if ! grep -q '^channel = "1.96.1"$' rust-toolchain.toml; then
    echo "release metadata: rust-toolchain.toml must pin Rust 1.96.1" >&2
    exit 1
fi

if ! grep -q 'GitHub CodeQL default setup only' docs/version-plan.md; then
    echo "release metadata: version plan must require CodeQL default setup only" >&2
    exit 1
fi

if ! grep -q 'rootless Podman on Wolfi' docs/version-plan.md; then
    echo "release metadata: version plan must include rootless Podman on Wolfi" >&2
    exit 1
fi

if [ ! -f docs/feature-parity.md ]; then
    echo "release metadata: missing docs/feature-parity.md" >&2
    exit 1
fi

for parity_heading in \
    "Core Request Flow" \
    "Seal, Key Lifecycle, And Storage" \
    "Audit, Logs, Telemetry, And Operations" \
    "Auth Methods" \
    "Identity, Policies, And Governance" \
    "Secrets Engines" \
    "Replication, HA, And Multi-Cluster" \
    "Client And Platform Integrations"; do
    if ! grep -q "^## $parity_heading$" docs/feature-parity.md; then
        echo "release metadata: feature parity audit missing $parity_heading" >&2
        exit 1
    fi
done

if ! grep -q '^## 1.0.0 - First Stable Release$' docs/version-plan.md; then
    echo "release metadata: version plan must include the 1.0.0 target" >&2
    exit 1
fi

if ! grep -q 'STOP' docs/version-plan.md; then
    echo "release metadata: version plan must include STOP gates" >&2
    exit 1
fi

for version in 0.1.0 0.2.0 0.3.0 0.4.0 0.5.0 0.6.0 0.7.0 0.8.0 0.9.0 0.10.0 1.0.0 1.1.0 1.2.0 1.3.0 1.4.0 1.5.0 2.0.0; do
    notes="release-notes/RELEASE_NOTES_${version}.md"
    if [ ! -f "$notes" ]; then
        echo "release metadata: missing $notes" >&2
        exit 1
    fi
    if ! grep -q "Lykilheim ${version} Release Notes" "$notes"; then
        echo "release metadata: $notes has the wrong title" >&2
        exit 1
    fi
    if ! grep -q '## Security And Stability Gate' "$notes"; then
        echo "release metadata: $notes must include a security gate section" >&2
        exit 1
    fi
    if ! grep -q '## Checksums And Signatures' "$notes"; then
        echo "release metadata: $notes must include checksums and signatures" >&2
        exit 1
    fi
done

if [ -f Cargo.toml ]; then
    cargo_version="$(
        sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | sed -n '1p'
    )"

    if [ -z "$cargo_version" ]; then
        echo "release metadata: Cargo.toml package version is missing" >&2
        exit 1
    fi

    if ! grep -q '^rust-version = "1.96"$' Cargo.toml; then
        echo "release metadata: Cargo.toml must declare rust-version = \"1.96\"" >&2
        exit 1
    fi

    if ! grep -q '^license = "EUPL-1.2"$' Cargo.toml; then
        echo "release metadata: Cargo.toml must declare license = \"EUPL-1.2\"" >&2
        exit 1
    fi

    if ! grep -q "^## $cargo_version " CHANGELOG.md; then
        echo "release metadata: CHANGELOG.md is missing a section for Cargo version $cargo_version" >&2
        exit 1
    fi

    if [ ! -f "release-notes/RELEASE_NOTES_${cargo_version}.md" ]; then
        echo "release metadata: missing release notes for Cargo version $cargo_version" >&2
        exit 1
    fi

    if ! grep -q '^ARG RUST_IMAGE=docker.io/library/rust:1.96.1-' Containerfile; then
        echo "release metadata: Containerfile must use Rust 1.96.1" >&2
        exit 1
    fi

    if ! grep -q '^ARG RUST_IMAGE=docker.io/library/rust:1.96.1-' containers/Containerfile.wolfi; then
        echo "release metadata: Wolfi Containerfile must use Rust 1.96.1" >&2
        exit 1
    fi
fi

echo "release metadata: ok"
