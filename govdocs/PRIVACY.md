<!-- Unlicense — cochranblock.org -->

# Privacy Impact Assessment

*What data Call Shield collects, stores, and transmits.*

## Data Collection

| Data Type | Collected? | Stored? | Transmitted? |
|-----------|-----------|---------|-------------|
| Caller audio | No (planned: in-memory only) | No | No — ever |
| Call transcripts | No (planned: in-memory, discarded after classification) | No | No |
| Phone numbers | No | No | No |
| Call metadata (time, duration) | No | No | No |
| User preferences | No | No | No |
| Telemetry | No | No | No |
| Crash reports | No | No | No |
| Analytics | No | No | No |

## Current State (v0.1.0)

Call Shield accepts text on the command line and classifies it. The input text is:
1. Read from argv
2. Compared against in-memory patterns
3. Result printed to stdout
4. Process exits — no data persists

**No data is written to disk. No data is sent over any network. No data survives process exit.**

## Planned Behavior (with Whisper)

When audio processing is added:
1. Audio captured from microphone/call interface
2. Processed by Whisper Tiny in-memory — speech to text
3. Text classified by intent classifier in-memory
4. Decision made (spam/legit/unknown)
5. **Audio buffer zeroed and deallocated**
6. **Transcript discarded — not logged, not stored**

If a local call log is added in the future, it will store only:
- Timestamp
- Verdict (spam/legit/unknown)
- No audio, no transcript, no phone number unless user opts in

## PII Assessment

| PII Type | Present? | Notes |
|----------|----------|-------|
| Names | No | CLI input may contain names but they are not stored |
| Phone numbers | No | Not captured or stored |
| Voice biometrics | No | No audio storage |
| Location | No | No GPS/network access |
| Device identifiers | No | No device fingerprinting |

## Regulatory Applicability

| Regulation | Applicable? | Notes |
|-----------|------------|-------|
| GDPR | Minimal | No personal data collected or stored. No data processor role. |
| CCPA | Minimal | No personal information sold or shared. |
| HIPAA | No | Not a healthcare application. |
| COPPA | No | Not directed at children. |
| FERPA | No | Not an education application. |

## The Core Privacy Guarantee

**Zero audio leaves the device. Ever.**

This is not a policy decision — it is an architectural constraint. There is no network code in the binary. There is no capability to transmit data even if the code were modified to try, because no networking crate is included.

Verification:
```bash
# Confirm no network dependencies
cargo tree --depth 1
# Output: call-shield v0.1.0 (no deps)

# Confirm no network use in source
grep -ri "tcp\|udp\|http\|socket\|connect\|reqwest\|hyper" src/
# Expected: no output
```

---

*Last updated: 2026-03-27*
