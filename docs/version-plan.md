# Lykilheim Version Plan

Status: normative implementation order

Lykilheim is a from-scratch Rust secrets manager targeting a defensible,
crypto-agile, quantum-resistant European vault. `1.0.0` is the first stable
release. All product work is split into small pre-1.0 milestones so trust
boundaries can be reviewed, migrated, documented, and pentested independently.

Tags use:

```text
v0.N.0      milestone release
v0.N.P      patch or security-fix release
v1.0.0      first stable production release
```

The list is intentionally granular and is not a maximum. Unreleased milestones
may be split when that gives one owner and one testable security boundary.
Released versions are never renumbered.

## Project Rules

- Project name: Lykilheim.
- License: EUPL-1.2.
- Toolchain: Rust 1.96.1, pinned in `rust-toolchain.toml`.
- Delivery targets: portable standalone compiled binary and rootless Podman on Wolfi.
- Portability target: Linux, macOS, Windows, and BSD-style Unix for the binary;
  the hardened Wolfi container remains Linux-only.
- API model: fully API-driven; CLIs and integrations are clients, not a second
  control plane.
- Code layout: focused crates or modules aligned to trust boundaries, with
  explicit dependency direction and isolated unsafe/platform code.
- Documentation: every feature, endpoint, configuration key, format, deployment,
  security boundary, failure mode, migration, and operator workflow is documented
  to the same practical standard as Fluxheim before it is done.
- Release notes: every milestone gets
  `release-notes/RELEASE_NOTES_X.Y.Z.md` before implementation begins; evidence,
  limitations, checksums, and signatures are completed before tagging.
- Security posture: fail closed when authorization, audit durability,
  cryptographic policy, storage integrity, or required memory guarantees fail.
- Quantum claim: use "quantum-resistant under current cryptanalytic knowledge,
  crypto-agile, and hybrid while standards and implementations mature." Never
  claim that an algorithm or deployment is unconditionally quantum-proof.
- CI posture: GitHub CodeQL default setup only; no advanced CodeQL workflow.
- Dependency posture: use maintained crates after license and security review;
  run update, audit, deny, and SBOM gates for every release.
- Parity posture: keep `docs/feature-parity.md` synchronized so every Vault and
  OpenBao capability is implemented, scheduled, experimental, or deliberately
  different with operator impact documented.

## Stable Profiles

All profiles share sealed storage, mandatory authorization, durable audit, and
safe defaults. Optional profiles are additive Cargo features and runtime policy;
they must not enlarge the default core dependency graph or ambient authority.

- `core`: local or Raft storage, seal/barrier, audit, policy, tokens, leases,
  identity, AppRole, userpass, KV v2, Transit, backup, and recovery.
- `quantum-resistant`: core plus hybrid API and cluster transport, PQ Transit and
  PKI, PQ checkpoints, and minimum authentication-assurance policy.
- `extended`: broad auth, external storage, native adapters, Wasm extensions,
  replication, operator intelligence, KMIP/Transform, and sync integrations.

Every binary reports its compiled features, active profile, provider assurance,
and unavailable capabilities. Feature combinations have compile and runtime
tests; a disabled optional component cannot weaken core behavior.

## Inherited Security Contracts

Every milestone inherits these requirements from the first boundary that can
use them:

- hard size, cardinality, depth, work, concurrency, and retention limits ship
  with each resource; later quota APIs make safe limits configurable;
- mutation APIs use idempotency keys, optimistic revisions/ETags, retained replay
  results, cancellation rules, snapshot pagination tokens, and explicit
  `IndeterminateCommit` where applicable;
- secret classes define maximum size, permitted owners, copies, lifetime,
  logging/redaction, clearing, and platform limitations;
- secret-bearing configuration uses protected files/file descriptors, system
  credential facilities, interactive input, or wrapped internal references;
  environment variables are weaker documented mode, and CLI arguments are forbidden;
- native outbound access uses the central destination-capability broker;
- persisted formats are canonical, bounded, authenticated, versioned, fuzzed,
  and have migration or explicit rejection fixtures;
- after `0.65.0`, every stateful component declares and tests backup preflight,
  snapshot/export, restore/import, vault/namespace rebinding, migration, and
  post-restore verification, or explicitly rejects backup participation;
- backup admission inventories operation states, audit outboxes, idempotency
  results, token lookup epochs, lease compensation, provider handles, plugin
  state, and non-exportable hardware dependencies; components that cannot meet
  configured recovery objectives fail admission rather than silently remain out;
- threat model, control map, API/config docs, feature parity, release notes,
  recovery, and known limitations change with the implementation.

## Roadmap Authority

This document is the normative implementation order. README summaries,
feature-parity labels, release-note placeholders, architecture, and security
model must be synchronized during `0.2.0` and before each affected milestone.
Old post-1.0 and `2.0.0` placeholders are superseded by the pre-1.0 milestones
below and are not implementation authority.

## Mandatory Release Gate

Every release candidate must pass:

- `scripts/checks.sh` and its matching release gate once added;
- formatting, clippy with warnings denied, unit, integration, documentation,
  property, fuzz, model, and failure-injection tests proportional to scope;
- `cargo deny check bans licenses sources` and `cargo audit`;
- API and format compatibility fixtures for every admitted contract;
- Linux, macOS, Windows, and practical BSD checks for changed platform boundaries;
- rootless Podman smoke tests for container or service changes;
- matching documentation, release notes, SBOM, provenance, checksums,
  signatures, and clean release evidence from the exact source commit.

Every version has an explicit exit criterion requiring this stop:

```text
vX.Y.Z implementation stop reached. Freeze scope and run the focused pentest
for this exact commit. Do not tag or begin the next version until findings are
fixed or accepted as documented residual risk and CI is green.
```

## Phase 0: Released Foundation

### 0.1.0 - Repository Foundation

Goal: establish the Rust crate, governance, initial boundaries, and release process.

Deliverables:

