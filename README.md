# Call Shield

On-device call screening without the cloud. Classify caller intent as spam, legitimate, or unknown — no audio ever leaves the device.

## Quick Start

```bash
cargo build --release
./target/release/call-shield --help
```

## Usage

```bash
# Interactive call screening session
call-shield screen

# Classify caller speech
call-shield classify "We've been trying to reach you about your car's extended warranty"
# → verdict: SPAM, score: 0.95

# Embedded compliance docs
call-shield govdocs sbom

# Machine-readable SBOM for federal scanners
call-shield --sbom > sbom.spdx
```

## Platforms

| Platform | Target | Status |
|----------|--------|--------|
| macOS ARM | `aarch64-apple-darwin` | Building |
| macOS Intel | `x86_64-apple-darwin` | Building |
| Linux x86_64 | `x86_64-unknown-linux-gnu` | Building |
| Linux ARM 64 | `aarch64-unknown-linux-gnu` | Cross (RPi 4/5, Graviton) |
| Linux ARM 32 | `armv7-unknown-linux-gnueabihf` | Cross (older RPi, IoT) |
| RISC-V 64 | `riscv64gc-unknown-linux-gnu` | Cross |
| Windows | `x86_64-pc-windows-gnu` | Cross (MinGW) |
| FreeBSD | `x86_64-unknown-freebsd` | Cross |
| IBM POWER | `powerpc64le-unknown-linux-gnu` | Cross (gov mainframes) |
| Android | `aarch64-linux-android` | Native app (API 35) |
| iOS | `aarch64-apple-ios` | Static lib + Swift |
| Web (PWA) | Browser | Offline-first, installable |

Build all: `./scripts/build-all-targets.sh`

## What Works Today

- Interactive call screening session (`screen`)
- Text-based intent classification (35 patterns, <1ms)
- Embedded federal compliance docs (`govdocs`)
- Machine-readable SPDX SBOM (`--sbom`)
- Android app with `CallScreeningService`
- iOS static library with Swift bridge
- PWA with service worker for offline use
- 360 KB stripped binary, zero dependencies

## What's Next

- Whisper Tiny (39MB quantized) for on-device speech-to-text
- Audio capture and telephony integration
- Configurable whitelist and sensitivity
- Local-only call log

## Architecture

See [WHITEPAPER.md](WHITEPAPER.md) for full design.

## Federal Compliance

See [govdocs/](govdocs/) — SBOM, SSDF, supply chain, security, privacy, FIPS, FedRAMP, CMMC, ITAR/EAR, accessibility, and federal use cases. Also embedded in the binary: `call-shield govdocs sbom`

**Stack:** Rust, zero dependencies, pattern-match classifier (ML classifier planned)
**Binary:** 360 KB (will grow to ~42MB with Whisper)
**License:** Unlicense

---

Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. [See all products](https://cochranblock.org/products)
