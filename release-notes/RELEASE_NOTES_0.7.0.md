# Lykilheim 0.7.0 Release Notes

## Version

- Version: `0.7.0`
- Release date: TBD
- Git tag: `v0.7.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: operations and container preview

## Scope

Lykilheim `0.7.0` makes the project operable as both a standalone binary and a
rootless Podman Wolfi service.

Planned stable preview scope:

- minimal Wolfi runtime image with non-root UID/GID;
- example configs for local dev, single-node sealed storage, and API-only
  operation;
- systemd/user service documentation for rootless Podman;
- encrypted backup and restore API;
- metrics and readiness endpoints that never expose secret material.

## Highlights

- Rootless Podman on Wolfi becomes a first-class release gate.
- Backup and restore are available through API-driven workflows.
- Container permissions and snapshot handling are release-blocking pentest
  areas.

## Documentation

Documentation required for this release:

- build-and-podman guide;
- rootless Podman service guide;
- Wolfi image usage;
- volume ownership and permission guide;
- backup/restore guide;
- metrics and readiness endpoint reference;
- upgrade flow for container deployments.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: `scripts/checks.sh`
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Rootless Podman smoke result: TBD
- Container permission tests: TBD
- Snapshot restore smoke result: TBD
- Metrics leakage review: TBD

## Reviewed Advisory Exceptions

- TBD. List every accepted advisory with reachability and removal condition.

## Breaking Changes

- Container paths and image tags are preview until `1.0.0`.

## Upgrade Notes

- Review mounted storage and audit volume ownership before replacing preview
  containers.
- Test encrypted snapshot restore before upgrading production-like data.

## Known Limitations

- No production multi-node mode.
- No auto-unseal.
- No Kubernetes operator.
- Container image set may be limited to Wolfi.

## Container Images

Planned image tags after release validation:

- GitHub Container Registry: `ghcr.io/valkyoth/lykilheim:v0.7.0-wolfi`
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
