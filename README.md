# Lykilheim

Lykilheim is planned as a Rust-native, API-driven secrets manager inspired by
the operational model of Vault and OpenBao, with a stricter focus on memory
safety, fail-closed audit behavior, rootless container deployment, and explicit
security gates before each release.

Current status: planning/bootstrap. Start with
[docs/version-plan.md](docs/version-plan.md).

Documentation is part of the definition of done. User-facing features,
configuration, APIs, deployment paths, and security behavior should ship with
practical docs and examples, following the standard set by Fluxheim.

The Vault/OpenBao coverage inventory lives in
[docs/feature-parity.md](docs/feature-parity.md).

Planned release notes live in [release-notes](release-notes/).
