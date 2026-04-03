<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

---

## Entries

### 2026-03-26 — Call Shield Whitepaper + Scaffold

**What:** Published whitepaper on on-device call screening without cloud. Rust binary scaffold. Architecture: Whisper Tiny (39MB quantized) + intent classifier (<1MB) + call handling logic, all compiled into a single ~42MB binary. Zero audio leaves the device.
**Why:** Founded by the contradiction of building zero-cloud infrastructure while using Google Call Screen. The alternative doesn't exist — so build it.
**Commit:** `1d6bad5`
**AI Role:** AI drafted whitepaper and technical analysis. Human identified the product gap from personal experience, directed the architecture based on existing Candle/Whisper work in kova and pixel-forge, and validated all size and latency claims.
**Proof:** [WHITEPAPER.md](WHITEPAPER.md)

### 2026-03-27 — README with backlink

**What:** Added README with product description and cochranblock.org backlink.
**Commit:** `f00f0b2`

### 2026-03-27 — Cargo.lock + QA Round 1 & 2

**What:** Committed Cargo.lock for reproducible binary builds. Ran two full QA rounds: clean compile, zero warnings, zero clippy errors, all paths verified.
**Commit:** `2fac3f5`
**QA Round 1:** PASS — `cargo build --release` clean, `git diff` empty, binary runs.
**QA Round 2:** PASS — `cargo clean && cargo build --release` clean, `cargo clippy --release -- -D warnings` zero warnings, `git status` clean.

### 2026-03-27 — P13 Tokenization + Binary Optimization

**What:** Applied Kova P13 compression mapping to all symbols. Created `docs/compression_map.md`. Added release profile for smallest binary: opt-level=z, LTO, single codegen unit, panic=abort, stripped. Binary: 285,936 bytes (279 KB) with zero deps.
**Commit:** `157adf7`
**AI Role:** AI applied tokenization and release profile per P13 protocol. Human directed.

### 2026-03-27 — User Story Analysis + Top 3 Fixes

**What:** Full end-to-end user story walkthrough. Scored scaffold 1.6/10. Implemented top 3 fixes:
1. CLI with `--help`, `--version`, error handling for bad input
2. Pattern-match intent classifier — 35 patterns across spam/legitimate/unknown categories
3. README rewrite with quick start, usage examples, and roadmap

Updated compression map to f0-f4, t0, s0-s1. Binary grew to 319,248 bytes (312 KB) with classifier.
**Commit:** `a8d679b`
**AI Role:** AI ran user story analysis, identified gaps, implemented all three fixes. Human directed the analysis.
**Proof:** [USER_STORY_ANALYSIS.md](USER_STORY_ANALYSIS.md)

### 2026-03-27 — Federal Compliance Documentation

**What:** Created `govdocs/` with 11 federal compliance documents: SBOM (EO 14028), SSDF (NIST SP 800-218), supply chain integrity, security posture, accessibility (Section 508), privacy impact assessment, FIPS 140-2/3 status, FedRAMP applicability notes, CMMC Level 1-2 mapping, ITAR/EAR export classification, and federal use cases for 7 agencies (DoD, DHS, VA, DOJ, NSF, DOE, GSA).
**Commit:** `efd8e91`
**AI Role:** AI drafted all compliance documents. Human directed scope and verified claims against source.

### 2026-03-27 — TOI/POA Update

**What:** Updated Timeline of Invention and Proof of Artifacts with all commits, binary sizes, QA results, P13 stats.
**Commit:** `151784d`

### 2026-03-28 — Embedded Govdocs + Interactive Screening + SPDX SBOM

**What:** The binary now serves its own compliance docs at runtime. Added:
1. `govdocs` subcommand — prints any of 11 embedded compliance docs to stdout
2. `--sbom` flag — outputs machine-readable SPDX 2.3 format SBOM parsed live from embedded Cargo.toml
3. `screen` command — interactive call screening session with multi-turn conversation, real-time classification, session stats, and automatic routing decisions (block spam, pass legit, prompt unknowns, voicemail after 3 inconclusive turns)
4. Enhanced classifier — now reports matched patterns in output
5. Compression map updated: f0-f10, t0-t1, s0-s1 (11 functions, 2 types, 2 fields)

