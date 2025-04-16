# syntax=docker/dockerfile:1

##########################################
# 🛠️ Stage 1 — Build the greenlight binary
FROM docker.io/rust:1.84-slim as builder

# Install musl-tools for static builds
RUN apt-get update && apt-get install -y musl-tools pkg-config && \
    rustup target add x86_64-unknown-linux-musl

# Set workdir and copy source
WORKDIR /app
COPY . .

# Build statically linked binary
RUN cargo build --release --target x86_64-unknown-linux-musl

##########################################
# 📦 Stage 2 — Minimal runtime image (UBI 10)
FROM quay.io/centos/centos:stream10

# Copy statically linked greenlight binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/greenlight /usr/bin/greenlight
RUN chmod +x /usr/bin/greenlight

# Optional: greenboot integration
# COPY contrib/greenboot-check /etc/greenboot/check/required.d/50-greenlight
# RUN chmod +x /etc/greenboot/check/required.d/50-greenlight

# Set entrypoint
CMD ["/usr/bin/greenlight"]

