# Architecture

Lykilheim is split into small Rust modules from the first release:

- `api`: versioned HTTP API shape and routing.
- `config`: TOML configuration loading and validation.
- `error`: stable internal errors and JSON API errors.
- `audit`: audit event and sink interfaces.
- `crypto`: cryptographic barrier traits.
- `storage`: opaque encrypted storage traits.
- `test_support`: test helpers.

The `0.1.0` foundation does not store secrets. It defines the ownership
boundaries needed for later releases so the storage backend never needs to see
plaintext once the cryptographic barrier lands in `0.2.0`.

## Request Flow

The planned request flow is:

1. HTTP request enters the versioned API router.
2. Request metadata is normalized and assigned a request identifier.
3. Authentication and authorization are evaluated.
4. Audit records are written through fail-closed audit sinks.
5. The request is routed to a mounted auth method, system backend, or secrets
   engine.
6. Storage operations cross the cryptographic barrier before reaching a backend.

Only the first public system endpoints exist in `0.1.0`.

## Platform Boundaries

The standalone binary is designed to stay portable across Linux, macOS,
Windows, and BSD-style Unix systems. Platform-specific behavior should live
behind narrow modules rather than spreading `cfg` checks through API, storage,
audit, or crypto code.

The hardened Wolfi container is a Linux-only delivery target. Container
requirements must not become requirements for running the standalone binary on
macOS, Windows, or BSD.
