# Release Checklist

Every release ends with a STOP gate before tagging.

## Local Gate

```bash
scripts/checks.sh
```

Once release artifacts exist, also record:

- `cargo audit` result;
- `cargo deny check` result;
- SBOM checksums;
- source archive checksums;
- binary checksums;
- container digests;
- signed tag verification;
- release-note evidence fields.

## 0.1.0 STOP Gate

- Rust crate scaffold reviewed.
- API shape reviewed against `docs/api-reference.md`.
- Threat model reviewed against `docs/security-model.md`.
- Dependency policy reviewed.
- CI permissions reviewed.
- Container user model reviewed.
- `release-notes/RELEASE_NOTES_0.1.0.md` evidence filled before tag.
