# Justfile for bootc-dpu-check

# Build statically linked binary for Bootc image
build:
    cargo build --release --target x86_64-unknown-linux-musl

# Run normal tests
test:
    cargo test

# Run ignored tests (e.g. for bootc env checks)
test-bootc:
    cargo test -- --ignored

# Build the Bootc container image
cbuild:
    podman build -t quay.io/samueldasilva/greenboot .
# push the Bootc container image
cpush:
    podman push -t quay.io/samueldasilva/greenboot

# Run the container
crun:
    podman run --rm -it quay.io/samueldasilva/greenboot

# Full flow: test, build, containerize
# all: test container-build
