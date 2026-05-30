# Release Binary Builds

Lykilheim release binaries are built natively on the operating system they
target. The helper script clones a clean copy of the repository, checks out the
requested tag or commit, installs the pinned Rust toolchain when requested,
builds with `cargo build --release --locked`, packages the binary, and prints
SHA256 values for the GitHub release notes.

## Supported Native Builds

| Platform argument | Run on | Package |
| --- | --- | --- |
| `linux` | Linux | `.tar.gz` |
| `macos` | macOS | `.tar.gz` |
| `bsd` | FreeBSD, OpenBSD, NetBSD, or DragonFly BSD | `.tar.gz` |
| `windows` | Windows | `.zip` |

The script does native builds only. Do not use it to imply that a Linux host can
produce official macOS, Windows, or BSD artifacts. Run it on each target
operating system, then copy the SHA256 lines into the release notes.

The script clones the repository before building. Only committed and pushed
content, or the explicitly requested tag or commit, is included in the artifact.
Do not use it to test uncommitted local changes.

## Examples

Linux:

```bash
python3 scripts/build_release_binary.py linux --ref v0.1.0 --install-prereqs
```

macOS:

```bash
python3 scripts/build_release_binary.py macos --ref v0.1.0 --install-prereqs
```

BSD:

```bash
python3 scripts/build_release_binary.py bsd --ref v0.1.0 --install-prereqs
```

Windows:

```powershell
py -3 scripts/build_release_binary.py windows --ref v0.1.0 --install-prereqs
```

`git` and Python must already be available so the script can run and clone the
repository. `--install-prereqs` installs Rust through `rustup` when Cargo is not
already available; package-manager setup for Git and Python remains an operator
bootstrap step.

## Output

Artifacts are written to:

```text
target/release-binaries/
```

The script prints two release-note lines:

- packaged artifact SHA256;
- raw binary SHA256.

Use the packaged artifact SHA256 for GitHub release assets. Keep the raw binary
SHA256 as additional evidence in the versioned release notes.
