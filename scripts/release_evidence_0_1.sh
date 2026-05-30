#!/usr/bin/env sh
set -eu

version="0.1.0"
out_dir="${LYKILHEIM_EVIDENCE_DIR:-target/release-evidence/${version}}"
image="${LYKILHEIM_IMAGE:-lykilheim:0.1.0-dev}"

mkdir -p "$out_dir"

report="$out_dir/RELEASE_EVIDENCE_${version}.md"
metadata="$out_dir/cargo-metadata.json"
tree="$out_dir/cargo-tree.txt"
source_sha="$out_dir/source-inputs.sha256"
binary_sha="$out_dir/binary.sha256"
image_report="$out_dir/container-image.txt"

commit="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
git_status="$(git status --short 2>/dev/null || true)"
if [ -z "$git_status" ]; then
    git_state="clean"
else
    git_state="dirty"
fi

cargo metadata --locked --format-version 1 >"$metadata"
cargo tree --locked >"$tree"

: >"$source_sha"
for file in \
    Cargo.lock \
    Cargo.toml \
    rust-toolchain.toml \
    deny.toml \
    Containerfile \
    containers/Containerfile.wolfi \
    examples/lykilheim.toml
do
    if [ -f "$file" ]; then
        sha256sum "$file" >>"$source_sha"
    fi
done

if [ -x target/release/lykilheim ]; then
    sha256sum target/release/lykilheim >"$binary_sha"
else
    printf 'target/release/lykilheim not found; run cargo build --release --locked first.\n' >"$binary_sha"
fi

if command -v podman >/dev/null 2>&1 && podman image exists "$image" >/dev/null 2>&1; then
    {
        printf 'Image: %s\n' "$image"
        podman image inspect "$image" --format 'ID: {{.Id}}'
        podman image inspect "$image" --format 'Digest: {{.Digest}}'
        podman image inspect "$image" --format 'Created: {{.Created}}'
    } >"$image_report"
else
    printf 'No local Podman image named %s was found.\n' "$image" >"$image_report"
fi

{
    printf '# Lykilheim %s Release Evidence\n\n' "$version"
    printf '%s\n' "- Generated UTC: $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
    printf '%s\n' "- Git commit: \`$commit\`"
    printf '%s\n' "- Git status at evidence generation: \`$git_state\`"
    printf '%s\n' "- Rust: \`$(rustc --version)\`"
    printf '%s\n' "- Cargo: \`$(cargo --version)\`"
    printf '%s\n' '- Gate command: `scripts/release_0_1_gate.sh`'
    printf '%s\n' '- Podman gate command: `LYKILHEIM_RELEASE_PODMAN=1 scripts/release_0_1_gate.sh`'
    printf '\n## Files\n\n'
    printf '%s\n' "- Cargo metadata: \`$metadata\`"
    printf '%s\n' "- Cargo tree: \`$tree\`"
    printf '%s\n' "- Source input checksums: \`$source_sha\`"
    printf '%s\n' "- Release binary checksum: \`$binary_sha\`"
    printf '%s\n' "- Container image details: \`$image_report\`"
    printf '\n## Pentest Handoff\n\n'
    printf 'When the release gate passes from a clean worktree and this evidence is reviewed, freeze the 0.1.0 scope and run the focused pentest described in `docs/pentest-0.1.0.md` before tagging.\n'
} >"$report"

echo "release evidence: $report"
