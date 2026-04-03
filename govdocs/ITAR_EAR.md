<!-- Unlicense — cochranblock.org -->

# Export Control Classification — ITAR/EAR

*Does Call Shield contain controlled technology?*

## ITAR (International Traffic in Arms Regulations)

**Not ITAR-controlled.** Call Shield is not on the United States Munitions List (USML). It is:
- Not a defense article
- Not designed for military-specific use
- Not funded under a classified contract
- Not derived from classified technology

The call screening function is commercially available from multiple providers (Google, Apple, Truecaller). This is a commercial implementation of a widely available capability.

## EAR (Export Administration Regulations)

### Classification

| Factor | Assessment |
|--------|-----------|
| ECCN | EAR99 (no specific ECCN) |
| Encryption | None in v0.1.0 |
| AI/ML | Pattern matching only (not ML) — not controlled under ECCN 4A005 or 4D004 |
| Speech recognition | Planned (Whisper) — open-source model, publicly available |

### EAR Category 5 Part 2 — Crypto

**Current (v0.1.0):** No encryption. Not subject to Category 5 Part 2 controls.

**Planned:** If AES-256-GCM is added for local storage:
- AES-256 exceeds the 56-bit threshold for EAR control
- However, encryption is for local data protection only (not communications)
- Likely qualifies for License Exception ENC § 740.17(b)(1) — mass market encryption
- Open-source exception: publicly available source code on GitHub may qualify for EAR § 742.15(b) exclusion
- TSR (Technical Security Review) filing with BIS may be required if crypto is added

### Whisper Model (Planned)

- Whisper is an open-source model published by OpenAI under MIT license
- Publicly available on GitHub and Hugging Face
- Not considered controlled technology under EAR
- Speech recognition is not on the Commerce Control List unless part of a military system

## Recommendations

1. **Current state:** EAR99, no restrictions, freely exportable
2. **If crypto is added:** File a classification request with BIS or self-classify under License Exception ENC
3. **If Whisper is embedded:** No additional controls — publicly available model
4. **If sold to DoD:** Still not ITAR unless integrated into a defense system by the contracting agency

## Sanctions Screening

Standard OFAC screening applies if distributing to end users. For open-source projects published on GitHub, GitHub's own sanctions compliance covers access restrictions.

---

*Last updated: 2026-03-27*

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture.*
