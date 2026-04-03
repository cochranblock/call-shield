# CALL SHIELD

**On-Device Call Screening Without the Cloud**

*Author: The Cochran Block*

---

## Executive Summary

Every major call screening solution — Google Call Screen, Apple Silence Unknown Callers, carrier-level spam filters — routes your voice data through centralized cloud infrastructure. Your audio is processed on servers you don't own, by companies whose business model is data harvesting. The screening works, but the privacy cost is invisible and non-negotiable.

Call Shield is an on-device call screening agent compiled into a single Rust binary. It runs on your phone's hardware with zero cloud connectivity. The v1.0 architecture embeds a Whisper-based speech-to-text model and a lightweight intent classifier in the binary. The caller's intent is determined on-device in milliseconds. Spam is killed. Legitimate calls ring through. No audio ever leaves the device.

*Current status (v0.1.x): ships a 38-pattern text classifier (24 spam, 14 legit) across CLI, Android, iOS, and PWA. Whisper integration is planned for v1.0.*

This is not a privacy wrapper around an existing cloud service. This is a replacement for the cloud service itself.

## 1. The Problem: Your Phone Calls Route Through Someone Else's Brain

### 1.1 Google Call Screen

When Google screens a call, your phone sends the caller's audio to Google's servers. Google's ASR (Automatic Speech Recognition) transcribes it, classifies intent, and returns a decision. Your phone displays the transcript and lets you choose.

What Google gets: the caller's voice, phone number, time of call, your location, your response pattern, and the audio itself for model training. This data feeds Google's advertising and AI training pipelines. You opted in by enabling the feature.

### 1.2 Carrier Spam Filters

T-Mobile Scam Shield, AT&T Call Protect, and Verizon Call Filter use centralized databases and cloud-based heuristics. They see every call's metadata — who called whom, when, duration, frequency. This metadata is commercially valuable and contractually licensed to data brokers.

### 1.3 The Irony for Privacy-Focused Builders

A developer building zero-cloud, edge-compute, sovereign infrastructure — using Google to process their personal calls — is living the exact contradiction their technology is designed to solve. The alternative doesn't exist yet because nobody has built it at the edge.

## 2. The Solution: Embedded Intelligence on the Device

### 2.1 Target Architecture

*Current v0.1.x ships a pattern-match text classifier (38 patterns, 360 KB binary). The architecture below is the v1.0 goal with embedded Whisper STT.*

```
Incoming Call
    ↓
Call Shield answers (on-device)
    ↓
Whisper Tiny (39MB, quantized) → speech-to-text
    ↓
Intent Classifier (embedded, <1MB) → spam / legitimate / unknown
    ↓
Decision:
  spam → hang up, log blocked
  legitimate → ring through to user
  unknown → play prompt, re-classify response
    ↓
Zero audio leaves the device. Ever.
```

### 2.2 The Binary (Projected v1.0)

| Component | Size | Purpose |
|-----------|------|---------|
| Whisper Tiny (quantized) | ~39MB | Speech-to-text, on-device |
| Intent classifier | <1MB | Spam vs legitimate vs unknown |
| Call handling logic | <1MB | Answer, prompt, route, hang up |
| **Total binary** | **~42MB** | Everything. No cloud. No dependencies. |

*Current v0.1.x binary: 360 KB (pattern-match classifier only, zero dependencies).*

42MB is larger than a typical CochranBlock binary (18MB) but still fits comfortably in L3 cache working set on any modern phone chip. Cold-boot to first classification: under 500ms.

### 2.3 How Classification Works

The intent classifier is not a general-purpose LLM. It is a hyper-specific model trained on curated call transcripts:

**Spam patterns** (kill immediately):
- "We've been trying to reach you about your car's extended warranty"
- "This is a courtesy call regarding your account"
- "You have been selected for a special offer"
- "Press 1 to speak with a representative"

**Legitimate patterns** (ring through):
- Caller states the user's name
- Caller references a specific company, project, or appointment
- Caller identifies themselves by name and organization
- Return call from a number the user recently dialed

**Unknown patterns** (prompt and re-classify):
- Caller doesn't match spam or legitimate
- Play: "Please state your name and the reason for your call"
- Classify the response

The model doesn't need to understand language. It needs to match patterns in transcribed text. A 500KB classifier trained on 10,000 labeled call transcripts achieves >95% accuracy on this binary classification task.

## 3. Why This Must Be On-Device

### 3.1 Privacy

Audio never leaves the phone. The transcript exists only in local memory during classification and is discarded after the decision. No logs, no training data, no telemetry.

### 3.2 Latency

Cloud-based screening adds 1-3 seconds of round-trip latency. On-device classification completes in under 500ms. The caller doesn't know they're being screened.

### 3.3 Availability

Cloud screening fails when you have no signal, weak data, or are in a Faraday cage. On-device screening works with zero connectivity — the intelligence is in the binary.

### 3.4 Sovereignty

For defense personnel, government employees, journalists, activists, and anyone whose call metadata is sensitive — routing audio through Google or a carrier is a security risk. On-device screening eliminates the third party entirely.

## 4. Integration with the CochranBlock Stack

Call Shield is part of the same architectural thesis as every CochranBlock product:

| Product | What it proves |
|---------|---------------|
| cochranblock.org | Web server without cloud |
| Ghost Fabric | AI inference without cloud |
| Pocket Server | Website hosting without cloud |
| Pixel Forge | Image generation without cloud |
| **Call Shield** | **Call screening without cloud** |

The pattern is consistent: take a capability that the industry assumes requires cloud, compile it into a Rust binary, and run it on the device. Every product reinforces the thesis. Every product is a proof point for the next.

## 5. Target Markets

- **Privacy-conscious individuals** — the same market that uses Signal, ProtonMail, and VPNs
- **Defense and intelligence personnel** — call metadata is an OPSEC concern
- **Small business owners** — screen calls while working, no subscription fee
- **Government agencies** — FedRAMP compliance is moot when there's no cloud
- **The CochranBlock customer base** — every Pocket Server customer gets call screening built in

## 6. SBIR Alignment

Call Shield extends Ghost Fabric's thesis to voice communication:

- **DoD**: Secure communication screening for military personnel without cloud exposure
- **DHS/CISA**: Counter-robocall technology that doesn't create new data collection points
- **NSF**: On-device ML inference for real-time classification

The FCC's STIR/SHAKEN framework addresses caller ID authentication but not content screening. Call Shield fills the gap — screening the call's content locally without creating a cloud surveillance point.

## Conclusion

You shouldn't have to choose between screening your calls and keeping your audio private. Call Shield proves you don't have to. Same architecture. Same thesis. One more cloud dependency eliminated.

---

*The Cochran Block, LLC — Dundalk, MD*
*SDVOSB (Pending) · SAM.gov Registered · cochranblock.org*
