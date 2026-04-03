<!-- Unlicense — cochranblock.org -->

> **HISTORICAL SNAPSHOT (2026-03-27)** — This analysis scored the pre-implementation scaffold at 1.6/10. The issues identified here drove the v0.1.0 and v0.2.0 implementations. For current status, see [README.md](README.md) and [PROOF_OF_ARTIFACTS.md](PROOF_OF_ARTIFACTS.md).

# User Story Analysis — Call Shield

*Full end-to-end walkthrough as a real user. 2026-03-27.*

---

## 1. Discovery

**First impression from README:** Clear in 5 seconds — on-device call screening, no cloud, Rust binary. The one-liner is strong: "Zero audio leaves the device." The whitepaper link gives depth for anyone who wants it.

**What's missing:** No installation instructions in the README. No usage examples. No screenshot or demo. A user landing on this repo has zero idea how to run it or what happens when they do.

**Score: 6/10** — thesis is clear, but there's no path from "I'm interested" to "I'm using it."

## 2. Installation

```
cargo build --release
```

Works. Compiles in under 2 seconds. Binary at `target/release/call-shield`, 279 KB stripped.

But then what? There's no `cargo install` instruction. No prebuilt binaries. No mention of supported platforms. A user who isn't a Rust developer is stuck.

**Score: 3/10** — builds for devs, invisible to everyone else.

## 3. First Use (Happy Path)

```
$ ./call-shield
call-shield
```

That's it. Prints the project name and exits. No help text. No subcommands. No indication of what to do next.

A user who just read the whitepaper about Whisper-based call screening finds... a binary that prints its own name. The gap between promise and reality is total.

**Steps recorded:**
1. Clone repo
2. `cargo build --release`
3. `./call-shield`
4. See "call-shield" printed
5. Confusion — what now?
6. Try `--help` — same output
7. Try `--version` — same output
8. Read whitepaper to see if I missed setup steps
9. Realize nothing is implemented

**Score: 1/10** — the happy path doesn't exist.

## 4. Second Use Case

There is no second use case. There is no first use case. The binary has one code path: print name, exit.

**Score: 0/10**

## 5. Edge Cases

| Input | Result | Expected |
|-------|--------|----------|
| `--help` | prints "call-shield" | Help text with usage |
| `--version` | prints "call-shield" | Version number (0.1.0) |
| `screen` | prints "call-shield" | Subcommand or error |
| `--config foo.toml` | prints "call-shield" | Config load or error |
| `-v` | prints "call-shield" | Verbose mode or error |

Every input produces the same output. No argument parsing at all. The binary doesn't crash (good), but it also doesn't respond to anything (bad). Zero error handling because there's zero functionality.

**Score: 1/10** — doesn't crash, but that's the floor.

## 6. Feature Gap Analysis

What a user expects after reading the whitepaper vs what exists:

| Expected Feature | Status |
|-----------------|--------|
| Answer incoming calls | Not implemented |
| Whisper speech-to-text | Not implemented |
| Intent classification (spam/legit/unknown) | Not implemented |
| Caller prompt ("state your name...") | Not implemented |
| Call routing (ring through / hang up) | Not implemented |
| Local call log | Not implemented |
| Configuration (whitelist, sensitivity) | Not implemented |
| CLI with `--help` | Not implemented |
| `--version` flag | Not implemented |
| Android/iOS integration | Not implemented |
| Audio capture | Not implemented |

**Everything described in the whitepaper is missing.** The entire product is a thesis document with an empty scaffold.

## 7. Documentation Gaps

Questions a user would ask that the docs don't answer:

1. How do I install this on my phone?
2. What platforms are supported?
3. How does it integrate with the phone's call system?
4. Can I whitelist contacts?
5. How do I train the intent classifier on my own data?
6. What's the false positive rate?
7. Does it work with VoIP?
8. How do I configure the screening prompt?
9. Where are blocked call logs stored?
10. Is there a GUI or is it CLI-only?

The whitepaper answers the "why" well. It answers zero "how" questions because there's nothing to explain yet.

## 8. Competitor Check

| Product | On-device | Open source | Free | Privacy |
|---------|-----------|-------------|------|---------|
| Google Call Screen | No (cloud) | No | Yes | Low — Google gets audio |
| Apple Silence Unknown | Partial | No | Yes | High — but binary filter only |
| Truecaller | No (cloud) | No | Freemium | Low — crowdsourced data |
| Hiya | No (cloud) | No | Freemium | Low — carrier partnerships |
| RoboKiller | No (cloud) | No | Paid | Medium |
| **Call Shield** | Yes (planned) | Yes | Yes | High (planned) |

**Honest assessment:** Call Shield's thesis is strong and differentiated — no competitor does on-device ML classification of call content. But every competitor has a shipping product. Call Shield has a whitepaper. The architectural gap between "compiled Whisper in a Rust binary" and "answers your phone calls" is enormous — it requires OS-level telephony integration that doesn't exist in any cross-platform Rust crate today.

## 9. Verdict

| Category | Score (1-10) | Notes |
|----------|-------------|-------|
| Usability | 1 | No features to use |
| Completeness | 1 | Scaffold only |
| Error Handling | 1 | No inputs processed |
| Documentation | 5 | Whitepaper is good, usage docs absent |
| Would You Pay For This? | 0 | Nothing to pay for yet |

**Overall: 1.6/10** — strong thesis, zero product.

## 10. Top 3 Fixes to Make This Closer to Shippable

### Fix 1: CLI with --help and --version
A binary that ignores all input is not a product. Add argument parsing so a user gets useful output.

### Fix 2: Demonstrate the classification pipeline
Even without real audio, show the intent classifier working on text input. Accept a string, classify it as spam/legit/unknown, print the result. Proves the core thesis without telephony.

### Fix 3: Clear README with usage instructions
Tell the user what they can do today, what's coming, and how to contribute.

---

*Analysis by Kova augment engine. Brutal and honest as requested.*

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture.*
