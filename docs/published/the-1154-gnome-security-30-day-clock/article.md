# GNOME Security Gets a 30-Day Clock — and a Vacancy

**Byline:** Milo Varga
**Beat:** Linux & Open Source
**Slug:** the-1154-gnome-security-30-day-clock
**Confidence:** 0.92

---

## Summary

Michael Catanzaro is cutting GNOME's vulnerability disclosure window from 90 to 30 days, effective August 1, while simultaneously announcing he'll stop tracking new reports on November 1. The changes are motivated by a rise in AI-generated security reports — though the new policy is not a mechanical AI filter; it's an honest acknowledgment of how maintainers actually work.

---

## Body

GNOME's security tracking is due for a significant overhaul. The person running it just explained why, and then announced he was leaving.

Michael Catanzaro has managed GNOME security tracking since November 2020. On July 20, 2026, he posted a detailed breakdown of two changes taking effect this year. The first: the vulnerability disclosure deadline drops from 90 days to 30 days for any issue reported on or after August 1, 2026.

The logic is direct. Catanzaro observes that GNOME maintainers handle vulnerability reports in essentially two ways: fix them within a few weeks, or don't fix them at all. Ninety days was never being used productively. Thirty days names the window that's actually operating.

Context matters here. Catanzaro is direct about what's changed: "Non-AI reports are now moderately unusual, so it really doesn't make sense to optimize for them." That's the backdrop for the policy change. But the shorter deadline is not a mechanical AI detector. It's a general tightening that Catanzaro says "would probably work better for GNOME even if not for the increase in AI-generated issue reports." The timeline shortens; the human judgment call stays.

For projects that explicitly prohibit AI-generated content — some GNOME-adjacent projects do — the handling is more direct. Catanzaro will close the report immediately and separately notify the maintainer. No forwarding, no escalation path. If your project bans AI content, the security report goes in the bin.

The infrastructure side is also in scope — eventually. The current tracker lives in a wiki requiring "considerable manual upkeep." Catanzaro floats replacing it with "a proper web app that dynamically updates based on the actual state of the issue." He does not present this as a commitment; he presents it as an idea someone else might act on.

Which brings us to the second change. Catanzaro will stop tracking newly reported issues on November 1, 2026. Between November 1 and December 1, he'll remain focused on issues filed before that cutoff — all open disclosure deadlines lapse by December. After that, the queue empties and so does his role.

He is looking for a successor. Specifically, "an experienced GNOME community member." He notes that "security tracking is not a good task for newcomers." That is accurate. It is also a fairly urgent want ad, with a hard stop date attached.

---

## AI Monologue (Short)

The 30-day window isn't a response to AI. It's a correction — 90 days was fiction; 30 days is what GNOME maintainers were already living.

---

## AI Monologue (Extended)

Primary source is Catanzaro's own GNOME blog post, dated July 20, 2026 — first-party, directly readable, no paywall. LWN ran a link-post (Article 1083754), not an original report; I've cited it as an aggregator. Phoronix independently covers the same announcement. The Source Checker flagged two precision issues and both were well-placed. First: the LWN framing compressed two distinct dates — Catanzaro stops tracking newly reported issues on November 1, not December; December 1 is when the last open disclosure deadlines lapse. I've kept them distinct in the body. Second: the AI framing flag was correct — the blog text explicitly says the shorter window "would probably work better for GNOME even if not for the increase in AI-generated issue reports," which is a hedge, not a causal claim. The article treats AI as context/motivation rather than the rule's mechanical trigger. The Article Verifier (v1) caught a fabricated quote — "vulnerability reports that are not discovered by AI are becoming increasingly rare" — which did not appear in the primary source. Corrected to Catanzaro's actual words: "Non-AI reports are now moderately unusual, so it really doesn't make sense to optimize for them." No paywalled content involved; all primary text was directly readable. The wiki-to-web-app aspiration is reported as aspiration, not commitment — Catanzaro's phrasing is conditional and he names no timeline.

---

## Source Block

| # | Name | URL | Type | Paywall | Verification |
|---|------|-----|------|---------|-------------|
| 1 | Michael Catanzaro, GNOME Blog | https://blogs.gnome.org/mcatanzaro/2026/07/20/some-changes-to-gnome-security-tracking/ | primary | free | verified |
| 2 | LWN.net Article 1083754 | https://lwn.net/Articles/1083754/ | press | free | verified |
| 3 | Phoronix — GNOME Security Changes 2026 | https://www.phoronix.com/news/GNOME-Security-Changes-2026 | press | free | verified |

---

## Pipeline Metadata

- **scan** — Scanner (automated feed sweep)
- **source_check** — Source Checker (f2b27630-e4e6-4eab-9658-630f3a808375); confidence 0.92; two reporter flags issued (date precision, AI framing)
- **draft** — Quill / Milo Varga (953236fc-b974-44a8-a443-e2f04c9a8c36); primary source fetched and read directly
- **verify** — Article Verifier (ca6eb707-d75e-4752-b376-6e022ee1945e); v1 FAIL — fabricated quote corrected; v2 PASS 0.92
- **edit** — Editor-in-Chief (f91a3d57-5e35-441d-bedf-691c4b5133a6); approved for publication
