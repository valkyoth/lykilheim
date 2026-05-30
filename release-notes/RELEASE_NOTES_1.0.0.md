# Lykilheim 1.0.0 Release Notes

## Release Metadata

- Version: `1.0.0`
- Release date: TBD
- Git tag: `v1.0.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: first stable vault foundation

## Summary

Lykilheim `1.0.0` is the first stable release target. It should ship the
smallest secure vault that is useful, documented, API-driven, and operable as a
standalone binary or rootless Wolfi container.

## Stable Scope

- API-driven init, seal status, unseal, health, version, auth, policy, token,
  KV v2, cubbyhole, response wrapping, identity, transit baseline, audit,
  storage, and backup/restore;
- standalone binary and rootless Wolfi container;
- fail-closed audit behavior;
- encrypted storage at rest behind the cryptographic barrier;
- token TTL, leases, renewal, revocation, and cascade revocation;
- AppRole and userpass baseline auth;
- rekey, root/recovery token generation, barrier key rotation, capabilities,
  mount lifecycle, and namespace-base system behavior;
- complete documentation set for installation, configuration, API usage,
  feature-parity status, security model, storage, audit, auth, identity,
  policies, tokens, leases, KV v2, cubbyhole, response wrapping, transit,
  containers, backup/restore, upgrades, troubleshooting, and release
  verification.

## Highlights

- First stable API and storage compatibility contract.
- Every stable operator workflow is documented and API-driven.
- Rootless Podman on Wolfi is a release-blocking deployment target.
- Audit failures fail closed by design.
- Release evidence includes dependency review, SBOMs, checksums, and signed tag
  verification.

## Not In Stable Scope Unless Promoted Earlier

- Production Wasm plugins.
- Production Raft HA beyond the tested stability bar.
- Production post-quantum, ZKP, TEE, eBPF, or external Merkle anchoring.
- Enterprise compatibility claims with Vault/OpenBao.
- FIPS validation claims.

## Documentation

Documentation required for this release:

- install guide;
- configuration reference;
- API reference with examples;
- operator guide;
- security model;
- feature-parity status and explicit non-claims;
- storage and backup/restore guide;
- audit guide;
- auth, identity, policy, token, lease, KV v2, cubbyhole, response wrapping,
  and transit guides;
- rootless Podman and Wolfi guide;
- upgrade and migration guide;
- troubleshooting guide;
- release verification guide;
- security disclosure process.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: TBD
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- API compatibility suite result: TBD
- Feature-parity audit review: TBD
- Storage migration suite result: TBD
- Rootless Podman Wolfi smoke result: TBD
- SBOM generation result: TBD
- Reproducible build result: TBD
- External or independent pentest result: TBD
- CodeQL/code scanning: no open release-blocking alerts before tag

## Reviewed Advisory Exceptions

List every accepted dependency advisory and why it is acceptable for this
release. Include whether the dependency is direct or transitive, whether the API
is reachable, and the removal condition.

- TBD

## Breaking Changes

- This is the first stable release. Future breaking changes require the stable
  compatibility policy documented for `1.0.0`.

## Upgrade Notes

- Upgrade paths from pre-`1.0.0` releases must be validated by the migration
  test suite before publishing.
- Operators should verify backups and run release verification before replacing
  production-like preview deployments.

## Known Limitations

- Advanced research features may be disabled or explicitly unsupported in
  production.
- Compatibility with Vault/OpenBao APIs is not claimed unless documented
  endpoint by endpoint.
- Full parity with every Vault/OpenBao auth method, secrets engine, enterprise
  feature, and agent/operator integration is not claimed for `1.0.0`.
- FIPS validation is not claimed.

## Container Images

Planned image tags after release validation:

- GitHub Container Registry: `ghcr.io/valkyoth/lykilheim:v1.0.0-wolfi`
- GitHub Container Registry: `ghcr.io/valkyoth/lykilheim:v1.0.0`
- Runtime user: `65532:65532` by default
- Default config path: `/etc/lykilheim/lykilheim.toml`
- State path: `/var/lib/lykilheim`
- Audit path: `/var/log/lykilheim`

## Checksums And Signatures

Record during the release:

- Commit: TBD
- Local gate: TBD
- CodeQL/code scanning: TBD
- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Reproducible build hash: TBD
- Container digests: TBD
- Tag signature: TBD
