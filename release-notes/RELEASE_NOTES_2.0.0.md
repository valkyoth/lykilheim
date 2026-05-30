# Lykilheim 2.0.0 Release Notes

## Version

- Version: `2.0.0`
- Release date: TBD
- Git tag: `v2.0.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: sandboxed extension platform

## Scope

Lykilheim `2.0.0` is planned as the first major release where the extension
platform can become stable, if the Wasm sandbox model passes independent review.

Planned scope:

- stable capability-based Wasm adapters/plugins;
- signed plugin manifests;
- explicit host-call capabilities;
- bounded CPU fuel and memory;
- no ambient filesystem access;
- network allowlists;
- third-party auth, secret, database, cloud, and notification adapters;
- plugin conformance certification;
- plugin provenance, SBOM, revocation, pinning, upgrade, and rollback
  workflows;
- major-version review of API, storage, plugin ABI, and policy compatibility.

## Highlights

- Lykilheim becomes a secure extension platform rather than only a vault server.
- Third-party adapters can be isolated and certified through the conformance
  framework.

## Documentation

Documentation required for this release:

- Wasm plugin author guide;
- capability manifest reference;
- host-call API reference;
- plugin signing and provenance guide;
- plugin conformance guide;
- plugin upgrade and rollback guide;
- major-version migration guide.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: TBD
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Wasm sandbox escape regression tests: TBD
- Fuel/memory/filesystem/network capability tests: TBD
- Plugin signing/provenance tests: TBD
- Plugin upgrade and rollback tests: TBD
- Independent sandbox review: TBD
- Full release pentest: TBD

## Reviewed Advisory Exceptions

- TBD

## Breaking Changes

- This is a major-version release. Any API, storage, plugin ABI, or policy
  compatibility break must be documented with migration tooling or an explicit
  unsupported path.

## Upgrade Notes

- Run major-version migration checks before upgrading from `1.x`.
- Pin and verify all plugins before enabling them in production.

## Known Limitations

- Plugin stability depends on the completed sandbox and supply-chain reviews.

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Container digests: TBD
- Tag signature: TBD
