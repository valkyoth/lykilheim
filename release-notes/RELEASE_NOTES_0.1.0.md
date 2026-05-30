# Lykilheim 0.1.0 Release Notes

## Version

- Version: `0.1.0`
- Release date: TBD
- Git tag: `v0.1.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: foundation preview

## Scope

Lykilheim `0.1.0` is the foundation and threat-model release. It establishes
the Rust project structure, governance, local checks, documentation standard,
and the first API shape before any secret storage is considered production
usable.

Planned stable preview scope:

- Rust crate or workspace scaffold pinned to Rust 1.96.0;
- small module boundaries for API, configuration, errors, crypto interfaces,
  storage interfaces, audit, and test support;
- public HTTP API shape for init, seal status, health, and version;
- threat model for sealed state, unsealed state, audit failure, storage
  compromise, token compromise, and plugin compromise;
- feature-parity audit that classifies Vault/OpenBao features as `1.0`,
  `Preview`, `Post-1.0`, `Research`, or `Different`;
- release metadata validation and release checklist;
- initial documentation index for architecture, API, local development, release
  process, security model, and container plans.

## Highlights

- First bounded implementation step for a Rust-native, API-driven vault.
- Documentation is part of the definition of done from the first release.
- CI uses local project checks and relies on GitHub CodeQL default setup, not an
  advanced CodeQL workflow.

## Documentation

Documentation required for this release:

- architecture overview;
- local development guide;
- API endpoint drafts with request, response, error, and `curl` examples;
- threat model;
- feature-parity audit;
- release checklist;
- container plan.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: `scripts/checks.sh`
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- API smoke result: TBD
- Cargo lockfile result: TBD
- Feature-parity audit review: TBD
- Documentation link check: TBD
- Podman smoke result: TBD, or not applicable before containers

## Reviewed Advisory Exceptions

- None expected for this bootstrap release.

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

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD, or not applicable
- SBOM checksums: TBD, or not applicable
- Container digests: TBD, or not applicable
- Tag signature: TBD
