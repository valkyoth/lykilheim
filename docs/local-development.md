# Local Development

Use the pinned toolchain:

```bash
rustup show
```

Run the full local check gate:

```bash
scripts/checks.sh
```

Run the `0.1.0` release gate without Podman:

```bash
scripts/release_0_1_gate.sh
```

Run the same gate with rootless Podman smoke enabled:

```bash
LYKILHEIM_RELEASE_PODMAN=1 scripts/release_0_1_gate.sh
```

Run the server locally:

```bash
cargo run -- --config examples/lykilheim.toml
```

Check the API:

```bash
curl -s http://127.0.0.1:8200/v1/sys/health
curl -s http://127.0.0.1:8200/v1/sys/version
```

The reusable API smoke runs the same foundation checks on an ephemeral local
port:

```bash
scripts/smoke_api_foundation.sh
```

Validate configuration without starting the server:

```bash
cargo run -- --check-config --config examples/lykilheim.toml
```

The default configuration binds to `127.0.0.1:8200`. Do not expose development
builds to untrusted networks.
