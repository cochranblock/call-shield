<!-- Unlicense — cochranblock.org -->

# CMMC Compliance Mapping

*Cybersecurity Maturity Model Certification — Level 1 and 2 practices.*

## Overview

Call Shield is an on-device tool with no network connectivity, no data storage, and no authentication. Many CMMC domains have limited applicability, but the following practices are supported by design.

## Level 1 Practices

### AC — Access Control

| Practice | Status | Evidence |
|----------|--------|----------|
| AC.L1-3.1.1 — Limit system access to authorized users | N/A | Standalone CLI tool. Access controlled by OS-level file permissions on the binary. |
| AC.L1-3.1.2 — Limit system access to authorized functions | Done | Binary has exactly 3 functions exposed to user: help, version, classify. No hidden commands. |
| AC.L1-3.1.20 — Control connections to external systems | Done | Zero external connections. No network code. Verified by zero dependencies. |

### IA — Identification and Authentication

| Practice | Status | Evidence |
|----------|--------|----------|
| IA.L1-3.5.1 — Identify system users | N/A | No user accounts. No authentication. Tool runs as the OS user who invoked it. |
| IA.L1-3.5.2 — Authenticate users | N/A | Same as above. |

### SC — System and Communications Protection

| Practice | Status | Evidence |
|----------|--------|----------|
| SC.L1-3.13.1 — Monitor/control communications at boundaries | Done | No communications. No boundaries. Binary is self-contained. |
| SC.L1-3.13.5 — Implement subnetworks for public components | N/A | No network components. |

### MP — Media Protection

| Practice | Status | Evidence |
|----------|--------|----------|
| MP.L1-3.8.3 — Sanitize media before disposal | Done | No data written to disk. Process memory deallocated on exit. Planned: explicit memory zeroing for audio buffers. |

## Level 2 Practices

### AU — Audit and Accountability

| Practice | Status | Evidence |
|----------|--------|----------|
| AU.L2-3.3.1 — Create audit logs | Partial | Classification results printed to stdout. Can be redirected to log file by operator. No built-in audit log yet. |
| AU.L2-3.3.2 — Trace actions to users | N/A | Single-user CLI tool. OS-level audit (auditd) covers this. |

### CM — Configuration Management

| Practice | Status | Evidence |
|----------|--------|----------|
| CM.L2-3.4.1 — Establish baselines | Done | Cargo.lock pins exact build. Git history tracks all changes. Release profile is deterministic. |
| CM.L2-3.4.2 — Enforce security config settings | Done | No configuration files. No runtime settings to misconfigure. Behavior is compiled in. |

### SI — System and Information Integrity

| Practice | Status | Evidence |
|----------|--------|----------|
| SI.L2-3.14.1 — Identify and correct flaws | Done | Clippy with -D warnings. Rust memory safety. QA rounds documented. |
| SI.L2-3.14.2 — Protect from malicious code | Done | Zero third-party deps. No dynamic loading. No plugins. Single static binary. |
| SI.L2-3.14.6 — Monitor for unauthorized use | N/A | Offline tool. No network monitoring point. |

## Summary

Call Shield's minimal attack surface (no network, no storage, no auth) means most CMMC controls are satisfied by absence. The tool can't leak CUI because it doesn't store or transmit data. The primary compliance value is in the development practices: source control, SBOM, reproducible builds, and memory-safe language.

---

*Last updated: 2026-03-27*