Binary: 368,896 bytes (360 KB), zero dependencies.
**Commit:** `9047b17`
**AI Role:** AI implemented all features. Human directed architecture and dogfooding requirement.

### 2026-03-28 — Crates.io Prep

**What:** Added crates.io metadata to Cargo.toml: description, repository URL, keywords, categories. `cargo publish --dry-run` passed clean.
**Commit:** `ad38176`

### 2026-03-29 — Android App (CallScreeningService, API 35)

**What:** Full Android app: `ShieldScreeningService` (registered `CallScreeningService`), `IntentClassifier` (35-pattern Java port), `ShieldActivity` (enable shield, test classifier, CRT green theme). No INTERNET permission. `network_security_config.xml` denies cleartext. Package: `org.cochranblock.callshield`, minSdk 29, targetSdk 35.
**Commit:** `459c6ca`
**AI Role:** AI implemented Android app. Human directed architecture and package naming.

### 2026-03-29 — iOS App + PWA + Multi-Arch Build Script

**What:** Three platforms in one commit:
1. iOS static library (`ios/src/lib.rs`) — C ABI bridge for Swift, `call_shield_classify`/`call_shield_free`. Swift AppDelegate with `@_silgen_name` bridge, CallKit integration. Builds for `aarch64-apple-ios` (5.3 MB .a file).
2. PWA (`web/`) — offline-first Progressive Web App with service worker, same 35-pattern classifier in JS, manifest.json, installable from any browser.
3. Multi-arch build script (`scripts/build-all-targets.sh`) — 12 targets: macOS ARM/Intel, Linux x86/ARM64/ARM32, RISC-V, Windows, FreeBSD, POWER, Android, iOS, WASM.
**Commit:** `64e31a5`
**AI Role:** AI implemented all three platforms. Human directed architecture.

### 2026-03-29 — Android AAB Build

**What:** Fixed `ShieldScreeningService` API (correct `getHandlePresentation()` + `TelecomManager` constants). Generated launcher icons for all densities. Added gradle wrapper. Built real `app-release.aab`: 14,105 bytes (14 KB), R8 minified, resources shrunk. Uploaded to GitHub Release v0.1.0.
**Commit:** `8954d1c`
**AI Role:** AI fixed API, generated icons, built AAB. Human directed.

### 2026-03-30 — Truth Audit + Supply Chain Audit

**What:** Adversarial fact-check of every claim in README, POA, TOI, and govdocs. Spot-checked 5 commit hashes against `git show --stat` — all match. Rebuilt binary and verified sizes. Ran `cargo audit` (0 advisories). Reviewed all source for unsafe blocks (0 in CLI, 2 justified in iOS FFI). Wrote `govdocs/SUPPLY_CHAIN_AUDIT.md` with full federal-grade dependency analysis. Updated TOI with all 12 commits. Updated POA with verified metrics. Hardened `.gitignore`.
**Commit:** `224aaf6`
**AI Role:** AI ran full audit and fixed all stale docs. Human directed.

### 2026-03-31 — TOI/POA Update

**What:** Updated Timeline and Proof of Artifacts with truth audit commit.
**Commit:** `4c6d8f0`

### 2026-04-02 — v0.2.0: Classifier Fix + Tests + Android Wiring

**What:** Guest analysis found 3 critical, 8 major issues. Fixed all critical:
1. **Classifier false positives:** `"irs"` matched inside "first"/"birthday" — changed to `"the irs"`/`"irs agent"`. Moved `"from your bank"` from legit to spam (vishing vector). Added `"verify your account"` and `"confirm your identity"` to spam. All 4 platforms updated (Rust, iOS, Android, PWA). Pattern count: 38 (24 spam + 14 legit).
2. **Android screening service:** Wired `IntentClassifier` into `ShieldScreeningService.onScreenCall()`. Removed unused `RECORD_AUDIO` permission.
3. **iOS dead code:** Removed `CXCallDirectoryManager.reloadExtension` call to non-existent extension.
4. **17 automated tests:** Classifier correctness, false-positive regression, vishing vector regression, score edge cases, SBOM validation.
5. **README truth-aligned:** Accurate pattern counts, test count, Whisper marked as target not current.

**Commit:** `3bb7db2`
**AI Role:** AI ran guest analysis, identified all issues, implemented fixes and tests. Human directed.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
