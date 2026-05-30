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

if ! grep -q '^channel = "1.96.0"$' rust-toolchain.toml; then
    echo "release metadata: rust-toolchain.toml must pin Rust 1.96.0" >&2
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

if ! grep -q '^## 1.0.0 - First Stable Release$' docs/version-plan.md; then
    echo "release metadata: version plan must include the 1.0.0 target" >&2
    exit 1
fi

if ! grep -q 'STOP' docs/version-plan.md; then
    echo "release metadata: version plan must include STOP gates" >&2
    exit 1
fi

if [ -f Cargo.toml ]; then
    if ! grep -q '^rust-version = "1.96"$' Cargo.toml; then
        echo "release metadata: Cargo.toml must declare rust-version = \"1.96\"" >&2
        exit 1
    fi

    if ! grep -q '^license = "EUPL-1.2"$' Cargo.toml; then
        echo "release metadata: Cargo.toml must declare license = \"EUPL-1.2\"" >&2
        exit 1
    fi
fi

echo "release metadata: ok"
