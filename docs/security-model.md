# Security Model

This document records the `0.1.0` threat model. Later releases must update it
when they change storage, cryptography, authentication, authorization, audit, or
plugin behavior.

## Sealed State

In sealed state, Lykilheim must not serve secret material. The `0.1.0`
foundation reports sealed state but does not yet implement secret storage.

## Unsealed State

Unsealed state begins in `0.2.0`. Plaintext keys and payloads must be wrapped in
types that zeroize memory when dropped. Debug output must not expose secret
material.

## Storage Compromise

Storage backends are treated as untrusted. They may reveal, delete, reorder, or
modify bytes. The cryptographic barrier must authenticate encrypted records
before returning plaintext.

## Audit Failure

Security-sensitive operations must fail closed when no required audit sink can
record the event. The `0.1.0` release defines the audit sink boundary; durable
audit implementations arrive later. Because `0.1.0` has no durable audit sink
and no secret-bearing mutating operations, `audit.required` defaults to `false`
and `audit.required = true` is rejected until a later release can enforce it
honestly.

## Transport Security

The server defaults to `127.0.0.1:8200`. The `0.1.0` preview does not terminate
TLS itself and must not be exposed on an untrusted network. Any non-loopback
deployment before native TLS exists must sit behind a trusted TLS-terminating
proxy and network access control.

## Token Compromise

Token compromise is assumed possible. Tokens need TTLs, revocation, accessor
lookups, child-token cascades, and audit trails before `1.0.0`.

## Plugin Compromise

Plugin compromise is assumed possible. Native adapters are preferred first.
Wasm plugins must not become stable until CPU, memory, filesystem, network,
host-call, signing, and provenance boundaries are reviewed.
