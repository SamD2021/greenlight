# syntax=docker/dockerfile:1

##########################################
# Stage 1 — Build the greenlight binary
FROM docker.io/rust:1.84-slim as builder

# Define build target (e.g., aarch64-unknown-linux-musl)
ARG BUILD_TARGET
ENV BUILD_TARGET=${BUILD_TARGET}

# Install musl-tools for static builds
RUN apt-get update && apt-get install -y musl-tools pkg-config && \
    rustup target add ${BUILD_TARGET}

# Set workdir and copy source
WORKDIR /app
COPY . .

# Build statically linked binary
RUN cargo build --release --target ${BUILD_TARGET}

# Explicitly copy it to known location for stage 2
RUN cp /app/target/${BUILD_TARGET}/release/greenlight-cli /greenlight

##########################################
# Stage 2 — Minimal runtime image
FROM scratch

# Copy the binary from stage 1
COPY --from=builder /greenlight /usr/bin/greenlight

# Set entrypoint
CMD ["/usr/bin/greenlight"]
