<!-- Unlicense — cochranblock.org -->

# Timeline of Invention — call-shield

*Dated, commit-level record of what was built, when, and why. Proves human-piloted AI development — not generated spaghetti.*

> Every entry below maps to real commits. Run `git log --oneline` to verify.

## How to Read This Document

Each entry follows this format:

- **Date**: When the work shipped (not when it was started)
- **What**: Concrete deliverable — binary, feature, fix, architecture change
- **Why**: Business or technical reason driving the decision
- **Commit**: Short hash(es) for traceability
- **AI Role**: What the AI did vs. what the human directed
- **Proof**: Link to artifact, screenshot, test output, or live URL

This document exists because AI-assisted code has a trust problem. Anyone can generate 10,000 lines of spaghetti. This timeline proves that a human pilot directed every decision, verified every output, and shipped working software.

---

## Human Revelations — Invented Techniques

*Novel ideas that came from human insight, not AI suggestion. These are original contributions to the field.*

### Sub-Millisecond Pattern-Match Call Screening (March 2026)

**Invention:** A call screening engine that classifies caller intent using 38 text patterns (24 spam + 14 legit) in a 360KB binary with zero dependencies — no ML model, no cloud API, no Whisper, just pattern matching fast enough to run between rings.

**The Problem:** Google Call Screen uses cloud ML. Apple's Silence Unknown Callers is binary (block all or nothing). Third-party apps require network access and phone permissions. Every solution either sends your call data to the cloud or lacks nuance (spam/not-spam with no middle ground).

**The Insight:** 90% of spam calls identify themselves in the first sentence. "This is the IRS." "Your car's extended warranty." "You've been selected." These patterns are so consistent that a regex-level classifier catches them. You don't need a 39MB Whisper model for the common case — you need 38 string patterns and a scoring function. Save the ML for the 10% of ambiguous calls.

**The Technique:**
1. 24 spam patterns: "irs agent", "extended warranty", "verify your account", "confirm your identity", etc.
2. 14 legit patterns: "this is dr", "pharmacy", "school calling", "appointment", etc.
3. Scoring: sum matched patterns per category, classify as spam/legit/unknown
4. Multi-turn screening: if unknown after 3 turns, route to voicemail
5. False-positive hardening: "irs" won't match inside "first" or "birthday" — patterns use context-aware matching

**Result:** Classification in <1ms. 360KB binary. Zero network calls. Runs on any Android via CallScreeningService, any iOS via CallKit, or any browser via PWA. The common case (obvious spam) is handled instantly; only ambiguous calls need human judgment.

**Named:** Pattern-Match Call Screening
**Commit:** `a8d679b` (classifier), `3bb7db2` (v0.2.0 false-positive fix)
**Origin:** Michael Cochran built zero-cloud infrastructure for a living, then realized his own phone used Google's cloud for call screening. The contradiction prompted the question: "What's the smallest possible binary that can screen calls?" Answer: 360KB.

### Vishing Vector Regression Testing (April 2026)

**Invention:** Automated regression tests that verify known vishing (voice phishing) attack vectors are correctly classified as spam, and known false-positive triggers are correctly classified as legit — preventing classifier updates from accidentally making the phone vulnerable.

**The Problem:** Classifier pattern updates can introduce regressions. Adding "from your bank" to the legit list (because your actual bank says it) also matches vishing attacks ("I'm calling from your bank, we need your SSN"). Moving patterns between categories requires testing every known attack vector, which humans forget to do.

**The Insight:** Vishing attack phrases are documented in FBI IC3 reports and CISA advisories. False-positive triggers (common words that match spam patterns) are discoverable by testing against normal conversation. If both sets are encoded as regression tests, every classifier update is automatically validated against known attacks AND known false positives.

**The Technique:**
1. Vishing vector tests: verify "from your bank" + SSN request = spam, "verify your account" = spam, "confirm your identity" = spam
2. False-positive tests: verify "first" doesn't trigger IRS pattern, "birthday" doesn't trigger IRS pattern
3. Legit regression tests: verify "this is dr smith" = legit, "pharmacy calling" = legit
4. All tests run through Triple Sims (3x), and the same vectors are mirrored into the Android JUnit suite so the Java port can't drift.

**Result:** P23 paranoia lens identified "from your bank" as a vishing vector that was classified as legit. Moved to spam. Regression tests now prevent reintroduction. Every classifier change is validated against known attack patterns on both Rust and Java.

