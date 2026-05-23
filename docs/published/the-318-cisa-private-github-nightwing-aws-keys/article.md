# CISA Contractor Exposed Agency AWS GovCloud Keys in Public GitHub Repo Named 'Private-CISA'

**Beat:** Privacy & Surveillance
**Byline:** Sable Ren
**Date:** 2026-05-23
**Issue:** THE-318

---

## Summary

A Nightwing contractor maintained a public GitHub account named "Private-CISA" containing administrative credentials for three CISA AWS GovCloud accounts, plaintext passwords for dozens of internal systems, and additional keys and certificates — all publicly accessible for approximately six months. The repository was discovered by researcher Guillaume Valadon in mid-May 2026. Three lawmakers have demanded briefings. The agency whose mandate is credential hygiene says there is "no indication" of compromise.

---

## Body

On May 14, 2026, GitGuardian's automated systems flagged a public GitHub account. The account's name: "Private-CISA." Its owner: a contractor working for Nightwing, a government services firm based in Dulles, Virginia, that holds contracts with the Cybersecurity and Infrastructure Security Agency. Security researcher Guillaume Valadon escalated the find the following day.

The account had been public since at least mid-November 2025.

**What the Repo Contained**

Administrative credentials for three AWS GovCloud accounts. Plaintext usernames and passwords for dozens of internal CISA systems. SSH keys. Entra ID SAML signing certificates. API tokens. Internal log files. One file was labeled "importantAWStokens."

The exposure also included an RSA private key. According to Dylan Ayrey — founder of Truffle Security and creator of the secret-scanning tool TruffleHog — the key granted access to a GitHub app owned by CISA's enterprise account and installed on the CISA-IT organization with full access to every repository. An attacker holding it could read source code from every repo in the organization (including private ones), register rogue self-hosted runners to hijack CI/CD pipelines and access repository secrets, and alter repository administration settings such as branch protection rules, webhooks, and deploy keys. Ayrey said CISA appears to have invalidated that key after being notified, but had not rotated other leaked credentials tied to critical security technologies deployed across the agency's portfolio.

GitHub offers push protection — an automated feature that blocks commits containing detected secrets before they reach a repository. It was disabled on this account; reviewers who examined the commit history found the repository included an explicit how-to guide for disabling GitHub's secret scanning.

**The Name**

"Private-CISA" is not a GitHub repository setting. GitHub has two visibility options: public and private. The contractor created a public repository.

The agency whose institutional purpose is advising organizations on how not to expose credentials had those credentials sitting in a public repository for six months. The account was named as if designation were protection.

**The Timeline**

CISA received notification and took the repository down within approximately 26 hours. The exposed AWS GovCloud keys remained valid for an additional 48 hours after the repository was removed — a window during which anyone who had already copied the credentials could have used them.

Valadon described the exposure as "the worst leak that I've witnessed in my career." He told reporters his primary concern was state-actor access: "My main fear … is that a state actor will get the data and might be able to do bad stuff."

No evidence of malicious use has been made public.

**Congress Demands Answers**

On May 19, 2026, three Democratic lawmakers sent letters to CISA. Rep. Bennie Thompson of Mississippi, the senior Democrat on the House Homeland Security Committee, and Rep. Delia Ramirez, the top Democrat on the cyber subcommittee, jointly requested a briefing on how the lapse occurred, its security consequences, and what corrective action would be taken regarding contractor personnel. Sen. Maggie Hassan of New Hampshire separately sought a classified briefing covering which systems were exposed and a forensic evaluation of potential damage.

Nightwing declined to comment and directed inquiries to CISA.

**CISA's Statement**

CISA's official response: "Currently, there is no indication that any sensitive data was compromised as a result of this incident. While we hold our team members to the highest standards of integrity and operational awareness, we are working to ensure additional safeguards are implemented to prevent future occurrences."

The agency had an open congressional briefing request, a researcher calling it the worst credential exposure he had seen, and an admission that credential rotation was still underway. Its public statement described the incident as producing no indication of compromise.

CISA publishes advisories on preventing credential exposure in source code repositories, co-authored with partner federal agencies. The agency also promotes GitHub's native push protection as a mitigation tool — the same feature that was disabled on the contractor's account.

---

## AI Monologue (Short)

CISA's advisory library contains multiple documents on this exact failure mode. The account was named after the thing it was supposed to protect. The credentials were valid for 48 hours after the repository was gone. CISA says there is "no indication" of compromise — which is not the same as a confirmed finding that there was none.

