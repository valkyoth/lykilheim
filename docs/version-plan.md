# Lykilheim Version Plan

Lykilheim is a from-scratch Rust secrets manager. The first stable target is
`1.0.0`; every pre-release below has a bounded scope and a mandatory STOP gate
for pentest, release evidence, and go/no-go review before the next version
starts.

## Project Rules

- Project name: Lykilheim.
- License: EUPL-1.2.
- Toolchain: Rust 1.96.0, pinned in `rust-toolchain.toml`.
- Delivery targets: standalone compiled binary and rootless Podman on Wolfi.
- API model: fully API-driven; CLI wrappers may exist only as API clients.
- Code layout: split Rust modules by ownership boundary; do not grow one large
  `.rs` file.
- Security posture: fail closed by default, minimize external requirements, use
  zeroization for plaintext secret material, and keep experimental features
  explicitly gated.
- CI posture: GitHub CodeQL default setup only; no advanced CodeQL workflow.
- Dependency posture: use current crates at the time each implementation starts,
  record crate choices in release notes, and run `cargo update`, `cargo audit`,
  and `cargo deny check` before release.

## Version Gates

Every release candidate must pass:

- `scripts/checks.sh`.
- `cargo fmt --all --check` once `Cargo.toml` exists.
- `cargo clippy --all-targets -- -D warnings` once `Cargo.toml` exists.
- `cargo test` once `Cargo.toml` exists.
- `cargo deny check` and `cargo audit` once dependency policy exists.
- Rootless Podman smoke tests once containers exist.
- API compatibility smoke tests for every stable endpoint in that version.
- SBOM generation once binary release artifacts exist.

Every version ends with:

**STOP:** freeze scope, run focused pentest, review threat model deltas, publish
release evidence, tag the release, and only then start the next version.

## 0.1.0 - Foundation And Threat Model

Goal: establish the Rust crate, governance, local checks, and security design
before implementing secret storage.

Scope:

- Scaffold a Rust workspace or single crate with small modules for API,
  configuration, error handling, crypto interfaces, storage interfaces, audit,
  and test support.
- Pin Rust 1.96.0 and configure `deny.toml`, `.cargo/audit.toml`, rustfmt, and
  clippy policy.
- Add `Containerfile` and `containers/Containerfile.wolfi` placeholders that
  build a non-root health-check binary once the crate exists.
- Define the public HTTP API shape for init, seal status, health, and version.
- Define the threat model for sealed state, unsealed state, audit failure,
  storage compromise, token compromise, and plugin compromise.
- Add release metadata validation and a minimal release checklist.

Tests and scripts:

- Unit tests for config parsing and error serialization.
- API smoke for health and version endpoints.
- CI gate for format, clippy, tests, audit, deny, and metadata.

**STOP:** pentest the empty API surface, dependency policy, CI permissions,
container user model, and release process. Release `0.1.0`.

## 0.2.0 - Sealed Storage And Cryptographic Barrier

Goal: store opaque encrypted records while the raw storage layer never sees
plaintext.

Scope:

- Implement a `Storage` trait with `get`, `put`, `delete`, and `list`.
- Add local filesystem or embedded development storage for single-node testing.
- Implement sealed/unsealed lifecycle and barrier API boundaries.
- Implement Shamir initialization and unseal flow with threshold validation.
- Wrap master key, barrier key, unseal shares, and plaintext payloads in
  zeroizing/secrecy types.
- Encrypt records with AEAD and authenticated metadata.
- Fail closed when entropy, key reconstruction, or authentication checks fail.

Tests and scripts:

- Property tests for storage key listing semantics.
- Known-answer and round-trip tests for barrier encryption.
- Negative tests for wrong key, tampered ciphertext, nonce reuse prevention,
  invalid threshold, duplicate share, and sealed access.
- Restart smoke proving encrypted storage survives process restart.

**STOP:** cryptographic design review and focused pentest of initialization,
unseal, storage tampering, error messages, and memory handling. Release `0.2.0`.

## 0.3.0 - API Router, Audit, And Policy Skeleton

Goal: make every request pass through authentication, authorization, routing,
and audit control points, even before all engines exist.

