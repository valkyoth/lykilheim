# Lykilheim 0.10.0 Release Notes

## Version

- Version: `0.10.0`
- Release date: TBD
- Git tag: `v0.10.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: advanced security incubation preview

## Scope

Lykilheim `0.10.0` prepares advanced security work without blocking the first
stable release on research risk.

Planned stable preview scope:

- feature-gated research tracks for TEE attestation, post-quantum hybrid
  transport, ZKP authentication, Merkle audit anchoring, and eBPF audit export;
- documented stable, beta, experimental, and research boundaries;
- compatibility policy for API, storage format, audit format, and plugin ABI;
- migration framework for storage and policy data;
- post-`1.0.0` parity designs for auth methods, secrets engines, system backend
  areas, enterprise-style governance, secret sync, auto-unseal, and
  agent/proxy integrations.

## Highlights

- The 1.0 compatibility contract is frozen after this release.
- Advanced features are explicitly classified instead of silently becoming
  production promises.
- Storage migration tests must cover every previous pre-release format.

## Documentation

Documentation required for this release:

- feature status matrix for stable, beta, experimental, and research work;
- compatibility policy;
- post-`1.0.0` parity roadmap;
- migration guide;
- advanced security design notes and non-claims;
- operator guidance for disabling research features in production.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: `scripts/checks.sh`
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Research feature compile gates: TBD
- Audit hash-chain verification tests: TBD
- Migration round-trip tests: TBD
- 1.0 compatibility review: TBD
- Post-1.0 parity roadmap review: TBD

## Reviewed Advisory Exceptions

- TBD. List every accepted advisory with reachability and removal condition.

## Breaking Changes

- This is the final planned pre-`1.0.0` release where compatibility-breaking
  changes may be accepted without a stable migration promise.

## Upgrade Notes

- Read the compatibility policy before upgrading from earlier previews.
- Run migration dry-runs against non-production snapshots before upgrading real
  data.

## Known Limitations

- TEE, PQC, ZKP, Merkle anchoring, and eBPF work may remain research-gated.
- Production readiness is limited to features promoted by the 1.0 compatibility
  review.

## Container Images

Planned image tags after release validation:

- GitHub Container Registry: `ghcr.io/valkyoth/lykilheim:v0.10.0-wolfi`
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
