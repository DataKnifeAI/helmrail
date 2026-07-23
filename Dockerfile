# Minimal operator image (M0 stub). Expand when kube-rs reconcile lands.
FROM rust:1.87-bookworm AS builder
WORKDIR /src
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates ./crates
RUN cargo build --release -p nauarchos-operator

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /src/target/release/nauarchos-operator /usr/local/bin/nauarchos-operator
USER nonroot:nonroot
ENTRYPOINT ["/usr/local/bin/nauarchos-operator"]
