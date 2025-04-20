# Use the CONTAINER_CMD environment variable if set, otherwise default to podman
container-cmd := env("CONTAINER_CMD", "podman")

# Use the IMAGE environment variable if set, otherwise default to localhost/greenboot
image := env("CONTAINER_IMAGE", "localhost/greenlight")

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
    {{container-cmd}} build -t {{image}} .

# Push the Bootc container image
cpush:
    {{container-cmd}} push {{image}}

# Run the container
crun:
    {{container-cmd}} run --rm -it {{image}}

# Generate and open host-side docs
host-docs:
    cargo doc --open

# Full flow: test, build, containerize
all: test build cbuild
