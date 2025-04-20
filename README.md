# Greenlight

**Greenlight** is a boot-time validation tool for embedded and edge systems such as SmartNICs (DPUs), automotive devices, and other RHEL-based IoT deployments.  
It is designed to run early in the boot process and ensure system correctness before continuing to full initialization. If a validation check fails, **Greenlight** can integrate with [Greenboot](https://github.com/fedora-iot/greenboot) to trigger a rollback.

---

## ✨ Features

- ✅ YAML-configurable system validation
- 🔜 Deployment-aware checks (`bootc`, `ostree`, `traditional`)
- ✅ Target-specific logic (`dpu`, `automotive`, `edge`)
- 🔜 Planned: Greenboot rollback integration
- ✅ Suitable for minimal, early-boot environments

---

## 🔧 Configuration

Example `greenlight.yaml`:

```yaml
system:
  deployment: bootc
  arch: aarch64
  target: dpu

logging:
  kind: basic
  level: info

checks:
  include:
    - rootfs_readonly
    - microshift_installed
```

---

## 📦 Project Structure

```
greenlight/
├── src/
│   ├── config.rs         # Configuration schema and system models
│   ├── checks.rs         # Built-in validation checks
│   └── lib.rs            # Library entry point
├── tests/
│   └── config_tests.rs   # Integration tests for parsing and logic
├── greenlight.yaml       # Example YAML configuration
├── justfile              # Just command runner tasks
├── flake.nix             # Optional Nix development environment
└── Containerfile         # OCI container definition
```

---

## ✅ Available Checks

| Check Kind                    | Applies To                  | Status           |
|------------------------------|-----------------------------|------------------|
| `rootfs_readonly`            | All except `traditional`    | ❌ Not implemented |
| `bootc_status_matches_os_release` | Only `bootc` deployment | ❌ Not implemented |
| `microshift_installed`       | Only `dpu` target           | ❌ Not implemented |
| `expected_interface_present` | Only `dpu` target           | ❌ Not implemented |
| `swap_disabled`              | Only `dpu` target           | ❌ Not implemented |
| `sshd_running`               | Only `dpu` target           | ❌ Not implemented |

---

## 🧪 Testing

All tests live in [`tests/`](./tests). They cover:

- YAML parsing
- Type-level validation (e.g. `bootc` vs `traditional`)
- Target enforcement for check applicability

```bash
cargo test
```

---

## 🛠 Development Guide

### Build a static binary (for container or bare-metal)

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

### Use Nix flakes for development

```bash
echo "use flake" >> .envrc
nix develop
```

### Use Just for workflow automation

Install [`just`](https://github.com/casey/just):

```bash
just build
just test
just cbuild     # Build container image
just cpush      # Push container image
```

Docs:

```bash
just host-docs
```

---

## 📋 License

This project is licensed under the **Apache 2.0 License**.  
See [LICENSE](./LICENSE) for more.

---

## ✍️ Author

Samuel Dasilva — [@SamDPenguin](https://github.com/SamD2021)
