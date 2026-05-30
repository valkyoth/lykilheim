# Contributing To Lykilheim

Lykilheim is planned as security-sensitive Rust infrastructure for API-driven
secrets management. Contributions are welcome when they keep the project small,
clear, tested, and honest about what is stable.

## License

Lykilheim is licensed under the European Union Public Licence 1.2. By
contributing, you agree that your contribution is provided under the same
license.

## Development Setup

Use the pinned Rust toolchain from `rust-toolchain.toml`. The project currently
targets Rust 1.96.0.

```bash
scripts/checks.sh
```

After the Rust crate is scaffolded, normal development checks must include:

```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo deny check
cargo audit
```

## Checks

Before opening a pull request, run:

```bash
scripts/checks.sh
```

## Security-Sensitive Changes

Treat these areas as high risk:

- cryptographic barrier, unseal, key wrapping, and zeroization;
- request parsing, body limits, and API compatibility;
- token, lease, identity, policy, and namespace handling;
- storage, Raft replication, snapshots, and rollback;
- audit integrity, telemetry, and privacy controls;
- plugin sandboxing, WASI capabilities, and dynamic secrets;
- TLS, post-quantum experiments, and FIPS-capable crypto profiles;
- dependency updates.

Do not post exploitable security details in public issues. Follow
[SECURITY.md](../SECURITY.md).

## Dependency Policy

Lykilheim will use `deny.toml`, `cargo-deny`, and `cargo-audit` once the Rust
crate is scaffolded.

When adding or updating crates:

- use crates.io releases unless there is a strong reason not to;
- avoid git dependencies;
- check maintenance status and license;
- keep `Cargo.lock` updated;
- run `cargo deny check` and `cargo audit`.

## Design Guidelines

- Prefer existing local patterns over new abstractions.
- Split Rust modules by ownership boundary instead of growing one large file.
- Keep modules feature-gated when they change the threat model or add external
  requirements.
- Keep default builds focused on stable core behavior.
- Make every operator workflow API-driven first.
- Keep rootless Podman and Wolfi deployment paths working as first-class gates.
- Document stable, beta, experimental, and research features honestly.

## Pull Requests

Good pull requests are small enough to review and include:

- a clear summary;
- tests for behavior changes;
- docs or examples when user-facing behavior changes;
- practical documentation for every new feature, endpoint, config key,
  deployment mode, and security-sensitive behavior;
- security notes for risky areas.

Large features should start with a roadmap or design-doc update before code.
