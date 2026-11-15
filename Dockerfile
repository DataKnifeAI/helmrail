# Minimal operator image (M0 stub). Expand when kube-rs reconcile lands.
FROM rust:1.87-bookworm AS builder
WORKDIR /src
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates ./crates
RUN cargo build --release -p helmrail-operator

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /src/target/release/helmrail-operator /usr/local/bin/helmrail-operator
USER nonroot:nonroot
ENTRYPOINT ["/usr/local/bin/helmrail-operator"]
