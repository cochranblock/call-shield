<!-- Unlicense — cochranblock.org -->

# Security Posture

*Attack surface analysis and mitigations for Call Shield.*

## Cryptography

**Current:** None. Call Shield v0.1.0 does not use cryptography. There is no data at rest, no data in transit, no secrets, and no authentication.

**Planned (with Whisper integration):**
- Audio processed in-memory only, never written to disk
- No network communication — no TLS needed
- If local call log is added: AES-256-GCM at rest, Argon2id for key derivation

## Attack Surface

| Surface | Risk | Mitigation |
|---------|------|------------|
| CLI argument parsing | Low | Rust's String type prevents buffer overflow. No unsafe code. Arguments parsed as UTF-8 strings only. |
| Pattern matching (classifier) | Low | Read-only string comparison. No regex engine. No external input beyond CLI args. |
| stdout/stderr output | None | Display only. No file writes. No network writes. |
| Audio capture (planned) | Medium | Will process in-memory only. No disk write. No network send. Memory zeroed after classification. |
| Model loading (planned) | Medium | Whisper model embedded in binary via rust-embed. No runtime file loading from untrusted paths. |

## No Plaintext Secrets

```bash
# Verify no secrets in source
grep -ri "password\|secret\|api_key\|token" src/
# Expected: no output

# Verify no .env files
ls .env* 2>/dev/null
# Expected: no output
```

## Memory Safety

- Language: Rust (memory-safe by default)
- Unsafe blocks: 0 in CLI binary (`src/main.rs`). iOS lib (`ios/src/lib.rs`) has 2 justified unsafe blocks at FFI boundary.
- Raw pointer use: 0 in CLI binary. iOS lib uses `CStr::from_ptr`/`CString::from_raw` for Swift interop.
- Manual memory management: 0
- Buffer overflows: prevented by compiler

## Input Validation

| Input | Validation | Behavior on Invalid |
|-------|-----------|-------------------|
| CLI command | Exact string match | Error message + exit code 1 |
| Classify text | Any UTF-8 string accepted | Classifies as UNKNOWN if no patterns match |
| No arguments | Detected | Shows help text |
| Empty classify | Detected | Error message + exit code 1 |

## Known Limitations

1. Pattern-match classifier is not ML — adversarial callers can evade by rephrasing
2. No rate limiting (CLI tool, not a service)
3. No audit logging (stdout only)

---

*Last updated: 2026-03-27*

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture.*
