<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

---

## Entries

### 2026-03-26 — Call Shield Whitepaper + Scaffold

**What:** Published whitepaper on on-device call screening without cloud. Rust binary scaffold. Architecture: Whisper Tiny (39MB quantized) + intent classifier (<1MB) + call handling logic, all compiled into a single ~42MB binary. Zero audio leaves the device.
**Why:** Founded by the contradiction of building zero-cloud infrastructure while using Google Call Screen. The alternative doesn't exist — so build it.
**Commit:** See `git log --oneline`
**AI Role:** AI drafted whitepaper and technical analysis. Human identified the product gap from personal experience, directed the architecture based on existing Candle/Whisper work in kova and pixel-forge, and validated all size and latency claims.
**Proof:** [WHITEPAPER.md](WHITEPAPER.md)

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
