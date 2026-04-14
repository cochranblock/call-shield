# Assumed Breach Threat Model

> **Operating assumption: every component below is already compromised. Design for damage containment and loud detection, not for prevention.**

This document is the canonical threat model for every project in the `cochranblock/*` portfolio. Each project adapts the Threat Surface section for its own context but shares the same first principles, mitigations, and verification protocol.

---

## First Principles

1. **Every record that matters has an external witness.** Hashes published to public git (or equivalent neutral timestamp authority) so tampering requires simultaneously corrupting your system AND the public chain.
2. **No single point of compromise.** Signing keys in hardware (YubiKey / TPM / Secure Enclave). Never in software. Never in env vars. Never in config files.
3. **Default air-gap.** No network dependency for correctness. Network is for backup + publishing hashes, both signed, both verifiable post-hoc.
4. **Append-only everything.** No delete path in any storage layer. Corrections are reversing entries referencing the original. Standard accounting discipline, enforced in code.
5. **Cryptographic audit chain.** Every day's state derives from the previous day's hash. Tampering with any day invalidates every subsequent day.
6. **Disclosure of methodology is a security feature.** If an auditor can independently verify the algorithm, they can independently verify the outputs. No "trust us" layers.
7. **Separation of duties enforced in software.** Entry, approval, and audit live in different trust zones. Compromise of one does not compromise the others.
8. **Redundancy across trust zones.** Local + different-cloud + different-format + offline. Attacker must compromise all to hide damage.
9. **Test breach scenarios regularly.** Triple Sims applied to tamper detection. If the chain does not detect a simulated tamper, the chain is broken.

---

## Threat Surface (project-specific — adapt below)

### What records this project emits

call-shield emits three categories of records, all on-device under `~/.call-shield/` (HOME-relative) or embedded in the shipped binary:

1. **Screening log** — `~/.call-shield/call_log.jsonl`, JSON Lines appended by `f13=log_entry`. One record per screened call: `{ts, verdict, score, matched, action, turns}`. No phone numbers. No audio. No transcripts.
2. **Contact whitelist** — `~/.call-shield/whitelist.txt`, newline-delimited numbers the user has opted to pass through. Written by `f11=whitelist`.
3. **Embedded compliance evidence** — 11 govdocs and a live SPDX 2.3 SBOM baked into the binary via `include_str!` against `govdocs/*.md` and `Cargo.toml`. Emitted on demand via `call-shield govdocs` and `call-shield --sbom`.

No phone numbers, audio, transcripts, call metadata, telemetry, crash reports, or analytics are collected, stored, or transmitted — both as policy (`govdocs/PRIVACY.md`) and as architectural constraint (no networking crate is linked in any binary).

### What's unique about this threat surface

