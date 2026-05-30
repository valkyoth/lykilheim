# Lykilheim 0.8.0 Release Notes

## Version

- Version: `0.8.0`
- Release date: TBD
- Git tag: `v0.8.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: Raft HA preview

## Scope

Lykilheim `0.8.0` introduces clustered encrypted state with leader election and
safe request forwarding.

Planned stable preview scope:

- Raft replication for metadata and encrypted storage;
- node identity and peer TLS;
- join, remove, leadership status, and cluster health APIs;
- write forwarding to the leader;
- split-brain rejection behavior;
- encrypted snapshots and restore across nodes;
- disaster recovery runbooks;
- documented boundaries for performance standby, read replicas, performance
  replication, and replication filters.

## Highlights

- First multi-node high-availability preview.
- Cluster join and peer authentication are release-blocking pentest areas.
- Network partition behavior must be tested before release.

## Documentation

Documentation required for this release:

- three-node cluster guide;
- node identity and peer TLS guide;
- join/remove API reference;
- leader forwarding behavior;
- network partition behavior;
- encrypted snapshot and disaster recovery runbooks;
- replication boundary and non-claim documentation.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: `scripts/checks.sh`
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Three-node smoke result: TBD
- Leader failover tests: TBD
- Network partition tests: TBD
- Snapshot and membership-change tests: TBD
- Replication boundary review: TBD

## Reviewed Advisory Exceptions

- TBD. List every accepted advisory with reachability and removal condition.

## Breaking Changes

- Cluster API and Raft storage internals are preview until promoted for stable
  use.

## Upgrade Notes

- Do not mix node versions unless the release checklist explicitly permits it.
- Back up encrypted snapshots before membership changes.

## Known Limitations

- Cross-region replication is not promised.
- Disaster recovery automation may be manual.
- Production support depends on the release pentest outcome.

## Container Images

Planned image tags after release validation:

- GitHub Container Registry: `ghcr.io/valkyoth/lykilheim:v0.8.0-wolfi`
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
