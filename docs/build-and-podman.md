# Build And Podman

The `0.1.0` foundation provides container build placeholders for the future
rootless Wolfi release gate.

Build the local binary:

```bash
cargo build --release
```

Build the Wolfi container:

```bash
podman build -f containers/Containerfile.wolfi -t lykilheim:dev .
```

Run rootless:

```bash
podman run --rm --user 65532:65532 -p 127.0.0.1:8200:8200 \
  lykilheim:dev --listen 0.0.0.0:8200
```

The image runs as `65532:65532` and exposes port `8200`. The explicit listen
override is required because the local binary defaults to loopback for safe
development. Future releases will add state, audit, and configuration volume
guidance as soon as persistent storage exists.

`--listen 0.0.0.0:8200` is required inside the container because container
loopback is not the host loopback. Keep host publishing bound to
`127.0.0.1:8200` for the `0.1.0` preview, or put the service behind a trusted
TLS-terminating proxy and network allowlist. Do not publish this unauthenticated
preview API on a public interface.

Run the reusable `0.1.0` Podman smoke:

```bash
scripts/podman_smoke_0_1.sh
```
