# Security Policy

Lykilheim is security-sensitive infrastructure. Treat dependency, crypto,
storage, authentication, authorization, audit, plugin, and container changes as
high-risk until tested.

## Routine Checks

Run these regularly and before releases:

```bash
scripts/checks.sh
```

After the Rust crate exists, the release gate must also include:

```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo deny check bans licenses sources
cargo audit --db target/advisory-db
scripts/generate-sbom.sh
```

GitHub Actions run CI, and GitHub CodeQL default setup should be enabled in the
repository security settings. Keep only one active CodeQL configuration:
GitHub rejects SARIF uploads when default setup and an advanced workflow both
try to analyze the same repository.

The versioned build plan is documented in
[Lykilheim Version Plan](docs/version-plan.md). Each planned release has a
mandatory STOP gate for security review, pentest, and release evidence.

## Dependency Policy

Unknown registries and git dependencies must be denied by default once
`Cargo.toml` and `deny.toml` exist. License exceptions must be narrow, named,
versioned, and documented with a removal condition.

Build scripts, procedural macros, `*-sys` crates, vendored native code, CI
workflow edits, release script edits, and container build changes are treated as
executable supply-chain changes. Review them before merging dependency updates.

## Cryptography Policy

Do not invent cryptography. Use reviewed primitives, constant-time APIs where
available, and sanitizing secret wrappers for plaintext keys, token material,
dynamic credentials, and transit payloads. Experimental post-quantum, TEE, ZKP,
and eBPF work must stay behind explicit feature gates until reviewed and
documented.

## Release Supply-Chain Evidence

Stable releases must publish SBOM files generated from the tagged source tree.
Release notes must include source archive checksums, binary checksums, container
digests when images are published, and the signed tag verification line.

Rootless Podman and Wolfi paths are first-class release gates. A release cannot
ship if standalone compiled execution works but rootless container execution is
broken.

## Reporting

Do not publish exploitable security details before a fix is available. Open a
[GitHub private security advisory](https://github.com/valkyoth/lykilheim/security/advisories/new)
or email `security@valkyoth.com` with the subject `Lykilheim Security Report`.
