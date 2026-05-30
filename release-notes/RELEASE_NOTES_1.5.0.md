# Lykilheim 1.5.0 Release Notes

## Version

- Version: `1.5.0`
- Release date: TBD
- Git tag: `v1.5.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: tamper-evident operations

## Scope

Lykilheim `1.5.0` is planned to make audit and incident evidence easier to
verify after an incident.

Planned scope:

- tamper-evident audit bundles with hash chaining and signed checkpoints;
- exportable evidence bundles for incident response, break-glass sessions, leak
  response, adapter rotations, and policy changes;
- optional external checkpoint publishing to operator-controlled storage;
- audit verification tooling and APIs;
- lifecycle event replay APIs for SIEM and compliance workflows.

## Highlights

- Audit trails can be verified for deletion, reordering, truncation, and
  mutation.
- Incident evidence can be exported and archived independently.

## Documentation

Documentation required for this release:

- tamper-evident audit design;
- evidence bundle export and verification guide;
- checkpoint publishing guide;
- lifecycle event replay API reference;
- incident response examples.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: TBD
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Hash-chain verification tests: TBD
- Bundle signing tests: TBD
- Tamper detection tests: TBD
- Evidence export/import smoke: TBD
- Pentest result: TBD

## Reviewed Advisory Exceptions

- TBD

## Breaking Changes

- None planned.

## Upgrade Notes

- Configure signing key storage and checkpoint destinations before enabling
  external publishing.

## Known Limitations

- Tamper-evident bundles prove mutation after checkpointing; they do not replace
  secure log storage and access control.

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Container digests: TBD
- Tag signature: TBD
