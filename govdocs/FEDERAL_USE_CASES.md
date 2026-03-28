<!-- Unlicense — cochranblock.org -->

# Federal Use Cases

*Which agencies could use Call Shield and how.*

---

## DoD — Department of Defense

**Use:** Secure call screening for military personnel without cloud exposure.

Deployed service members receive spam calls on government and personal devices. Current screening solutions (Google Call Screen, carrier filters) route call audio through commercial cloud infrastructure — a direct OPSEC violation for anyone handling classified work or operating in sensitive locations.

Call Shield screens calls on-device with zero network transmission. Deployed on government-issued mobile devices via MDM, it eliminates the third-party audio processing entirely.

**Relevant offices:** DISA (Defense Information Systems Agency), each service's CIO, Cyber Command for operational security tooling.

**SBIR topics:** Secure mobile communications, edge computing for tactical environments, OPSEC tools for personnel security.

---

## DHS — Department of Homeland Security

**Use:** Counter-robocall technology that doesn't create new surveillance points.

CISA tracks robocall campaigns as a social engineering threat vector. Current counter-robocall tools (STIR/SHAKEN, carrier filters) address caller ID spoofing but not call content screening. They also create centralized databases of call metadata.

Call Shield fills the gap: screen call content locally without creating a new data collection point. Useful for DHS personnel who receive targeted social engineering calls (vishing) and need screening without metadata exposure.

**Relevant offices:** CISA, ICE HSI (for counter-fraud), TSA (personnel security).

---

## VA — Department of Veterans Affairs

**Use:** Spam call protection for veterans without data harvesting.

Veterans are disproportionately targeted by robocalls — debt scams, VA impersonation, benefits fraud calls. Commercial call screening solutions add these veterans' call data to advertising and broker pipelines.

Call Shield deployed via the VA's mobile health platform (VA Health Chat, My HealtheVet) screens calls for veterans without exposing their call patterns to commercial data brokers.

**Relevant offices:** OIT (Office of Information and Technology), VHA (Veterans Health Administration) for telehealth integration.

---

## DOJ — Department of Justice

**Use:** Secure communications screening for prosecutors and investigators.

Federal prosecutors and FBI agents receive calls from unknown numbers during active investigations. Using commercial call screening exposes investigation-related call patterns to commercial cloud providers.

Call Shield provides screening without creating discoverable records in third-party systems. No subpoena-able call screening logs on Google or carrier servers.

**Relevant offices:** FBI (personnel security), EOUSA (Executive Office for US Attorneys), DEA.

---

## NSF — National Science Foundation

**Use:** On-device ML inference research platform.

Call Shield demonstrates real-time ML inference (Whisper STT + classification) compiled into a single binary with no cloud dependency. This is a reference implementation for NSF-funded research in:
- Edge computing
- Privacy-preserving AI
- On-device inference performance

**Relevant programs:** CISE (Computer and Information Science and Engineering), Smart and Connected Communities.

---

## DOE — Department of Energy

**Use:** Communications security for national laboratory personnel.

National lab employees (Los Alamos, Sandia, Oak Ridge, Lawrence Livermore) work on classified and sensitive programs. Their personal devices are targets for intelligence collection, including call metadata analysis.

Call Shield on personal devices screens calls without exposing patterns to commercial providers. Complements existing SCIF communications security.

**Relevant offices:** NNSA (National Nuclear Security Administration), Office of Cybersecurity, Energy Security, and Emergency Response (CESER).

---

## GSA — General Services Administration

**Use:** Shared service for federal workforce call screening.

GSA could deploy Call Shield as a standard tool in the federal device baseline — available to all agencies via the GSA IT Schedule. Single binary deployment via MDM, no cloud infrastructure to authorize, no FedRAMP process needed.

**Relevant offices:** FAS (Federal Acquisition Service), Technology Transformation Services (TTS), 18F.

---

## Procurement Path

| Step | Action |
|------|--------|
| 1 | SAM.gov registration: done (The Cochran Block, LLC) |
| 2 | SDVOSB certification: pending |
| 3 | SBIR Phase I proposal targeting DoD/DHS call screening |
| 4 | GSA Schedule listing for wider federal availability |
| 5 | Agency-level ATO for deployment on government devices |

---

*The Cochran Block, LLC — Dundalk, MD. SDVOSB (Pending). cochranblock.org*
*Last updated: 2026-03-27*
