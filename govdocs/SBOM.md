<!-- Unlicense — cochranblock.org -->

# Software Bill of Materials (SBOM)

*Per Executive Order 14028 — Improving the Nation's Cybersecurity*

## Project

- **Name:** call-shield
- **Version:** 0.1.0
- **Language:** Rust (edition 2024)
- **Build tool:** Cargo

## Dependencies

```
call-shield v0.1.0
└── (no dependencies)
```

**Total external dependencies: 0**

This project has zero third-party dependencies. The entire binary is built from the Rust standard library and project source code.

## Rust Standard Library

| Component | Version | License | Source |
|-----------|---------|---------|--------|
| std | Matches rustc version | MIT/Apache-2.0 | rust-lang/rust |

## Verification

```bash
cargo tree --depth 1
# Output: call-shield v0.1.0 (no deps listed)

cargo tree --format "{p} {l}"
# Output: call-shield v0.1.0 Unlicense
```

## SBOM Format

This document serves as a human-readable SBOM. For machine-readable formats:

```bash
# CycloneDX format (requires cargo-cyclonedx)
cargo cyclonedx --format json

# SPDX format (requires cargo-spdx)
cargo spdx
```

## Supply Chain Notes

- All source available at github.com/cochranblock/call-shield
- No vendored binaries
- No pre-built artifacts
- Build is deterministic via Cargo.lock pinning
- Unlicense — public domain equivalent

---

*Last updated: 2026-03-27*
