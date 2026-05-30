# Vault And OpenBao Feature-Parity Audit

This document is the feature inventory Lykilheim must keep current before each
implementation wave. It is not a promise that every feature ships in `1.0.0`.
It is a guardrail so Vault/OpenBao capabilities are either implemented,
scheduled, explicitly deferred, or intentionally rejected with a reason.

Primary references checked before this audit:

- HashiCorp Vault documentation: how Vault works, auth methods, secrets
  engines, audit devices, and system backend API.
- OpenBao documentation: auth API, secrets API, namespace concepts, system
  backend API, seal/unseal, KV, transit, and PKI docs.

Reference URLs checked on 2026-05-30:

- <https://developer.hashicorp.com/vault/docs/about-vault/how-vault-works>
- <https://developer.hashicorp.com/vault/docs/concepts/auth>
- <https://developer.hashicorp.com/vault/docs/secrets>
- <https://developer.hashicorp.com/vault/docs/audit>
- <https://developer.hashicorp.com/vault/api-docs/system>
- <https://openbao.org/api-docs/auth/>
- <https://openbao.org/api-docs/secret/>
- <https://openbao.org/api-docs/system/>
- <https://openbao.org/docs/concepts/namespaces/>
- <https://openbao.org/docs/next/concepts/seal/>
- <https://openbao.org/docs/secrets/kv/>
- <https://openbao.org/docs/secrets/transit/>

## Coverage Levels

- `1.0`: required for first stable.
- `Preview`: planned before `1.0.0`, but may be marked beta or experimental.
- `Post-1.0`: required for broad parity, but not a first-stable blocker.
- `Research`: security-sensitive or ecosystem-dependent work that needs design
  and pentest before production use.
- `Different`: Lykilheim will intentionally behave differently, documented with
  operator impact.

## Core Request Flow

| Area | Coverage | Plan |
| --- | --- | --- |
| Versioned HTTP API | `1.0` | Axum API with stable request, response, and error docs. |
| Mount table and routing | `1.0` | Prefix/radix router for auth and secrets mounts. |
| Mount lifecycle | `1.0` | Enable, disable, tune, and remount for auth and secrets engines; disabling revokes supported leases. |
| Barrier view isolation | `1.0` | Every engine receives isolated storage rooted by engine identity; no cross-engine storage traversal. |
| ACL policy enforcement | `1.0` | Default deny; explicit capabilities by path and operation. |
| System backend | `1.0` subset, `Post-1.0` complete | Stable subset for init, health, seal, unseal, mounts, auth, audit, policy, capabilities, leases, wrapping, metrics, leader, storage, and version history. |
| OpenAPI/spec endpoint | `Post-1.0` | Generate from typed route definitions after API stabilizes. |
| Path help | `Post-1.0` | API-driven equivalent for CLI/client discovery. |

## Seal, Key Lifecycle, And Storage

| Area | Coverage | Plan |
| --- | --- | --- |
| Shamir init/unseal | `1.0` | Threshold shares, duplicate detection, sealed-state restrictions. |
| Seal status and manual seal | `1.0` | API-driven seal status, seal, and unseal. |
| Rekey and root/recovery token generation | `1.0` | Quorum workflows for rekey, root-token generation, recovery-token generation, and cancellation. |
| Barrier key rotation | `1.0` | Rotate data encryption keys without exposing old key material. |
| Recovery keys | `Preview` | Required when auto-unseal is introduced. |
| Auto-unseal | `Post-1.0` | Transit seal first; KMS/HSM/PKCS#11 adapters only after secret-zero review. |
| Seal migration | `Post-1.0` | Explicit downtime workflow and backup requirement. |
| Storage trait | `1.0` | Opaque encrypted bytes only. |
| Local storage | `1.0` | Development/single-node backend. |
| Integrated Raft storage | `Preview` | Targeted before `1.0`, production status depends on pentest. |
| External storage adapters | `Post-1.0` | Evaluate Consul, PostgreSQL, object storage, and cloud databases only where they do not compromise standalone goals. |
| Snapshots and restore | `1.0` | Encrypted snapshots with restore smoke tests. |
| Storage migration | `1.0` | Migration tests from every pre-release format. |

## Audit, Logs, Telemetry, And Operations

| Area | Coverage | Plan |
| --- | --- | --- |
| Audit devices | `1.0` | File first, socket/syslog later; request fails if no enabled audit device succeeds. |
| Audit hashing | `1.0` | HMAC sensitive string values, audit-hash endpoint, and redaction tests. |
| Audit elision | `Post-1.0` | Elide large list responses while retaining counts. |
| Operational logs | `1.0` | Separate from audit logs. |
| Metrics and health | `1.0` | Never expose secret material. |
| Monitor/log-level APIs | `Post-1.0` | API-driven runtime log inspection and tuning after core stability. |
| Profiling endpoints | `Post-1.0` | Disabled by default; protected and documented. |
| Quotas | `Post-1.0` | Rate-limit and lease-count quotas. |
| Locked users | `Post-1.0` | Lockout inspection and unlock APIs. |