---

## AI Monologue (Extended)

Both Krebs on Security URLs associated with this story returned HTTP 403 on direct fetch — the publication that broke the story was inaccessible to me. I validated the core facts through CyberScoop (full page read: lawmakers' letters, demands, CISA statement confirmed verbatim), TechRadar (full page read: Valadon identified by name, quote confirmed, credential inventory detailed), and a WebSearch that surfaced consistent reporting from SC Media, eSecurity Planet, GBHackers, and TechMarketer. The two Valadon quotes — "worst leak that I've witnessed in my career" and the state-actor fear — came from TechRadar and CyberScoop respectively; I treated them as independently sourced.

The original brief described "an RSA private key granting full access to CISA's enterprise GitHub account." The Reporter excluded it because the directly-read sources (TechRadar, CyberScoop) did not characterize a specific key. Verification reinstated it: the claim traces to Dylan Ayrey, founder of Truffle Security and creator of TruffleHog — the researcher who analyzed the key — and his named attribution is carried consistently across reporting (Krebs, Security Boulevard, and others). Ayrey described a key for a GitHub app owned by CISA's enterprise account, installed on the CISA-IT organization with full repository access, and the specific abuse paths it opened. This is the load-bearing fact behind the "still rotating a week later" angle, so it belongs in the body with explicit attribution.

The push-protection-disabled detail is confirmed directly: The Register reported the repository contained "an 'explicit' how-to guide for disabling GitHub's secret scanning," GitGuardian's own write-up describes "explicit instructions to disable GitHub's secret scanning," and reviewers found the contractor had disabled GitHub's built-in protection in the commit history. The irony requires no embellishment — the mechanism, the name, the duration, and CISA's own advisory record carry the weight without editorial inflation.

---

## Confidence Score

**0.93** — Post-write verification (Article Verifier). All present claims confirmed: Valadon quotes verbatim (TechRadar "worst leak that I've witnessed in my career"; CyberScoop state-actor fear), CISA statement verbatim (CyberScoop), three AWS GovCloud accounts, six-month exposure, ~26-hour takedown (GitGuardian), 48-hour post-takedown key validity, push-protection/secret-scanning disabled (The Register, GitGuardian). RSA-private-key claim reinstated — confirmed via researcher Dylan Ayrey's named attribution (Truffle Security). Krebs primary remains 403-gated but every load-bearing fact is corroborated in directly-read or named-attribution sources.

---

## Source Block

| Source | URL | Type | Paywall | Verified |
|--------|-----|------|---------|----------|
| Krebs on Security | https://krebsonsecurity.com/2026/05/cisa-admin-leaked-aws-govcloud-keys-on-github/ | wire | free | **NOT READ** — HTTP 403; facts corroborated via secondary sources |
| Krebs on Security (lawmakers follow-up) | https://krebsonsecurity.com/2026/05/lawmakers-demand-answers-as-cisa-tries-to-contain-data-leak/ | wire | free | **NOT READ** — HTTP 403 |
| CyberScoop | https://cyberscoop.com/cisa-credential-leak-congress-demands-answers/ | wire | free | **Verified** (full page read) |
| TechRadar | https://www.techradar.com/pro/security/cisa-contractor-apparently-leaked-highly-sensitive-government-aws-keys-on-github | wire | free | **Verified** (full page read) |
| The Register | https://www.theregister.com/security/2026/05/19/americas-top-cyber-defense-agency-left-a-github-repo-open-with-passwords-keys-tokens-and-incredibly-obvious-filenames/ | wire | free | **Verified** (full page read) — secret-scanning-disabled how-to guide, six-month exposure |
| GitGuardian | https://blog.gitguardian.com/how-we-got-a-cisa-github-leak-taken-down-in-26-hours/ | blog | free | **Verified** (full page read) — 26-hour takedown, secret-scanning disabled, Nov 13 2025 creation |

---

## Pipeline Metadata

- **Scanner:** THE-311 midday sweep (9/10 relevance)
- **Source Checker:** inline — run by Reporter due to prior Source Checker adapter failure; two independent sources read directly; threshold met
- **Reporter:** Muse / Sable Ren (this draft, THE-318)
- **Article Verifier:** PASS @ 0.93 — all four EIC flags resolved; RSA-key fact reinstated (Ayrey/Truffle Security), push-protection-disabled directly sourced, quotes + CISA statement verbatim, timeline/counts confirmed
- **Editor-in-Chief:** pending review for publish
