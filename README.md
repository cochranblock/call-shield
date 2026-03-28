# Call Shield

On-device call screening without the cloud. Classify caller intent as spam, legitimate, or unknown — no audio ever leaves the device.

## Quick Start

```bash
cargo build --release
./target/release/call-shield --help
```

## Usage

```bash
# Classify caller speech
call-shield classify "We've been trying to reach you about your car's extended warranty"
# → verdict: SPAM, score: 0.95

call-shield classify "Hi, this is Dr. Smith's office confirming your appointment"
# → verdict: LEGITIMATE, score: 0.85

call-shield classify "Hello?"
# → verdict: UNKNOWN, score: 0.50
```

## What Works Today

- Text-based intent classification (pattern matching, 35 patterns)
- CLI with `--help`, `--version`, `classify` command
- Error handling for bad input
- 312 KB stripped binary, zero dependencies

## What's Next

- Whisper Tiny (39MB quantized) for on-device speech-to-text
- Audio capture and telephony integration
- Configurable whitelist and sensitivity
- Local-only call log

## Architecture

See [WHITEPAPER.md](WHITEPAPER.md) for full design.

## Federal Compliance

See [govdocs/](govdocs/) — SBOM, SSDF, supply chain, security, privacy, FIPS, FedRAMP, CMMC, ITAR/EAR, accessibility, and federal use cases.

**Stack:** Rust, zero dependencies, pattern-match classifier (ML classifier planned)
**Binary:** 312 KB (will grow to ~42MB with Whisper)
**License:** Unlicense

---

Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. [See all products](https://cochranblock.org/products)
