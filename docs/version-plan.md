# Lykilheim Version Plan

Status: normative implementation order

Lykilheim is a from-scratch Rust secrets manager targeting a defensible
quantum-resistant, crypto-agile European vault. `1.0.0` is the first stable
release. All planned product work, including the former `1.1.0` through
`2.0.0` scope, is now split into small pre-1.0 milestones so interfaces can
change while security boundaries are still being proven.

Tags use:

```text
v0.N.0      milestone release
v0.N.P      patch or security-fix release
v1.0.0      first stable production release
```

The list is intentionally granular and is not a maximum. Adjacent unreleased
milestones may be split when that makes review, documentation, migration, or
pentesting clearer. Released versions are never renumbered.

## Project Rules

- Project name: Lykilheim.
- License: EUPL-1.2.
- Toolchain: Rust 1.96.1, pinned in `rust-toolchain.toml`.
- Delivery targets: portable standalone compiled binary and rootless Podman on Wolfi.
- Portability target: Linux, macOS, Windows, and BSD-style Unix for the binary;
  the hardened Wolfi container remains Linux-only.
- API model: fully API-driven; CLIs and integrations are API clients, not a
  second control plane.
- Code layout: focused crates or modules aligned to trust and ownership
  boundaries; no large multipurpose source files.
- Documentation: every feature, endpoint, configuration key, storage format,
  deployment path, security boundary, failure mode, migration, and operator
  workflow is documented to the same practical standard as Fluxheim before it
  is done.
- Release notes: every milestone gets
  `release-notes/RELEASE_NOTES_X.Y.Z.md` before implementation begins; evidence,
  limitations, checksums, and signatures are completed before tagging.
- Security posture: fail closed when authorization, audit durability,
  cryptographic policy, storage integrity, or secret-memory guarantees cannot
  be established.
- Quantum claim: use "quantum-resistant under current cryptanalytic knowledge,
  crypto-agile, and hybrid while standards and implementations mature." Never
  claim that an algorithm or deployment is unconditionally quantum-proof.
- CI posture: GitHub CodeQL default setup only; no advanced CodeQL workflow.
- Dependency posture: use maintained crates after license and security review;
  run update, audit, deny, and SBOM gates for every release.
- Parity posture: keep `docs/feature-parity.md` synchronized so every Vault and
  OpenBao capability is implemented, scheduled, experimental, or deliberately
  different with operator impact documented.

## Roadmap Authority

This document is the normative implementation order. README summaries,
feature-parity labels, release-note placeholders, and architecture documents
must be synchronized before work starts on the affected milestone. Existing
post-1.0 and `2.0.0` placeholders are superseded by the pre-1.0 milestones
below and are not implementation authority.

Each milestone should change one primary trust boundary. A milestone may ship
an explicitly labeled preview only when its insecure fallback is disabled and
the preview cannot weaken stable paths. No compatibility promise is made before
`1.0.0`, but every persisted format must have migration or explicit rejection
tests from the release that introduces it.

## Mandatory Release Gate

Every release candidate must pass:

- `scripts/checks.sh` and its matching `scripts/release_0_N_gate.sh` once added;
- formatting, clippy with warnings denied, unit, integration, documentation,
  property, fuzz, model, and failure-injection tests proportional to scope;
- `cargo deny check bans licenses sources` and `cargo audit`;
- API and format compatibility fixtures for every admitted public contract;
- Linux, macOS, Windows, and practical BSD portability checks for changed
  platform boundaries;
- rootless Podman smoke tests for container or service changes;
- threat-model, security-control, feature-parity, API, configuration, operator,
  migration, recovery, and limitation documentation for the change;
- matching release notes, SBOM, provenance, checksums, signatures, and clean
  release evidence from the exact source commit.

Every version below has an explicit exit criterion requiring this stop:

```text
vX.Y.Z implementation stop reached. Freeze scope and run the focused pentest
for this exact commit. Do not tag or begin the next version until findings are
fixed or documented as accepted residual risk and CI is green.
```

## Phase 0: Released Foundation

### 0.1.0 - Repository Foundation

Goal: establish the Rust crate, governance, initial boundaries, and release
process without storing secrets.

Deliverables:

- API, configuration, error, audit, crypto, and storage boundary modules;
- pinned toolchain, dependency policy, CI, Wolfi placeholders, and local gates;
- threat model, API shape, portability policy, feature audit, and release docs.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_1_gate.sh`
- foundation API and configuration tests

Exit criteria:

- Foundation scope and known non-implemented security boundaries are explicit.
- Focused `0.1.0` pentest passes for the exact tagged commit.

## Phase 1: Architectural Contracts

### 0.2.0 - Trust Boundary Corrections

Goal: correct prospective design blockers before secret-bearing interfaces
harden.

Deliverables:

- ADRs for storage consistency, crypto broker ownership, audit gating, Raft,
  rollback resistance, cluster identity, and plugin process isolation;
- typed error taxonomy and capability declarations for security boundaries;
- corrected quantum-resistant product claim and suite-policy requirements.

Verification:

- architecture consistency and documentation-link checks;
- compile tests proving unsupported capabilities fail explicitly.

Exit criteria:

- No future core interface depends on the superseded synchronous traits.
- Focused `0.2.0` architecture pentest passes for the exact release commit.

### 0.3.0 - Canonical IDs And Paths

Goal: give routing, policy, audit, and storage one canonical identity model.

Deliverables:

- typed vault, namespace, mount, object, key, revision, and operation IDs;
- segment-aware canonical paths with defined decoding and Unicode behavior;
- keyed or random opaque storage identifiers that hide logical path shape.

Verification:

- cross-platform canonical vectors and malformed-path property tests;
- collision, traversal, separator, normalization, and size-limit tests.

Exit criteria:

- Every subsystem consumes the same typed canonical request identity.
- Focused `0.3.0` parser and path-confusion pentest passes.

### 0.4.0 - Storage Capability Contract

Goal: define an encrypted record-store contract valid across materially
different consistency models.

Deliverables:

- revision reads, compare-and-swap, atomic batch, and durable acknowledgment;
- snapshot-consistent paginated iteration and caller-provided read buffers;
- explicit limits and typed conflict, unavailable, corrupt, unsupported, and
  indeterminate-commit errors.

Verification:

- backend conformance model and linearizable CAS tests;
- fault tests for partial, duplicate, delayed, and indeterminate commits.

Exit criteria:

- Filesystem, SQL, SurrealDB, and Raft semantics can be declared honestly.
- Focused `0.4.0` storage-contract pentest passes.

### 0.5.0 - Authenticated Record Format

Goal: specify bounded opaque records before implementing durable storage.

Deliverables:

- immutable superblock, fixed header, segmented ciphertext, and commit manifest;
- authenticated suite, object, generation, key epoch, lengths, and tombstones;
- encrypted logical index, optional size padding, and versioned migration rules.

Verification:

- canonical format vectors, parser fuzzing, and checked-arithmetic tests;
- truncation, reorder, substitution, downgrade, and segment-corruption tests.

Exit criteria:

- Parsers allocate nothing from untrusted lengths and never expose logical paths.
- Focused `0.5.0` format and tampering pentest passes.

### 0.6.0 - Durable Local Record Store

Goal: implement a crash-safe single-node backend for authenticated records.

Deliverables:

- atomic manifest publication, fsync policy, lock ownership, and tombstones;
- snapshot-consistent iteration and bounded compaction;
- platform-specific durability profiles with explicit weaker guarantees.

Verification:

- crash-point, disk-full, torn-write, stale-lock, and recovery tests;
- Linux, macOS, Windows, and BSD filesystem fixtures.

Exit criteria:

- A committed record survives documented crashes or reports indeterminate state.
- Focused `0.6.0` persistence and rollback pentest passes.

### 0.7.0 - Locked Secret Memory

Goal: establish high-assurance secret allocation and input decoding.

Deliverables:

- locked fixed-capacity secret arenas using reviewed `sanitization` facilities;
- direct bounded Base64/JSON decoding into secret memory without ordinary
  plaintext `String`, `Bytes`, or `Vec` intermediates;
- dump, fork, swap, ptrace, guard, and platform-assurance policy.

Verification:

- allocation, residue, lock-failure, bounds, Miri, and platform tests;
- startup remains sealed when configured memory guarantees cannot be met.

Exit criteria:

- Secret ingress and key storage avoid ordinary heap plaintext by construction.
- Focused `0.7.0` memory-handling pentest passes.

### 0.8.0 - Crypto Broker And Key Hierarchy

Goal: prevent engines from receiving master keys or broad decrypt capability.

Deliverables:

- serialized broker with opaque, generation-fenced, non-cloneable key handles;
- domain-separated seal, barrier, token, audit, Transit, PKI, and cluster keys;
- seal sequence that closes request gates, drains work, clears arenas, and
  invalidates handles.

Verification:

- compile-fail capability tests and concurrent seal/use race tests;
- key-purpose separation and stale-handle property tests.

Exit criteria:

- Key material crosses no engine, task, serialization, or shared-ownership boundary.
- Focused `0.8.0` key-ownership pentest passes.

### 0.9.0 - Algorithm Suite Registry

Goal: make every cryptographic object closed-enum, policy-bound, and migratable.

Deliverables:

- authenticated suite IDs, profiles, key versions, epochs, contexts, and minimum
  reader/verifier versions;
- provider-neutral classical and post-quantum interfaces;
- downgrade policy, provider self-tests, and algorithm migration state machine.

Verification:

- unknown-suite, disabled-suite, component-stripping, and downgrade tests;
- provider differential and known-answer test harnesses.

Exit criteria:

- Requests cannot select arbitrary algorithms or silently fall back.
- Focused `0.9.0` crypto-agility pentest passes.

## Phase 2: Seal And Barrier

### 0.10.0 - Symmetric Barrier Core

Goal: encrypt authenticated segmented records behind the broker.

Deliverables:

- AEAD barrier with deterministic domain-separated nonce derivation;
- bounded in-place encrypt/decrypt paths and authenticated metadata;
- sealed-state gate and indistinguishable authentication failures.

Verification:

- KAT, round-trip, nonce uniqueness, tamper, oracle, and large-record tests;
- restart tests proving the backend observes ciphertext only.

Exit criteria:

- No plaintext or unauthenticated metadata reaches raw storage.
- Focused `0.10.0` barrier pentest and cryptographic review pass.

### 0.11.0 - Initialization And Shamir Shares

Goal: initialize a vault and split its seal secret without unsafe copies.

Deliverables:

- entropy health handling and direct locked-memory seal-secret generation;
- versioned shares bound to vault UUID, generation, threshold, total, and index;
- wrapped immutable keyring with authenticated reconstruction canary.

Verification:

- threshold, entropy failure, duplicate index, corrupt share, and residue tests;
- deterministic format vectors without deterministic secret generation.

Exit criteria:

- Initialization persists no share or reconstructed seal secret.
- Focused `0.11.0` initialization and share-handling pentest passes.

### 0.12.0 - Bounded Unseal Lifecycle

Goal: reconstruct keys through one replay-resistant, expiring unseal attempt.

Deliverables:

- collector bound to vault UUID, attempt nonce, seal generation, and TTL;
- duplicate-safe fixed slots and direct secret-memory request decoder;
- reconstruction, keyring verification, immediate clearing, and cancellation.

Verification:

- replay, mixed-generation, timeout, concurrency, crash, and oracle tests;
- sealed request-gate and memory-residue tests.

Exit criteria:

- Invalid submissions reveal no share validity and leave no reusable partial state.
- Focused `0.12.0` unseal pentest passes.

### 0.13.0 - Crash-Safe Shamir Rekey

Goal: change seal shares without mixing generations or creating crash ambiguity.

Deliverables:

- idle, old-quorum, pending-new, committed, and cancelled rekey states;
- generation-bound share verification and resumable cancellation;
- recovery and root-token quorum workflow foundations.

Verification:

- crash at every transition, stale share, replay, cancel, and restart tests;
- state-machine model checking for generation mixing.

Exit criteria:

- Recovery always accepts one explicitly committed generation only.
- Focused `0.13.0` rekey and recovery pentest passes.

### 0.14.0 - Barrier Rotation And Rewrap

Goal: separate key rotation, rewrap, rekey, and suite migration lifecycles.

Deliverables:

- immediate new-write epoch rotation with old decrypt-only epochs;
- bounded resumable DEK and keyring rewrap with progress evidence;
- generation-safe pause, restart, rollback, and retirement rules.

Verification:

- concurrent read/write, crash, mixed epoch, rollback, and retirement tests;
- migration from every admitted barrier format.

Exit criteria:

- Rotation cannot strand ciphertext or reactivate retired write keys.
- Focused `0.14.0` rotation and rewrap pentest passes.

## Phase 3: Request, Audit, And Policy

### 0.15.0 - Canonical API Router

Goal: route every request through one typed, bounded security pipeline.

Deliverables:

- versioned Axum routes, request IDs, body limits, and stable errors;
- one-pass normalization into namespace, mount, path, operation, and context;
- sealed/unsealed and method/path allowlists with API specification fixtures.

Verification:

- malformed HTTP, normalization, smuggling, body-limit, and route tests;
- API compatibility smoke for all admitted endpoints.

Exit criteria:

- Policy, audit, routing, and storage see exactly the same request identity.
- Focused `0.15.0` HTTP and routing pentest passes.

### 0.16.0 - Bounded Layered Rate Control

Goal: replace the process-local unbounded IP limiter with configurable controls.

Deliverables:

- fixed-point refill, injected clock, bounded sharded state, and eviction;
- global, network, endpoint, credential, token, and namespace layers;
- trusted-proxy rules, IPv6 aggregation, and separate durable lockout.

Verification:

- deterministic clock, cardinality, proxy spoofing, fairness, and DoS tests;
- config validation for every positive rate, burst, and memory limit.

Exit criteria:

- Untrusted clients cannot cause unbounded limiter work or memory growth.
- Focused `0.16.0` rate-limit and proxy pentest passes.

### 0.17.0 - Typed Durable Audit Journal

Goal: make security audit intent durable without blocking async executors.

Deliverables:

- typed intent/result schema, redaction registry, sequence, actor, namespace,
  policy revision, phase, and outcome;
- bounded dedicated writer process or isolated thread pool with durable journal;
- one-way `AuditUnavailable` gate and restricted repair state.

Verification:

- disk-full, queue-full, fsync, timeout, panic, corruption, and restart tests;
- schema redaction and deterministic hash tests.

Exit criteria:

- Audit writer failure cannot starve health or manual seal control paths.
- Focused `0.17.0` audit durability and leakage pentest passes.

### 0.18.0 - Fail-Closed Audit Transaction Gate

Goal: prevent mutation or secret release before required audit evidence commits.

Deliverables:

- bounded intent, operation, result, commit/release request pipeline;
- transactional encrypted mutation plus audit outbox where supported;
- withheld response buffers and idempotent asynchronous sink export.

Verification:

- failure injection at every gate and mutation/audit atomicity tests;
- proof that no secret byte streams before durable acknowledgment.

Exit criteria:

- Audit failure blocks all secret-bearing reads and security mutations.
- Focused `0.18.0` audit-bypass and partial-commit pentest passes.

### 0.19.0 - Policy Compiler And Evaluator

Goal: enforce deterministic default-deny capabilities over canonical requests.

Deliverables:

- segment-aware `*` and `**`, explicit deny precedence, and constraints;
- immutable policy revisions, shadowing warnings, and system-path restrictions;
- deterministic evaluator with resource budgets and no ambient inputs.

Verification:

- golden policy matrix, fuzzing, TOCTOU, overlap, and budget tests;
- namespace escape and contradictory-rule tests.

Exit criteria:

- Every protected operation binds to one immutable policy revision.
- Focused `0.19.0` policy bypass and ambiguity pentest passes.

### 0.20.0 - Mounts And Barrier Views

Goal: isolate engines by immutable mount identity and lifecycle.

Deliverables:

- enable, disable, tune, remount, conflict, and capability APIs;
- per-mount barrier views with no cross-mount traversal;
- generation-aware lease revocation on disable or remount.

Verification:

- conflict, race, cross-view, remount, and cascade tests;
- API compatibility fixtures and audit assertions.

Exit criteria:

- A mount cannot access another mount's namespace, storage, keys, or leases.
- Focused `0.20.0` mount-isolation pentest passes.

### 0.21.0 - Response Wrapping And Cubbyhole

Goal: provide at-most-once secret delivery and token-private storage.

Deliverables:

- random wrapping token, independent payload DEK, lookup, rewrap, and unwrap;
- atomic consumed state with documented network-failure semantics;
- cubbyhole rooted by immutable token ID and erased on revocation.

Verification:

- replay, race, expiry, disconnect, rewrap, isolation, and cleanup tests;
- no bearer token is ever used as an encryption key.

Exit criteria:

- Wrapping is auditable, at-most-once, and cross-token isolated.
- Focused `0.21.0` wrapping and cubbyhole pentest passes.

## Phase 4: Tokens, Leases, Identity, And Auth

### 0.22.0 - Token Engine

Goal: issue independently random tokens with atomic lifecycle semantics.

Deliverables:

- versioned 256-bit bearer format, keyed lookup digest, and random accessor;
- child, orphan, periodic, service, and batch-mode decisions;
- immutable namespace, parent, max TTL, and policy-revision bindings.

Verification:

- forgery, prefix, accessor, clock rollback, renewal, and revocation-race tests;
- indistinguishable invalid, missing, expired, and revoked errors.

Exit criteria:

- Revocation wins every authorization and renewal race.
- Focused `0.22.0` token pentest passes.

### 0.23.0 - Lease And Expiration Engine

Goal: make dynamic credential leases durable, idempotent, and recoverable.

Deliverables:

- issue, renew, revoke, cascade, intent, result, and compensation states;
- bounded expiration worker with restart recovery;
- synchronous expiry enforcement independent of cleanup timing.

Verification:

- simulated time, upstream outage, partial creation, cascade, and restart tests;
- generation-aware idempotency and clock-rollback tests.

Exit criteria:

- No cleanup delay or crash extends authorization beyond its durable lease.
- Focused `0.23.0` lease lifecycle pentest passes.

### 0.24.0 - KV v2

Goal: deliver useful versioned static secret storage.

Deliverables:

- CAS writes, versions, metadata, list, and maximum-version retention;
- soft delete, undelete, irreversible destroy, and created-by metadata;
- bounded payloads, encrypted indexes, and migration fixtures.

Verification:

- compatibility, concurrent CAS, history, metadata disclosure, and destroy tests;
- init-to-KV API smoke through restart and seal/unseal.

Exit criteria:

- KV semantics remain atomic and reveal no forbidden historical data.
- Focused `0.24.0` KV v2 pentest passes.

### 0.25.0 - Identity Entities And Groups

Goal: attach policy to stable identities rather than transient login records.

Deliverables:

- entities, aliases, groups, metadata, merge, and policy attachment;
- immutable IDs, cycle rejection, depth bounds, and visited-set evaluation;
- audit-safe identity resolution and alias uniqueness rules.

Verification:

- cycle, depth, alias collision, merge, isolation, and race tests;
- identity-policy golden matrices.

Exit criteria:

- Group traversal is bounded and cannot grant cross-identity authority.
- Focused `0.25.0` identity and group pentest passes.

### 0.26.0 - Namespace Isolation

Goal: enforce hierarchical tenant isolation and delegated administration.

Deliverables:

- child namespaces, root-only system paths, quotas hooks, and policy inheritance;
- namespace-bound identity, token, mount, storage, lease, and audit views;
- explicit cross-namespace administration and deletion workflows.

Verification:

- cross-tenant matrix, deletion, inheritance, confused-deputy, and quota tests;
- namespace value is always derived from stored authority, never a header alone.

Exit criteria:

- No tenant can enumerate, infer, or operate on another tenant's resources.
- Focused `0.26.0` namespace isolation pentest passes.

### 0.27.0 - AppRole Authentication

Goal: provide replay-resistant machine authentication.

Deliverables:

- RoleID and SecretID with TTL, uses, CIDR, accessor, wrapping, and revocation;
- durable atomic use consumption and role-bound token issuance;
- bootstrap, rotation, and least-privilege operator workflows.

Verification:

- replay, race, CIDR/proxy, wrap, brute-force, and revocation tests;
- audit redaction and rate-limit conformance.

Exit criteria:

- SecretID use and token issuance cannot partially commit or be replayed.
- Focused `0.27.0` AppRole pentest passes.

### 0.28.0 - Userpass And Password Policy

Goal: support local human authentication with explicit production guardrails.

Deliverables:

- Argon2id policy, password generation rules, lockout, unlock, and recovery;
- bootstrap/development defaults and production enablement warnings;
- identity, MFA hook, audit redaction, and rate-control integration.

Verification:

- offline cost, brute-force, lockout DoS, enumeration, and migration tests;
- password-policy and recovery-path fixtures.

Exit criteria:

- User enumeration and lockout cannot bypass policy or disclose credentials.
- Focused `0.28.0` userpass pentest passes.

### 0.29.0 - JWT, OIDC, And TLS Certificate Auth

Goal: support standards-based human and workload identity.

Deliverables:

- static/JWKS JWT validation, discovery pinning, claims, audiences, and clock rules;
- OIDC browser/device flow decisions and callback protection;
- mTLS client certificate auth with chain, SAN, EKU, and revocation policy.

Verification:

- algorithm confusion, JWKS rotation, SSRF, nonce/state, replay, and CRL tests;
- identity alias collision and claim-bound policy tests.

Exit criteria:

- External identity cannot select validation keys, algorithms, or namespaces.
- Focused `0.29.0` federated and certificate-auth pentest passes.

### 0.30.0 - Kubernetes And Cloud Workload Auth

Goal: authenticate workloads without long-lived ambient credentials.

Deliverables:

- Kubernetes TokenReview and offline service-account JWT modes;
- AWS, Azure, and GCP workload identity adapters;
- audience, project, cluster, account, nonce, and replay bindings.

Verification:

- metadata SSRF, confused deputy, stale identity, replay, and outage tests;
- local Kubernetes fixtures and provider protocol vectors.

Exit criteria:

- Workload identity is bound to an explicit trust domain and role.
- Focused `0.30.0` workload-auth pentest passes.

### 0.31.0 - Enterprise Auth And MFA

Goal: complete the planned authentication families before stable freeze.

Deliverables:

- LDAP, Kerberos, RADIUS, and GitHub/OIDC-compatible operator auth;
- identity-bound TOTP/WebAuthn-style MFA framework after dependency review;
- per-method lockout, group mapping, TLS, channel-binding, and limitations docs.

Verification:

- protocol replay, downgrade, group injection, MFA bypass, and outage tests;
- rootless integration fixtures where practical.

Exit criteria:

- Every enabled method has bounded inputs, redacted audit, and revoke semantics.
- Focused `0.31.0` enterprise-auth and MFA pentest passes.

## Phase 5: Cryptographic Engines

### 0.32.0 - Transit Classical Baseline

Goal: provide symmetric cryptographic services without exporting raw keys.

Deliverables:

- create, encrypt, decrypt, rewrap, rotate, key-version, hash, HMAC, and random;
- derived key and datakey APIs with convergent encryption disabled by default;
- strict contexts, minimum encrypt/decrypt versions, and opaque key handles.

Verification:

- KAT, misuse, wrong context, old-version, oracle, rotation, and fuzz tests;
- API compatibility and audit-redaction fixtures.

Exit criteria:

- Transit never exposes key material or silently weakens policy.
- Focused `0.32.0` Transit pentest and crypto review pass.

### 0.33.0 - Post-Quantum Provider Baseline

Goal: admit reviewed ML-KEM and ML-DSA implementations as production-capable
providers.

Deliverables:

- ML-KEM-768/1024 and ML-DSA-65/87 provider profiles;
- KATs, differential tests, self-tests, locked-key generation, and fault checks;
- exact dependency, feature, CPU, memory, and pre-auth DoS policy.

Verification:

- NIST vectors, malformed inputs, implicit rejection, side-channel review, and
  provider differential campaigns.

Exit criteria:

- PQ algorithms are no longer research placeholders and fail closed on self-test.
- Focused `0.33.0` independent PQ provider pentest and crypto review pass.

### 0.34.0 - Hybrid Transit Envelopes

Goal: combine classical and post-quantum protection without downgrade paths.

Deliverables:

- versioned KEM+DEM envelope binding both shared secrets, public keys, suite,
  recipient, key version, ciphertext, and application context;
- X25519 all-zero rejection and indistinguishable decapsulation errors;
- dual classical plus ML-DSA signing requiring both under hybrid policy.

Verification:

- component stripping, malformed component, context swap, downgrade, and KATs;
- harvest-now-decrypt-later threat-model evidence.

Exit criteria:

- Hybrid policy never accepts either component alone or retries classical-only.
- Focused `0.34.0` hybrid-envelope pentest and crypto review pass.

### 0.35.0 - Classical PKI Core

Goal: issue constrained classical certificates through isolated PKI keys.

Deliverables:

- root/intermediate issuers, CSR signing, roles, SAN/EKU/path/TTL constraints;
- issuer rotation, CRL, OCSP, and revocation lifecycle;
- key-purpose isolation and audited operator workflows.

Verification:

- chain, role bypass, SAN, path length, expiry, revocation, and rotation tests;
- interoperability fixtures with common TLS stacks.

Exit criteria:

- Issuance cannot exceed issuer, role, namespace, or policy constraints.
- Focused `0.35.0` PKI pentest and crypto review pass.

### 0.36.0 - Quantum-Resistant PKI

Goal: provide standards-based PQ and hybrid certificate deployment modes.

Deliverables:

- RFC 9881 pure ML-DSA issuers, leaves, and CRLs;
- parallel independent classical and ML-DSA chains requiring both internally;
- composite draft mode isolated behind an explicit experimental flag.

Verification:

- RFC vectors, dual-chain policy, stripping, rotation, and interop tests;
- signature fault and key-reuse checks.

Exit criteria:

- Stable modes use final standards and never present draft composite PKI as final.
- Focused `0.36.0` PQ PKI pentest and crypto review pass.

### 0.37.0 - Transit And PKI Completion

Goal: complete the planned stable cryptographic service surface.

Deliverables:

- Transit sign/verify, import/BYOK policy, export restrictions, and batch APIs;
- PKI ACME, multiple issuers, tidy, delta/full CRLs, and OCSP operations;
- migration, backup, performance budgets, and complete operator/API docs.

Verification:

- compatibility suites, migration tests, protocol fuzzing, and load bounds;
- key-import provenance and forbidden-export tests.

Exit criteria:

- All stable crypto operations have misuse-resistant APIs and recovery docs.
- Focused `0.37.0` complete crypto-surface pentest passes.

### 0.38.0 - KV v1, SSH, And TOTP Engines

Goal: add bounded compatibility and common credential services.

Deliverables:

- explicitly mounted KV v1 with documented lack of history and CAS guarantees;
- SSH OTP and CA signing with user/host roles;
- TOTP generation and validation with replay windows and secret isolation.

Verification:

- compatibility, replay, role escape, host/user confusion, and clock tests;
- engine isolation and audit-redaction fixtures.

Exit criteria:

- Compatibility engines cannot weaken KV v2, PKI, or identity defaults.
- Focused `0.38.0` KV v1, SSH, and TOTP pentest passes.

## Phase 6: Recovery, Operations, And Clustering

### 0.39.0 - Encrypted Backup And Migration

Goal: make backup, restore, and every pre-release format change recoverable.

Deliverables:

- streaming encrypted snapshot and dual-signed manifest format;
- restore preflight, isolated verification, vault binding, and authorization;
- resumable migration journal and fixtures from every admitted version.

Verification:

- fresh/wrong vault, interruption, corruption, disk-full, and rollback tests;
- restore drills from every prior persisted format.

Exit criteria:

- Restore never mutates live state before complete authenticated verification.
- Focused `0.39.0` backup, restore, and migration pentest passes.

### 0.40.0 - Rootless Wolfi Operations

Goal: operate the standalone binary and hardened Wolfi service safely.

Deliverables:

- non-root image, read-only filesystem, volume ownership, health, and readiness;
- metrics without sensitive dimensions, operational logs separate from audit;
- install, upgrade, recovery, systemd-user, and rootless Podman documentation.

Verification:

- rootless init/unseal/KV/restart/restore smoke and permission tests;
- core-dump, ptrace, secret-file, signal, and shutdown tests.

Exit criteria:

- The service survives documented restarts without needing root or hidden state.
- Focused `0.40.0` container and operations pentest passes.

### 0.41.0 - Deterministic Raft State Machine

Goal: replicate encrypted state through a deterministic consensus boundary.

Deliverables:

- encrypted command model, log/snapshot persistence, and deterministic apply;
- leader forwarding, consistency tokens, stale-read policy, and backpressure;
- no barrier key material in ordinary log values.

Verification:

- deterministic replay, failover, partition, stale read, and corruption tests;
- three-node destructive local cluster smoke.

Exit criteria:

- Raft surrounds the state machine and is not treated as a generic storage adapter.
- Focused `0.41.0` consensus and forwarding pentest passes.

### 0.42.0 - Raft Membership And Recovery

Goal: make cluster admission and membership changes explicit and recoverable.

Deliverables:

- signed join tokens, learner admission, and joint-consensus promotion/removal;
- cluster ID, membership epoch, role, address, nonce, and expiry bindings;
- encrypted snapshots, disaster recovery, and quorum-loss runbooks.

Verification:

- rogue join, replay, membership race, quorum loss, snapshot, and recovery tests;
- model checking for configuration transitions.

Exit criteria:

- No node gains voting authority outside a committed membership transition.
- Focused `0.42.0` cluster-membership pentest passes.

### 0.43.0 - Hybrid Cluster Identity

Goal: protect cluster confidentiality and authentication with independent
classical and PQ credentials.

Deliverables:

- pinned hybrid TLS KEX profile with explicit draft/version migration path;
- independent classical plus ML-DSA node identity and dual-signed membership;
- replicated minimum suite and no node-local classical fallback.

Verification:

- downgrade, stripping, impersonation, rotation, expiry, and DoS tests;
- independent cluster identity and transport review.

Exit criteria:

- Hybrid key exchange and quantum-resistant authentication are both enforced.
- Focused `0.43.0` cluster identity pentest and crypto review pass.

### 0.44.0 - Threshold Multi-Seal And Auto-Unseal

Goal: move secret zero behind provider-neutral threshold protection without
misrepresenting provider guarantees.

Deliverables:

- split seal KEK across independent KMS/HSM/TPM or external providers;
- challenge responses bound to vault, node, generation, provider, nonce, expiry;
- hybrid application envelope, downgrade rejection, recovery keys, and seal migration.

Verification:

- provider loss, replay, stale response, downgrade, quorum, and migration tests;
- per-provider end-to-end quantum-assurance documentation.

Exit criteria:

- One provider compromise cannot silently unseal or weaken configured policy.
- Focused `0.44.0` auto-unseal and secret-zero pentest passes.

### 0.45.0 - Rollback Detection And Checkpoints

Goal: detect valid whole-store rollback where deployment topology permits it.

Deliverables:

- hash-chained generations and classical plus ML-DSA signed checkpoints;
- Raft quorum, TPM/HSM monotonic, and external publication anchor interfaces;
- explicit standalone impossibility statement and operator alarm/recovery flow.

Verification:

- full snapshot rollback, checkpoint deletion, equivocation, and anchor outage tests;
- offline checkpoint verification tooling.

Exit criteria:

- Each deployment declares and tests its rollback-detection assurance level.
- Focused `0.45.0` rollback and checkpoint pentest passes.

### 0.46.0 - Replication And Multi-Cluster

Goal: add safe read scaling, disaster recovery, and multi-region foundations.

Deliverables:

- performance standbys/read replicas with consistency tokens;
- DR secondary promotion and activation workflow;
- performance replication and namespace/path filters with conflict policy.

Verification:

- lag, partition, promotion, replay, stale-read, filter escape, and failback tests;
- multi-cluster recovery drills.

Exit criteria:

- Replication never silently turns stale or filtered data into authoritative state.
- Focused `0.46.0` replication and DR pentest passes.

## Phase 7: Native Dynamic Adapters

### 0.47.0 - Native Adapter SDK

Goal: standardize dynamic credential lifecycle behind least-privilege adapters.

Deliverables:

- create, renew, revoke, rotate, rollback, and capability contracts;
- host-owned provider connections and opaque root-credential handles;
- optional Cargo features, dependency isolation, redaction, and test harness.

Verification:

- generic conformance, idempotency, outage, compensation, and leakage tests;
- compile matrices with every adapter independently enabled and disabled.

Exit criteria:

- An adapter cannot obtain master keys, global storage, or arbitrary network access.
- Focused `0.47.0` adapter-SDK boundary pentest passes.

### 0.48.0 - PostgreSQL Dynamic Secrets

Goal: deliver the first production native database adapter.

Deliverables:

- dynamic users, static roles, renew, revoke, root rotation, and statements;
- minimum management privileges and transaction/failure semantics;
- TLS, connection, username, TTL, and audit documentation.

Verification:

- rootless PostgreSQL smoke and full adapter conformance suite;
- partial create/revoke, outage, injection, race, and privilege tests.

Exit criteria:

- PostgreSQL credentials are always revocable or reported as unresolved with evidence.
- Focused `0.48.0` PostgreSQL adapter pentest passes.

### 0.49.0 - MySQL, MariaDB, And SurrealDB

Goal: expand SQL and multi-model support without pretending semantics are identical.

Deliverables:

- MySQL/MariaDB dynamic and static roles with root rotation;
- SurrealDB system-user rotation plus separately scoped record-access/JWT helpers;
- capability discovery and provider-specific limitation docs.

Verification:

- rootless service smokes and independent conformance suites;
- grant, revoke, TTL, injection, outage, and rollback tests per provider.

Exit criteria:

- Each backend declares its true transaction and revocation guarantees.
- Focused `0.49.0` database adapter pentest passes.

### 0.50.0 - MongoDB, Redis, Valkey, And RabbitMQ

Goal: support common document, cache, and messaging credentials.

Deliverables:

- MongoDB users and role grants;
- Redis/Valkey ACL creation or documented static rotation by server capability;
- RabbitMQ users, vhosts, tags, permissions, renewal, and revocation.

Verification:

- rootless provider smokes and independent conformance suites;
- ACL escape, stale user, partial revoke, outage, and audit tests.

Exit criteria:

- Unsupported dynamic behavior is explicit and never emulated insecurely.
- Focused `0.50.0` service-adapter pentest passes.

### 0.51.0 - AWS, Azure, And GCP Dynamic Secrets

Goal: manage major-cloud credentials where provider APIs permit safe lifecycle.

Deliverables:

- scoped credential creation, lease, revoke, rotate, and root recovery workflows;
- provider-specific identity, API, retry, eventual-consistency, and quota handling;
- cloud root-credential protection and least-privilege templates.

Verification:

- emulator/protocol fixtures plus controlled provider tests;
- partial revoke, retry, confusion, quota, and leaked-response tests.

Exit criteria:

- Provider ambiguity never reports a credential revoked without evidence.
- Focused `0.51.0` major-cloud adapter pentest passes.

### 0.52.0 - European Cloud And Platform Secrets

Goal: add Hetzner, DigitalOcean, Kubernetes, and LDAP credential lifecycle.

Deliverables:

- Hetzner and DigitalOcean project/token operations supported by their APIs;
- Kubernetes service-account/token secrets engine distinct from Kubernetes auth;
- LDAP credential management distinct from LDAP authentication.

Verification:

- provider protocol fixtures, rootless platform smokes, and conformance tests;
- scope, revocation, token leakage, outage, and eventual-consistency tests.

Exit criteria:

- Every provider documents exactly which lifecycle guarantees its API supports.
- Focused `0.52.0` cloud/platform adapter pentest passes.

### 0.53.0 - Adapter Certification

Goal: make adapter safety evidence machine-readable and non-self-asserted.

Deliverables:

- certification runner for create, renew, revoke, rotate, rollback, outage,
  idempotency, privileges, and audit redaction;
- signed capability metadata and independent result provenance;
- promotion policy for native and future Wasm adapters.

Verification:

- certification bypass, feature-flag, manifest-forgery, and stale-result tests;
- complete conformance matrix for all shipped adapters.

Exit criteria:

- No adapter is labeled stable without reproducible conformance evidence.
- Focused `0.53.0` adapter certification pentest passes.

## Phase 8: Process-Isolated Extensions

### 0.54.0 - Component ABI And Signed Manifests

Goal: define a narrow versioned extension contract before executing third-party code.

Deliverables:

- WIT/component ABI for auth, secrets, database, cloud, and notifications;
- dual classical/PQ signed manifests, hashes, SBOM, capabilities, and ABI ranges;
- publisher trust, transparency, revocation, and provenance model.

Verification:

- canonical ABI fixtures, manifest fuzzing, signature stripping, and revocation tests;
- incompatible ABI and unknown capability rejection.

Exit criteria:

- Signed provenance is verified but never represented as proof of safety.
- Focused `0.54.0` plugin supply-chain pentest passes.

### 0.55.0 - Restricted Plugin Worker

Goal: execute Wasmtime outside the vault process and key address space.

Deliverables:

- separate unprivileged worker with no WASI, inherited environment, filesystem,
  stdio, clock, randomness, or sockets by default;
- Linux namespaces, no-new-privileges, seccomp/Landlock, cgroups, and reduced-
  assurance profiles for other platforms;
- fuel, epoch, stack, memory, table, instance, output, call, and deadline limits.

Verification:

- escape regressions, resource exhaustion, worker crash, restart, and IPC fuzzing;
- exact Wasmtime-version and precompiled-artifact compatibility tests.

Exit criteria:

- Worker compromise does not share master-process memory or ambient authority.
- Focused `0.55.0` independent sandbox and worker-OS pentest passes.

### 0.56.0 - Capability Handles And Network Broker

Goal: expose only invocation-scoped high-level host operations to plugins.

Deliverables:

- opaque handles bound to plugin, mount, namespace, operation, epoch, and budget;
- host-owned provider connections and per-mount barrier storage views;
- SSRF-safe network broker validating DNS, resolved IPs, TLS, method, size, and deadlines.

Verification:

- confused deputy, reentrancy, stale handle, cross-mount, DNS rebinding, metadata,
  loopback, link-local, Unix-socket, and response-size tests.

Exit criteria:

- Plugins receive no raw root credentials, arbitrary sockets, or general crypto access.
- Focused `0.56.0` host-capability and SSRF pentest passes.

### 0.57.0 - Plugin Lifecycle And Certification

Goal: make extension installation, operation, upgrade, rollback, and revocation safe.

Deliverables:

- pin, install, enable, disable, upgrade, rollback, quarantine, and revoke workflows;
- structured audit-only host interface and secret-copy limitation docs;
- adapter conformance certification and compromised-publisher response.

Verification:

- rollback, downgrade, revoked signer, stale instance, cross-tenant, and recovery tests;
- third-party fixture plugins through the full lifecycle.

Exit criteria:

- Stable extension status requires sandbox, supply-chain, and conformance evidence.
- Focused `0.57.0` full extension-platform pentest passes.

## Phase 9: Operator Intelligence And Governance

### 0.58.0 - Secret Inventory

Goal: expose actionable metadata without creating a secret-existence oracle.

Deliverables:

- owner, engine, type, static/dynamic, access, expiry, rotation, and dependencies;
- namespace-aware inventory API with pagination and immutable snapshots;
- explicit unknown, unavailable, unsupported, and redacted states.

Verification:

- consistency across KV, leases, auth, adapters, and audit;
- enumeration, inference, stale index, authorization, and pagination tests.

Exit criteria:

- Inventory reveals no path or relationship beyond caller capability.
- Focused `0.58.0` inventory leakage pentest passes.

### 0.59.0 - Policy Simulator, Dry Run, And Developer Mode

Goal: explain and preview operations without granting mutation capability.

Deliverables:

- revision-pinned policy explanation with distinct `policy:explain` capability;
- read-only blast-radius engine for policy, mount, namespace, delete, revoke, and rotate;
- safe local developer profile with reset, samples, test PKI, and production guardrails.

Verification:

- simulator oracle, policy race, mutation-host-call, blast-radius, and guardrail tests;
- deterministic golden explanations and developer workflow smoke.

Exit criteria:

- Simulation and dry run cannot mutate state or disclose secret existence.
- Focused `0.59.0` simulator and developer-mode pentest passes.

### 0.60.0 - Leak Intake And Private Correlation

Goal: turn scanner findings into safe managed-secret response inputs.

Deliverables:

- authenticated finding schema with source, detector, confidence, and evidence hash;
- redacted storage that never persists raw leaked values;
- privacy-preserving correlation to secrets, leases, SecretIDs, tokens, and keys.

Verification:

- raw-secret rejection, false positive, collision, inference, replay, and race tests;
- scanner/CI integration fixtures.

Exit criteria:

- Correlation cannot become an oracle or a repository for leaked plaintext.
- Focused `0.60.0` leak-evidence and correlation pentest passes.

### 0.61.0 - Rotation Readiness And Lifecycle Webhooks

Goal: make leak and lifecycle findings actionable and auditable.

Deliverables:

- readiness states for automatic, manual, blocked, unsupported, ownerless, and
  non-revocable secrets;
- lease/revoke/rotate workflows with approvals where required;
- signed, replay-safe created/read/rotated/expiring/revoked/leak/denied webhooks.

Verification:

- scoring matrix, rotation race, retry, idempotency, signing, and delivery tests;
- no raw secret appears in payloads or logs.

Exit criteria:

- Automation never claims successful rotation without verification evidence.
- Focused `0.61.0` rotation and webhook pentest passes.

### 0.62.0 - Human Approval Controls

Goal: require policy-selected quorum approval for sensitive operations.

Deliverables:

- pending request, approve, deny, cancel, expiry, and quorum evaluator;
- security/DBA, multi-maintainer, namespace-admin, and custom role rules;
- integration for unwrap, root access, rekey, mount/namespace delete, and rotation.

Verification:

- replay, quorum mismatch, role change, timeout, cancellation, and audit-failure tests;
- immutable operation/context binding.

Exit criteria:

- Approval for one operation cannot authorize another or survive expiry.
- Focused `0.62.0` approval-bypass pentest passes.

### 0.63.0 - Break-Glass Recovery

Goal: provide time-limited emergency authority with forensic accountability.

Deliverables:

- quorum, reason, bounded scope, expiry, forced marking, and privilege rollback;
- emergency token isolation, revocation, post-incident summary, and evidence bundle;
- recovery path for audit-degraded and partial-outage scenarios.

Verification:

- replay, scope escape, clock, privilege retention, audit loss, and rollback tests;
- destructive incident drills.

Exit criteria:

- Emergency authority expires and leaves independently verifiable evidence.
- Focused `0.63.0` break-glass abuse pentest passes.

### 0.64.0 - Tamper-Evident Evidence Bundles

Goal: make security events independently verifiable after incidents.

Deliverables:

- append-only audit hash chain and classical plus ML-DSA checkpoints;
- redacted bundles for incidents, approvals, break-glass, leaks, rotations, and policy;
- offline verifier, external publication, archival, and lifecycle replay APIs.

Verification:

- deletion, reorder, mutation, truncation, key compromise, rollback, and import tests;
- deterministic offline verification fixtures.

Exit criteria:

- Evidence tampering or incomplete history is detected and reported explicitly.
- Focused `0.64.0` evidence-integrity pentest passes.

## Phase 10: Broad Parity And Integrations

### 0.65.0 - Transform, KMIP, And Key Management

Goal: add specialized key and data-protection services behind isolated listeners.

Deliverables:

- format-preserving transform, masking, and tokenization after crypto review;
- KMIP listener, managed objects, scopes, roles, and certificate identity;
- KMS/TDE key-management adapters with explicit provider guarantees.

Verification:

- format-domain, collision, misuse, protocol fuzz, role, and key-lifecycle tests;
- independent transform and KMIP security review.

Exit criteria:

- Specialized protocols cannot bypass normal identity, policy, audit, or key isolation.
- Focused `0.65.0` Transform/KMIP/key-management pentest passes.

### 0.66.0 - Advanced Policy And Operator APIs

Goal: complete governance and day-two system controls.

Deliverables:

- deterministic CEL-style policy extension and group-policy propagation;
- rate and lease-count quotas, locked-user, audit elision, monitor, and log tuning APIs;
- disabled-by-default protected profiling and raw-storage access remains absent.

Verification:

- policy determinism, quota race, elision leakage, unlock, and profiling auth tests;
- system API compatibility fixtures.

Exit criteria:

- Operational controls cannot weaken mandatory audit or expose raw storage.
- Focused `0.66.0` advanced-policy and operator-API pentest passes.

### 0.67.0 - Clients, Agent, Proxy, And Auto-Auth

Goal: let applications consume the stable API without embedding ambient authority.

Deliverables:

- Rust client and generated clients from the OpenAPI contract;
- separate agent/proxy for auto-auth, caching, renewal, and templates;
- bounded local IPC, sink permissions, token handoff, and shutdown behavior.

Verification:

- client compatibility, cache isolation, template injection, IPC, and token theft tests;
- Linux, macOS, Windows, and BSD smoke where supported.

Exit criteria:

- Client helpers cannot become an unaudited alternate control plane.
- Focused `0.67.0` client and agent/proxy pentest passes.

### 0.68.0 - Platform Reconciliation And Secret Sync

Goal: integrate Lykilheim with deployment tooling without normalizing secret sprawl.

Deliverables:

- Kubernetes operator, CSI/injector, and scoped synchronization workflows;
- Terraform provider or explicitly tested compatibility layer;
- destination allowlists, drift detection, GitOps config reconciliation, and no
  secret payloads in declarative source by default.

Verification:

- namespace escape, stale sync, drift race, destination auth, and rollback tests;
- rootless local platform integration smoke.

Exit criteria:

- Sync destinations and reconcilers receive only explicitly authorized secrets.
- Focused `0.68.0` platform-integration and secret-sync pentest passes.

### 0.69.0 - Experimental Security Integrations

Goal: isolate promising TEE, ZKP, and eBPF work without weakening stable behavior.

Deliverables:

- attestation-bound key release experiments with explicit trust roots;
- zero-knowledge authentication predicates with bounded proof verification;
- eBPF audit export with durable journal remaining the security gate.

Verification:

- compile/runtime feature isolation and fallback-rejection tests;
- attestation replay, proof DoS, verifier soundness, and audit-loss tests.

Exit criteria:

- Experimental features are disabled by default and cannot alter stable guarantees.
- Focused `0.69.0` experimental-feature pentest passes before any preview label.

## Phase 11: Stable Qualification

### 0.70.0 - Portability And Reproducible Artifacts

Goal: qualify native binaries and Wolfi artifacts across supported environments.

Deliverables:

- Linux x86_64/aarch64, macOS x86_64/aarch64, Windows x86_64/aarch64 where
  supported, and practical BSD build/test evidence;
- hardened Linux-only Wolfi image, signed SBOM, provenance, and checksums;
- deterministic release scripts and platform assurance/limitation matrix.

Verification:

- clean-VM build and smoke on each target;
- reproducibility comparison, artifact signing, install, upgrade, and rollback tests.

Exit criteria:

- Artifacts are traceable to the exact source and require no developer-local state.
- Focused `0.70.0` supply-chain and cross-platform pentest passes.

### 0.71.0 - API And Feature-Parity Closeout

Goal: close every planned Vault/OpenBao inventory item before compatibility freeze.

Deliverables:

- complete system backend subset, OpenAPI endpoint, path help, and version history;
- every parity item marked implemented, experimental, or deliberately different;
- complete user, operator, API, config, migration, recovery, and limitations docs.

Verification:

- endpoint inventory and compatibility fixtures against documented behavior;
- documentation link, example, stale-version, and unsupported-behavior audits.

Exit criteria:

- No feature is silently missing, implied compatible, or undocumented.
- Focused `0.71.0` API inventory and parity pentest passes.

### 0.72.0 - Composed Security Campaign

Goal: test interactions across all admitted trust boundaries at production scale.

Deliverables:

- continuous parser/format fuzzing, Miri, Loom, property, and model campaigns;
- destructive storage, audit, seal, token, lease, adapter, plugin, and Raft testing;
- resource, latency, memory, recovery, and denial-of-service budgets.

Verification:

- long-running fuzz corpus and model-check reports;
- full multi-node, multi-namespace, adapter, plugin, backup, and recovery scenario.

Exit criteria:

- No security-critical boundary first receives fuzzing or failure injection here;
  this campaign proves their composition.
- Focused `0.72.0` full-system pentest passes for the exact release commit.

### 0.73.0 - Stable Release Candidate Freeze

Goal: freeze features and compatibility with no new production behavior.

Deliverables:

- final API, storage, audit, plugin ABI, crypto-suite, and migration contracts;
- all accepted residual risks, deprecations, experimental features, and platform
  limitations documented;
- complete release candidate runbooks, evidence, SBOM, provenance, and artifacts.

Verification:

- clean-checkout release gates and migration from every pre-release format;
- full portability, Wolfi, API, recovery, replication, and reproducibility matrix.

Exit criteria:

- Only release-blocking fixes and documentation corrections may follow.
- Independent `0.73.0` release-candidate pentest passes for the exact commit.

## 1.0.0 - First Stable Release

Goal: release a documented, production-qualified, API-driven secrets manager
whose stable profile is quantum-resistant under current cryptanalytic knowledge.

Deliverables:

- all stable pre-1.0 capabilities, migration paths, and compatibility contracts;
- mandatory hybrid/PQ production profile for protected roles with no silent
  classical-only fallback;
- standalone binaries, hardened Wolfi image, complete documentation, recovery
  runbooks, SBOM, provenance, checksums, signatures, and signed release evidence;
- experimental features remain isolated, disabled by default, and outside the
  stable compatibility promise.

Verification:

- every repository, release, portability, migration, compatibility, backup,
  restore, Raft, adapter, plugin, and disaster-recovery gate from a clean checkout;
- independent review of cryptography, storage, memory, audit, token/lease,
  policy, cluster identity, extensions, and supply chain;
- all findings fixed or explicitly documented as accepted residual risk.

Exit criteria:

- The exact release candidate has green CI, reproducible artifacts, complete
  evidence, no unresolved release blocker, and maintainer sign-off.
- STOP: run the final independent `1.0.0` pentest for the exact commit; only
  after it passes may the signed `v1.0.0` tag and release be published.