Scope:

- Implement axum API routing under versioned paths.
- Add request IDs, structured errors, body limits, and method/path allowlists.
- Add mount table and radix/prefix routing for auth and secrets engines.
- Add fail-closed audit interface with at least one durable local audit device.
- Add policy data model and deterministic path capability checks.
- Add token metadata structs without full token issuance yet.

Tests and scripts:

- API route tests for method rejection, path normalization, body limits, and
  sealed/unsealed behavior.
- Audit tests proving requests fail when all audit sinks fail.
- Policy matrix tests for allow, deny, wildcard, namespace, and default deny.

**STOP:** pentest request parsing, route normalization, audit bypass attempts,
policy default-deny behavior, and log redaction. Release `0.3.0`.

## 0.4.0 - Token Engine, Leases, And KV v2

Goal: provide useful static secret storage with tracked tokens and leases.

Scope:

- Implement token creation, renewal, revocation, TTL, parent/child revocation,
  and accessor lookup.
- Add expiration manager as a bounded tokio background task.
- Implement KV v2 with versioning, soft delete, undelete, destroy, metadata,
  check-and-set, and max versions.
- Add initial namespace model, even if single namespace is the only supported
  production mode.
- Add API documentation examples using `curl`.

Tests and scripts:

- Token lifecycle tests with simulated time.
- Lease cascade tests.
- KV v2 compatibility-style tests for version conflicts, delete/destroy,
  metadata, list behavior, and sealed storage.
- API smoke covering init, unseal, token login, KV write/read/list/delete.

**STOP:** pentest token forgery, TTL bypass, KV metadata disclosure,
namespace separation, and replay behavior. Release `0.4.0`.

## 0.5.0 - AppRole, Userpass, And Baseline Auth

Goal: support practical machine authentication while keeping every login
API-driven and auditable.

Scope:

- Implement AppRole with RoleID, SecretID, TTL, wrapping option, CIDR binding,
  use limits, and revocation.
- Implement userpass for local development and break-glass bootstrap only.
- Add token policy attachment and identity alias metadata.
- Add rate limits and lockout controls for credential-bearing endpoints.
- Add redaction rules for all credential-bearing audit fields.

Tests and scripts:

- AppRole tests for use limits, TTL, wrapping, CIDR restrictions, and replay.
- Userpass tests for password hashing, lockout, and policy attachment.
- Audit redaction tests for every login payload.

**STOP:** pentest AppRole replay, brute force, audit leakage, rate limiting,
and break-glass account handling. Release `0.5.0`.

## 0.6.0 - Transit And PKI Core

Goal: add cryptographic services without exposing raw key material.

Scope:

- Implement transit key creation, encrypt, decrypt, rewrap, rotate, and key
  version controls.
- Add signing and verification only with reviewed primitives selected at
  implementation time.
- Implement PKI root/intermediate CA issuance, CSR signing, CRL/OCSP planning,
  and certificate role constraints.
- Add explicit FIPS/ISO19790 profile planning without claiming validation.

Tests and scripts:

- Known-answer tests for transit payload formats.
- Negative tests for disabled decrypt, wrong context, old key restrictions,
  and tampered ciphertext.
- PKI tests for SAN constraints, TTL caps, CA path length, and revocation.

**STOP:** cryptographic review and pentest of transit misuse resistance, PKI
role enforcement, key rotation, and downgrade paths. Release `0.6.0`.

## 0.7.0 - Rootless Wolfi Containers And Operations

Goal: make Lykilheim operable as a standalone binary and rootless Podman Wolfi
service.

Scope:

- Build minimal Wolfi runtime image with non-root UID/GID.
- Add example configs for local dev, single-node sealed storage, and API-only
  operation.
- Add systemd/user service documentation for rootless Podman.
- Add backup/restore API for encrypted snapshots.
- Add metrics and readiness endpoints that never expose secret material.

Tests and scripts:

- Rootless Podman smoke for init, unseal, KV, restart, and logs.
- Container permission tests for config, storage, and audit paths.
- Snapshot restore smoke into a fresh container.

