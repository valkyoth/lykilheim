ARG RUST_IMAGE=docker.io/library/rust:1.96.0-bookworm@sha256:6514fcea6ed535a18ede60db527d6c0edcbd625d3b5f3ca0b0c16096a55cbcba
ARG RUNTIME_IMAGE=cgr.dev/chainguard/wolfi-base@sha256:802712befa6f8f12a4f13dbe6df75fe65400ebe8eb7f570ee7976b0d738b6751

FROM ${RUST_IMAGE} AS build
WORKDIR /src
COPY . .
RUN cargo build --locked --release

FROM ${RUNTIME_IMAGE}
COPY --from=build /src/target/release/lykilheim /usr/local/bin/lykilheim
USER 65532:65532
EXPOSE 8200
ENTRYPOINT ["/usr/local/bin/lykilheim"]
