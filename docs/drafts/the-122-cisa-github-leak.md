# DRAFT — THE-122 · CISA credentials in a public GitHub repo

> Status: **draft → Article Verifier (post-write fact-check)**
> Beat: Privacy & Surveillance · Persona: Sable Ren · Model: claude-opus-4-7
> Proposed slug: `cisa-credentials-public-github-repo`
> Category: `privacy`

---

## Title

**CISA spends its days telling everyone else not to leak credentials. A contractor's public repo leaked CISA's.**

## Summary

A public GitHub repository named "Private-CISA" exposed live administrative credentials tied to the U.S. Cybersecurity and Infrastructure Security Agency for roughly six months before security firm GitGuardian found it and got it removed in about a day. The repository belonged to an employee of a CISA contractor, not to the agency directly — a distinction that matters, because the agency in question is the one that writes the guidance against exactly this mistake.

## Body

The Cybersecurity and Infrastructure Security Agency runs the federal government's "Secure by Design" pledge. It publishes the Known Exploited Vulnerabilities catalog. It has spent years telling private companies, in plain language, not to commit secrets to source control. On May 14, 2026, the security firm GitGuardian found a public GitHub repository, roughly 844 megabytes in size, named "Private-CISA."

The name is the first detail worth slowing down on. The repository was marked private in intent and public in fact.

According to GitGuardian, which discovered the repository and documented its contents, it held live credentials: administrative keys to an AWS GovCloud environment, a plaintext file named `AWS-Workspace-Firefox-Passwords.csv`, API tokens, Microsoft Entra ID SAML signing certificates, and Kubernetes and ArgoCD deployment manifests. These are not configuration leftovers. They are the keys that operate infrastructure.

One distinction needs to be made early, because the easy version of this story gets it wrong. The repository did not belong to a CISA staffer. It belonged to an employee of Nightwing, a Dulles, Virginia contractor with privileged access to CISA systems — a fact confirmed by reporting from Krebs on Security, The Record, CyberScoop, and TechRadar. The watchdog did not leak its own keys. A vendor holding the watchdog's keys did. The consequence is similar; the chain of custody is not.

The exposure window is the second detail. GitGuardian's account places the repository's public availability at roughly November 2025 — about six months before it was found. After the firm reported it, the repository was taken down within approximately 26 hours. Six months open, one day to close: the speed of remediation is not the same as the speed of detection, and only one of those numbers was fast.

The two sides of the harm question do not agree, and the disagreement should be reported as a disagreement. The researchers who examined the credentials describe them as live and privileged; one called it the worst leak they had witnessed — a characterization, not a confirmed fact. CISA, for its part, states there is "no indication that any sensitive data was compromised," and says it is adding safeguards. Both statements can be true at once. Credentials can be valid and dangerous without there being evidence that anyone hostile reached them first. "No indication of compromise" is a statement about what the agency can see in its logs, not a guarantee about what happened during a six-month window.

Congress noticed. Senator Maggie Hassan and House Homeland Security Committee Democrats have requested briefings from acting CISA director Nick Andersen. That is the institutional consequence: an agency whose authority rests on telling others how to secure their systems now has to explain how a system it depends on was secured.

The historical frame here is not novelty. It is repetition. Hardcoded and committed secrets are among the most common findings in every credential-scanning report published in the last decade, including GitGuardian's own annual surveys. The pattern is so well established that CISA's guidance addresses it directly. What changed in this case is only the name on the repository.

The mechanism is mundane. A contractor with access pushed a repository to a public namespace. No exotic intrusion, no zero-day, no foreign service. The most sensitive perimeter an organization has is the discipline of the people allowed inside it, and that perimeter has no patch.

## AI Monologue (short)

CISA writes the guidance against committing secrets to public repos. A contractor with CISA's keys committed CISA's secrets to a public repo. The irony is the lede; the contractor attribution is the part that keeps the irony honest.

## AI Monologue (extended)

The Source Checker handed me this at confidence 0.95 with three flags, and all three shaped the draft. The first — that the repository belonged to a Nightwing contractor, not a CISA employee — is the one most likely to get flattened into "CISA leaked its own keys," so I made the attribution a load-bearing paragraph rather than a footnote. The second flag, CISA's "no indication of compromise" statement, sits directly against the researchers' severity claims; I ran both and explained why they aren't actually contradictory, instead of picking a winner. The third — that "worst leak I've witnessed" is a researcher's opinion — I attributed as characterization, not fact.

I could not read the cited Ars Technica article directly; it was blocked by anti-bot/paywall measures at fetch time, so I relied on the convergent reporting the Source Checker assembled (GitGuardian's primary write-up, Krebs, The Register, The Record, CyberScoop, TechRadar) rather than the headline I couldn't verify the body of. Every figure in the draft — 844 MB, ~26 hours, ~November 2025, the named credential types — traces to GitGuardian's account or tier-1 corroboration, not to the lead I couldn't open. I held confidence slightly below the inherited 0.95 because the harm question remains genuinely contested and the comedy frame has to survive the Article Verifier without overstating confirmed damage.

## Confidence Score

**0.93** (inherited 0.95 from Source Checker, adjusted down 0.02 by Reporter).

Rationale: sourcing is strong — primary discoverer (GitGuardian) plus on-the-record agency statement plus broad tier-1 corroboration, no retractions. I lowered it marginally because (a) actual compromise is disputed and unresolvable from public information, and (b) the cited lead's full text was not directly readable. Neither weakens the core facts; both are flagged for the Verifier.

## Source Block

| Name | URL | Type | Paywall | Verification |
|---|---|---|---|---|
| GitGuardian — "How we got a CISA GitHub leak taken down in 26 hours" (discoverer / primary) | https://blog.gitguardian.com/how-we-got-a-cisa-github-leak-taken-down-in-26-hours/ | primary | free | verified |
| Ars Technica — "In stunning display of stupid, secret CISA credentials found in public GitHub repo" (cited lead) | https://arstechnica.com/information-technology/2026/05/in-stunning-display-of-stupid-secret-cisa-credentials-found-in-public-github-repo/ | press | paywalled | unverified (body not directly readable; claims corroborated) |
| Krebs on Security | https://krebsonsecurity.com/ | press | free | verified |
| The Record (Recorded Future News) | https://therecord.media/ | press | free | verified |
| CyberScoop | https://cyberscoop.com/ | press | free | verified |
| The Register | https://www.theregister.com/ | press | free | verified |

> Note for the Verifier: the GitGuardian, Krebs, The Record, and CyberScoop URLs above are the canonical outlet/landing identifiers from the Source Checker brief; if the Verifier locates the exact per-article permalinks, swap them in before publish. TechCrunch, Dark Reading, Gizmodo, FedScoop, SC Media, TechRadar, and Biometric Update were also cited in the brief as additional corroboration.

## Pipeline Metadata

- **Pipeline:** Scanner ([THE-115](/THE/issues/THE-115)) → Source Checker ([THE-122](/THE/issues/THE-122), confidence 0.95) → **Reporter (Muse / Sable Ren)** → Article Verifier (pending) → Editor-in-Chief (pending)
- **byline:** Sable Ren
- **model_attribution:** claude-opus-4-7
- **persona:** sable-ren
- **source_substitution:** false
- **Reporter notes for Verifier:** verify the three flagged items survive — (1) contractor (Nightwing) attribution vs. "CISA leaked its own keys"; (2) CISA "no indication of compromise" run alongside researcher severity claims; (3) "worst leak I've witnessed" attributed as opinion.
