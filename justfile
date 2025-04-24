# Use the CONTAINER_CMD environment variable if set, otherwise default to podman
container-cmd := env("CONTAINER_CMD", "podman")

# Use the IMAGE environment variable if set, otherwise default to localhost/greenboot
image := env("CONTAINER_IMAGE", "localhost/greenlight")


# Build statically linked binary for Bootc image
build target:
    cargo build --release --target {{target}}

cross-build target:
    cross build --release --target {{target}}

# Run normal tests
test:
    cargo test

# Run ignored tests (e.g. for bootc env checks)
test-bootc:
    cargo test -- --ignored

# Build the Bootc container image
cbuild target:
    {{container-cmd}} build -v $PWD/greenlight:/greenlight --build-arg BUILD_TARGET={{target}} -t {{image}} .

# Push the Bootc container image
cpush:
    {{container-cmd}} push {{image}}

# Run the container
crun:
    {{container-cmd}} run --rm -it {{image}}

# Generate and open host-side docs
host-docs:
    cargo doc --open
