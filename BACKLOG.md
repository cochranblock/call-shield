<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized work items for call-shield. Most important at top. Max 20.

> Tags: `[build]` `[test]` `[fix]` `[feature]` `[docs]` `[research]`
>
> Cross-project deps in **bold**. This backlog self-reorganizes based on recency and relevance.

---

1. `[feature]` Shared pattern table — extract classifier patterns to single canonical source, generate per-platform code at build time. Currently copy-pasted 4x (Rust, iOS, Java, JS). Any pattern update requires 4 edits.
2. `[feature]` Whisper Tiny integration — embed quantized Whisper model for on-device speech-to-text. Core whitepaper promise, biggest claim-vs-reality gap. **Depends on [kova](https://github.com/cochranblock/kova) Candle/Whisper work in pixel-forge and kova inference modules**
3. `[test]` iOS lib tests — add `#[cfg(test)] mod tests` to `ios/src/lib.rs` with same classifier regression tests as CLI. Currently untested.
4. `[fix]` iOS CString null byte panic — `ios/src/lib.rs:25` `CString::new(result).unwrap()` will panic across FFI if result contains a null byte. Use `unwrap_or_else` with safe fallback.
5. `[build]` Add `rust-toolchain.toml` — edition 2024 requires recent stable. Pin toolchain for reproducible builds. Recommended by SUPPLY_CHAIN_AUDIT.md but never done.
6. `[feature]` Contact whitelist — skip classification for known contacts. Android: query ContactsContract. CLI: `--whitelist` file. Most requested missing feature per README roadmap.
7. `[fix]` Unbounded memory in `screen` command — each line of stdin allocates a Rust String with no limit. Attacker could exhaust memory. Add max input length.
8. `[build]` Embed SUPPLY_CHAIN_AUDIT.md in binary — `govdocs/SUPPLY_CHAIN_AUDIT.md` is the only govdoc not embedded via `include_str!`. Add to `f5` dispatch.
9. `[feature]` Local call log — write screening decisions to local file (JSON lines). No cloud. Enables post-hoc review of blocked calls.
10. `[build]` Bump Cargo.toml version to 0.2.0 — commit message says v0.2.0 but `--version` still prints 0.1.0.
11. `[fix]` Android `startActivityForResult` deprecated — replace with `registerForActivityResult` Activity Result API in ShieldActivity. Lint warning on API 35.
12. `[build]` iOS Xcode project — no `.xcodeproj` or `Package.swift` exists. Swift code compiles nowhere. Need extension target for CXCallDirectoryExtension. **Depends on Apple developer account for provisioning**
13. `[feature]` Configurable sensitivity — `--threshold` flag for spam/legit cutoff (currently hardcoded 0.5). Let users tune false positive rate.
14. `[research]` IRONHIVE model upgrade — nodes run qwen2.5-coder:0.5b (Light tier). Evaluate 7B/14B models for code review quality. n1/gd has 31G RAM, n2/bt has 48G RAM. **Depends on [kova](https://github.com/cochranblock/kova) IRONHIVE cluster config**
15. `[fix]` st node DNS — `/etc/resolv.conf` empty on st (kova-elite-support). Needs sudo to fix. Blocks IRONHIVE 4/4 coverage.
16. `[test]` Android unit tests — no JUnit tests for IntentClassifier.java or ShieldScreeningService. Add to gradle build.
17. `[docs]` Update govdoc dates — SECURITY, CMMC, ACCESSIBILITY, SSDF, SBOM, PRIVACY, FIPS, FedRAMP still say "Last updated: 2026-03-27" despite content changes in 2026-04-02.
18. `[feature]` PWA service worker scope fix — `sw.js` registration is relative, may fail in subdirectory deployments. Use absolute path.
19. `[research]` Telephony integration — Android `CallScreeningService` gets no audio/transcript. Evaluate `InCallService` (Android 14+) or accessibility-based transcript capture for real-time classification. **Depends on Android test device**
20. `[build]` crates.io publish — metadata ready (`cargo publish --dry-run` passed). Publish when version bump and shared pattern table land.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture.*