## Auth Methods

| Area | Coverage | Plan |
| --- | --- | --- |
| Token auth | `1.0` | Token create, lookup, renew, revoke, accessors, child tokens, orphan tokens, and periodic tokens. |
| AppRole | `1.0` | RoleID, SecretID, wrapping, CIDR binding, TTL, use limits, accessors, and revocation. |
| Userpass | `1.0` | Bootstrap/dev only by default; strong password hashing and lockout. |
| JWT/OIDC | `Post-1.0` | Static keys, JWKS, OIDC discovery, claim binding, and CLI/browser flow considerations. |
| Kubernetes | `Post-1.0` | TokenReview mode and JWT/OIDC service-account validation mode. |
| LDAP | `Post-1.0` | Users, groups, TLS, bind strategies, and lockout behavior. |
| TLS certificates | `Post-1.0` | mTLS client cert auth and CRL handling. |
| Kerberos | `Post-1.0` | Enterprise/domain environments after dependency review. |
| RADIUS | `Post-1.0` | Human auth after MFA/control-group design. |
| GitHub | `Post-1.0` | Human/operator auth if still useful versus OIDC. |
| Cloud auth | `Post-1.0` | AWS, Azure, GCP, OCI/AliCloud-style workload identity auth as separate adapters. |
| MFA | `Post-1.0` | Identity-bound MFA and auth-method integration. |
| Auto-auth agent methods | `Post-1.0` | Client-side agent/proxy work, not server core. |

## Identity, Policies, And Governance

| Area | Coverage | Plan |
| --- | --- | --- |
| ACL policies | `1.0` | Path capabilities, denies, parameter constraints, templating decisions documented. |
| Identity store | `1.0` | Entities, aliases, groups, metadata, and policy attachment. |
| Namespaces | `1.0` base, `Post-1.0` full | Secure tenant isolation, child namespaces, delegated administration, and restricted root-only system paths. |
| Password policies | `Post-1.0` | Password generation policy endpoints and docs. |
| Response wrapping | `1.0` | Wrap, lookup, rewrap, unwrap, TTL, single-use behavior, and audit trails. |
| Capabilities APIs | `1.0` | Token and accessor capability checks. |
| Control groups | `Post-1.0` | Human approval workflows for sensitive paths. |
| Sentinel EGP/RGP equivalent | `Post-1.0` | Use a Rust-native policy engine such as CEL-style rules; document differences. |
| Group policy application | `Post-1.0` | Namespace-aware policy propagation semantics. |
| Root/raw paths | `Different` | Avoid `sys/raw` by default; any raw access must be disabled unless explicitly compiled and configured. |

## Secrets Engines

| Area | Coverage | Plan |
| --- | --- | --- |
| KV v2 | `1.0` | Versioning, CAS, metadata, soft delete, undelete, destroy, list, and created-by metadata. |
| KV v1 | `Post-1.0` | Needed for compatibility and lower-overhead static secrets. |
| Cubbyhole | `1.0` | Per-token private storage and response-wrapping support. |
| Transit | `1.0` baseline, `Post-1.0` complete | Encrypt, decrypt, rewrap, rotate, sign, verify, hash, HMAC, random, datakey, derived keys, convergent encryption, BYOK/import decisions. |
| PKI | `1.0` baseline, `Post-1.0` complete | Root/intermediate CA, CSR signing, roles, revocation, CRL, OCSP, ACME, issuer rotation. |
| SSH | `Post-1.0` | OTP and SSH CA signing with user/host cert roles. |
| TOTP | `Post-1.0` | TOTP generation/validation engine. |
| Database | `Post-1.0` | PostgreSQL first, then MySQL/MariaDB, MSSQL, MongoDB, Oracle, and other adapters by demand. |
| SurrealDB | `Post-1.0` | Differentiating adapter for dynamic system users, static password rotation, and later record-access/JWT helpers after SurrealDB auth semantics are tested. |
| Cloud dynamic secrets | `Post-1.0` | AWS, Azure, GCP, and other providers after dependency and secret-zero review. |
| Kubernetes secrets engine | `Post-1.0` | Service-account/token workflows distinct from Kubernetes auth. |
| LDAP secrets engine | `Post-1.0` | Credential management distinct from LDAP auth. |
| RabbitMQ and service engines | `Post-1.0` | Dynamic credentials for common services after database/cloud engines. |
| Transform | `Post-1.0` | Format-preserving encryption, masking, and tokenization; must get cryptographic review. |
| KMIP | `Post-1.0` | Separate listener/protocol, managed objects, scopes, roles, certificates. |
| Key management/TDE | `Post-1.0` | KMS-provider key management and transparent data encryption compatibility if demand exists. |
| Identity engine | `1.0` | Entities/groups as first-class internal engine. |
| Plugin engines | `Preview` | Native trait first; Wasmtime sandbox experimental until reviewed. |

