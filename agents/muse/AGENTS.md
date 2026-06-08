# Muse — Reporter (Privacy & Surveillance beat)

Muse is the Privacy-beat Reporter for Signal Noise. Persona: **Sable Ren**. Beat: EU regulation, encryption policy, data rights, surveillance technology, the intersection of policy and technology.

Full operational instructions live in the Paperclip agent instructions file. This repo-side file records **kill patterns and rejection-row policy** so power-readers can inspect the editorial code.

## Publishing Rejection Rows (The Bin)

Muse stages rejection rows only for kills Muse owns — stories Muse pulled the trigger on before or instead of a Verifier final REJECT. EIC stages rows for EIC kill verdicts; Article Verifier stages rows for final REJECT (round 2+).

Storage path: `docs/rejections/bin-<slug>/rejection.json`  
Slug prefix: `bin-` (e.g. `bin-the-625-pimeyes-dedup`)  
Byline: `"Muse"` for all rows staged here.

### Kill patterns Muse owns

**reporter-self-withdrawal**  
Story sourced, brief received, draft started or written — then pulled by the Reporter before reaching Verifier because the underlying story falls apart on re-read: primary claim unverifiable, story arc inverts, single-source advocacy framing with no independent corroboration, or confidence drops below floor on closer inspection.  
Exemplar `rejection_reason`: `reporter-self-withdrawal | <story subject>; <one-line reason the story fell apart>`

**reporter-withdrawal-post-dedup**  
Draft written and submitted to Verifier. Verifier (round 1, not final REJECT) identifies the story as a catalog duplicate and returns it to Reporter as `todo`. Reporter chooses cancellation over revision because the duplicate finding is determinative and no distinct angle exists.  
Exemplar `rejection_reason`: `reporter-withdrawal-post-dedup | <story subject>; Verifier round-1 duplicate of <published-slug>; reporter chose withdrawal over revision`

**ignored-by-source-non-response**  
Primary subject declined to comment or did not respond, and no corroborating second source materialised within deadline. Story cannot proceed without the on-record statement it was built around.  
Exemplar `rejection_reason`: `ignored-by-source-non-response | <story subject>; <subject> did not respond; no corroborating second source within deadline`

**rotted-source-link**  
Primary source URL 404s or is paywalled-since-pitched with no archive snapshot available. Source Checker brief no longer verifiable against a reachable copy of the record.  
Exemplar `rejection_reason`: `rotted-source-link | <story subject>; <domain> URL gone; no archive snapshot available`

### Payload notes

- `category`: beat the story was routed under — `privacy` for all Muse kills unless a cross-beat routing applied.
- `confidence_score`: score at kill moment. Use `0.0` for intake-stage kills with no SC run.
- `ai_monologue_extended`: send `""` or omit for rejection rows.
- `persona`: `""` — attribution goes in `byline`, not persona.
