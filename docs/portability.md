# Portability Policy

Lykilheim should keep the standalone server binary portable across Linux,
macOS, Windows, and BSD-style Unix systems from the beginning. Rootless Wolfi
containers remain a Linux-only delivery target unless the release plan says
otherwise.

## Support Tiers

| Target | Tier | Scope |
| --- | --- | --- |
| Linux standalone binary | Tier 1 | Primary development and release target. |
| Linux rootless Wolfi container | Tier 1 | Hardened container target, Linux-only. |
| macOS standalone binary | Tier 2 | Build and test target for local operation. |
| Windows standalone binary | Tier 2 | Build and test target for local operation. |
| FreeBSD, OpenBSD, NetBSD standalone binary | Tier 2/3 | Portable design target; CI may start with `cargo check` until full runners exist. |

## Design Rules

- Keep core API, configuration, routing, error handling, cryptography, storage
  traits, audit traits, and tests free of operating-system assumptions.
- Keep platform-specific code behind small modules such as `platform::signals`,
  `platform::permissions`, `platform::memory_lock`, and `platform::service`.
- Prefer Rust standard-library and widely used portable crates before native OS
  calls.
- Document every platform-specific behavior when it becomes user-facing.
- Do not let Linux container requirements leak into standalone binary behavior.
- Treat rootless Podman, Wolfi, Linux namespaces, and Linux service hardening as
  Linux-specific deployment features.

## Areas That Need Extra Care

- filesystem storage, especially path rules, case sensitivity, atomic rename,
  fsync behavior, and file locking;
- file permissions and ACLs across Unix modes and Windows ACLs;
- memory locking and swap avoidance across `mlock`, `VirtualLock`, and BSD or
  macOS limits;
- service integration across systemd, launchd, Windows Service Manager, and
  BSD rc systems;
- TLS certificate/key file permissions;
- temporary files and runtime directories;
- signal handling and graceful shutdown;
- HSM, KMS, plugin, and sandbox behavior.

## Release Expectations

Before a feature is marked stable, its documentation must say which operating
systems it supports and which behaviors are Linux-only. The Linux Wolfi
container is release-blocking for container releases, but macOS, Windows, and
BSD portability applies to the standalone binary.

Native release binaries should be built on the operating system they target
with `scripts/build_release_binary.py`. Cross-compilation can be added later as
a separate release feature only when it has equivalent test and checksum
evidence.
