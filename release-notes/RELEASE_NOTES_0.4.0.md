# Lykilheim 0.4.0 Release Notes

## Version

- Version: `0.4.0`
- Release date: TBD
- Git tag: `v0.4.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: token and KV preview

## Scope

Lykilheim `0.4.0` introduces useful static secret storage with tracked tokens,
leases, and KV v2 semantics.

Planned stable preview scope:

- token creation, renewal, revocation, TTL, parent/child revocation, and
  accessor lookup;
- bounded expiration manager background task;
- KV v2 versioning, soft delete, undelete, destroy, metadata, check-and-set,
  and max versions;
- initial namespace model, even if production remains single-namespace;
- API examples for init, unseal, token login, KV write, KV read, KV list, and
  KV delete.

## Highlights

- First release that can store and retrieve versioned static secrets.
- Lease cascade behavior is tested with simulated time.
- KV metadata disclosure and token replay are release-blocking pentest areas.

## Documentation

Documentation required for this release:

- token lifecycle guide;
- lease and expiration behavior;
- KV v2 API reference with `curl` examples;
- namespace model and current limits;
- automation examples for common KV workflows.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: `scripts/checks.sh`
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Token lifecycle tests: TBD
- Lease cascade tests: TBD
- KV v2 compatibility tests: TBD
- API smoke result: TBD

## Reviewed Advisory Exceptions

- TBD. List every accepted advisory with reachability and removal condition.

## Breaking Changes

- Pre-`1.0.0` token formats, KV metadata formats, and namespace behavior may
  change.

## Upgrade Notes

- Reinitialize non-production data if storage migrations are not yet available.
- Update API clients to send tokens according to the documented header rules.

## Known Limitations

- AppRole is not available yet.
- Userpass is not available yet.
- No dynamic secrets.
- No production cluster mode.

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Container digests: TBD, or not applicable
- Tag signature: TBD
