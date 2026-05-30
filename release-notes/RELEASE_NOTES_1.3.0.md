# Lykilheim 1.3.0 Release Notes

## Version

- Version: `1.3.0`
- Release date: TBD
- Git tag: `v1.3.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: adapter certification

## Scope

Lykilheim `1.3.0` is planned to make database, cache, service, and cloud
adapters testable and certifiable through a common contract.

Planned scope:

- adapter conformance framework;
- certification metadata for native and future Wasm adapters;
- stable PostgreSQL adapter conformance;
- MySQL/MariaDB adapter conformance;
- experimental tracks for MongoDB, Redis/Valkey, SurrealDB, RabbitMQ, AWS,
  Azure, GCP, Hetzner, and DigitalOcean where upstream APIs allow safe
  credential lifecycle management;
- adapter capability discovery API.

## Highlights

- Adapters must prove create, renew, revoke, rotate, failure, idempotency, and
  audit behavior.
- Operators can inspect adapter capabilities before enabling roles.

## Documentation

Documentation required for this release:

- adapter conformance guide;
- adapter certification metadata reference;
- PostgreSQL and MySQL/MariaDB adapter guides;
- experimental adapter status matrix;
- capability discovery API reference.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: TBD
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Generic adapter conformance result: TBD
- Local Podman adapter smoke result: TBD
- Failure injection tests: TBD
- Certification metadata validation: TBD
- Pentest result: TBD

## Reviewed Advisory Exceptions

- TBD

## Breaking Changes

- None planned for stable adapters.

## Upgrade Notes

- Re-run adapter conformance before promoting any experimental adapter to
  production.

## Known Limitations

- Some public cloud providers may not support every lifecycle operation needed
  for full dynamic credentials.

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Container digests: TBD
- Tag signature: TBD