**Named:** Vishing Vector Regression
**Commit:** `3bb7db2` (Rust), `72de8c0` (Java mirror)
**Origin:** P23 paranoia lens applied to call-shield — "what if an attacker knows our legit patterns and crafts phrases to match them?" The paranoia perspective identified a real vulnerability.

---

## Entries

*Reverse chronological. Most recent first.*

### 2026-04-09 — Android Privacy Hardening + JUnit Mirror Suite

**What:** Cleared backlog #2 with two changes:
1. **Removed `READ_CALL_LOG` permission** from `AndroidManifest.xml` — was never read by any Java source. Granting it would have given Call Shield full access to device call history, contradicting the privacy story and flagging in Play Store review. `READ_PHONE_STATE` is sufficient for `CallScreeningService`.
2. **JUnit test suite for `IntentClassifier`** — 60+ JVM unit tests in `android/app/src/test/java/org/cochranblock/callshield/IntentClassifierTest.java`, mirroring the Rust suite in `src/main.rs`. Coverage: every spam pattern, every legit pattern, unknown/no-match, case insensitivity, false-positive regression, multi-pattern resolution, score boundary conditions, and high-stakes vishing vectors. Wired JUnit 4.13.2 as `testImplementation` only — build-time, never shipped to device, zero-runtime-deps preserved.

