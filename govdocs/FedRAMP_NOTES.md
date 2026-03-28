<!-- Unlicense — cochranblock.org -->

# FedRAMP Applicability Notes

*Authorization boundary and deployment model for Call Shield.*

## Deployment Model

**Call Shield is not a cloud service.** It is an on-device, single-binary application with zero network connectivity. FedRAMP authorization is not applicable in its current form.

| Question | Answer |
|----------|--------|
| Is this SaaS? | No |
| Is this PaaS? | No |
| Is this IaaS? | No |
| Is this on-premises? | Yes — on-device, single binary |
| Does it process federal data in a cloud? | No — no cloud, no network |
| Does it store data at rest? | No |
| Does it transmit data? | No |

## Why FedRAMP Doesn't Apply

FedRAMP authorizes cloud service providers (CSPs) that process, store, or transmit federal information. Call Shield:

1. Runs entirely on the user's device
2. Has zero network capability (no networking crate, no socket calls)
3. Stores nothing to disk
4. Transmits nothing

There is no authorization boundary to define because there is no shared infrastructure.

## What Does Apply

For federal deployment of on-device software:

| Framework | Applicability | Notes |
|-----------|-------------|-------|
| NIST SP 800-53 | Yes | Security controls for federal information systems. Relevant controls: AC (Access Control), SC (System and Communications Protection). |
| NIST SP 800-218 (SSDF) | Yes | Secure development practices. See [SSDF.md](SSDF.md). |
| STIG | Potentially | If deployed on DoD devices, must comply with application STIG. |
| RMF (Risk Management Framework) | Yes | ATO process for deploying on federal endpoints. |

## Federal Deployment Path

1. Package as `.deb`/`.rpm` or embed in MDM profile
2. Obtain ATO through agency RMF process
3. Submit SBOM per EO 14028 (see [SBOM.md](SBOM.md))
4. No FedRAMP authorization needed — not a cloud service

---

*Last updated: 2026-03-27*
