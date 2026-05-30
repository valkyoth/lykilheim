# Lykilheim 0.9.0 Release Notes

## Version

- Version: `0.9.0`
- Release date: TBD
- Git tag: `v0.9.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: plugin and dynamic secrets preview

## Scope

Lykilheim `0.9.0` introduces tightly constrained extensibility and a dynamic
secrets preview without making third-party plugins part of the trusted core.

Planned stable preview scope:

- `SecretEngine` and `AuthEngine` host traits;
- native development dynamic secrets engine for PostgreSQL or a fake SQL target
  used in tests;
- Wasmtime plugin prototype with fuel limits, memory limits, no ambient
  filesystem access, and explicit outbound capability injection;
- plugin signing and verification design;
- preview/experimental status for plugins until sandbox review is complete.

## Highlights

- First dynamic secrets preview.
- Plugin execution is resource-limited and capability-based.
- Sandbox escape, plugin supply chain, and denial-of-service behavior are
  release-blocking pentest areas.

## Documentation

Documentation required for this release:

- engine trait overview;
- dynamic secrets lifecycle guide;
- plugin capability model;
- Wasmtime resource limit behavior;
- plugin signing design;
- clear production warning for experimental plugin support.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: `scripts/checks.sh`
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Plugin fuel exhaustion tests: TBD
- Capability denial tests: TBD
- Dynamic lease revocation tests: TBD
- Plugin boundary fuzz result: TBD
- Sandbox review result: TBD

## Reviewed Advisory Exceptions

- TBD. List every accepted advisory with reachability and removal condition.

## Breaking Changes

- Plugin ABI and dynamic secret APIs are experimental and may change before
  `1.0.0`.

## Upgrade Notes

- Rebuild experimental plugins against the exact Lykilheim release.
- Recheck plugin capabilities after upgrade.

## Known Limitations

- Plugin support is not stable.
- Only one development dynamic engine may exist.
- Plugin signing may be design-only in this preview.
- External network capabilities are intentionally narrow.

## Container Images

Planned image tags after release validation:

- GitHub Container Registry: `ghcr.io/valkyoth/lykilheim:v0.9.0-wolfi`
- Runtime user: `65532:65532` by default
- Default config path: `/etc/lykilheim/lykilheim.toml`
- State path: `/var/lib/lykilheim`
- Audit path: `/var/log/lykilheim`

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Container digests: TBD
- Tag signature: TBD
