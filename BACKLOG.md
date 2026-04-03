<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized work items for call-shield. Most important at top. Max 20.

> Tags: `[build]` `[test]` `[fix]` `[feature]` `[docs]` `[research]`
>
> Cross-project deps in **bold**. This backlog self-reorganizes based on recency and relevance.

---

1. `[feature]` Whisper Tiny integration — embed quantized Whisper model for on-device speech-to-text. Core whitepaper promise, biggest claim-vs-reality gap. **Depends on [kova](https://github.com/cochranblock/kova) Candle/Whisper work in pixel-forge and kova inference modules**
2. `[feature]` Contact whitelist — skip classification for known contacts. Android: query ContactsContract. CLI: `--whitelist` file. Most requested missing feature per README roadmap.
3. `[feature]` Local call log — write screening decisions to local file (JSON lines). No cloud. Enables post-hoc review of blocked calls.
4. `[fix]` Android `startActivityForResult` deprecated — replace with `registerForActivityResult` Activity Result API in ShieldActivity. Lint warning on API 35.
5. `[build]` iOS Xcode project — no `.xcodeproj` or `Package.swift` exists. Swift code compiles nowhere. Need extension target for CXCallDirectoryExtension. **Depends on Apple developer account for provisioning**
6. `[feature]` Configurable sensitivity — `--threshold` flag for spam/legit cutoff (currently hardcoded 0.5). Let users tune false positive rate.
7. `[research]` IRONHIVE model upgrade — nodes run qwen2.5-coder:0.5b (Light tier). Evaluate 7B/14B models for code review quality. n1/gd has 31G RAM, n2/bt has 48G RAM. **Depends on [kova](https://github.com/cochranblock/kova) IRONHIVE cluster config**
8. `[fix]` st node DNS — `/etc/resolv.conf` empty on st (kova-elite-support). Needs sudo to fix. Blocks IRONHIVE 4/4 coverage.
9. `[test]` Android unit tests — no JUnit tests for IntentClassifier.java or ShieldScreeningService. Add to gradle build.
10. `[docs]` Update govdoc dates — SECURITY, CMMC, ACCESSIBILITY, SSDF, SBOM, PRIVACY, FIPS, FedRAMP still say "Last updated: 2026-03-27" despite content changes.
11. `[feature]` PWA service worker scope fix — `sw.js` registration is relative, may fail in subdirectory deployments. Use absolute path.
12. `[research]` Telephony integration — Android `CallScreeningService` gets no audio/transcript. Evaluate `InCallService` (Android 14+) or accessibility-based transcript capture for real-time classification. **Depends on Android test device**
13. `[build]` crates.io publish — metadata ready (`cargo publish --dry-run` passed). Publish when shared pattern table lands.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture.*
