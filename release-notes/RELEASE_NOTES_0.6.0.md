# Lykilheim 0.6.0 Release Notes

## Version

- Version: `0.6.0`
- Release date: TBD
- Git tag: `v0.6.0`
- Git commit: TBD
- License: EUPL-1.2
- Release type: transit and PKI preview

## Scope

Lykilheim `0.6.0` adds cryptographic services without exposing raw key material
to callers.

Planned stable preview scope:

- transit key creation, encrypt, decrypt, rewrap, rotate, and key version
  controls;
- signing and verification only with reviewed primitives selected at
  implementation time;
- PKI root and intermediate CA issuance;
- CSR signing, CRL/OCSP planning, and certificate role constraints;
- FIPS/ISO19790 profile planning without validation claims.

## Highlights

- First encryption-as-a-service release.
- Transit misuse resistance and key rotation are release-blocking pentest areas.
- PKI role constraints must be tested before release.

## Documentation

Documentation required for this release:

- transit API reference with safe usage examples;
- key rotation and rewrap guide;
- PKI role and issuance guide;
- certificate constraint examples;
- explicit cryptographic non-claims, including no FIPS validation claim.

## Security And Stability Gate

Release evidence to record immediately before publishing:

- Gate command: `scripts/checks.sh`
- Gate report directory: TBD
- Result: TBD
- `cargo audit` result: TBD
- `cargo deny check` result: TBD
- Transit known-answer tests: TBD
- Transit negative tests: TBD
- PKI constraint tests: TBD
- Cryptographic review result: TBD

## Reviewed Advisory Exceptions

- TBD. List every accepted advisory with reachability and removal condition.

## Breaking Changes

- Pre-`1.0.0` transit payload formats and PKI APIs may change.

## Upgrade Notes

- Back up encrypted storage before enabling transit or PKI in preview
  deployments.
- Recheck key version limits and role TTL caps after upgrade.

## Known Limitations

- No KMIP support.
- No production FIPS validation.
- PKI revocation behavior may be incomplete.
- Advanced post-quantum work remains research-gated.

## Checksums And Signatures

Record during the release:

- Source archive checksum: TBD
- Binary checksums: TBD
- SBOM checksums: TBD
- Container digests: TBD, or not applicable
- Tag signature: TBD
