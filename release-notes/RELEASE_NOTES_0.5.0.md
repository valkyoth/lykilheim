# Lykilheim 0.5.0 Release Notes

## Version

- Version: `0.5.0`
- Release date: TBD
- Git tag: `v0.5.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: baseline auth preview

## Scope

Lykilheim `0.5.0` adds practical machine authentication while keeping all login
and administration paths API-driven and auditable.

Planned stable preview scope:

- AppRole with RoleID, SecretID, TTL, wrapping option, CIDR binding, use
  limits, and revocation;
- userpass for local development and break-glass bootstrap only;
- token policy attachment and identity alias metadata;
- rate limits and lockout controls for credential-bearing endpoints;
- audit redaction for every credential-bearing field.

## Highlights

- First practical machine-auth release.
- Credential replay, brute force, and audit leakage are release-blocking tests.
- Userpass remains scoped for development and break-glass bootstrap, not broad
  production identity.

## Documentation

Documentation required for this release:

- AppRole setup and login guide;
- SecretID wrapping and revocation examples;
- userpass bootstrap and break-glass guidance;
- rate-limit and lockout behavior;
- audit redaction examples for auth payloads.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: `scripts/checks.sh`
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- AppRole replay tests: TBD
- Rate-limit tests: TBD
- Userpass password hashing review: TBD
- Audit redaction tests: TBD

## Reviewed Advisory Exceptions

- TBD. List every accepted advisory with reachability and removal condition.

## Breaking Changes

- Pre-`1.0.0` auth payloads, wrapping responses, and token metadata may change.

## Upgrade Notes

- Recreate preview AppRole roles if storage or auth schema changes are not
  migrated automatically.
- Review userpass accounts and remove bootstrap-only credentials after setup.

## Known Limitations

- No OIDC/JWT auth yet.
- No Kubernetes auth yet.
- No LDAP auth yet.
- No dynamic secrets yet.

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Container digests: TBD, or not applicable
- Tag signature: TBD
