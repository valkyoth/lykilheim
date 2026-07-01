ARG RUST_IMAGE=docker.io/library/rust:1.96.1-bookworm@sha256:d99f7b31f49909348dc59b51f3c95d1efded1701ffb222f095aaab7de3c4abd8
ARG RUNTIME_IMAGE=cgr.dev/chainguard/wolfi-base@sha256:eba430503496d7a3b3bbf96cb0656e1daa37b6044c61c362778b7e17d371db3a

FROM ${RUST_IMAGE} AS build
WORKDIR /src
COPY . .
RUN cargo build --locked --release

FROM ${RUNTIME_IMAGE}
COPY --from=build /src/target/release/lykilheim /usr/local/bin/lykilheim
USER 65532:65532
EXPOSE 8200
ENTRYPOINT ["/usr/local/bin/lykilheim"]