**STOP:** pentest container filesystem permissions, rootless isolation,
snapshot handling, metrics leakage, and restart behavior. Release `0.7.0`.

## 0.8.0 - Raft High Availability

Goal: support clustered encrypted state with leader election and safe request
forwarding.

Scope:

- Integrate Raft for metadata and encrypted storage replication.
- Add node identity, peer TLS, join/remove APIs, and leadership status.
- Forward writes to the leader and reject unsafe split-brain operations.
- Add encrypted snapshots and restore across nodes.
- Define disaster recovery runbooks.

Tests and scripts:

- Three-node local cluster smoke.
- Leader failover tests.
- Network partition tests for read/write behavior.
- Snapshot and membership-change tests.

**STOP:** pentest cluster join, peer authentication, split-brain handling,
snapshot restore, and follower forwarding. Release `0.8.0`.

## 0.9.0 - Sandboxed Plugins And Dynamic Secrets Preview

Goal: introduce tightly constrained extensibility without making plugins part
of the trusted core.

Scope:

- Define `SecretEngine` and `AuthEngine` host traits.
- Add native development dynamic secrets engine for PostgreSQL or a fake SQL
  target used in tests.
- Add Wasmtime plugin prototype with fuel limits, memory limits, no ambient
  filesystem access, and explicit outbound capability injection.
- Add plugin signing/verification design.
- Keep plugin support preview/experimental until the 1.0 pentest proves the
  sandbox boundary.

Tests and scripts:

- Plugin fuel exhaustion and memory limit tests.
- Capability denial tests for filesystem and network access.
- Dynamic lease creation and revocation tests.
- Fuzz request envelopes crossing the plugin boundary.

**STOP:** sandbox escape review, plugin supply-chain review, dynamic secret
revocation pentest, and denial-of-service testing. Release `0.9.0`.

## 0.10.0 - Advanced Security Incubation

Goal: prepare differentiating features without blocking 1.0 on research risk.

Scope:

- Add feature-gated research tracks for TEE attestation, post-quantum hybrid
  transport, ZKP authentication, Merkle audit anchoring, and eBPF audit export.
- Document which features are stable, beta, experimental, or research.
- Add compatibility policy for API, storage format, audit format, and plugin
  ABI.
- Add migration framework for storage and policy data.

Tests and scripts:

- Compile-only gates for research features where runtime infrastructure is not
  available.
- Audit hash-chain verification tests.
- Migration round-trip tests from every previous pre-release storage version.

**STOP:** decide which incubation features are excluded from 1.0 stable,
pentest enabled beta features, and freeze the 1.0 compatibility contract.
Release `0.10.0`.

## 1.0.0 - First Stable Release

Goal: ship the smallest secure vault that is useful, documented, and stable.

Required stable scope:

- API-driven init, seal status, unseal, health, version, auth, policy, token,
  KV v2, transit baseline, audit, storage, and backup/restore.
- Standalone binary and rootless Wolfi container.
- Fail-closed audit behavior.
- Encrypted storage at rest behind the barrier.
- Token TTL, leases, renewal, revocation, and cascade revocation.
- AppRole and userpass baseline auth.
- Documented threat model, operator guide, API reference, release checklist,
  recovery runbook, and security disclosure process.

Explicitly not required for 1.0 stable unless completed and pentested earlier:

- Production Wasm plugins.
- Production Raft HA beyond the tested stability bar.
- Production PQC, ZKP, TEE, eBPF, or external Merkle anchoring.
- Enterprise compatibility claims with Vault/OpenBao.
- FIPS validation claims.

Release gate:

- Full local and CI release gates pass from a clean checkout.
- Dependency and license review is clean or has documented narrow exceptions.
- SBOM and checksums are generated from the tagged source tree.
- Rootless Podman Wolfi smoke passes.
- API compatibility suite passes.
- Storage migration tests pass from all pre-release formats.
- External or independent pentest findings are triaged and fixed or explicitly
  documented as accepted residual risk.

**STOP:** final 1.0 pentest, release-candidate freeze, maintainer sign-off,
signed tag, release evidence publication, and post-release monitoring window.
Release `1.0.0`.
