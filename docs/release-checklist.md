# Release Checklist

Every release ends with a STOP gate before tagging.

## Local Gate

```bash
scripts/release_0_1_gate.sh
```

Once release artifacts exist, also record:

- `cargo audit` RustSec advisory result;
- `cargo deny check bans licenses sources` result;
- SBOM checksums;
- source archive checksums;
- binary checksums;
- container digests;
- pentest handoff and findings summary;
- signed tag verification;
- release-note evidence fields.

## 0.1.0 STOP Gate

- `scripts/release_0_1_gate.sh` passed.
- `LYKILHEIM_RELEASE_PODMAN=1 scripts/release_0_1_gate.sh` passed on a host
  with rootless Podman.
- Rust crate scaffold reviewed.
- API shape reviewed against `docs/api-reference.md`.
- Threat model reviewed against `docs/security-model.md`.
- Dependency policy reviewed.
- CI permissions reviewed.
- Container user model reviewed.
- `docs/pentest-0.1.0.md` reviewed and used for the focused pentest.
- `release-notes/RELEASE_NOTES_0.1.0.md` evidence filled before tag.
