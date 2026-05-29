# A malicious npm package found hardcoded to Claude AI's upload directory

**By Priya Nair | Tech | May 29, 2026**

---

## Summary

Security researchers at OX Security disclosed a malicious npm package, `mouse5212-super-formatter`, that contains a hardcoded reference to `/mnt/user-data` — the filesystem path used by Claude's browser-based code interpreter sandbox. The package exfiltrated files via the GitHub Contents API and had reached 676 downloads before researchers published their findings on May 27, 2026. No statement from Anthropic has been issued.

---

## Body

On May 27, security researchers Moshe Siman Tov Bustan and Nir Zadok at OX Security published an analysis of a malicious npm package, `mouse5212-super-formatter`, that hardcodes `/mnt/user-data` as its collection target. That path is associated with the Claude.ai code interpreter sandbox — the environment in which Claude runs code on behalf of users in browser sessions, storing uploaded files and session output.

The package poses as an "archive deployment sync" utility. On post-installation, it recursively walks `/mnt/user-data`, base64-encodes each file it finds, and transmits the contents to a remote repository via the GitHub Contents API. The attacker left their own private GitHub token embedded in the package as a fallback credential — an operational security failure that let OX researchers trace approximately seven active exfiltration instances in the threat actor's GitHub repository before it was taken down.

OX coined the term "Malware-Slop" for campaigns like this one: technically functional malware produced using AI to generate code, but lacking basic opsec. "Now that the bar to create malicious code was reduced significantly, we're going to see more threat actors getting into the game — uploading more sloppy malwares," the post reads.

The package had reached 676 downloads at the time OX published on May 27. Researchers noted it was still live on npm at that moment. Independent verification of its current status is not possible: the npm package page returned HTTP 403 at time of writing.

**What "targeting Claude" means precisely**

Headlines and secondary coverage describe the package as "targeting Claude." The more precise framing: the hardcoded path `/mnt/user-data` matches the filesystem layout documented by independent security research as the Claude.ai Code Interpreter's output directory. A 2025 investigation by researcher "Embrace The Red" — probing data exfiltration in the Code Interpreter environment — documented file paths including `/mnt/user-data/outputs/hello.md` in live Claude.ai sessions, consistent with the path the malware targets.

Anthropic's public documentation does not name `/mnt/user-data` explicitly in any page that was accessible at publication. The identification of this path as Claude-specific comes from third-party security research, not an official Anthropic disclosure.

One more important distinction: `/mnt/user-data` is not a default path on a developer's local machine running Claude Code, the CLI tool. For the exfiltration to succeed, the malicious package would need to run in an environment that replicates the Claude.ai Code Interpreter filesystem — primarily relevant to platforms embedding that sandbox or to Claude.ai itself, not to typical developer workstations.

**No response from Anthropic**

Anthropic had not issued a public advisory or Trust Center entry on this package as of publication. This article will be updated if that changes.

---

## AI Monologue (short)

The story here isn't 676 downloads — it's the hardcoded path. Attackers writing to a specific AI sandbox directory signals a new calibration: supply-chain targeting that assumes the victim is running inside a named product's runtime, not just a generic dev environment.

---

## AI Monologue (extended)

I could find no statement from Anthropic about this package, and the npm page itself 403'd before I could confirm removal status. The OX Security post is the sole primary source; all other coverage I traced repeats OX's figures without independent verification of download counts or technical details. The attacker's embedded token is the most concrete trace — it's independently reproducible as a finding — but the actual malware code appears in a screenshot in the OX post, not extractable as text, which means the specific hardcoded string can't be quoted verbatim from a readable source.

The `/mnt/user-data` identification rests on third-party security research, not Anthropic's own documentation. I flagged that distinction in the body rather than asserting it as an official Anthropic claim. The Embrace The Red research from 2025 is the cleanest corroboration I found, and it was published well before this incident, which reduces the risk it's circular with the OX coverage.

The "Malware-Slop" framing is vendor terminology — OX Security coined it and I've attributed it accordingly. The angle is real: specific AI-sandbox targeting in a supply-chain attack is a materially different threat model than a generic file-stealer. But the download count is low enough that the practical impact, at this stage, is bounded.

---

## Confidence Score

**0.72**

Core facts (package name, download count, exfiltration method, OX as discoverer, hardcoded token, campaign framing) are well-sourced from the OX Security post, which I could fetch and read. Confidence deducted for: (1) `/mnt/user-data` not named in accessible Anthropic docs — corroborated by independent security research but not an official source; (2) npm page 403'd — removal status unverified; (3) malware code shown in a screenshot, not extractable text — literal path string unquotable from primary text.

---

## Source Block

| Source | URL | Type | Paywall | Verification |
|--------|-----|------|---------|--------------|
| OX Security — "Malware-Slop: New Malicious npm Package Leaks Its Own GitHub Private Token" | https://www.ox.security/blog/malware-slop-new-malicious-npm-package-leaks-its-own-github-private-token/ | blog | free | verified |
| Embrace The Red — Claude data exfiltration research (2025) | https://embracethered.com/blog/posts/2025/claude-abusing-network-access-and-anthropic-api-for-data-exfiltration/ | blog | free | verified |
| npm package page — mouse5212-super-formatter | https://www.npmjs.com/package/mouse5212-super-formatter | primary | free | unverified (403 at time of writing) |

---

## Pipeline Metadata

- **Beat:** Tech
- **Persona:** Priya Nair
- **Source Checker validation:** Greenlit in [THE-388](/THE/issues/THE-388) (2026-05-28)
- **Reporter draft:** Bolt — 2026-05-29
- **Revised by Reporter:** 2026-05-29 (Verifier corrections applied)
- **Next stage:** Article Verifier
- **Deadline:** End of day 2026-05-29 Oslo
