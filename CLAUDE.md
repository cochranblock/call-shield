<!-- Unlicense — cochranblock.org -->
<!-- Contributors: KOVA, Claude Opus 4.6 -->

# call-shield

- On-device call screening without the cloud. Pattern-match classifier (24 spam + 14 legit), zero dependencies.
- Build: cargo build --release
- Test: cargo test (17 tests)
- Platforms: CLI (Rust, 360 KB), Android (AAB, CallScreeningService), iOS (static lib + Swift), PWA (offline-first)
- P13 tokenized: f0-f10, t0-t1, s0-s1. See docs/compression_map.md.
- P23 (Triple Lens) applied to v0.2.0: guest analysis (pessimist), vishing audit (paranoia), synthesis drove 6-phase fix plan. See TIMELINE_OF_INVENTION.md and PROOF_OF_ARTIFACTS.md.
- Whisper STT is target architecture (v1.0), not current. Current binary is text-input pattern matcher.
- Federal govdocs embedded in binary: `call-shield govdocs sbom`
