<!-- Unlicense — cochranblock.org -->

# Supply Chain Integrity

*How Call Shield's build pipeline is secured.*

## Dependency Sources

| Source | Count | Verification |
|--------|-------|-------------|
| crates.io | 0 | N/A — no external dependencies |
| Rust std | 1 | Shipped with rustc, signed by Rust project |

**This project has zero third-party dependencies.** The supply chain attack surface is limited to the Rust compiler and standard library.

## Build Reproducibility

- **Cargo.lock committed:** Yes — exact dependency versions (std) pinned
- **Edition pinned:** Rust 2024
- **Release profile deterministic:** LTO + single codegen unit reduces non-determinism
- **No vendored binaries:** Everything builds from source
- **No build scripts (build.rs):** No code execution during compilation beyond rustc

## Verification Steps

```bash
# Clone and build — should produce identical binary
git clone https://github.com/cochranblock/call-shield.git
cd call-shield
cargo build --release

# Verify no build scripts
ls src/build.rs
# Expected: No such file

# Verify no proc macros
grep "proc-macro" Cargo.toml
# Expected: no output

# Verify no network access during build
# Cargo only downloads crates; with zero deps, no downloads occur
cargo build --release --frozen
# Expected: success (no network needed)
```

## Source Availability

- All source code: github.com/cochranblock/call-shield
- License: Unlicense (public domain equivalent)
- Full git history available
- All commits document AI vs human contributions in TIMELINE_OF_INVENTION.md

## What We Don't Do

- No pre-built binary distribution (build from source)
- No Docker images (no container supply chain risk)
- No CI/CD pipeline (no third-party build service access)
- No vendored C libraries
- No FFI calls in CLI binary (iOS lib exposes C ABI for Swift interop -- see [SUPPLY_CHAIN_AUDIT.md](SUPPLY_CHAIN_AUDIT.md))
- No dynamic linking beyond libc (stripped in release)

---

*Last updated: 2026-04-02*

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture.*