- initial API, config, error, audit, crypto, and storage modules;
- pinned toolchain, dependency policy, CI, Wolfi placeholders, and local gates;
- threat model, API shape, portability, feature audit, and release documentation.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_1_gate.sh`

Exit criteria:

- Foundation scope and non-implemented security boundaries are explicit.
- Focused `0.1.0` pentest passes for the exact tagged commit.

## Phase 1: Architecture And Foundational Services

### 0.2.0 - Workspace, Profiles, And Boundary Reset

Goal: replace prospective bootstrap interfaces before secret-bearing work starts.

Deliverables:

- workspace/crate graph, allowed dependency directions, and isolated platform code;
- skeleton async/capability storage, crypto-broker, audit, and extension interfaces;
- core/quantum-resistant/extended feature policy and forbidden combinations;
- synchronized README, feature parity, architecture, security model, and release plan.

Verification:

- dependency-direction, feature-matrix, compile-fail, and stale-document tests;
- old synchronous traits are removed or formally deprecated without consumers.

Exit criteria:

- ADRs and skeleton interfaces supersede every bootstrap trust boundary.
- Focused `0.2.0` architecture and feature-isolation pentest passes.

### 0.3.0 - Canonical IDs And Paths

Goal: give routing, policy, audit, and storage one canonical identity model.

Deliverables:

- typed vault, namespace, mount, object, key, revision, and operation IDs;
- segment-aware paths with defined decoding, separators, Unicode, and limits;
- keyed or random opaque storage identifiers hiding logical path shape.

Verification:

- cross-platform vectors, parser properties, traversal, collision, and limit tests.

Exit criteria:

- Every subsystem consumes the same typed canonical identity.
- Focused `0.3.0` parser and path-confusion pentest passes.

### 0.4.0 - Wire Suite Identity

Goal: define cryptographic wire identity before any persisted format uses it.

Deliverables:

- fixed canonical suite-ID encoding, reserved ranges, and allocation policy;
- unknown/disabled-suite behavior, key-purpose distinction, format version, and
  minimum-reader/verifier semantics;
- provider-neutral KEM, signature, AEAD, hash, and KDF family identifiers.

Verification:

- canonical vectors, unknown/reserved ID, downgrade, and allocation tests.

Exit criteria:

- Persisted formats can authenticate suite identity without choosing providers.
- Focused `0.4.0` suite-encoding and downgrade pentest passes.

### 0.5.0 - Storage Capability Contract

Goal: define authoritative storage across materially different consistency models.

Deliverables:

- revision reads, CAS, atomic batches, and durable acknowledgment;
- snapshot-consistent pagination and caller-provided or streaming read buffers;
- typed conflict, unavailable, corrupt, unsupported, and indeterminate errors;
- backend capability declarations and hard key/value/batch limits;
- typed `RequiredStorageCapabilities` covering linearizable CAS, atomic batch,
  durable acknowledgment, snapshot reads, transactional audit outbox, migration
  locking, and deterministic iteration;
- startup and mount admission intersect requirements with backend capabilities;
  unsupported compositions fail before routes are reachable and are never emulated.

Verification:

- conformance model, admission matrix, linearizable CAS, partial, duplicate,
  delayed commit, and forbidden weak-emulation tests.

Exit criteria:

- Local, SQL, SurrealDB, and Raft semantics can be declared honestly.
- Focused `0.5.0` storage-contract pentest passes.

### 0.6.0 - Authenticated Record Format

Goal: specify bounded opaque records before implementing durable storage.

Deliverables:

- immutable superblock, fixed header, segments, and commit manifest;
- authenticated suite, root `NamespaceId`, object, generation, key epoch, lengths,
  and tombstone on every persisted object from the first schema;
- encrypted logical index, optional size padding, and migration rules;
- `no_std` parser core returning borrowed header/segment views and using
  caller-owned fixed scratch buffers.

Verification:

- format vectors, parser fuzzing, checked arithmetic, allocator-counting proof of
  allocation-free parsing, truncation, reorder, substitution, downgrade, and
  corruption tests.

Exit criteria:

- The record parser performs zero dynamic allocations and reveals no logical paths.
- Focused `0.6.0` format and tampering pentest passes.

### 0.7.0 - Secret Taxonomy And Secure Sources

Goal: define handling rules for all secret data and bootstrap credentials.

Deliverables:

- taxonomy for keys, tokens, SecretIDs, passwords, MFA seeds, dynamic credentials,
  KV/Transit plaintext, PKI/TLS keys, provider credentials, and plugin I/O;
- per-class size, owner, copy, lifetime, logging, and clearing rules;
- `SecretSource` for protected files/FDs, system credentials, interactive input,
  wrapped references, and explicitly weaker environment variables.

Verification:

- config redaction, debug, CLI rejection, source-permission, and size-bound tests.

Exit criteria:

- No secret-bearing config type is debug-serializable or accepted as a CLI argument.
- Focused `0.7.0` secret-source and configuration pentest passes.

### 0.8.0 - Locked Secret Memory

Goal: protect decoded application-owned plaintext with honest platform guarantees.

Deliverables:

- locked fixed-capacity arenas using reviewed `sanitization` facilities;
- direct bounded decoding into secret memory and high-assurance failure policy;
- dump, fork, swap, ptrace, guard, and platform-assurance documentation.

Verification:

- allocation, residue, lock-failure, bounds, Miri, and platform tests.

Exit criteria:

- Decoded share/key material and application-owned plaintext avoid the ordinary
  application heap; bounded short-lived transport copies are documented.
- Focused `0.8.0` memory-handling pentest passes.

### 0.9.0 - Clock And Entropy Services

Goal: centralize time and CSPRNG behavior before state machines depend on them.

Deliverables:

- monotonic durations, persisted wall time, boot/time epoch, uncertainty, and
  backward-jump degraded mode;
- deterministic test clock;
- brokered entropy with health/failure/fork detection and direct secret-buffer fill.

Verification:

- backward/forward jump, reboot, uncertainty, entropy failure, fork, and race tests.

Exit criteria:

- No security subsystem reads ambient clock or randomness directly.
- Focused `0.9.0` time and entropy pentest passes.

### 0.10.0 - Configuration Lifecycle

Goal: make configuration changes atomic, revisioned, and secret-safe.

Deliverables:

- typed schemas, source precedence, immutable/reloadable fields, and revisions;
- parse, validate, stage, atomically apply, rollback, and failure state machine;
- `SecretSource` refresh, certificate/provider credential reload, and redacted
  diagnostics without partially applied configuration.

Verification:

- precedence, stale revision, concurrent reload, invalid secret refresh, rollback,
  crash, and immutable-field tests.

Exit criteria:

- A failed reload leaves the prior complete configuration active.
- Focused `0.10.0` configuration lifecycle pentest passes.

### 0.11.0 - Operation State Contract

Goal: define long-running operation semantics without persistence or public APIs.

Deliverables:

- `Pending`, `Running`, `CancelRequested`, `Succeeded`, `Failed`, and
  `Indeterminate` states and legal transitions;
- typed owner/context/revision references, idempotency identity, progress,
  result, error, cancellation, takeover, and external-effect classifications;
- deterministic transition model shared by rewrap, migration, backup/restore,
  PKI tidy, mass revocation, sync, and compromise response;
- no storage, authorization, identity, audit implementation, or public route.

Verification:

- exhaustive transition/property model, duplicate intent, cancellation race,
  takeover precondition, terminal immutability, and indeterminate-effect tests.

Exit criteria:

- Every operation family can map effects to one closed state transition contract.
- Focused `0.11.0` operation-state-contract pentest passes.

### 0.12.0 - Crypto Broker And Hierarchical Keys

Goal: prevent engines from receiving master keys or broad decrypt authority.

Deliverables:

- opaque generation-fenced non-cloneable handles and purpose-separated keys;
- per-namespace, per-mount, per-object/version, and per-token wrapped DEKs;
- bounded priority queues with reserved seal, audit, rotation, and emergency capacity;
- cancellation and queue-exhaustion cleanup for buffers, operations, and handles;
- atomic `Sealing` transition that rejects new secret work, cancels or drains
  admitted work, and bumps the broker generation before handle reuse;
- clearing of arenas, queued inputs/outputs, and provider sessions followed by
  invalidation of every handle, including explicit indeterminate external effects.

Verification:

- compile-fail capability, starvation, cancellation, stale-handle, cross-tenant,
  cryptographic deletion, provider-session, queue cleanup, and seal-race tests.

Exit criteria:

- Transit/PQ load cannot starve administrative security operations; after the
  seal fence commits, no pre-fence handle can perform or complete crypto work.
- Focused `0.12.0` key hierarchy and broker-scheduling pentest passes.

### 0.13.0 - Foundational Database Egress

Goal: provide destination-scoped database connectivity before external storage.

Deliverables:

- destination handles, DNS/rebinding and address policy, TLS roots/pins, proxies,
  pools, deadlines, body/frame limits, and redacted diagnostics;
- database protocol/driver capability boundary with retry and idempotency classes;
- loopback, link-local, metadata, private-range, and Unix-socket policy with
  explicit local deployment exceptions.

Verification:

- SSRF, DNS rebinding, proxy bypass, pool exhaustion, timeout, retry, and
  credential-leakage tests.

Exit criteria:

- External storage cannot instantiate arbitrary network or database clients.
- Focused `0.13.0` foundational database-egress pentest passes.

### 0.14.0 - Durable Local Storage Backend

Goal: implement a crash-safe single-node authoritative backend.

Deliverables:

- atomic publication, fsync policy, lock ownership, tombstones, and compaction;
- snapshot iteration and platform-specific durability profiles;
- conformance evidence distinguishing persistence from dynamic provider adapters.

Verification:

- crash-point, disk-full, torn-write, stale-lock, recovery, and platform tests.

Exit criteria:

- A committed record survives documented crashes or reports indeterminate state.
- Focused `0.14.0` local-storage pentest passes.

### 0.15.0 - PostgreSQL Storage Backend

Goal: implement PostgreSQL as authoritative Lykilheim persistence.

Deliverables:

- serializable transactions, revision CAS, atomic batches, and snapshot pagination;
- durability acknowledgment, schema migration, and `SecretSource` credentials;
- explicit failover, timeout, and indeterminate-commit behavior through brokered egress.

Verification:

- rootless PostgreSQL conformance, fault, failover, migration, and rollback tests.

Exit criteria:

- PostgreSQL storage passes the same opaque-record contract as local storage.
- Focused `0.15.0` PostgreSQL storage-backend pentest passes.

### 0.16.0 - SurrealDB Storage Backend

Goal: implement SurrealDB as authoritative Lykilheim persistence.

Deliverables:

- transaction/revision mapping, atomic batches, snapshot pagination, and migrations;
- capability declarations for unsupported consistency or durability semantics;
- `SecretSource` credentials and brokered outage/retry policy.

Verification:

- rootless SurrealDB conformance, fault, migration, and indeterminate-commit tests.

Exit criteria:

- SurrealDB never claims guarantees its selected deployment mode lacks.
- Focused `0.16.0` SurrealDB storage-backend pentest passes.

### 0.17.0 - Provider Registry And Crypto Migration

Goal: bind suite policy to providers without hard-coding one algorithm family.

Deliverables:

- provider capability discovery, self-tests, assurance labels, and closed policy;
- alternate standardized KEM/signature slots; the initial hash-based hedge is
  exactly FIPS 205 `SLH-DSA-SHAKE-256s` for low-frequency offline roots and
  checkpoints, with other parameter sets forbidden until separately registered;
- each suite pins the standard publication/revision, incorporated errata set,
  provider version, and cryptographic digest of its accepted test-vector corpus;
- protected-profile floor of 256-bit barrier/DEK keys, quantum-appropriate
  hash/KDF outputs, approved AEAD tags, and no 128-bit-only suite;
- downgrade, provider replacement, key-version, and algorithm migration states.

Verification:

- provider differential, KAT, unknown/disabled provider, floor, and migration tests.

Exit criteria:

- Requests cannot choose arbitrary algorithms, providers, or silent fallback.
- Focused `0.17.0` crypto-agility and symmetric-floor pentest passes.

## Phase 2: Seal And Barrier

### 0.18.0 - Symmetric Barrier Core

Goal: encrypt authenticated segmented records behind the broker.

Deliverables:

- AEAD barrier with domain-separated nonce derivation, authenticated write-key
  incarnation, object generation, key epoch, and segment identity;
- nonce-misuse-resistant suite required when no trustworthy monotonic state exists;
- restored or cloned vault remains mutation-disabled until it commits a fresh
  write-key/incarnation epoch that cannot overlap the source nonce namespace;
- indeterminate retries replay identical committed ciphertext for the same
  idempotency result; changed plaintext always requires a new nonce namespace;
- compaction copies authenticated ciphertext and rewrap changes only DEK wrapping;
  any re-encryption creates a new object generation/incarnation;
- bounded in-place encrypt/decrypt paths and sealed-state gate;
- indistinguishable authentication and corruption failures.

Verification:

- KAT, crash/indeterminate retry, restore, rollback, clone, compaction, rewrap,
  same/different-plaintext replay, nonce uniqueness, tamper, and oracle tests.

Exit criteria:

- No plaintext reaches storage, and no restore/clone can resume writes in an old
  key/nonce namespace.
- Focused `0.18.0` barrier pentest and cryptographic review pass.

### 0.19.0 - Internal Durable Operation Runtime

Goal: persist and recover the operation-state contract behind the encrypted barrier.

Deliverables:

- encrypted operation records, progress, terminal results, idempotency indexes,
  ownership references, and bounded retention/concurrency;
- atomic state CAS, restart recovery, safe takeover, cancellation, and
  `Indeterminate` external-effect reconciliation;
- internal integration for rewrap and future migration/backup/tidy/revocation/sync;
- authorization, identity/policy revision, and audit hook points only; public
  operation routes remain disabled until audit and policy milestones bind them.

Verification:

- crash at every persistence transition, duplicate start, stale revision,
  cancel/takeover race, barrier failure, result retention, and reconciliation tests.

Exit criteria:

- Internal operations recover without duplicate effects; no public operation API
  exists before durable audit and policy authorization are available.
- Focused `0.19.0` internal durable-operation runtime pentest passes.

### 0.20.0 - Initialization And Shamir Shares

Goal: atomically initialize a root-namespaced vault and split its seal secret.

Deliverables:

- entropy health handling and locked-memory seal-secret generation;
- independent seal and recovery share sets bound to vault, purpose, generation,
  threshold, total, and index;
- wrapped immutable keyring and authenticated reconstruction canary;
- materialized root namespace and mandatory `NamespaceId` on every persisted object;
- one CAS lifecycle: `Uninitialized -> CryptoInitializedPendingBootstrap -> Active`;
- crash/resume/cancel rules for pending bootstrap, which exposes no normal routes
  and can only resume the same initialization or securely cancel before activation.

Verification:

- seal/recovery independence, threshold, entropy, duplicate, corruption,
  concurrent init, pending crash/resume/cancel, namespace, and residue tests.

Exit criteria:

- Initialization creates both share sets and one root namespace, then remains
  `CryptoInitializedPendingBootstrap` until the first authority commits.
- Focused `0.20.0` initialization and share-handling pentest passes.

### 0.21.0 - Bounded Unseal Lifecycle

Goal: reconstruct keys through one replay-resistant expiring attempt.

Deliverables:

- collector bound to vault, nonce, seal generation, TTL, and hard attempt limits;
- fixed slots, direct secret-memory decoder, verification, clearing, and cancellation;
- fixed-length binary unseal content type and local console/Unix-socket option;
- documented bounded TLS/socket/HTTP encoded transport copies.

Verification:

- replay, mixed generation, timeout, concurrency, crash, oracle, and residue tests.

Exit criteria:

- Invalid submissions reveal no share validity and leave no reusable partial state.
- Focused `0.21.0` unseal transport and lifecycle pentest passes.

### 0.22.0 - Crash-Safe Shamir Rekey

Goal: change seal shares without mixing generations or crash ambiguity.

Deliverables:

- idle, old-quorum, pending-new, committed, and cancelled states;
- generation-bound verification, hard attempt limits, and cancellation;
- modelled crash and restart behavior.

Verification:

- crash at every transition, stale share, replay, cancel, and model tests.

Exit criteria:

- Recovery accepts one explicitly committed seal-share generation only.
- Focused `0.22.0` Shamir rekey pentest passes.

### 0.23.0 - Recovery Share Lifecycle

Goal: manage recovery shares independently from seal shares and recovery tokens.

Deliverables:

- rekey, threshold change, cancellation, loss handling, and generation binding
  for the recovery share set created during `0.20.0`;
- locked collector memory, duplicate/replay rules, and auto-unseal migration contract;
- explicit distinction among recovery shares, seal shares, and generated recovery tokens.

Verification:

- mixed-share, stale generation, threshold, loss, rekey crash, collector, and migration tests.

Exit criteria:

- Recovery-share operations cannot unseal directly or mix with a seal-share generation.
- Focused `0.23.0` recovery-share lifecycle pentest passes.

### 0.24.0 - Barrier Rotation And Rewrap

Goal: separate rotation, rewrap, rekey, and suite migration lifecycles.

Deliverables:

- immediate new-write epoch and old decrypt-only epochs;
- bounded resumable rewrap with progress evidence;
- pause, restart, rollback, retirement, and cryptographic deletion rules.

Verification:

- concurrent I/O, crash, mixed epoch, rollback, retirement, and migration tests.

Exit criteria:

- Rotation cannot strand ciphertext or reactivate retired write keys.
- Focused `0.24.0` rotation and rewrap pentest passes.

## Phase 3: API, Transport, Audit, And Policy

### 0.25.0 - Canonical API And Concurrency Contract

Goal: define one bounded API pipeline before exposing network endpoints.

Deliverables:

- typed routes, request IDs, body limits, stable errors, and one-pass normalization;
- idempotency keys, ETags/revisions, replay result retention, cancellation,
  pagination snapshot tokens, and `IndeterminateCommit`;
- global `Compiled -> Configured -> PrerequisitesSatisfied -> Reachable` route gate;
- per-route TLS, rate, audit, auth, policy, seal-state, and profile prerequisites;
- local/test-only listeners until every declared prerequisite is satisfied.

Verification:

- malformed HTTP, normalization, replay, concurrency, pagination, and smuggling tests.

Exit criteria:

- Every mutation/page has explicit retry semantics, and no listener makes a route
  reachable merely because the socket is bound.
- Focused `0.25.0` API parser and idempotency pentest passes.

### 0.26.0 - Native API TLS

Goal: secure every network-accessible client endpoint before secret APIs ship.

Deliverables:

- reviewed rustls/provider choice and TLS 1.3-only production policy;
- certificate reload, SNI/listener separation, trusted proxies, and client IP rules;
- HTTP/2 limits, connection deadlines, TLS-key secret memory, and downgrade policy.

Verification:

- protocol downgrade, SNI, reload, proxy spoofing, slow connection, and H2 DoS tests.

Exit criteria:

- TLS completion satisfies only the transport prerequisite; it cannot expose init,
  unseal, login, token, or secret routes before rate, audit, auth, and policy gates.
- Focused `0.26.0` public API transport pentest passes.

### 0.27.0 - Bounded Layered Rate Control

Goal: replace unbounded process-local IP limiting with configurable controls.

Deliverables:

- fixed-point refill, shared clock, bounded sharded state, and eviction;
- global, network, endpoint, credential, token, and namespace layers;
- trusted proxy, IPv6 aggregation, cardinality caps, and durable lockout separation.

Verification:

- deterministic time, spoofing, cardinality, fairness, eviction, and DoS tests.

Exit criteria:

- Untrusted clients cannot cause unbounded limiter work or memory growth.
- Focused `0.27.0` rate-control pentest passes.

### 0.28.0 - Typed Durable Audit Journal

Goal: durably record security intent without blocking async executors.

Deliverables:

- typed intent/result schema, redaction registry, sequence, actor, policy, and outcome;
- per-record MAC, previous-record digest, journal epoch, and startup chain verification;
- separate audit-authentication and sensitive-value hashing key handles, with rotation;
- explicit corruption/truncation state and keyed hashing of sensitive identifiers;
- bounded writer subprocess for the high-assurance profile; an isolated pool is
  permitted only in documented lower-assurance development deployments;
- main process canonicalizes/redacts, performs value hashing and record MAC with
  broker-held keys, then sends immutable record bytes; audit keys never cross IPC;
- authenticated sequence-bound IPC with per-launch subprocess identity/session,
  bounded frames, queue integrity, and impersonation/replay rejection;
- ACK binds journal epoch, sequence, and record digest and means the record and
  required metadata have completed durable fsync, never merely queued;
- restart handshake reconciles last durable sequence/digest; key rotation changes
  main-process journal/value-hash epochs without transferring key custody;
- seal ordering closes request admission, emits a final authenticated seal record,
  and waits boundedly for durable ACK before key clearing; manual seal still
  completes if audit is unavailable and records an incomplete terminal state for
  startup reconciliation rather than retaining secret authority;
- a late ACK after timeout may repair journal state but can never release the
  timed-out mutation/secret response or automatically clear `AuditUnavailable`;
- authoritative durable local journal; remote sinks consume an asynchronous outbox
  unless configured as additional acknowledgers and never replace the local journal;
- one-way `AuditUnavailable` gate and restricted repair state.

Verification:

- disk/queue full, fsync, timeout, panic, MAC, truncation, reorder, key rotation,
  IPC replay/impersonation/corruption, queued-versus-fsynced ACK, late ACK, seal,
  subprocess death/restart handshake, and redaction tests.

Exit criteria:

- Writer failure cannot starve health or manual seal control paths.
- Focused `0.28.0` audit durability and leakage pentest passes.

### 0.29.0 - Fail-Closed Audit Transaction Gate

Goal: prevent mutation or secret release before required audit evidence commits.

Deliverables:

- bounded intent, operation, result, and commit/release pipeline;
- transactional mutation plus audit outbox where supported;
- withheld responses and idempotent asynchronous sink export.

Verification:

- failure injection at every gate and mutation/audit atomicity tests.

Exit criteria:

- Audit failure blocks secret-bearing reads and security mutations.
- Focused `0.29.0` audit bypass and partial-commit pentest passes.

### 0.30.0 - Policy Compiler And Evaluator

Goal: enforce deterministic default-deny capabilities over canonical requests.

Deliverables:

- closed typed capability vocabulary and complete route-to-capability mapping;
- segment wildcards, deny precedence, typed parameter constraints, unknown-field
  rejection, and immutable policy revisions;
- shadowing warnings, system-path restrictions, and hard rule/work budgets;
- deterministic evaluator with no ambient clock, network, or filesystem input;
- decision binding to policy, identity/group, namespace, mount, assurance, and
  configuration revisions, revalidated at mutation commit or secret release;
- bind the internal durable-operation runtime to authorization and audit, then
  enable only its explicitly mapped public operation routes.

Verification:

- route coverage, unknown capability/field, typed constraint, golden matrix,
  durable-operation authorization/audit, fuzzing, revision TOCTOU, overlap,
  escape, and budget tests.

Exit criteria:

- Every protected operation has a typed capability and releases effects only
  while its complete decision context remains current.
- Focused `0.30.0` policy bypass and ambiguity pentest passes.

### 0.31.0 - Mounts And Barrier Views

Goal: isolate engines by immutable mount identity and lifecycle contract.

Deliverables:

- enable, disable, tune, remount, conflict, revision, and hard mount limits;
- per-mount barrier views and KEKs with no cross-mount traversal;
- lease-revocation contract for integration after the lease engine exists.

Verification:

- conflict, race, cross-view, remount, deletion, and cryptographic-erasure tests.

Exit criteria:

- A mount cannot access another mount's storage, keys, namespace, or authority.
- Focused `0.31.0` mount-isolation pentest passes.

## Phase 4: Tokens, Bootstrap, Leases, And Static Secrets

### 0.32.0 - Token Engine

Goal: issue independently random tokens with atomic lifecycle semantics.

Deliverables:

- versioned 256-bit bearer, keyed lookup digest, and random accessor;
- service tokens with child, orphan, periodic, accessor, renewal, and revocation rules;
- optional short-lived stateless batch tokens with embedded namespace, policy revision,
  issuance epoch, expiry, and explicit denial of renewal, children, Cubbyhole, accessors;
- bounded batch deny set plus token/policy/namespace epoch invalidation, with explicitly
  weaker individual revocation guarantees than service tokens;
- lookup-key epoch in token prefix and old keyed-digest keys retained until every
  corresponding token expires, is replaced, or is revoked;
- maximum lookup-key epoch lifetime; periodic/service renewal crossing retirement
  atomically issues a client-visible replacement under the active epoch and revokes
  the old token, otherwise renewal fails closed;
- remaining-token counts and metadata by lookup epoch without bearer disclosure;
- bounded epoch-wide revocation as final retirement fallback, with ordinary
  rotation distinct from forced compromise retirement and fully audited;
- immutable namespace, parent, TTL, policy, per-parent, and per-namespace bounds.

Verification:

- forgery, lookup-key rotation/retirement, indefinitely periodic replacement,
  forced epoch revoke, bearer-free enumeration, prefix, accessor, batch
  epoch/TTL/deny-set, clock rollback, renewal, cascade, and race tests.

Exit criteria:

- Service-token revocation wins every race; batch-token exceptions are bounded,
  short-lived, policy-visible, and never described as ordinary individual revocation.
- Focused `0.32.0` token pentest passes.

### 0.33.0 - Bootstrap, Root, And Recovery Tokens

Goal: complete initialization and quorum-controlled administrative bootstrap.

Deliverables:

- pending-bootstrap revision CAS preventing concurrent completion races;
- local-only bootstrap channel or one-time out-of-band nonce;
- atomic first policy/admin and initial root token issuance;
- generate-root, recovery-token, cancel, replay, expiry, and root revocation states;
- atomic `CryptoInitializedPendingBootstrap -> Active` transition only after the
  first policy/admin and root authority are durably committed and audited.

Verification:

- concurrent init, nonce theft, partial bootstrap, quorum, cancel, and revoke tests.

Exit criteria:

- Pending bootstrap becomes `Active` exactly once; crash or retry cannot issue a
  second initial authority or expose normal routes early.
- Focused `0.33.0` bootstrap/root/recovery pentest passes.

### 0.34.0 - Lease And Expiration Engine

Goal: make dynamic leases durable, idempotent, bounded, and recoverable.

Deliverables:

- issue, renew, revoke, cascade, intent, result, and compensation states;
- bounded expiration worker, active/pending limits, and restart recovery;
- mount disable/remount revocation integration and synchronous expiry checks;
- deterministic fake dynamic provider covering create, renew, revoke, partial
  create, compensation, outage, idempotency, and indeterminate upstream effects.

Verification:

- simulated time and fake-provider outage/partial-create/compensation/renew/revoke,
  cascade, mount, restart, idempotency, and race tests.

Exit criteria:

- Cleanup delay or crash cannot extend durable authorization.
- Focused `0.34.0` lease lifecycle pentest passes.

### 0.35.0 - Response Wrapping And Cubbyhole

Goal: provide at-most-once delivery and token-private storage after tokens exist.

Deliverables:

- random wrapping token, independent payload DEK, lookup, rewrap, and unwrap;
- atomic consumed state, expiry, hard pending limits, and network failure semantics;
- Cubbyhole rooted by immutable token ID with a per-token DEK;
- wrapped-DEK tombstone and atomic deletion of the only active wrapped Cubbyhole
  DEK on token revocation, with backup/replica retention documented.

Verification:

- replay, race, disconnect, rewrap, isolation, quota, DEK deletion, retained
  backup, and cleanup tests.

Exit criteria:

- Wrapping is at-most-once and Cubbyhole is cross-token isolated.
- Focused `0.35.0` wrapping and Cubbyhole pentest passes.

### 0.36.0 - KV v2

Goal: deliver bounded versioned static secret storage.

Deliverables:

- CAS, versions, metadata, list, per-secret versions, and total-byte bounds;
- soft delete and undelete with a distinct per-object/per-version DEK hierarchy;
- cryptographic destroy through atomic deletion of the only usable wrapped DEK
  and an authenticated wrapped-DEK tombstone;
- explicit SSD, copy-on-write, replica, snapshot, and backup retention limitations.

Verification:

- compatibility, CAS, history, metadata, single-version DEK destruction,
  unaffected sibling versions, tombstone, backup, replica, and retention tests.

Exit criteria:

- Destroy removes active logical recovery while documenting physical limitations.
- Focused `0.36.0` KV v2 pentest passes.

### 0.37.0 - Identity Entities And Groups

Goal: attach policy to bounded stable identities rather than login records.

Deliverables:

- entities, aliases, groups, metadata, merge, and policy attachment;
- immutable IDs, cycle/depth/membership limits, and visited-set evaluation;
- audit-safe resolution and alias uniqueness.

Verification:

- cycle, depth, cardinality, alias collision, merge, isolation, and race tests.

Exit criteria:

- Group traversal cannot grant cross-identity authority or exhaust evaluation.
- Focused `0.37.0` identity and group pentest passes.

### 0.38.0 - Namespace Isolation

Goal: enforce hierarchical tenant isolation and cryptographic deletion.

Deliverables:

- child namespaces, root-only paths, hard limits, and policy inheritance;
- namespace-bound identity, token, mount, storage, lease, audit, and KEKs;
- delegated administration and deletion workflow.

Verification:

- cross-tenant, deletion, inheritance, confused-deputy, and limit tests.

Exit criteria:

- No tenant can enumerate or operate on another tenant's resources.
- Focused `0.38.0` namespace isolation pentest passes.

### 0.39.0 - AppRole Authentication

Goal: provide replay-resistant machine authentication.

Deliverables:

- RoleID and SecretID TTL, uses, CIDR, accessor, wrapping, and revocation;
- bounded atomic consumption and role-bound issuance;
- bootstrap, rotation, least-privilege, and redaction workflows.

Verification:

- replay, race, CIDR/proxy, brute-force, quota, and revoke tests.

Exit criteria:

- SecretID consumption and token issuance cannot partially commit.
- Focused `0.39.0` AppRole pentest passes.

### 0.40.0 - Userpass And Password Policy

Goal: support local human authentication with production guardrails.

Deliverables:

- Argon2id policy, generated-password rules, lockout, unlock, and recovery;
- identity/MFA hooks, bounded users/attempts, redaction, and dev-only defaults.

Verification:

- offline cost, brute force, lockout DoS, enumeration, and migration tests.

Exit criteria:

- User enumeration and lockout cannot bypass policy or disclose credentials.
- Focused `0.40.0` userpass pentest passes.

## Phase 5: Native Egress And Authentication Assurance

### 0.41.0 - HTTP And Cloud Egress Expansion

Goal: extend foundational database egress for HTTP auth, cloud, and providers.

Deliverables:

- HTTP method/path schemas, redirects, proxy policy, TLS roots/pins, and response types;
- cloud metadata-service denial by default with provider-auth-specific capability;
- body/response limits, pools, deadlines, retry/idempotency classes, and diagnostics;
- OIDC/JWKS, webhooks, cloud auth, adapters, checkpoints, and sync must reuse it.

Verification:

- SSRF, DNS rebinding, proxy bypass, redirect, pool exhaustion, and retry tests.

Exit criteria:

- Native HTTP/cloud integrations cannot instantiate arbitrary outbound clients.
- Focused `0.41.0` egress and SSRF pentest passes.

### 0.42.0 - Authentication Assurance Model

Goal: let policy distinguish how strongly an identity was authenticated.

Deliverables:

- local-secret, classical assertion, hybrid/PQ-bound, hardware-attested, and
  MFA/approval-strengthened assurance labels;
- immutable result binding to identity, method, issuer, context, and expiry;
- minimum-assurance policy constraints for protected roles.

Verification:

- label escalation, method confusion, stale assertion, and policy matrix tests.

Exit criteria:

- Successful authentication cannot imply assurance the upstream did not provide.
- Focused `0.42.0` assurance-label pentest passes.

### 0.43.0 - JWT Authentication

Goal: validate bounded JWT assertions through explicit issuer policy.

Deliverables:

- static/JWKS keys, algorithms, claims, audiences, clocks, and issuer binding;
- egress-brokered JWKS fetch, rotation, cache limits, and assurance labels.

Verification:

- algorithm/key confusion, JWKS SSRF/rotation, replay, clock, and claim tests.

Exit criteria:

- Tokens cannot select validation algorithms, keys, namespaces, or assurance.
- Focused `0.43.0` JWT pentest passes.

### 0.44.0 - OIDC Authentication

Goal: support browser/device login without weakening JWT validation.

Deliverables:

- discovery pinning, authorization code/device flows, PKCE, state, nonce, and callback;
- bounded sessions, issuer mapping, logout, and identity aliases.

Verification:

- state/nonce replay, callback confusion, discovery SSRF, mix-up, and session tests.

Exit criteria:

- OIDC flow authority remains bound to its configured issuer and callback.
- Focused `0.44.0` OIDC pentest passes.

### 0.45.0 - TLS Certificate Authentication

Goal: authenticate mTLS clients independently from API transport setup.

Deliverables:

- chain, SAN, EKU, name constraints, CRL/OCSP policy, and role mapping;
- certificate rotation, revocation caching, bounded chains, and assurance result.

Verification:

- chain confusion, revoked cert, SAN/EKU bypass, cache, and mapping tests.

Exit criteria:

- Transport success alone never grants certificate-auth identity.
- Focused `0.45.0` TLS certificate-auth pentest passes.

### 0.46.0 - Kubernetes Authentication

Goal: authenticate Kubernetes workloads within explicit trust domains.

Deliverables:

- TokenReview and offline service-account JWT modes through native egress;
- cluster, audience, namespace, service account, cache, and assurance binding.

Verification:

- replay, stale token, cluster confusion, reviewer compromise, and outage tests.

Exit criteria:

- Kubernetes identity cannot escape its configured cluster trust domain.
- Focused `0.46.0` Kubernetes-auth pentest passes.

### 0.47.0 - Cloud Workload Authentication

Goal: authenticate AWS, Azure, and GCP workloads without ambient credentials.

Deliverables:

- separate provider protocols through native egress and metadata protection;
- account/project/tenant, audience, role, nonce, replay, and assurance binding.

Verification:

- metadata SSRF, confused deputy, stale identity, replay, and provider outage tests.

Exit criteria:

- Cloud identity is bound to an explicit provider trust domain and role.
- Focused `0.47.0` cloud-workload-auth pentest passes.

### 0.48.0 - LDAP Authentication

Goal: add bounded LDAP user/group authentication.

Deliverables:

- TLS, bind/search strategy, filters, group mapping, lockout, and assurance labels;
- egress destination controls and bounded result/cardinality handling.

Verification:

- filter injection, group escalation, downgrade, outage, and enumeration tests.

Exit criteria:

- LDAP results cannot inject identity, groups, namespace, or policy.
- Focused `0.48.0` LDAP-auth pentest passes.

### 0.49.0 - Kerberos Authentication

Goal: add channel-bound Kerberos authentication after dependency review.

Deliverables:

- realm/service mapping, replay cache, channel binding, delegation policy, and limits;
- identity aliases, assurance result, and keytab `SecretSource` handling.

Verification:

- replay, downgrade, delegation, realm confusion, and keytab leakage tests.

Exit criteria:

- Kerberos delegation cannot silently become broader vault authority.
- Focused `0.49.0` Kerberos-auth pentest passes.

### 0.50.0 - RADIUS Authentication

Goal: add bounded RADIUS authentication with explicit transport limitations.

Deliverables:

- server, shared-secret, attribute, timeout, replay, and failover policy;
- identity mapping, rate control, assurance labels, and MFA interaction.

Verification:

- replay, response spoofing, downgrade, failover, and enumeration tests.

Exit criteria:

- RADIUS limitations and assurance are visible to policy and operators.
- Focused `0.50.0` RADIUS-auth pentest passes.

### 0.51.0 - GitHub Authentication

Goal: support bounded GitHub operator auth where OIDC is not suitable.

Deliverables:

- organization/team mapping, token verification, egress policy, and cache bounds;
- identity aliases, revocation, rate-limit, and assurance behavior.

Verification:

- org/team confusion, stale membership, token leakage, outage, and cache tests.

Exit criteria:

- GitHub availability or stale membership cannot silently grant authority.
- Focused `0.51.0` GitHub-auth pentest passes.

### 0.52.0 - Multi-Factor Authentication

Goal: strengthen identities with policy-bound second factors.

Deliverables:

- TOTP and WebAuthn-style factors, enrollment, recovery, replay, and lifecycle;
- bounded challenges, assurance elevation, anti-phishing distinctions, and audit.

Verification:

- enrollment takeover, replay, downgrade, recovery abuse, and factor-removal tests.

Exit criteria:

- MFA assurance cannot outlive or detach from the authenticated identity/session.
- Focused `0.52.0` MFA pentest passes.

## Phase 6: Cryptographic Engines And Protected Transport

### 0.53.0 - Transit Classical Baseline

Goal: provide symmetric cryptographic services without exporting raw keys.

Deliverables:

- create, encrypt, decrypt, rewrap, rotate, version, hash, HMAC, and random;
- derived/datakey APIs, bounded batches, contexts, and opaque handles;
- convergent encryption disabled by default.

Verification:

- KAT, misuse, context, old-version, oracle, rotation, and fuzz tests.

Exit criteria:

- Transit never exports keys or silently weakens policy.
- Focused `0.53.0` Transit pentest and crypto review pass.

### 0.54.0 - Post-Quantum Provider Baseline

Goal: admit production-capable PQ providers without structural monoculture.

Deliverables:

- ML-KEM-768/1024 and ML-DSA-65/87 protected profiles;
- ML-DSA-44 supported only in an explicit compatibility profile and forbidden
  for protected infrastructure identity, root, checkpoint, and long-retention roles;
- FIPS 205 `SLH-DSA-SHAKE-256s` offline root/checkpoint profile and
  provider-neutral alternate-family extension slots;
- KATs, differential/self-tests, locked generation, fault checks, and DoS budgets.

Verification:

- NIST vectors, malformed input, implicit rejection, differential, and fault tests.

Exit criteria:

- PQ providers fail closed and remain replaceable by suite policy.
- Focused `0.54.0` independent PQ-provider pentest and crypto review pass.

### 0.55.0 - Hybrid Transit Envelopes

Goal: combine classical and PQ protection without downgrade paths.

Deliverables:

- pre-implementation ADR freezing one exact published NIST/IETF hybrid combiner:
  specification revision, errata, KDF/hash, component order/encoding, domain labels,
  output length, and test-vector corpus; no XOR or ad hoc concatenation combiner;
- canonical length-prefixed transcript binding both secrets, both public keys,
  ML-KEM ciphertext, suite, recipient, key version, and application context;
- X25519 all-zero rejection and indistinguishable decapsulation errors;
- dual classical/PQ signatures requiring both under hybrid policy.

Verification:

- combiner/transcript vectors, non-canonical encoding, component order, stripping,
  malformed component, context swap, downgrade, and KATs.

Exit criteria:

- Hybrid policy never accepts either component alone or retries classical-only.
- Focused `0.55.0` hybrid-envelope pentest and crypto review pass.

### 0.56.0 - Hybrid Public API Transport

Goal: protect ordinary client-to-vault traffic with the quantum-resistant profile.

Deliverables:

- pinned X25519+ML-KEM TLS KEX profile and exact draft/version migration path;
- mandatory protected-profile PQ server authentication: native PQ TLS identity
  when available, otherwise an application-layer PQ signature over the TLS
  exporter/transcript, server identity, listener, nonce, and negotiated suite;
- protected profile requiring hybrid transport with no classical fallback.

Verification:

- downgrade, exporter/transcript substitution, missing PQ authentication,
  stripping, interop, load, certificate, resumption, and rotation tests.

Exit criteria:

- Protected secret endpoints reject connections below configured hybrid assurance.
- Focused `0.56.0` public hybrid-transport pentest and crypto review pass.

### 0.57.0 - Classical PKI Core

Goal: issue constrained classical certificates through isolated PKI keys.

Deliverables:

- root/intermediate issuers, CSR signing, SAN/EKU/path/TTL roles;
- issuer rotation, CRL, OCSP, revocation, and bounded issuance.

Verification:

- chain, role, SAN, path, expiry, revocation, rotation, and interop tests.

Exit criteria:

- Issuance cannot exceed issuer, role, namespace, or policy constraints.
- Focused `0.57.0` classical PKI pentest and crypto review pass.

### 0.58.0 - Quantum-Resistant PKI

Goal: provide standards-based PQ and hybrid certificate modes.

Deliverables:

- RFC 9881 ML-DSA issuers, leaves, and CRLs with HashML-DSA prohibited for
  certificates, CRLs, OCSP, and issuance;
- parallel classical/PQ chains cryptographically bound to the same identity,
  role, validity interval, policy revision, and issuance transaction;
- composite draft mode isolated behind an experimental feature.

Verification:

- RFC vectors, HashML-DSA rejection, chain-binding mismatch, stripping,
  dual-chain, rotation, fault, and interop tests.

Exit criteria:

- Stable modes use final standards and never present draft PKI as final.
- Focused `0.58.0` PQ PKI pentest and crypto review pass.

### 0.59.0 - Transit Completion

Goal: complete the stable Transit surface independently from PKI.

Deliverables:

- sign/verify, import/BYOK, export restrictions, batch, derivation, and migration;
- performance budgets, backup/recovery, misuse-resistant API, and full docs.

Verification:

- compatibility, import provenance, forbidden export, batch, and migration tests.

Exit criteria:

- Every Transit operation has explicit key lifecycle and misuse semantics.
- Focused `0.59.0` Transit-completion pentest passes.

### 0.60.0 - PKI Completion

Goal: complete the stable PKI protocol and issuer surface.

Deliverables:

- ACME, multiple issuers, tidy, delta/full CRLs, and OCSP operations;
- migration, backup, recovery, performance bounds, and operator/API docs.

Verification:

- protocol fuzzing, compatibility, migration, load, and recovery tests.

Exit criteria:

- PKI protocol behavior is bounded, documented, and independently recoverable.
- Focused `0.60.0` PKI-completion pentest passes.

### 0.61.0 - Hardware And Non-Exportable Providers

Goal: support opaque keys whose provider lifecycle differs from software keys.

Deliverables:

- PKCS#11/HSM/TPM handles for Transit, PKI, audit, cluster, and seal use;
- capability discovery, sessions, PIN sources, failover, outage, and provenance;
- provider-specific rotate, backup, import/export, and deletion semantics.

Verification:

- mock/real device conformance, session exhaustion, failover, PIN, and outage tests.

Exit criteria:

- Software assumptions never cause export or loss of non-exportable keys.
- Focused `0.61.0` hardware-provider pentest passes.

### 0.62.0 - KV v1 Engine

Goal: add bounded KV v1 compatibility without weakening KV v2.

Deliverables:

- explicit mount type, payload limits, list, delete, and no-history semantics;
- migration and operator warnings distinguishing absent CAS/history.

Verification:

- compatibility, isolation, size, migration, and accidental-mount tests.

Exit criteria:

- KV v1 limitations cannot silently apply to KV v2 paths.
- Focused `0.62.0` KV v1 pentest passes.

### 0.63.0 - SSH Secrets Engine

Goal: issue bounded SSH OTP and CA credentials.

Deliverables:

- OTP and CA signing with user/host roles, principals, TTLs, and revocation;
- key isolation, audit, compatibility, and hard issuance limits.

Verification:

- role escape, host/user confusion, replay, principal, and expiry tests.

Exit criteria:

- SSH credentials cannot exceed role or policy constraints.
- Focused `0.63.0` SSH-engine pentest passes.

### 0.64.0 - TOTP Secrets Engine

Goal: generate and validate isolated TOTP credentials.

Deliverables:

- key generation/import, validation windows, replay state, rotation, and export policy;
- secret taxonomy integration and per-key attempt limits.

Verification:

- clock, replay, brute force, seed leakage, import, and rotation tests.

Exit criteria:

- Accepted codes cannot be replayed outside documented policy.
- Focused `0.64.0` TOTP-engine pentest passes.

## Phase 7: Recovery, Operations, And Clustering

### 0.65.0 - Encrypted Backup And Migration

Goal: make every admitted persisted state recoverable.

Deliverables:

- streaming encrypted snapshots and dual-signed manifests;
- restore preflight, isolated verification, vault binding, and authorization;
- resumable migrations from every prior format and filtered namespace backup;
- backup-participant registry and initial inventory for operation records,
  audit/outbox state, idempotency results, token epochs, leases/compensation,
  provider handles, and non-exportable hardware dependency manifests;
- admission check against configured recovery point/time and restore objectives.

Verification:

- wrong vault, omitted participant, unsupported recovery objective, non-exportable
  dependency, interruption, corruption, disk full, rollback, and restore drills.

Exit criteria:

- Restore never mutates live state before complete authenticated verification.
- Focused `0.65.0` backup and migration pentest passes.

### 0.66.0 - Rootless Wolfi Operations

Goal: operate native and hardened Wolfi deployments safely.

Deliverables:

- non-root image, read-only filesystem, volume ownership, health, and readiness;
- non-sensitive metrics, separate operational logs, upgrade/recovery, and systemd docs;
- core-dump, ptrace, secret-file, signal, and shutdown policy.

Verification:

- rootless init/unseal/KV/restart/restore and permission smokes.

Exit criteria:

- Service operation needs neither root nor hidden developer-local state.
- Focused `0.66.0` container and operations pentest passes.

### 0.67.0 - Raft Consensus Core And Deterministic State Machine

Goal: implement a complete persistent Raft core around deterministic encrypted apply.

Deliverables:

- persistent current term and voted-for with crash-safe ordering;
- pre-vote, election, AppendEntries conflict resolution/log truncation,
  check-quorum, and leadership transfer;
- linearizable reads through ReadIndex; wall-clock leader leases are forbidden;
- hard-state/log/commit fsync ordering before acknowledgment;
- atomic snapshot installation bound to last index/term, membership configuration,
  storage format, and cryptographic incarnation;
- corrupt/truncated hard-state, log, and snapshot detection/recovery;
- bounded transport messages, append batches, in-flight replication windows,
  peer queues, and snapshot chunks;
- leader-finalized encrypted commands with nonce/ciphertext chosen before replication;
- deterministic follower apply, log/snapshot persistence, and backpressure;
- no barrier keys or follower-generated randomness in state-machine apply.

Verification:

- model tests for elections, split votes, partitions, stale leaders, ReadIndex,
  transfer, AppendEntries conflicts, snapshot races, and membership changes;
- fsync crash points, corrupt/truncated log recovery, bounded-message/window DoS,
  deterministic replay, failover, and three-node conformance tests.

Exit criteria:

- Hard state and committed logs survive documented crashes, linearizable reads do
  not depend on wall time, and identical commits produce identical follower state.
- Focused `0.67.0` Raft consensus-core pentest passes.

### 0.68.0 - Cluster Trust Bootstrap And Hybrid Identity

Goal: establish authenticated node identity before protected Raft networking.

Deliverables:

- offline/in-process Raft conformance remains the only mode before this release;
- pinned hybrid TLS KEX with explicit draft/version migration;
- independent classical/PQ node credentials and dual-signed trust bootstrap;
- replicated minimum suite and cheap pre-verification DoS controls;
- two-phase suite upgrades: node capability advertisement, voter readiness,
  committed minimum-suite transition, credential overlap, rollback window, and
  refusal rules that cannot strand an unready voting quorum.

Verification:

- downgrade, stripping, impersonation, bootstrap, mixed-version readiness,
  stranded-voter prevention, suite rollback, identity rotation, expiry, and load tests.

Exit criteria:

- No forwarding, network join, or key-package recipient is trusted without hybrid identity.
- Focused `0.68.0` cluster trust and identity pentest passes.

### 0.69.0 - Forwarded Identity And Cache Coherence

Goal: preserve original client authority across leader forwarding.

Deliverables:

- identity-authenticated forwarded client, request, policy, namespace, TLS, and audit context;
- follower anti-forgery credentials and leader reauthorization rules;
- token revocation, policy change, identity merge, and seal-generation cache invalidation.

Verification:

- forged forwarding, stale cache, replay, context swap, and leadership-change tests.

Exit criteria:

- Followers cannot forge client identity or bypass leader authorization/audit.
- Focused `0.69.0` forwarding and cache-coherence pentest passes.

### 0.70.0 - Raft Membership And Recovery

Goal: make identity-authenticated admission and membership transitions recoverable.

Deliverables:

- dual-signed join tokens, learner admission, and joint-consensus promote/remove;
- cluster, node, both public keys, role, address, nonce, expiry, and epoch binding;
- encrypted snapshots, quorum-loss, and disaster-recovery runbooks.

Verification:

- rogue join, replay, identity mismatch, race, quorum loss, snapshot, and model tests.

Exit criteria:

- Voting authority changes only through identity-verified committed joint consensus.
- Focused `0.70.0` Raft membership pentest passes.

### 0.71.0 - Cluster Key Distribution And Seal Coordination

Goal: distribute and revoke keyring access per identity-authenticated node.

Deliverables:

- node-specific hybrid-wrapped key packages bound to both node identities;
- identity fencing and denial of future packages/epochs on removal;
- removal-triggered write-key rotation and documented blast radius for keys and
  ciphertext already exposed to a compromised node;
- cooperative node seal/clearing separated from adversarial removal, which cannot
  remotely force a malicious process to erase memory;
- cluster-wide seal/unseal coordination and sealed follower/unsealed leader rules.

Verification:

- wrong recipient, compromised/removed node, retained old key, fresh write epoch,
  stale package, cooperative seal, partial unseal, leader change, replay, and race tests.

Exit criteria:

- Removed nodes cannot participate, receive future epochs, or authorize new work;
  removal fences identity and rotates write keys while prior exposure remains
  documented. Cooperative sealing clears local state but is not claimed for attackers.
- Focused `0.71.0` cluster key-distribution pentest passes.

### 0.72.0 - Threshold Multi-Seal And Auto-Unseal

Goal: move secret zero behind provider-neutral threshold protection.

Deliverables:

- protected profile requires threshold `t >= 2` across unique provider authorities
  and failure domains, with no duplicate shares under one controlling KMS identity;
- KEK shares across independent KMS/HSM/TPM providers and explicit independence proof;
- challenge binding to vault, node, generation, provider, nonce, and expiry;
- hybrid envelope, recovery keys, seal migration, and no downgrade to one-provider unseal.

Verification:

- shared-authority/failure-domain rejection, duplicate provider, provider loss,
  replay, stale response, one-provider downgrade, quorum, and migration tests.

Exit criteria:

- One provider compromise cannot silently unseal or weaken policy.
- Focused `0.72.0` auto-unseal and secret-zero pentest passes.

### 0.73.0 - Rollback Detection And Checkpoints

Goal: detect valid whole-store rollback where topology permits it.

Deliverables:

- hash-chained generations and classical/PQ/hash-based signed checkpoints;
- Raft quorum, hardware monotonic, and external publication anchors;
- explicit standalone impossibility statement and recovery flow.

Verification:

- snapshot rollback, checkpoint deletion, equivocation, outage, and verifier tests.

Exit criteria:

- Every deployment declares and tests its rollback-detection assurance.
- Focused `0.73.0` rollback and checkpoint pentest passes.

### 0.74.0 - Compromise Response And Trust Replacement

Goal: recover from suspected key or trust-root compromise, not only planned rotation.

Deliverables:

- emergency state machines for audit authentication/value-hash keys, token lookup
  epochs, API TLS and cluster identities, barrier epochs, and recovery shares;
- Transit keys, PKI issuers, plugin publishers, hardware providers, external KMS
  identities, and attestation/trust-root replacement workflows;
- revoke, fence, reissue, rewrap, re-encrypt, republish, evidence, and client-impact rules;
- bounded degraded modes, blast-radius inventory, and provider/operator runbooks.

Verification:

- stolen-key, forged identity, compromised publisher/provider, partial replacement,
  rollback, stale cache, outage, and multi-key incident drills.

Exit criteria:

- Compromised authority can be fenced and replaced without ordinary rotation assumptions.
- Focused `0.74.0` compromise-response pentest passes.

### 0.75.0 - Replication And Multi-Cluster

Goal: add safe read scaling, DR, and multi-region replication.

Deliverables:

- standbys/read replicas with consistency tokens;
- DR promotion/activation and performance replication;
- namespace/path filters with explicit conflict and stale-read policy.

Verification:

- lag, partition, promotion, replay, filter escape, failback, and recovery drills.

Exit criteria:

- Stale or filtered replicas cannot silently become authoritative.
- Focused `0.75.0` replication and DR pentest passes.

## Phase 8: Native Dynamic Provider Adapters

### 0.76.0 - Native Adapter SDK

Goal: standardize dynamic credential lifecycle behind least privilege.

Deliverables:

- create, renew, revoke, rotate, rollback, and capability contracts;
- host-owned egress connections and opaque management credentials;
- feature isolation, hard operation limits, redaction, and conformance harness;
- native adapters classified as trusted computing base: first-party or explicitly
  reviewed only, with forbidden/default-deny unsafe and FFI policy;
- untrusted third-party adapters must use the process-isolated Wasm path.

Verification:

- idempotency, outage, compensation, leakage, unsafe/FFI policy, provenance,
  third-party rejection, and feature-matrix tests.

Exit criteria:

- Native adapters receive no broad authority through supported APIs but remain
  explicitly trusted in-process code; untrusted code cannot load natively.
- Focused `0.76.0` adapter-SDK pentest passes.

### 0.77.0 - PostgreSQL Dynamic Provider

Goal: manage PostgreSQL credentials, distinct from PostgreSQL storage.

Deliverables:

- dynamic users, static roles, renew, revoke, root rotation, and statements;
- minimum privileges, transactions, limits, TLS, and failure semantics.

Verification:

- rootless conformance, injection, partial revoke, race, and outage tests.

Exit criteria:

- Unresolved revocation is reported with durable evidence.
- Focused `0.77.0` PostgreSQL provider pentest passes.

### 0.78.0 - MySQL And MariaDB Dynamic Provider

Goal: manage MySQL/MariaDB credentials through provider-specific semantics.

Deliverables:

- dynamic/static roles, root rotation, statements, grants, and revocation;
- version capability discovery and least-privilege docs.

Verification:

- rootless conformance, injection, grant, revoke, race, and outage tests.

Exit criteria:

- MySQL-family behavior is not inferred from PostgreSQL semantics.
- Focused `0.78.0` MySQL/MariaDB provider pentest passes.

### 0.79.0 - SurrealDB Dynamic Provider

Goal: manage SurrealDB credentials separately from authoritative storage.

Deliverables:

- system-user rotation and separately scoped record-access/JWT helpers;
- capability discovery, limits, revocation, and failure semantics.

Verification:

- rootless conformance, scope, replay, revoke, and outage tests.

Exit criteria:

- Record-access helpers cannot inherit system-user authority.
- Focused `0.79.0` SurrealDB provider pentest passes.

### 0.80.0 - MongoDB Dynamic Provider

Goal: manage MongoDB users and role grants.

Deliverables:

- dynamic/static users, roles, TTL, revoke, root rotation, and limits;
- topology/version capability discovery and least-privilege docs.

Verification:

- rootless conformance, role escape, stale user, and outage tests.

Exit criteria:

- MongoDB role scope cannot exceed configured provider policy.
- Focused `0.80.0` MongoDB provider pentest passes.

### 0.81.0 - Redis And Valkey Dynamic Provider

Goal: manage ACL users where server capabilities permit it.

Deliverables:

- ACL create/rotate/revoke or explicit static-only fallback by version;
- command/key/channel scope, limits, and capability discovery.

Verification:

- rootless conformance, ACL escape, stale user, downgrade, and outage tests.

Exit criteria:

- Unsupported dynamic behavior is explicit and never insecurely emulated.
- Focused `0.81.0` Redis/Valkey provider pentest passes.

### 0.82.0 - RabbitMQ Dynamic Provider

Goal: manage RabbitMQ users, vhosts, tags, and permissions.

Deliverables:

- create, renew, revoke, rotate, permission, vhost, and limit behavior;
- management API egress, least privilege, and audit redaction.

Verification:

- rootless conformance, vhost escape, stale user, and outage tests.

Exit criteria:

- RabbitMQ credentials cannot exceed role-scoped vhost permissions.
- Focused `0.82.0` RabbitMQ provider pentest passes.

### 0.83.0 - AWS Dynamic Provider

Goal: manage scoped AWS credentials where APIs permit safe lifecycle.

Deliverables:

- credential create/lease/revoke/rotate, eventual consistency, and quotas;
- root credential protection and least-privilege templates.

Verification:

- emulator/provider protocol, partial revoke, retry, scope, and leakage tests.

Exit criteria:

- AWS ambiguity never reports revocation without evidence.
- Focused `0.83.0` AWS provider pentest passes.

### 0.84.0 - Azure Dynamic Provider

Goal: manage scoped Azure credentials through explicit tenant policy.

Deliverables:

- credential lifecycle, tenant/subscription binding, retries, and quotas;
- root credential protection and least-privilege templates.

Verification:

- protocol, partial revoke, tenant confusion, retry, and leakage tests.

Exit criteria:

- Azure authority remains bound to configured tenant and subscription.
- Focused `0.84.0` Azure provider pentest passes.

### 0.85.0 - GCP Dynamic Provider

Goal: manage scoped GCP credentials through explicit project policy.

Deliverables:

- credential lifecycle, project/account binding, retries, and quotas;
- root credential protection and least-privilege templates.

Verification:

- protocol, partial revoke, project confusion, retry, and leakage tests.

Exit criteria:

- GCP authority remains bound to configured project and account.
- Focused `0.85.0` GCP provider pentest passes.

### 0.86.0 - Hetzner Dynamic Provider

Goal: manage Hetzner credentials where provider APIs permit lifecycle control.

Deliverables:

- project/token create, scope, rotate, revoke, limitations, and quotas;
- egress, root credential protection, and least-privilege docs.

Verification:

- protocol, scope, partial revoke, retry, and leakage tests.

Exit criteria:

- Unsupported Hetzner lifecycle operations are explicit.
- Focused `0.86.0` Hetzner provider pentest passes.

### 0.87.0 - DigitalOcean Dynamic Provider

Goal: manage DigitalOcean credentials where provider APIs permit lifecycle control.

Deliverables:

- project/token create, scope, rotate, revoke, limitations, and quotas;
- egress, root credential protection, and least-privilege docs.

Verification:

- protocol, scope, partial revoke, retry, and leakage tests.

Exit criteria:

- Unsupported DigitalOcean lifecycle operations are explicit.
- Focused `0.87.0` DigitalOcean provider pentest passes.

### 0.88.0 - Kubernetes Secrets Provider

Goal: manage Kubernetes service-account/token secrets separately from auth.

Deliverables:

- namespace/service-account scope, token modes, lease, revoke, and quotas;
- cluster destination binding and least-privilege RBAC templates.

Verification:

- rootless cluster, namespace escape, stale token, revoke, and outage tests.

Exit criteria:

- Kubernetes secret issuance cannot inherit authentication reviewer authority.
- Focused `0.88.0` Kubernetes secrets-provider pentest passes.

### 0.89.0 - LDAP Secrets Provider

Goal: manage LDAP credentials separately from LDAP authentication.

Deliverables:

- static password rotation, supported dynamic entries, revoke, and limits;
- directory scope, TLS, least-privilege, and failure semantics.

Verification:

- rootless fixture, DN/filter injection, partial rotation, and outage tests.

Exit criteria:

- LDAP management authority cannot be used as login authority.
- Focused `0.89.0` LDAP secrets-provider pentest passes.

### 0.90.0 - Adapter Certification

Goal: make adapter safety evidence machine-readable and non-self-asserted.

Deliverables:

- signed create/renew/revoke/rotate/rollback/outage/idempotency/redaction results;
- capability metadata, provenance, promotion, expiry, and revocation policy.

Verification:

- bypass, feature flag, forged manifest, stale result, and full matrix tests.

Exit criteria:

- No adapter is stable without reproducible conformance evidence.
- Focused `0.90.0` adapter-certification pentest passes.

## Phase 9: Process-Isolated Extensions

### 0.91.0 - Component ABI And Signed Manifests

Goal: define a narrow extension contract before third-party execution.

Deliverables:

- WIT/component ABI for auth, secrets, provider, cloud, and notification roles;
- dual classical/PQ manifests, hashes, SBOM, capabilities, and ABI ranges;
- manifest-declared persistent-state schema version, supported reader/writer
  ranges, migration entry points, budgets, and backup participation;
- publisher trust, transparency, revocation, and provenance.

Verification:

- ABI fixtures, manifest fuzzing, stripping, revocation, and compatibility tests.

Exit criteria:

- Provenance is verified but never represented as proof of safety.
- Focused `0.91.0` plugin supply-chain pentest passes.

### 0.92.0 - Restricted Worker And Authenticated IPC

Goal: execute Wasmtime outside vault memory through bounded authenticated IPC.

Deliverables:

- separate unprivileged worker under a distinct OS principal, with no WASI or
  inherited ambient resources and explicit ptrace/process-inspection denial;
- portable assurance profiles: separately launched worker/container or user
  namespace for high assurance, and explicitly lower-assurance same-UID process;
- startup fails when configured high-assurance OS identity/process isolation
  cannot be established, including rootless deployment constraints;
- authenticated length-delimited IPC, protocol version, worker identity, nonce,
  replay/stale rejection, in-flight limits, backpressure, cancellation, and cleanup;
- sandbox, cgroup, fuel, epoch, stack, memory, table, output, and deadline limits;
- compiled-module cache integrity and exact Wasmtime version.

Verification:

- Linux/rootless and portable profile negotiation, same-UID labeling, isolation
  failure, escape, IPC fuzz, replay, resource exhaustion, crash, orphan, and cache tests.

Exit criteria:

- High-assurance worker starts only with its promised separate process/OS boundary;
  weaker platforms are labeled and cannot claim equivalent isolation.
- Focused `0.92.0` independent worker/IPC sandbox pentest passes.

### 0.93.0 - Plugin Capability And Network Broker

Goal: expose invocation-scoped high-level operations only.

Deliverables:

- handles bound to plugin, mount, namespace, operation, epoch, target, and budget;
- host-generated audit intent/result records surrounding every plugin invocation
  and broker action; plugin-supplied audit fields are always untrusted data;
- per-mount storage and structured provider methods whose approved destination
  cannot be reused as a generic byte-oriented exfiltration channel;
- SSRF-safe provider connection calls with operation-specific request/response schemas;
- no raw management credentials, arbitrary sockets, or general crypto capability.

Verification:

- confused deputy, audit forgery/omission, approved-destination exfiltration,
  reentrancy, stale handle, cross-mount, and SSRF tests.

Exit criteria:

- Plugins cannot exceed invocation-scoped host capabilities.
- Focused `0.93.0` plugin-capability pentest passes.

### 0.94.0 - Controlled Plugin Host Services

Goal: provide legitimate time and randomness without ambient access.

Deliverables:

- bounded monotonic/wall-time and CSPRNG host calls through shared services;
- capability, rate, size, audit, cancellation, and deterministic test rules;
- explicit treatment of plugin plaintext copies and outputs.

Verification:

- clock manipulation, randomness abuse, budget, replay, and secret-copy tests.

Exit criteria:

- Plugins cannot emulate or obtain uncontrolled clock, entropy, or secret sources.
- Focused `0.94.0` plugin-host-service pentest passes.

### 0.95.0 - Plugin Lifecycle And Certification

Goal: make install, operation, upgrade, rollback, and revocation safe.

Deliverables:

- pin, install, enable, disable, upgrade, rollback, quarantine, and revoke;
- staged copy-on-write state migration through the durable operation runtime,
  with bounded work/storage, crash-safe verification, and atomic activation;
- instance/schema generation fence preventing old plugin processes from using
  newly migrated state;
- rollback only while old code can read the resulting schema; incompatible or
  indeterminate migration is quarantined for explicit recovery;
- backup preflight/export/import/rebind/post-restore support for per-mount state;
- conformance certification and compromised-publisher response;
- complete operator, recovery, provenance, and limitations docs.

Verification:

- schema range, copy-on-write migration crash, budget exhaustion, activation,
  stale instance, incompatible rollback, quarantine, backup/restore, downgrade,
  revoked signer, cross-tenant, and recovery tests.

Exit criteria:

- Stable extension status requires sandbox, supply-chain, conformance, and
  crash-safe persistent-state compatibility evidence.
- Focused `0.95.0` extension-lifecycle pentest passes.

## Phase 10: Operator Intelligence And Governance

### 0.96.0 - Secret Inventory

Goal: expose actionable metadata without a secret-existence oracle.

Deliverables:

- owner, engine, type, access, expiry, rotation, and dependency metadata;
- namespace-aware bounded paginated immutable inventory snapshots;
- explicit unknown, unavailable, unsupported, and redacted states.

Verification:

- consistency, enumeration, inference, stale index, auth, and pagination tests.

Exit criteria:

- Inventory reveals no path or relationship beyond caller capability.
- Focused `0.96.0` inventory-leakage pentest passes.

### 0.97.0 - Policy Simulator

Goal: explain revision-pinned policy without mutating state.

Deliverables:

- distinct `policy:explain` capability and complete hypothetical request;
- allowed, denied, approval-required, indeterminate, and redacted explanations;
- read-only capability object, rate limits, and existence suppression.

Verification:

- oracle, policy race, mutation-host-call, redaction, and golden tests.

Exit criteria:

- Simulation cannot mutate state or disclose unauthorized existence.
- Focused `0.97.0` policy-simulator pentest passes.

### 0.98.0 - Dangerous-Change Dry Run

Goal: preview blast radius through non-mutating capabilities.

Deliverables:

- policy, mount, namespace, delete, revoke, root rotation, and adapter previews;
- revision-bound result, pagination, limits, and no mutation host calls.

Verification:

- hidden mutation, stale revision, race, scope, and blast-radius tests.

Exit criteria:

- Dry run produces no durable or upstream side effect.
- Focused `0.98.0` dry-run safety pentest passes.

### 0.99.0 - Local Developer Profile

Goal: make local use easy without disguising non-production guarantees.

Deliverables:

- reset, samples, test PKI, local storage, and obvious production guardrails;
- no network exposure by default and deterministic disposable workflows.

Verification:

- reset, sample, accidental production, listener, and persistence tests.

Exit criteria:

- Developer mode cannot be silently promoted to production configuration.
- Focused `0.99.0` developer-profile pentest passes.

### 0.100.0 - Leak Intake And Private Correlation

Goal: turn scanner findings into safe managed-secret response inputs.

Deliverables:

- authenticated finding schema and evidence hash;
- no raw leaked-value persistence;
- privacy-preserving bounded correlation to secrets, leases, IDs, tokens, and keys.

Verification:

- raw-secret rejection, false positive, collision, inference, replay, and race tests.

Exit criteria:

- Correlation is neither an oracle nor leaked-plaintext repository.
- Focused `0.100.0` leak-correlation pentest passes.

### 0.101.0 - Rotation Readiness

Goal: report and execute verifiable rotation readiness.

Deliverables:

- automatic, manual, blocked, unsupported, ownerless, and non-revocable states;
- adapter/engine rotation plans, verification evidence, approvals, and limits.

Verification:

- scoring matrix, race, partial rotation, false success, and rollback tests.

Exit criteria:

- Automation never claims rotation without verification evidence.
- Focused `0.101.0` rotation-readiness pentest passes.

### 0.102.0 - Signed Lifecycle Webhooks

Goal: export replay-safe lifecycle events through native egress.

Deliverables:

- created/read/rotated/expiring/revoked/leak/denied event schemas;
- signing, nonce, sequence, retry, idempotency, destination, and delivery limits.

Verification:

- replay, SSRF, signature, retry, reorder, leakage, and outage tests.

Exit criteria:

- Webhooks contain no raw secrets and cannot target unauthorized destinations.
- Focused `0.102.0` lifecycle-webhook pentest passes.

### 0.103.0 - Human Approval Controls

Goal: require policy-selected quorum approval for sensitive operations.

Deliverables:

- pending, approve, deny, cancel, expiry, quorum, role, and hard pending limits;
- immutable operation/context binding and integration for sensitive workflows.

Verification:

- replay, mismatch, role change, timeout, cancellation, and audit-failure tests.

Exit criteria:

- Approval cannot authorize another operation or survive expiry.
- Focused `0.103.0` approval-bypass pentest passes.

### 0.104.0 - Break-Glass And Emergency Journal

Goal: issue emergency authority only after independent durable evidence.

Deliverables:

- preallocated emergency journal/reserve with separate key and storage path;
- quorum, reason, scope, expiry, forced marking, rollback, and incident summary;
- break-glass remains blocked if both mandatory and emergency journals fail.

Verification:

- disk full, reserve exhaustion, replay, scope escape, privilege retention, and drills.

Exit criteria:

- No best-effort audit path can issue break-glass authority.
- Focused `0.104.0` break-glass and emergency-journal pentest passes.

### 0.105.0 - Tamper-Evident Evidence Bundles

Goal: make security events independently verifiable after incidents.

Deliverables:

- checkpoint, classical/PQ/hash-based sign, export, and bundle the authenticated
  append-only journal chain introduced in `0.28.0`; do not create a second chain;
- redacted incident, approval, break-glass, leak, rotation, and policy bundles;
- offline verifier, publication, archival, and replay APIs.

Verification:

- delete, reorder, mutate, truncate, key compromise, rollback, and import tests.

Exit criteria:

- Evidence tampering or incomplete history is reported explicitly.
- Focused `0.105.0` evidence-integrity pentest passes.

## Phase 11: Specialized Services And Integrations

### 0.106.0 - Transform Engine

Goal: add format-preserving transform, masking, and tokenization independently.

Deliverables:

- reviewed algorithms, tweak/domain policy, masking, tokenization, rotation, and limits;
- isolated keys, misuse-resistant API, migration, and operator docs.

Verification:

- domain, collision, leakage, misuse, migration, and crypto-review vectors.

Exit criteria:

- Transform cannot weaken Transit or expose reversible mappings without policy.
- Focused `0.106.0` Transform pentest and crypto review pass.

### 0.107.0 - KMIP Service

Goal: expose a separately authenticated bounded KMIP listener.

Deliverables:

- managed objects, operations, scopes, roles, TLS identity, limits, and lifecycle;
- isolated listener, parser, audit, policy, recovery, and interoperability docs.

Verification:

- protocol fuzzing, role escape, object confusion, TLS, and interop tests.

Exit criteria:

- KMIP cannot bypass normal identity, policy, audit, or key isolation.
- Focused `0.107.0` KMIP listener pentest passes.

### 0.108.0 - KMS And TDE Key Management

Goal: manage provider keys for KMS/TDE use without conflating providers.

Deliverables:

- provider-specific create, rotate, disable, delete, import/export, and provenance;
- opaque handles, egress, quotas, recovery, and limitation docs.

Verification:

- partial operation, outage, provenance, scope, retry, and deletion tests.

Exit criteria:

- Provider ambiguity never reports key destruction or rotation without evidence.
- Focused `0.108.0` KMS/TDE pentest passes.

### 0.109.0 - Advanced Policy Language

Goal: add deterministic CEL-style governance without ambient inputs.

Deliverables:

- typed expressions, bounded evaluation, group propagation, obligations, and versions;
- deterministic inputs, migration, diagnostics, and compatibility policy.

Verification:

- determinism, budget, ambiguity, bypass, migration, and policy fuzz tests.

Exit criteria:

- Advanced rules cannot override explicit denies or root restrictions.
- Focused `0.109.0` advanced-policy pentest passes.

### 0.110.0 - Configurable Resource Quotas

Goal: make inherited hard limits safely operator-configurable.

Deliverables:

- token, lease, KV, identity, group, policy, mount, namespace, wrapping, approval,
  adapter, plugin, and request quota APIs;
- defaults, reservation, hierarchy, race, accounting, and recovery semantics.

Verification:

- exhaustion, race, hierarchy, restart, rollback, and bypass tests.

Exit criteria:

- Configuration can tighten limits and cannot silently disable safety ceilings.
- Focused `0.110.0` quota-enforcement pentest passes.

### 0.111.0 - Audit And Runtime Operator APIs

Goal: add bounded audit elision, monitoring, and runtime log control.

Deliverables:

- audit elision with counts, monitor stream, log-level tuning, and safe diagnostics;
- authorization, rate, size, lifetime, redaction, and restore-to-default behavior.

Verification:

- leakage, bypass, stream DoS, log injection, and runtime-race tests.

Exit criteria:

- Runtime observability cannot weaken mandatory audit or expose secrets.
- Focused `0.111.0` operator-observability pentest passes.

### 0.112.0 - Locked-User And Profiling APIs

Goal: expose lockout repair and protected diagnostics independently.

Deliverables:

- list/unlock state with authorization and evidence;
- disabled-by-default profiling with explicit build/runtime gates and hard limits.

Verification:

- unlock escalation, enumeration, profiling leakage, DoS, and disabled-state tests.

Exit criteria:

- Production profiling remains absent unless explicitly compiled and enabled.
- Focused `0.112.0` lockout/profiling pentest passes.

### 0.113.0 - API Client Libraries

Goal: provide typed clients without embedding ambient authority.

Deliverables:

- Rust client and generated clients from the OpenAPI contract;
- idempotency, retries, TLS/hybrid policy, pagination, redaction, and cancellation.

Verification:

- compatibility, retry, secret logging, pagination, and cross-platform tests.

Exit criteria:

- Clients preserve server security semantics and never log credentials.
- Focused `0.113.0` client-library pentest passes.

### 0.114.0 - Agent, Proxy, And Auto-Auth

Goal: provide a separate least-authority client helper process.

Deliverables:

- auto-auth, cache, renewal, templates, bounded local IPC, sinks, and shutdown;
- token handoff, process identity, file permissions, and platform profiles.

Verification:

- cache isolation, template injection, IPC, token theft, race, and platform tests.

Exit criteria:

- Agent/proxy cannot become an unaudited alternate control plane.
- Focused `0.114.0` agent/proxy pentest passes.

### 0.115.0 - Kubernetes Operator And Injection

Goal: integrate Kubernetes operator, CSI, and injection workflows.

Deliverables:

- namespace-scoped reconciliation, CSI/injector delivery, renewal, and cleanup;
- RBAC templates, destination policy, quotas, and failure behavior.

Verification:

- namespace escape, stale mount, pod identity, token, and cleanup tests.

Exit criteria:

- Kubernetes integration cannot cross namespace or service-account scope.
- Focused `0.115.0` Kubernetes integration pentest passes.

### 0.116.0 - Secret Synchronization

Goal: sync explicitly authorized secrets with drift detection.

Deliverables:

- destination allowlists, versions, drift, ownership, retry, revoke, and audit;
- no broad export, bounded fanout, and source/destination conflict policy.

Verification:

- destination escape, stale sync, drift race, partial write, and revoke tests.

Exit criteria:

- Sync exports only specifically authorized secret versions and destinations.
- Focused `0.116.0` secret-sync pentest passes.

### 0.117.0 - Terraform Integration

Goal: provide Terraform workflows without placing secrets in state by default.

Deliverables:

- custom provider or tested compatibility layer with sensitive-state policy;
- idempotency, drift, import, delete, retry, and documentation.

Verification:

- state leakage, plan output, drift, retry, import, and destroy tests.

Exit criteria:

- Terraform state and plans do not contain secret payloads by default.
- Focused `0.117.0` Terraform integration pentest passes.

### 0.118.0 - GitOps Configuration Reconciliation

Goal: reconcile non-secret configuration without storing payloads in source.

Deliverables:

- signed declarative mounts, policies, auth config, revisions, and dry-run;
- no secret payload schema, conflict, rollback, approval, and audit behavior.

Verification:

- secret rejection, signature, stale revision, conflict, and rollback tests.

Exit criteria:

- GitOps cannot smuggle secret payloads or bypass approval/policy.
- Focused `0.118.0` GitOps reconciliation pentest passes.

## Phase 12: Isolated Experimental Security

### 0.119.0 - TEE Attestation Preview

Goal: experiment with attestation-bound release behind isolated features.

Deliverables:

- explicit trust roots, evidence freshness, workload identity, key release, and limits;
- provider-specific residual risks and no stable fallback changes.

Verification:

- replay, stale evidence, root replacement, downgrade, and outage tests.

Exit criteria:

- TEE preview is disabled by default and cannot weaken stable key release.
- Focused `0.119.0` TEE-preview pentest passes.

### 0.120.0 - Zero-Knowledge Policy Preview

Goal: admit bounded proofs only for reviewed hidden policy predicates.

Deliverables:

- proof-system version, setup/trust model, transcript, context, and verification limits;
- isolated policy capability and explicit unsupported predicates.

Verification:

- independent vectors, malformed proof, replay, context, soundness, and DoS tests.

Exit criteria:

- ZKP preview cannot grant authority outside its exact admitted predicate.
- Focused `0.120.0` ZKP-preview pentest passes.

### 0.121.0 - eBPF Audit Export Preview

Goal: export observability without making eBPF the security gate.

Deliverables:

- bounded event schema, loader privileges, kernel compatibility, loss counters, and limits;
- durable journal remains authoritative and export failure is explicit.

Verification:

- event loss, verifier/load failure, privilege, kernel mismatch, and DoS tests.

Exit criteria:

- eBPF loss or disablement cannot bypass mandatory durable audit.
- Focused `0.121.0` eBPF-export pentest passes.

## Phase 13: Stable Qualification

### 0.122.0 - Portability And Reproducible Artifacts

Goal: qualify binaries and Wolfi artifacts across supported environments.

Deliverables:

- Linux x86_64/aarch64, macOS x86_64/aarch64, Windows targets where supported,
  and practical BSD build/test evidence;
- hardened Linux-only Wolfi image, SBOM, provenance, checksums, and signatures;
- deterministic release scripts and platform assurance matrix.

Verification:

- clean-VM build/smoke, reproducibility, install, upgrade, and rollback tests.

Exit criteria:

- Artifacts trace to exact source without developer-local state.
- Focused `0.122.0` supply-chain and portability pentest passes.

### 0.123.0 - API, Documentation, And Parity Closeout

Goal: close every planned inventory item before compatibility freeze.

Deliverables:

- complete system subset, OpenAPI, path help, and version history;
- every parity item implemented, experimental, or deliberately different;
- complete user, operator, API, config, migration, recovery, and limitation docs.

Verification:

- endpoint inventory, compatibility, links, examples, stale-version, and docs audit.

Exit criteria:

- No capability is silently missing, implied compatible, or undocumented.
- Focused `0.123.0` API/parity/documentation pentest passes.

### 0.124.0 - Composed Security Campaign And RC Freeze

Goal: prove composition at scale and freeze stable compatibility.

Deliverables:

- parser/format fuzzing, Miri, Loom, property, model, and destructive campaigns;
- full storage, audit, seal, token, lease, auth, Raft, adapter, plugin, backup,
  recovery, resource, and denial-of-service evidence;
- final API/storage/audit/plugin/crypto contracts, residual risks, runbooks,
  release artifacts, SBOM, provenance, and evidence.

Verification:

- long-running campaigns, all-format migrations, full portability/Wolfi matrix,
  multi-node/multi-namespace/provider/extension disaster scenario, and clean gates.

Exit criteria:

- Only release-blocking fixes and documentation corrections may follow.
- Independent `0.124.0` release-candidate pentest passes for the exact commit.

## 1.0.0 - First Stable Release

Goal: release a documented, production-qualified API-driven secrets manager
whose protected profile is quantum-resistant under current cryptanalytic knowledge.

Deliverables:

- all stable pre-1.0 capabilities, profiles, migrations, and contracts;
- protected profile requiring hybrid/PQ transport, services, checkpoints, and
  authentication assurance with no silent classical fallback;
- standalone binaries, hardened Wolfi image, complete docs, recovery runbooks,
  SBOM, provenance, checksums, signatures, and signed evidence;
- previews isolated, disabled by default, and outside stable compatibility.

Verification:

- every repository, release, portability, migration, API, storage, recovery,
  Raft, adapter, plugin, profile, and disaster-recovery gate from clean checkout;
- independent review of cryptography, storage, memory, transport, audit,
  authorization, tokens, identity, clustering, extensions, and supply chain;
- every finding fixed or explicitly accepted as documented residual risk.

Exit criteria:

- Exact candidate has green CI, reproducible artifacts, complete evidence,
  no unresolved blocker, and maintainer sign-off.
- STOP: final independent `1.0.0` pentest passes for the exact commit before the
  signed `v1.0.0` tag and release are published.
