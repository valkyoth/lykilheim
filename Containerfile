ARG RUST_IMAGE=docker.io/library/rust:1.97.1-bookworm@sha256:77fac8b98f9f46062bb680b6d25d5bcaabfc400143952ebc572e924bcbedc3fa
ARG RUNTIME_IMAGE=cgr.dev/chainguard/wolfi-base@sha256:02dab76bd852a70556b5b2002195c8a5fdab77d323c433bf6642aab080489795

FROM ${RUST_IMAGE} AS build
WORKDIR /src
COPY . .
RUN cargo build --locked --release

FROM ${RUNTIME_IMAGE}
COPY --from=build /src/target/release/lykilheim /usr/local/bin/lykilheim
USER 65532:65532
EXPOSE 8200
ENTRYPOINT ["/usr/local/bin/lykilheim"]
