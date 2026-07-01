# Lykilheim 0.1.0 Release Notes

## Version

- Version: `0.1.0`
- Release date: 2026-05-30
- Git tag: `v0.1.0`
- Git commit: signed tag target for `v0.1.0`
- License: EUPL-1.2
- Release type: foundation preview

## Scope

Lykilheim `0.1.0` is the foundation and threat-model release. It establishes
the Rust project structure, governance, local checks, documentation standard,
and the first API shape before any secret storage is considered production
usable.

Planned stable preview scope:

- Rust crate or workspace scaffold pinned to Rust 1.96.1;
- small module boundaries for API, configuration, errors, crypto interfaces,
  storage interfaces, audit, and test support;
- public HTTP API shape for init, seal status, health, and version;
- threat model for sealed state, unsealed state, audit failure, storage
  compromise, token compromise, and plugin compromise;
- feature-parity audit that classifies Vault/OpenBao features as `1.0`,
  `Preview`, `Post-1.0`, `Research`, or `Different`;
- release metadata validation and release checklist;
- release evidence generation under `target/release-evidence/0.1.0/`;
- native release binary builder for Linux, macOS, Windows, and BSD hosts;
- focused pentest handoff for the `0.1.0` STOP gate;
- portability policy for Linux, macOS, Windows, BSD-style Unix systems, and
  Linux-only Wolfi containers;
- initial documentation index for architecture, API, local development, release
  process, security model, and container plans.

## Highlights

- First bounded implementation step for a Rust-native, API-driven vault.
- Documentation is part of the definition of done from the first release.
- CI uses local project checks and relies on GitHub CodeQL default setup, not an
  advanced CodeQL workflow.
- Pentest follow-up tightened the foundation API and release process before
  tag: zeroizing crypto interface contracts, sanitized unimplemented errors,
  explicit request body limits, baseline security headers, per-IP token-bucket
  rate limiting with `Retry-After`, stricter storage-key validation, pinned
  container base images, pinned checkout action SHAs, pinned CI security tool
  versions, safer smoke-test temporary files, and required local security
  tooling.

## Documentation

Documentation required for this release:

- architecture overview;
- local development guide;
- API endpoint drafts with request, response, error, and `curl` examples;
- threat model;
- feature-parity audit;
- release checklist;
- container plan;
- portability policy;
- release binary build guide;
- pentest handoff.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: `scripts/release_0_1_gate.sh`
- Gate report directory: `target/release-evidence/0.1.0/`
- Result: passed before signed tag.
- `cargo audit` RustSec advisory result: passed with no vulnerabilities found.
- `cargo deny check bans licenses sources` result: passed.
- API smoke result: passed.
- Rootless Podman release gate: passed with `LYKILHEIM_RELEASE_PODMAN=1`.
- Cargo lockfile result: passed with `--locked` builds.
- Feature-parity audit review: reviewed for the foundation scope.
- Documentation link check: passed through `scripts/checks.sh`.
- Podman smoke result: passed.
- Focused pentest result: follow-up findings addressed; no open 0.1.0
  release blockers before tag.

## Reviewed Advisory Exceptions

- `zmij 1.0.21` was reviewed after pentest flagged it as unfamiliar. It is a
  legitimate crates.io dependency used by `serde_json 1.0.150`; repository:
  <https://github.com/dtolnay/zmij>. `cargo owner --list zmij` reports
  `dtolnay (David Tolnay)`, `cargo owner --list serde_core` reports
  `dtolnay (David Tolnay)` and `github:serde-rs:publish`, and
  `cargo verify-project` succeeds. No exception is required.

## Breaking Changes

- This is a pre-`1.0.0` preview release. API, config, crate layout, and storage
  choices may change before stable.

## Upgrade Notes

- No previous Lykilheim release exists.

## Known Limitations

- No secret storage.
- No token engine.
- No cryptographic barrier.
- No auth engines.
- No production container image.
- No native TLS termination; keep the preview API on loopback or behind a
  trusted TLS-terminating proxy.

## Checksums And Signatures

Record during the release:

- Source archive checksum: record from the generated `v0.1.0` release asset.
- Binary checksums: record per uploaded native binary artifact.
- Native binary artifacts: build with `scripts/build_release_binary.py` on each
  target OS that publishes an asset. Artifact names should use the package
  version, OS label, and architecture, for example
  `lykilheim-0.1.0-linux-x86_64.tar.gz`.
- SBOM checksums: record from `scripts/generate-sbom.sh` output if SBOM files
  are published.
- Container digests: record from `target/release-evidence/0.1.0/container-image.txt`
  if a preview image is published.
- Tag signature: signed tag `v0.1.0`.
