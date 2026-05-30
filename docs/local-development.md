# Local Development

Use the pinned toolchain:

```bash
rustup show
```

Run the full local check gate:

```bash
scripts/checks.sh
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

Validate configuration without starting the server:

```bash
cargo run -- --check-config --config examples/lykilheim.toml
```

The default configuration binds to `127.0.0.1:8200`. Do not expose development
builds to untrusted networks.