**Why:** Privacy story integrity (the permission bloat would have nuked the "zero data leaves device" claim) and platform parity (the Java port had no test coverage; a regression in a pattern weight would silently break Android while Rust stayed green). Backlog renumbered (former #1, the `THRESHOLD` AtomicU64 fix, had already shipped in `6821641`). Rust gate: 152/152 still green.
**Commit:** `72de8c0`
**AI Role:** AI removed the permission, ported the Rust tests to JUnit, and wired Gradle. Human directed.
**Proof:** [android/app/src/test/java/org/cochranblock/callshield/IntentClassifierTest.java](android/app/src/test/java/org/cochranblock/callshield/IntentClassifierTest.java)

### 2026-04-08 — Human Revelations Documentation Pass

**What:** Documented novel human-invented techniques across the full CochranBlock portfolio. Added Human Revelations section with Pattern-Match Call Screening and Vishing Vector Regression.
**Why:** AI-generated code carries a credibility tax. Naming the human-originated inventions, with origin stories, separates the genuinely novel from the AI-formatted boilerplate.
**Commit:** See git log
**AI Role:** AI formatted and wrote the sections. Human identified which techniques were genuinely novel, provided the origin stories, and directed the documentation pass.

### 2026-04-02 — Truth-Align All Docs + Cross-Link

**What:** P23 synthesis pass: updated 16 files with accurate metrics (LOC, pattern counts, commit counts), scoped unsafe/FFI claims to correct binaries, updated SSDF PW.9 to Done (17 tests), fixed CMMC command count (3 -> 6), added cochranblock.org rendered footer to all 18 .md files. Marked WHITEPAPER Whisper architecture as target, added historical snapshot banner to USER_STORY_ANALYSIS.
**Why:** Adversarial fact-checking is the only defense against drift between code and docs. Without it, every claim erodes.
**Commit:** `f9eb6b8`
**AI Role:** AI ran full doc audit and applied all fixes. Human directed.

### 2026-04-02 — v0.2.0: P23 Guest Analysis + Classifier Fix + Tests

**What:** P23 Triple Lens analysis (pessimist lens: guest code review across all platforms) found 3 critical, 8 major, 9 minor issues. All critical fixed:
1. **Classifier false positives:** `"irs"` matched inside "first"/"birthday" — changed to `"the irs"`/`"irs agent"`. Moved `"from your bank"` from legit to spam (vishing vector). Added `"verify your account"` and `"confirm your identity"` to spam. All 4 platforms updated (Rust, iOS, Android, PWA). Pattern count: 38 (24 spam + 14 legit).
2. **Android screening service:** Wired `IntentClassifier` into `ShieldScreeningService.onScreenCall()`. Removed unused `RECORD_AUDIO` permission.
3. **iOS dead code:** Removed `CXCallDirectoryManager.reloadExtension` call to non-existent extension.
4. **17 automated tests:** Classifier correctness, false-positive regression, vishing vector regression, score edge cases, SBOM validation.
5. **README truth-aligned:** Accurate pattern counts, test count, Whisper marked as target not current.

**Why:** P23 pessimist lens (guest analysis as outside reviewer) identified claim-vs-reality gaps. Paranoia lens flagged vishing vectors. Synthesis drove the fix priority order.
**Commit:** `3bb7db2`
**AI Role:** AI ran P23 guest analysis, identified all issues, implemented fixes and tests. Human directed.

### 2026-03-31 — TOI/POA Update

**What:** Updated Timeline and Proof of Artifacts with truth audit commit.
**Why:** Keep the provenance documents in lockstep with the code; stale TOI/POA defeats their purpose.
**Commit:** `4c6d8f0`

### 2026-03-30 — Truth Audit + Supply Chain Audit

**What:** Adversarial fact-check of every claim in README, POA, TOI, and govdocs. Spot-checked 5 commit hashes against `git show --stat` — all match. Rebuilt binary and verified sizes. Ran `cargo audit` (0 advisories). Reviewed all source for unsafe blocks (0 in CLI, 2 justified in iOS FFI). Wrote `govdocs/SUPPLY_CHAIN_AUDIT.md` with full federal-grade dependency analysis. Updated TOI with all 12 commits. Updated POA with verified metrics. Hardened `.gitignore`.
**Why:** Federal procurement only trusts what it can verify. Every claim in our docs must survive an adversarial audit by an outsider with `git show`.
**Commit:** `224aaf6`
**AI Role:** AI ran full audit and fixed all stale docs. Human directed.

### 2026-03-29 — Android AAB Build

**What:** Fixed `ShieldScreeningService` API (correct `getHandlePresentation()` + `TelecomManager` constants). Generated launcher icons for all densities. Added gradle wrapper. Built real `app-release.aab`: 14,105 bytes (14 KB), R8 minified, resources shrunk. Uploaded to GitHub Release v0.1.0.
**Why:** A real `.aab` is the proof an Android app actually compiles. Without it, the platform claim is vapor.
**Commit:** `8954d1c`
**AI Role:** AI fixed API, generated icons, built AAB. Human directed.

### 2026-03-29 — iOS App + PWA + Multi-Arch Build Script

**What:** Three platforms in one commit:
1. iOS static library (`ios/src/lib.rs`) — C ABI bridge for Swift, `call_shield_classify`/`call_shield_free`. Swift AppDelegate with `@_silgen_name` bridge, CallKit integration. Builds for `aarch64-apple-ios` (5.3 MB .a file).
2. PWA (`web/`) — offline-first Progressive Web App with service worker, same 35-pattern classifier in JS, manifest.json, installable from any browser.
3. Multi-arch build script (`scripts/build-all-targets.sh`) — 12 targets: macOS ARM/Intel, Linux x86/ARM64/ARM32, RISC-V, Windows, FreeBSD, POWER, Android, iOS, WASM.

**Why:** A privacy-first call screener has to cover every place a phone runs. One platform isn't enough.
**Commit:** `64e31a5`
**AI Role:** AI implemented all three platforms. Human directed architecture.

### 2026-03-29 — Android App (CallScreeningService, API 35)

**What:** Full Android app: `ShieldScreeningService` (registered `CallScreeningService`), `IntentClassifier` (35-pattern Java port), `ShieldActivity` (enable shield, test classifier, CRT green theme). No INTERNET permission. `network_security_config.xml` denies cleartext. Package: `org.cochranblock.callshield`, minSdk 29, targetSdk 35.
**Why:** Android `CallScreeningService` is the only API that lets a third-party app intercept calls before they ring. It's the ground truth for the "between rings" claim.
**Commit:** `459c6ca`
**AI Role:** AI implemented Android app. Human directed architecture and package naming.

### 2026-03-28 — Crates.io Prep

**What:** Added crates.io metadata to Cargo.toml: description, repository URL, keywords, categories. `cargo publish --dry-run` passed clean.
**Why:** Stage the package so a single `cargo publish` ships it the moment we declare stable.
**Commit:** `ad38176`

### 2026-03-28 — Embedded Govdocs + Interactive Screening + SPDX SBOM

**What:** The binary now serves its own compliance docs at runtime. Added:
1. `govdocs` subcommand — prints any of 11 embedded compliance docs to stdout
2. `--sbom` flag — outputs machine-readable SPDX 2.3 format SBOM parsed live from embedded Cargo.toml
3. `screen` command — interactive call screening session with multi-turn conversation, real-time classification, session stats, and automatic routing decisions (block spam, pass legit, prompt unknowns, voicemail after 3 inconclusive turns)
4. Enhanced classifier — now reports matched patterns in output
5. Compression map updated: f0-f10, t0-t1, s0-s1 (11 functions, 2 types, 2 fields)

Binary: 368,896 bytes (360 KB), zero dependencies.
**Why:** Dogfood the compliance story — the binary should be its own audit artifact.
**Commit:** `9047b17`
**AI Role:** AI implemented all features. Human directed architecture and dogfooding requirement.

### 2026-03-27 — TOI/POA Update

**What:** Updated Timeline of Invention and Proof of Artifacts with all commits, binary sizes, QA results, P13 stats.
**Commit:** `151784d`

### 2026-03-27 — Federal Compliance Documentation

**What:** Created `govdocs/` with 11 federal compliance documents: SBOM (EO 14028), SSDF (NIST SP 800-218), supply chain integrity, security posture, accessibility (Section 508), privacy impact assessment, FIPS 140-2/3 status, FedRAMP applicability notes, CMMC Level 1-2 mapping, ITAR/EAR export classification, and federal use cases for 7 agencies (DoD, DHS, VA, DOJ, NSF, DOE, GSA).
**Why:** Federal procurement starts with the paperwork; if you can't show the SBOM, you can't bid.
**Commit:** `efd8e91`
**AI Role:** AI drafted all compliance documents. Human directed scope and verified claims against source.

### 2026-03-27 — User Story Analysis + Top 3 Fixes

**What:** Full end-to-end user story walkthrough. Scored scaffold 1.6/10. Implemented top 3 fixes:
1. CLI with `--help`, `--version`, error handling for bad input
2. Pattern-match intent classifier — 35 patterns across spam/legitimate/unknown categories
3. README rewrite with quick start, usage examples, and roadmap

Updated compression map to f0-f4, t0, s0-s1. Binary grew to 319,248 bytes (312 KB) with classifier.
**Why:** Score the gap between scaffold and shippable, then close the highest-impact gaps first.
**Commit:** `a8d679b`
**AI Role:** AI ran user story analysis, identified gaps, implemented all three fixes. Human directed the analysis.
**Proof:** [USER_STORY_ANALYSIS.md](USER_STORY_ANALYSIS.md)

### 2026-03-27 — P13 Tokenization + Binary Optimization

**What:** Applied Kova P13 compression mapping to all symbols. Created `docs/compression_map.md`. Added release profile for smallest binary: opt-level=z, LTO, single codegen unit, panic=abort, stripped. Binary: 285,936 bytes (279 KB) with zero deps.
**Why:** P13 token economy across CochranBlock + smallest-possible binary for embedded use.
**Commit:** `157adf7`
**AI Role:** AI applied tokenization and release profile per P13 protocol. Human directed.

### 2026-03-27 — Cargo.lock + QA Round 1 & 2

**What:** Committed Cargo.lock for reproducible binary builds. Ran two full QA rounds: clean compile, zero warnings, zero clippy errors, all paths verified.
**Why:** Reproducibility is the prerequisite for every other federal claim.
**Commit:** `2fac3f5`
**QA Round 1:** PASS — `cargo build --release` clean, `git diff` empty, binary runs.
**QA Round 2:** PASS — `cargo clean && cargo build --release` clean, `cargo clippy --release -- -D warnings` zero warnings, `git status` clean.

### 2026-03-27 — README with backlink

**What:** Added README with product description and cochranblock.org backlink.
**Commit:** `f00f0b2`

### 2026-03-26 — Call Shield Whitepaper + Scaffold

**What:** Published whitepaper on on-device call screening without cloud. Rust binary scaffold. Architecture: Whisper Tiny (39MB quantized) + intent classifier (<1MB) + call handling logic, all compiled into a single ~42MB binary. Zero audio leaves the device.
**Why:** Founded by the contradiction of building zero-cloud infrastructure while using Google Call Screen. The alternative doesn't exist — so build it.
**Commit:** `1d6bad5`
**AI Role:** AI drafted whitepaper and technical analysis. Human identified the product gap from personal experience, directed the architecture based on existing Candle/Whisper work in kova and pixel-forge, and validated all size and latency claims.
**Proof:** [WHITEPAPER.md](WHITEPAPER.md)

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