- **Four-port classifier drift.** The 38-pattern classifier (24 spam + 14 legit) is ported to Rust (CLI), Java (Android `CallScreeningService`), Rust-with-C-ABI + Swift wrapper (iOS), and JavaScript (PWA). The ports share no code. An attacker — or a careless merge — that modifies only one port (e.g., weakens `"verify your account"` in the PWA's JS) leaves Rust tests green while Android or PWA silently fail open. Triple Sims must cover all four ports. JUnit mirror suite covers Java (`IntentClassifierTest.java`, 60+ tests, commit `72de8c0`); the JS port has no mirror suite — this is a known gap.
- **Pattern-table tampering beats binary replacement.** The classifier *is* the product. Flipping one pattern's weight (demoting `"extended warranty"` from spam to unknown, promoting `"from your bank"` from spam back to legit) is more valuable to an attacker than replacing the whole binary and easier to smuggle through review — the patterns are dense string literals in `src/main.rs` where diff noise hides them. Vishing Vector Regression tests (`3bb7db2`) are the countermeasure.
- **Delete path in the local log.** `call-shield log clear` calls `std::fs::remove_file` on `call_log.jsonl` (src/main.rs:621). This directly violates First Principle #4 (append-only everything). A compromised binary or local attacker can erase call history without audit trace. No hash chain exists over the log today.
- **Android permission drift.** Backlog #2 dropped `READ_CALL_LOG` (commit `72de8c0`); earlier work dropped `RECORD_AUDIO` and never granted `INTERNET`. Re-introduction of any of these in a future PR silently breaks the "zero data leaves device" guarantee. The `grep` checks in `PROOF_OF_ARTIFACTS.md` §Verification #7–#8 are the countermeasure and must run on every build.
- **iOS FFI unsafe surface.** Two justified `unsafe` blocks at the C-ABI boundary (`call_shield_classify`, `call_shield_free`). Memory corruption here escapes into the Swift host and any CallKit-integrated app using the static lib.
- **Govdoc embed tampering.** 11 compliance documents are baked into the binary at compile time. Pre-build tampering with `govdocs/*.md` ships a binary that emits false compliance claims through `call-shield govdocs` / `--sbom` — federal procurement would consume forged SBOMs. Reproducible build + external SBOM witness is the countermeasure.
- **Single build-time dep (Android).** `junit:junit:4.13.2` as `testImplementation`. Cannot reach the shipped AAB, but a compromised JUnit could poison test output to mask a classifier regression that gets blamed on pattern tuning. Rust side has zero runtime and zero build-time deps.
- **Physical device seizure.** Local log and whitelist live unencrypted under `$HOME/.call-shield/`. Device seizure = full access to screening history for that user. Full-disk encryption is the host OS's responsibility, not this binary's.

### What's N/A

- **Network MITM.** No networking crate is linked in any binary. Verified by `cargo tree --depth 1` (0 deps) and `grep -ri "tcp\|udp\|http\|socket\|connect\|reqwest\|hyper" src/` (no matches). Mitigation by construction, not by policy — there is no code path to MITM.
- **Cloud / remote account compromise.** No accounts, no cloud, no login, no server-side state. Nothing to compromise remotely.
- **Shared audit log tampering.** No shared or central audit log exists. Each device's log is independent and local.
- **Backup tampering propagates to recovery.** No automated backup exists. Records are intentionally local-only; loss of device equals loss of local records *by design*. This is a privacy feature, not a resilience gap — call history is not a record-of-consequence for anyone but the device owner.
- **Public-chain deployment for user-emitted records.** The records of consequence at the user level (screening log, whitelist) are intentionally never published; publishing them would violate the privacy guarantee that motivates the project. The public-chain primitive below applies only to release artifacts (binary hashes, SBOM), not to per-device call history. See `PROOF_OF_ARTIFACTS.md` for the artifact-hash chain; a per-device chain is not planned.

---

## Mitigations

| Assume | Mitigation | Verification |
|--------|-----------|--------------|
| Binary compromised | Hardware-key signatures for every output of consequence | Anyone can verify the public key matches expected fingerprint |
| Storage compromised | Append-only sled trees. Delete is not a function, not a policy. | Hash chain breaks on any rewrite. External witness detects. |
| Network MITM | Air-gap capable. Network used only for signed backups + hash publishing. | NTP + GitHub timestamp + hardware counter cross-checked. |
| Signing key stolen | Daily hash committed to public git. Stolen key cannot retroactively change committed days. | Any day older than the public commit is immutable in evidence. |
| Audit log tampered | Separate sled tree, write-only from main app. Auditor tool reads both + cross-checks. | Compromise of main app leaves audit log intact. |
| Backup tampered | 3 different targets with 3 different credentials (local USB + off-site cloud + paper). | Attacker needs all three to hide damage. |
| Insider / self-tampering | No admin role. No delete. Reversing entries only. | Legal record immune to author second-thoughts. |
| Clock manipulation | Multiple time sources: local clock, NTP, git commit timestamp, hardware-key counter. | Divergence flags exception requiring supervisor approval. |
| Supply chain (deps) | `cargo audit` in CI. Pinned SBOM. Reproducible builds where possible. | Anyone can reproduce the binary from source + lockfile. |
| Physical device seizure | Full-disk encryption. Hardware key physically separate from device. | Stolen laptop without key is useless for forgery. |

---

## Public-Chain Deployment

This project publishes tamper-evident hashes to a public companion repo: `cochranblock/<project>-chain` (where `<project>` is the project name).

- **Daily cycle:** at 23:59 local, compute BLAKE3 of all records-of-consequence from the day. Sign with hardware key. Commit to chain repo. Push.
- **GitHub timestamp** on the commit = neutral third-party witness. Anyone can cold-verify records were not rewritten after commit time.
- **Verification:** `<project> verify` reads the chain and re-derives hashes. Any divergence = tampering detected.

This pattern is a private Certificate Transparency log for project state. Same primitive Google uses for TLS certs, applied to whatever the project tracks.

---

## Triple Sims for Tamper Detection

Standard Triple Sims gate (run 3x identically) extended with a tamper-scenario sim:

1. Normal run → produce canonical output
2. Simulated tampering (flip one bit in storage) → `verify` must flag it
3. Simulated clock rewind → `verify` must flag it

If any sim fails to detect, the chain is broken. Fix before merge.

---

## Scope of this Document

- Covers: any artifact this project emits that has legal, financial, or audit consequence.
- Does NOT cover: source code itself (public under Unlicense, not sensitive), build outputs (reproducible), marketing content (public by design).
- If your project emits no records of consequence, the relevant sections are zero-length and the public-chain deployment is skipped. Document that explicitly.

---

## Relation to Other Docs

- **TIMELINE_OF_INVENTION.md** — establishes priority dates for contributions. Feeds into the chain's initial state.
- **PROOF_OF_ARTIFACTS.md** — cryptographic signatures on release artifacts. Adjacent pattern, same first principles.
- **DCAA_COMPLIANCE.md** (where applicable) — how this threat model satisfies FAR/DFARS audit requirements.

---

## Status

- [ ] Threat Surface section adapted for this project
- [ ] Hardware-key signing integrated or N/A documented
- [ ] Public-chain repo created and connected or N/A documented
- [ ] Triple Sims tamper-detection test present or N/A documented
- [ ] External verification procedure documented

---

*Unlicensed. Public domain. Fork, strip attribution, adapt, ship.*

*Canonical source: cochranblock.org/threat-model — last revision 2026-04-14*