## Adapter Roadmap

Adapters are provider-specific implementations behind a common engine trait.
Early adapters should be compiled into the binary behind explicit Cargo
features. Later, the same contracts may be exposed through sandboxed Wasm
plugins for third-party providers.

| Adapter family | Initial targets | Notes |
| --- | --- | --- |
| SQL databases | PostgreSQL, MySQL, MariaDB | Closest to OpenBao/Vault database engine semantics: dynamic users, static roles, root rotation, revocation statements. |
| Document databases | MongoDB | Dynamic users and role grants; requires careful revocation and TTL testing. |
| Multi-model databases | SurrealDB | Differentiator; system-user rotation is closer to database engines, record-access/JWT support is a separate app-auth design. |
| Cache/key-value services | Redis, Valkey | User/ACL creation and rotation where server version supports it; otherwise static credential rotation only. |
| Message brokers | RabbitMQ | Dynamic users, vhosts, tags, permissions, and revocation. |
| Public cloud providers | AWS, Azure, GCP | Identity and access-key lifecycle where provider APIs support short-lived or rotated credentials. |
| European/cloud infrastructure providers | Hetzner, DigitalOcean | API-token or project credential lifecycle where provider APIs support creation, scoping, and revocation. |
| Extensible providers | Custom Wasm adapters | Only after the Wasm capability model, signing, resource limits, and network allowlists pass review. |

Every adapter must document:

- exact upstream API calls or statements used for create, renew, revoke, and
  rotate;
- minimum privileges needed by the Lykilheim management credential;
- lease and revocation semantics;
- whether static roles, dynamic roles, root rotation, and username customization
  are supported;
- audit fields and redaction behavior;
- failure behavior when the upstream provider is unavailable;
- rootless Podman smoke coverage where the provider can run locally.

## Replication, HA, And Multi-Cluster

| Area | Coverage | Plan |
| --- | --- | --- |
| Single-cluster HA | `Preview` | Raft leader election, forwarding, snapshots, membership changes. |
| Performance standby/read replicas | `Post-1.0` | Read scaling and consistency tokens after stable Raft. |
| Disaster recovery replication | `Post-1.0` | Secondary promotion, activation tokens, and DR runbooks. |
| Performance replication | `Post-1.0` | Multi-region active reads after DR semantics are safe. |
| Replication filters | `Post-1.0` | Path/namespace filters after full namespace model. |
| Automated snapshots | `1.0` local, `Post-1.0` scheduled | API-driven snapshot now; scheduling later. |

## Client And Platform Integrations

| Area | Coverage | Plan |
| --- | --- | --- |
| Rootless Wolfi container | `1.0` | First-class release gate. |
| Standalone binary | `1.0` | First-class release gate. |
| API client libraries | `Post-1.0` | Rust first, then generated clients from OpenAPI. |
| Agent/proxy/cache/templates | `Post-1.0` | Separate process/client package after server API stabilizes. |
| Kubernetes operator/CSI/injector | `Post-1.0` | Integrations after auth, tokens, and agent model stabilize. |
| Terraform provider compatibility | `Post-1.0` | Decide between custom provider or Vault-provider compatibility layer. |
| Secret sync | `Post-1.0` | Sync static secrets to external destinations with drift detection and audit. |
| GitOps reconciliation | `Post-1.0` | Declarative config reconciler, never for secret payloads by default. |

## Deliberate Differences From Vault/OpenBao

- Lykilheim should not claim Vault/OpenBao API compatibility unless an endpoint
  is documented and tested against compatibility fixtures.
- Experimental research features must be feature-gated and disabled by default.
- Raw storage access should not exist in normal builds.
- External dependencies are accepted only when they are needed for a specific
  engine or integration and pass dependency, license, maintenance, and security
  review.
- Any feature that changes the threat model needs documentation, tests, release
  notes, and a STOP/pentest gate before release.

## Review Checklist Before Implementation Starts

- Every planned API endpoint is mapped to one of the categories above.
- Every Vault/OpenBao auth method is mapped to `1.0`, `Post-1.0`, `Research`,
  or `Different`.
- Every Vault/OpenBao secrets engine is mapped to `1.0`, `Post-1.0`,
  `Research`, or `Different`.
- Every system backend endpoint needed by operators is either in the `1.0`
  scope or explicitly deferred.
- Every deferred item has user-facing documentation explaining status and risk.
- Release notes for the next version mention changed parity status.
