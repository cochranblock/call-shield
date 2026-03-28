<!-- Unlicense — cochranblock.org -->

# FIPS 140-2/3 Compliance Status

*Cryptographic module validation status for Call Shield.*

## Current State (v0.1.0)

**Call Shield does not use cryptography.**

There are no cryptographic primitives, no encryption, no hashing, no key derivation, and no random number generation in the current binary. FIPS 140-2/3 validation is not applicable.

## Verification

```bash
# Confirm no crypto crates
cargo tree --depth 1
# Output: call-shield v0.1.0 (no deps)

# Confirm no crypto in source
grep -ri "aes\|sha256\|hmac\|hkdf\|argon\|encrypt\|decrypt\|hash\|cipher" src/
# Expected: no output
```

## Planned Cryptography

If a local encrypted call log is added:

| Primitive | Planned Implementation | FIPS Status |
|-----------|----------------------|-------------|
| AES-256-GCM | ring or RustCrypto | ring uses BoringSSL (FIPS-validated module). RustCrypto is not FIPS-validated. |
| Argon2id | argon2 crate | Not FIPS-validated. NIST recommends PBKDF2 for FIPS environments. |
| HKDF-SHA256 | hkdf crate | SHA-256 is FIPS-approved. HKDF itself is in SP 800-56C. |

## Path to FIPS Compliance

If FIPS validation is required:

1. **Replace Argon2id with PBKDF2-HMAC-SHA256** — PBKDF2 is FIPS-approved (SP 800-132)
2. **Use ring crate** — ring's crypto is backed by BoringCrypto, which has FIPS 140-2 validation (Certificate #4407)
3. **Or use aws-lc-rs** — AWS-LC has FIPS 140-3 validation
4. **No custom crypto** — all primitives from validated modules

## Export Classification

See [ITAR_EAR.md](ITAR_EAR.md) for crypto export considerations.

---

*Last updated: 2026-03-27*
