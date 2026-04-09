<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized work items for call-shield. Most important at top. Max 20.

> Tags: `[build]` `[test]` `[fix]` `[feature]` `[docs]` `[research]`
>
> Cross-project deps in **bold**. This backlog self-reorganizes based on recency and relevance.

---

1. `[fix]` Implement `call_shield_classify` / `call_shield_free` C-ABI exports in Rust lib target — iOS `AppDelegate.swift` calls these via `@_silgen_name` but they don't exist in any Rust source. Add `#[no_mangle] pub extern "C"` functions outputting `VERDICT|score|matched` pipe-delimited. Unblocks iOS platform entirely.
2. `[build]` crates.io publish — metadata ready (`cargo publish --dry-run` passed). Bump version done (0.2.0). Publish when stable.
3. `[feature]` Whisper Tiny integration — embed quantized Whisper model for on-device speech-to-text. Core whitepaper promise. **Depends on [kova](https://github.com/cochranblock/kova) Candle/Whisper work in pixel-forge and kova inference modules**
4. `[build]` iOS Xcode project — no `.xcodeproj` or `Package.swift` exists. Swift code compiles nowhere. Need extension target for CXCallDirectoryExtension. **Depends on Apple developer account for provisioning**
5. `[research]` IRONHIVE model upgrade — nodes run qwen2.5-coder:0.5b (Light tier). Evaluate 7B/14B models for code review quality. n1/gd has 31G RAM, n2/bt has 48G RAM. **Depends on [kova](https://github.com/cochranblock/kova) IRONHIVE cluster config**
6. `[fix]` st node DNS — `/etc/resolv.conf` empty on st (kova-elite-support). Needs sudo to fix. Blocks IRONHIVE 4/4 coverage.
7. `[research]` Telephony integration — Android `CallScreeningService` gets no audio/transcript. Evaluate `InCallService` (Android 14+) or accessibility-based transcript capture for real-time classification. **Depends on Android test device**
8. `[feature]` CLI output: `--json` flag for `classify` and `screen` — structured output for scripting/piping. Currently no machine-readable mode from the Rust CLI.
9. `[ux]` `whitelist remove` silent on missing entry — no feedback if number was never whitelisted. Should print "not found: <number>" and exit nonzero.
10. `[ux]` `log` command: add `--tail N` option to show last N entries instead of full dump.
11. `[ux]` Terminal emoji in `screen` — 📞 ☎️ ✅ 🛑 ⚠️ may render as boxes on non-UTF8 terminals. No `--no-emoji` or `TERM` check.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture.*
