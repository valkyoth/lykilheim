# Lykilheim 0.2.0 Release Notes

## Version

- Version: `0.2.0`
- Release date: TBD
- Git tag: `v0.2.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: sealed-storage preview

## Scope

Lykilheim `0.2.0` introduces sealed storage and the cryptographic barrier. The
storage layer must only handle opaque encrypted records, and all plaintext key
material must be protected with sanitizing secret wrappers.

Planned stable preview scope:

- async `Storage` trait with `get`, `put`, `delete`, and `list`;
- local development storage for single-node testing;
- sealed and unsealed lifecycle boundaries;
- Shamir initialization and unseal flow with threshold validation;
- AEAD encryption for records and authenticated metadata;
- sanitizing secret wrappers for master keys, barrier keys, unseal shares, and
  plaintext payloads.

## Highlights

- First encrypted-at-rest barrier implementation.
- Tampered storage must fail closed.
- Wrong keys, invalid shares, duplicate shares, and sealed access must be tested
  as release-blocking cases.

## Documentation

Documentation required for this release:

- sealed/unsealed lifecycle;
- init and unseal API examples;
- storage backend behavior;
- encrypted record model;
- operator guidance for protecting shares and storage paths;
- cryptographic design notes and explicit non-claims.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: `scripts/checks.sh`
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Barrier round-trip tests: TBD
- Storage tamper tests: TBD
- Restart smoke result: TBD
- Cryptographic review result: TBD

## Reviewed Advisory Exceptions

- TBD. List every accepted advisory with reachability and removal condition.

## Breaking Changes

- Pre-`1.0.0` storage formats may change. Operators should not rely on
  long-term data migration yet.

## Upgrade Notes

- Export any test data before upgrading from `0.1.0`; persistent storage is
  introduced in this release.

## Known Limitations

- Single-node storage only.
- No production token engine.
- No KV engine.
- No cluster replication.
- No auto-unseal.

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Container digests: TBD, or not applicable
- Tag signature: TBD
