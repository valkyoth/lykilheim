# Lykilheim 0.3.0 Release Notes

## Version

- Version: `0.3.0`
- Release date: TBD
- Git tag: `v0.3.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: API-router and audit preview

## Scope

Lykilheim `0.3.0` makes every request pass through the same control points:
versioned API routing, request normalization, authorization skeleton, and
fail-closed audit dispatch.

Planned stable preview scope:

- axum API routing under versioned paths;
- request IDs, structured errors, body limits, and method/path allowlists;
- mount table and radix or prefix routing for auth and secrets engines;
- fail-closed audit interface with at least one durable local audit device;
- deterministic policy data model and path capability checks;
- token metadata structs without full production token issuance.

## Highlights

- First release where audit failure blocks requests.
- Policy behavior is default deny.
- Route normalization and body limits become release-blocking security tests.

## Documentation

Documentation required for this release:

- API routing and path normalization rules;
- structured error format;
- audit device configuration and failure semantics;
- policy model with allow, deny, wildcard, and namespace examples;
- security notes for log redaction.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: `scripts/checks.sh`
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- API routing tests: TBD
- Audit fail-closed tests: TBD
- Policy matrix tests: TBD
- Log redaction review: TBD

## Reviewed Advisory Exceptions

- TBD. List every accepted advisory with reachability and removal condition.

## Breaking Changes

- Pre-`1.0.0` API paths and policy syntax may change.

## Upgrade Notes

- Recheck API clients against the versioned paths and structured error format.
- Recheck audit configuration because requests should fail if all audit sinks
  are unavailable.

## Known Limitations

- Token issuance is still incomplete.
- KV storage is not available.
- Auth engines are not available.
- Audit backends are intentionally minimal.

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Container digests: TBD, or not applicable
- Tag signature: TBD
