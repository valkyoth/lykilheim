# Lykilheim 1.1.0 Release Notes

## Version

- Version: `1.1.0`
- Release date: TBD
- Git tag: `v1.1.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: operator intelligence

## Scope

Lykilheim `1.1.0` is planned to improve day-two operations after the first
stable vault foundation.

Planned scope:

- secret inventory APIs for owner, engine, path, type, dynamic/static status,
  last access, lease/expiry state, rotation status, and known dependencies;
- policy simulator APIs that explain allow/deny decisions;
- dry-run mode for dangerous changes and rotations;
- local-first developer mode with safe defaults, reset, samples, generated test
  PKI, and non-production guardrails.

## Highlights

- Operators can understand access and blast radius before changing production.
- Policy debugging becomes API-driven and explainable.
- Local development becomes easier without weakening production defaults.

## Documentation

Documentation required for this release:

- inventory API guide;
- policy simulator guide;
- dry-run and blast-radius guide;
- local developer-mode guide;
- security notes for simulator and inventory visibility.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: TBD
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Inventory consistency tests: TBD
- Policy simulator tests: TBD
- Dry-run no-mutation tests: TBD
- Developer-mode smoke tests: TBD
- Pentest result: TBD

## Reviewed Advisory Exceptions

- TBD

## Breaking Changes

- None planned.

## Upgrade Notes

- Review policy simulator visibility before enabling it for non-admin users.

## Known Limitations

- Inventory dependency detection may be best-effort for external systems.
- Policy simulation is advisory and must match live enforcement tests before
  release.

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Container digests: TBD
- Tag signature: TBD
