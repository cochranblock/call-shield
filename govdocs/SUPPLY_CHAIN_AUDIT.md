<!-- Unlicense — cochranblock.org -->

# Supply Chain Security Audit

*Federal-grade dependency audit per EO 14028. Performed 2026-03-30.*

## Summary

**PASS — zero attack surface.** This project has zero third-party dependencies.

## Audit Results

| Check | Result | Detail |
|-------|--------|--------|
| `cargo audit` | PASS | 0 advisories on 1 crate (ourselves) |
| `cargo tree --duplicates` | PASS | No duplicates (no deps) |
| `Cargo.lock` committed | PASS | Pinned for reproducible builds |
| Copyleft license conflict | PASS | No deps = no license conflicts |
| Yanked crates | PASS | No deps to yank |
| Unmaintained crates | PASS | No deps |
| Typosquatting risk | PASS | No deps to impersonate |
| Vendored binaries | PASS | None |
| `unsafe` blocks in source | PASS | 0 in main binary (`src/main.rs`) |
| `unsafe` blocks in iOS lib | 2 | Required for C ABI (`extern "C"` FFI boundary) |
| Network calls | PASS | No networking code in any binary |
| Shell-out (`Command`) | PASS | No `std::process::Command` usage |
| Environment variable reads | PASS | Only `CARGO_PKG_VERSION` at compile time |
| Secret exfiltration vectors | PASS | No file reads, no network, no env at runtime |

## Deep Code Review

### Rust CLI (`src/main.rs`, 474 LOC)

- **unsafe blocks:** 0
- **unwrap() calls:** 1 (on `stdin.lock().read_line()` — panics only on broken stdin pipe, acceptable)
- **Raw pointers:** 0
- **Buffer handling:** Rust `String`/`Vec` with bounds checking
- **Integer overflow:** No size calculations on untrusted input
- **Command injection:** No shell execution
- **Input validation:** CLI args matched by exact string comparison

### iOS Library (`ios/src/lib.rs`, 114 LOC)

- **unsafe blocks:** 2 (both at FFI boundary — `CStr::from_ptr` and `CString::from_raw`)
- **Justified:** Required for C ABI interop with Swift. Null pointer checks on both functions.
- **Memory:** Caller must free via `call_shield_free` — documented in function signature

### Android Java (`android/app/src/main/java/`, 248 LOC)

- **Permissions:** `READ_PHONE_STATE`, `READ_CALL_LOG`, `RECORD_AUDIO` — all required for call screening
- **No INTERNET permission:** Verified in AndroidManifest.xml
- **No reflection:** No dynamic class loading
- **ProGuard:** Minification enabled, keeps only screening service and classifier

### PWA (`web/index.html`, 166 LOC)

- **No external scripts:** All JS inline
- **No fetch/XHR:** No network calls
- **Service worker:** Cache-first, no phone-home
- **No cookies/localStorage for tracking:** None used

## Cargo Audit Output

```
Scanning Cargo.lock for vulnerabilities (1 crate dependencies)
(no advisories found)
```

## Recommendations

1. When deps are added (Whisper, audio capture): re-run this full audit
2. Add `cargo audit` to CI gate
3. Pin Rust toolchain version in `rust-toolchain.toml` for build reproducibility
4. Consider `cargo-vet` for first-party audits when deps arrive

---

*Last updated: 2026-03-30*
