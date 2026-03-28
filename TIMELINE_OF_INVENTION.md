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

### 2026-03-28 — Dogfooding: Embedded Govdocs + Interactive Screening + SPDX SBOM

**What:** The binary now serves its own compliance docs at runtime. Added:
1. `govdocs` subcommand — prints any of 11 embedded compliance docs to stdout
2. `--sbom` flag — outputs machine-readable SPDX 2.3 format SBOM parsed live from embedded Cargo.toml
3. `screen` command — interactive call screening session with multi-turn conversation, real-time classification, session stats, and automatic routing decisions (block spam, pass legit, prompt unknowns, voicemail after 3 inconclusive turns)
4. Enhanced classifier — now reports matched patterns in output
5. Compression map updated: f0-f10, t0-t1, s0-s1 (11 functions, 2 types, 2 fields)

Binary: 368,896 bytes (360 KB), zero dependencies. Every compliance doc a contracting officer needs ships in the executable.
**Commit:** See `git log --oneline`
**AI Role:** AI implemented all features. Human directed architecture and dogfooding requirement.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
