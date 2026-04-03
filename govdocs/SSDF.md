<!-- Unlicense — cochranblock.org -->

# NIST SP 800-218 — Secure Software Development Framework (SSDF)

*Mapping Call Shield practices to SSDF tasks.*

## PS: Prepare the Organization

| Practice | Status | Evidence |
|----------|--------|----------|
| PS.1 — Define security requirements | Done | Zero-cloud architecture eliminates network attack surface. No audio exfiltration by design. See WHITEPAPER.md §2, §3. |
| PS.2 — Implement roles and responsibilities | Done | Single maintainer (The Cochran Block). AI-assisted development with human-directed architecture. All commits document AI role in TIMELINE_OF_INVENTION.md. |
| PS.3 — Implement supporting toolchains | Done | Rust compiler (memory safety by default), Clippy (lint enforcement with -D warnings), Cargo.lock (dependency pinning). |

## PW: Protect the Software

| Practice | Status | Evidence |
|----------|--------|----------|
| PW.1 — Design software to meet security requirements | Done | On-device only. No network listeners. No file I/O beyond stdout/stderr. No IPC. Attack surface is limited to CLI argument parsing. |
| PW.4 — Reuse existing, well-secured software | Done | Zero external dependencies. Rust std only. No third-party code to audit. |
| PW.5 — Create source code by adhering to secure coding practices | Done | Rust's borrow checker prevents memory corruption. No unsafe blocks. No raw pointer use. Clippy clean with warnings-as-errors. |
| PW.6 — Configure the compilation and build processes | Done | Release profile: LTO enabled, single codegen unit, panic=abort, stripped symbols. Cargo.lock committed for reproducible builds. |
| PW.7 — Review and/or analyze human-readable code | Done | QA Round 1 and Round 2 passed. Clippy --release -- -D warnings clean. |
| PW.9 — Test executable code | Done | 17 automated tests via `cargo test`: classifier correctness, false-positive regression, vishing vector regression, score edge cases, SBOM validation. Zero failures. |

## RV: Respond to Vulnerabilities

| Practice | Status | Evidence |
|----------|--------|----------|
| RV.1 — Identify and confirm vulnerabilities | Ready | Zero dependencies means zero CVE surface from third-party code. Rust std vulnerabilities tracked via rustup/advisory-db. |
| RV.2 — Assess, prioritize, and remediate | Ready | Single binary, single maintainer — remediation path is direct. |
| RV.3 — Analyze vulnerabilities to identify root causes | N/A | No vulnerabilities reported. |

## PO: Protect Operations

| Practice | Status | Evidence |
|----------|--------|----------|
| PO.1 — Secure build environments | Done | Builds from source on maintainer hardware. No CI/CD pipeline (no third-party build exposure). Deterministic via Cargo.lock. |

## Verification

```bash
# Confirm zero unsafe blocks
grep -r "unsafe" src/
# Expected: no output

# Confirm zero external deps
cargo tree --depth 1
# Expected: call-shield v0.1.0 (no deps)

# Confirm clippy clean
cargo clippy --release -- -D warnings
# Expected: zero warnings
```

---

*Last updated: 2026-03-27*

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture.*
