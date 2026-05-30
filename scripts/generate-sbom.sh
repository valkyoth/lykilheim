#!/usr/bin/env sh
set -eu

out_dir="${LYKILHEIM_SBOM_DIR:-target/sbom}"
mkdir -p "$out_dir"

cargo metadata --locked --format-version 1 >"$out_dir/cargo-metadata.json"
cargo tree --locked >"$out_dir/cargo-tree.txt"
sha256sum "$out_dir/cargo-metadata.json" "$out_dir/cargo-tree.txt" >"$out_dir/SHA256SUMS"

echo "sbom: $out_dir"
