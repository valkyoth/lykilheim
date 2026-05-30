# Lykilheim 1.2.0 Release Notes

## Version

- Version: `1.2.0`
- Release date: TBD
- Git tag: `v1.2.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: leak response and rotation readiness

## Scope

Lykilheim `1.2.0` is planned to connect secret leak findings to auditable
rotation, revocation, and notification workflows.

Planned scope:

- secret leak intake API for scanners and CI systems;
- leak correlation with managed static secrets, dynamic leases, AppRole
  SecretIDs, tokens, transit keys, and adapter credentials where safe;
- rotation readiness scores for supported secrets and roles;
- leak-to-rotation workflows for supported engines and adapters;
- signed lifecycle webhooks for create, read, rotate, lease-expiring, revoke,
  leak-reported, and policy-denied events.

## Highlights

- Leak findings become actionable instead of living only in scanner reports.
- Operators can see which credentials can rotate automatically.
- SIEM and automation systems can subscribe to signed lifecycle events.

## Documentation

Documentation required for this release:

- leak intake API reference;
- rotation readiness guide;
- leak-to-rotation runbook;
- lifecycle webhook guide;
- privacy and evidence-handling notes.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: TBD
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Leak intake validation tests: TBD
- Correlation privacy tests: TBD
- Rotation readiness matrix tests: TBD
- Webhook signing and retry tests: TBD
- Pentest result: TBD

## Reviewed Advisory Exceptions

- TBD

## Breaking Changes

- None planned.

## Upgrade Notes

- Configure webhook receivers and signing keys before enabling lifecycle
  delivery in production.

## Known Limitations

- Leak correlation should avoid storing raw leaked values.
- Not every static secret can rotate automatically.

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Container digests: TBD
- Tag signature: TBD
