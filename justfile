# Environment-backed defaults
container-cmd := env("CONTAINER_CMD", "podman")
bootc-image := env("BOOTC_IMAGE", "localhost:5000/greenlight-bootc-test:rhel9")
runtime-image := env("GREENLIGHT_IMAGE", "localhost/greenlight")
target := env("BUILD_TARGET", "x86_64-unknown-linux-musl")

# ─── Build statically linked binary ─────────────────────────────────────────────
build:
    cargo build --release --target {{target}}

cross-build:
    cross build --release --target {{target}}

# ─── Tests ──────────────────────────────────────────────────────────────────────
test:
    cargo test

test-bootc:
    cargo test -- --ignored

# ─── Bootc VM Testing Image ─────────────────────────────────────────────────────
bootc-build ssh-key user-passwd:
    {{container-cmd}} build \
        --build-arg SSH_KEY='{{ssh-key}}' \
        --build-arg USER_PASSWD='{{user-passwd}}' \
        --build-arg BUILD_TARGET='{{target}}' \
        -f Containerfile.microshift \
        -t {{bootc-image}} .

bootc-push:
    {{container-cmd}} push --tls-verify=false {{bootc-image}}

bootc-run:
    {{container-cmd}} run --rm -it {{bootc-image}}

# ─── Greenlight Runtime Container ───────────────────────────────────────────────
runtime-build:
    {{container-cmd}} build \
        -f Containerfile \
        -t {{runtime-image}} .

runtime-push:
    {{container-cmd}} push {{runtime-image}}

runtime-run:
    {{container-cmd}} run --rm -it {{runtime-image}}

# ─── Documentation ─────────────────────────────────────────────────────────────
host-docs:
    cargo doc --open

