ARG RUST_IMAGE=docker.io/library/rust:1.96.0-bookworm
ARG RUNTIME_IMAGE=cgr.dev/chainguard/wolfi-base:latest

FROM ${RUST_IMAGE} AS build
WORKDIR /src
COPY . .
RUN cargo build --locked --release

FROM ${RUNTIME_IMAGE}
COPY --from=build /src/target/release/lykilheim /usr/local/bin/lykilheim
USER 65532:65532
EXPOSE 8200
ENTRYPOINT ["/usr/local/bin/lykilheim"]
