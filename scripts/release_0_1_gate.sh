#!/usr/bin/env sh
set -eu

scripts/checks.sh
cargo run --quiet -- --check-config --config examples/lykilheim.toml
cargo build --release --locked
scripts/smoke_api_foundation.sh

if [ "${LYKILHEIM_RELEASE_PODMAN:-0}" = "1" ]; then
    scripts/podman_smoke_0_1.sh
else
    echo "release 0.1 gate: set LYKILHEIM_RELEASE_PODMAN=1 to run rootless Podman smoke"
fi

echo "release 0.1 gate: ok"
