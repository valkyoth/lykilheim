#!/usr/bin/env sh
set -eu

port="${LYKILHEIM_SMOKE_PORT:-18200}"
base_url="http://127.0.0.1:${port}"
log_file="${TMPDIR:-/tmp}/lykilheim-api-smoke-$$.log"
pid=""

cleanup() {
    if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
        kill "$pid" 2>/dev/null || true
        wait "$pid" 2>/dev/null || true
    fi
    rm -f "$log_file"
}
trap cleanup EXIT HUP INT TERM

cargo run --quiet -- --config examples/lykilheim.toml --listen "127.0.0.1:${port}" >"$log_file" 2>&1 &
pid="$!"

ready=0
for _ in 1 2 3 4 5 6 7 8 9 10; do
    if curl -sSf "${base_url}/v1/sys/health" >/dev/null 2>&1; then
        ready=1
        break
    fi
    sleep 0.2
done

if [ "$ready" -ne 1 ]; then
    echo "api smoke: server did not become ready" >&2
    cat "$log_file" >&2
    exit 1
fi

health="$(curl -sSf "${base_url}/v1/sys/health")"
printf '%s' "$health" | grep -q '"initialized":false'
printf '%s' "$health" | grep -q '"sealed":true'
printf '%s' "$health" | grep -q '"version":"0.1.0"'

seal_status="$(curl -sSf "${base_url}/v1/sys/seal-status")"
printf '%s' "$seal_status" | grep -q '"initialized":false'
printf '%s' "$seal_status" | grep -q '"sealed":true'
printf '%s' "$seal_status" | grep -q '"threshold":0'
printf '%s' "$seal_status" | grep -q '"progress":0'

version="$(curl -sSf "${base_url}/v1/sys/version")"
if [ "$version" != '{"version":"0.1.0"}' ]; then
    echo "api smoke: unexpected version response: $version" >&2
    exit 1
fi

init_response="$(curl -sS -X POST "${base_url}/v1/sys/init")"
printf '%s' "$init_response" | grep -q '"code":"not_implemented"'

echo "api smoke: ok"
