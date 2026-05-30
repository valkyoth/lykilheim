#!/usr/bin/env sh
set -eu

image="${LYKILHEIM_IMAGE:-lykilheim:0.1.0-dev}"
container="${LYKILHEIM_CONTAINER:-lykilheim-010-smoke}"
host_port="${LYKILHEIM_PODMAN_PORT:-18201}"
base_url="http://127.0.0.1:${host_port}"

cleanup() {
    podman stop --time 3 "$container" >/dev/null 2>&1 || true
}
trap cleanup EXIT HUP INT TERM

podman build -f containers/Containerfile.wolfi -t "$image" .
podman run -d --rm --name "$container" -p "${host_port}:8200" "$image" --listen 0.0.0.0:8200 >/dev/null

ready=0
for _ in 1 2 3 4 5 6 7 8 9 10; do
    if curl -sSf "${base_url}/v1/sys/health" >/dev/null 2>&1; then
        ready=1
        break
    fi
    sleep 0.2
done

if [ "$ready" -ne 1 ]; then
    echo "podman smoke: container did not become ready" >&2
    podman logs "$container" >&2 || true
    exit 1
fi

health="$(curl -sSf "${base_url}/v1/sys/health")"
printf '%s' "$health" | grep -q '"initialized":false'
printf '%s' "$health" | grep -q '"sealed":true'
if printf '%s' "$health" | grep -q '"version"'; then
    echo "podman smoke: health response must not expose version" >&2
    exit 1
fi

version="$(curl -sSf "${base_url}/v1/sys/version")"
if [ "$version" != '{"version":"0.1.0"}' ]; then
    echo "podman smoke: unexpected version response: $version" >&2
    exit 1
fi

podman stop --time 3 "$container" >/dev/null
trap - EXIT HUP INT TERM
echo "podman smoke: ok"
