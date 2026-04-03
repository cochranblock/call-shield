<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized work items for call-shield. Most important at top. Max 20.

> Tags: `[build]` `[test]` `[fix]` `[feature]` `[docs]` `[research]`
>
> Cross-project deps in **bold**. This backlog self-reorganizes based on recency and relevance.

---

1. `[docs]` Update govdoc dates — SECURITY, CMMC, ACCESSIBILITY, SSDF, SBOM, PRIVACY, FIPS, FedRAMP still say "Last updated: 2026-03-27" despite content changes in 2026-04-02/03.
2. `[feature]` PWA service worker scope fix — `sw.js` registration is relative, may fail in subdirectory deployments. Use absolute path.
3. `[test]` Android unit tests — no JUnit tests for IntentClassifier.java or ShieldScreeningService. Add to gradle build.
4. `[build]` crates.io publish — metadata ready (`cargo publish --dry-run` passed). Bump version done (0.2.0). Publish when stable.
5. `[feature]` Whisper Tiny integration — embed quantized Whisper model for on-device speech-to-text. Core whitepaper promise. **Depends on [kova](https://github.com/cochranblock/kova) Candle/Whisper work in pixel-forge and kova inference modules**
6. `[build]` iOS Xcode project — no `.xcodeproj` or `Package.swift` exists. Swift code compiles nowhere. Need extension target for CXCallDirectoryExtension. **Depends on Apple developer account for provisioning**
7. `[research]` IRONHIVE model upgrade — nodes run qwen2.5-coder:0.5b (Light tier). Evaluate 7B/14B models for code review quality. n1/gd has 31G RAM, n2/bt has 48G RAM. **Depends on [kova](https://github.com/cochranblock/kova) IRONHIVE cluster config**
8. `[fix]` st node DNS — `/etc/resolv.conf` empty on st (kova-elite-support). Needs sudo to fix. Blocks IRONHIVE 4/4 coverage.
9. `[research]` Telephony integration — Android `CallScreeningService` gets no audio/transcript. Evaluate `InCallService` (Android 14+) or accessibility-based transcript capture for real-time classification. **Depends on Android test device**
10. `[feature]` CLI output: `--json` flag for `classify` and `screen` — structured output for scripting/piping. Currently no machine-readable mode from the Rust CLI.
11. `[ux]` `whitelist remove` silent on missing entry — no feedback if number was never whitelisted. Should print "not found: <number>" and exit nonzero.
12. `[ux]` `log` command: add `--tail N` option to show last N entries instead of full dump.
13. `[ux]` Terminal emoji in `screen` — 📞 ☎️ ✅ 🛑 ⚠️ may render as boxes on non-UTF8 terminals. No `--no-emoji` or `TERM` check.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture.*
