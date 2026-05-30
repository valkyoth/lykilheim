# Lykilheim 1.4.0 Release Notes

## Version

- Version: `1.4.0`
- Release date: TBD
- Git tag: `v1.4.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: human approval and break-glass

## Scope

Lykilheim `1.4.0` is planned to add open-source approval workflows for sensitive
operations and emergency access.

Planned scope:

- control-group style human approval workflows;
- quorum rules for security, DBA, maintainer, namespace-admin, and emergency
  approval patterns;
- break-glass mode with reason capture, time-limited elevation, forced audit
  markers, post-incident summary, and signed evidence bundle;
- approval APIs for pending requests, approval, denial, expiration,
  cancellation, and audit trail inspection;
- policy integration for unwrap, root credential access, mount deletion, rekey,
  namespace deletion, and adapter root rotation.

## Highlights

- Sensitive operations can require human approval without enterprise lock-in.
- Emergency access gets a forensic trail by default.

## Documentation

Documentation required for this release:

- approval workflow guide;
- quorum policy guide;
- break-glass runbook;
- approval API reference;
- incident evidence guide.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: TBD
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Approval workflow tests: TBD
- Break-glass rollback tests: TBD
- Policy integration tests: TBD
- Pentest result: TBD

## Reviewed Advisory Exceptions

- TBD

## Breaking Changes

- None planned.

## Upgrade Notes

- Review which paths require approval before enabling approval policy in
  production.

## Known Limitations

- Approval workflows must not become a substitute for least-privilege policy.

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Container digests: TBD
- Tag signature: TBD
