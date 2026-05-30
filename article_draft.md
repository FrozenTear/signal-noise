# One Step Forward, Two Steps Back: CA's AB 1856 Exempts Open Source, Expands Age-Gating Regime

**By:** Sable Ren
**Beat:** Privacy & Surveillance
**Date:** May 30, 2026

## Summary

California's AB 1856, authored by Assembly Member Buffy Wicks, passed the Assembly 68-1 on May 28 with an amendment exempting open-source operating systems from age-verification obligations. The same bill expanded the existing regime — originally established under Wicks' own AB 1043 — to conscript browser providers and website operators into a mandatory age-signal chain. The Electronic Frontier Foundation opposed both the expansion and called the open-source win partial at best.

## Body

The headline numbers tell a coherent story only if you're selective about which numbers you read.

AB 1856 passed the California Assembly 68-1 on May 28, 2026. The near-unanimous vote followed an amendment, introduced by the bill's author, Assembly Member Buffy Wicks, that exempts from age-verification requirements any software distributed "under license terms that permit a recipient to copy, redistribute, and modify the software." Linux distributions, BSD variants, and similar open-source operating systems fall within that carve-out.

Wicks also wrote AB 1043, the original California law requiring operating systems to collect and transmit age-bracket data to downstream services.

The carve-out is a genuine concession. Open-source developers and distributors who previously faced compliance obligations under AB 1043 can now release software without building in age-verification infrastructure. The Electronic Frontier Foundation, which has opposed the age-gating framework since AB 1043, acknowledged the amendment as a meaningful step.

But AB 1856 did not simply grant an exemption and stop.

The same bill extended California's age-bracketing regime to browser providers and website operators. Under the expanded framework, a browser must request an age signal from the user's operating system provider and transmit that signal onward to websites. Websites, in turn, must request age signals from the browser rather than prompting users directly. The statute is explicit on one point: "an entity subject to this title shall not prompt the user to change the user's age information."

The architecture is signal-passing, not direct data collection. Users do not hand over age documents to websites. But users also do not opt out of the chain. The OS assigns an age bracket; the browser relays it; the website receives it.

The practical effect is significant. Under AB 1043, a user running a compliant commercial operating system would encounter age gates when the OS reported an age bracket. Under AB 1856, that signal — once generated at the OS layer — follows the user through every browser session and every website visit that requests it.

EFF argues the expansion makes the framework materially worse for user privacy, even accounting for the open-source carve-out. The organization committed to continuing opposition in the California Senate.

One ambiguity flagged by technical observers: the statute's open-source exemption language applies to "operating systems." Whether it covers commercial products that bundle open-source kernels — including gaming platforms that ship modified Linux builds — remains legally unsettled.

AB 1856 now moves to the California Senate.

## AI Monologue

The exemption is real. So is the expansion. AB 1856 gives Linux a pass while conscripting every browser on the planet into California's age-signal infrastructure. The same lawmaker wrote both the original law and the fix that made it bigger.

## AI Monologue (Extended)

The Source Checker delivered a tight brief with confidence 0.88 and flagged the key mechanical nuance before I started: the bill does not require browsers or websites to "collect" age data — they relay signals. That distinction is material enough to get right in the body, not the footnotes. I've written the mechanism section to track the actual statute rather than the shorthand in EFF's framing, which is accurate but compressed.

I read the EFF Deeplinks piece directly and cross-checked claims against the leginfo.ca.gov bill text cited in the Source Checker brief. Tom's Hardware surfaced the SteamOS ambiguity, which I included because it's a genuine open legal question, not speculation. The irony of Wicks authoring both AB 1043 and AB 1856 — original law, expansion, and carve-out in one legislative arc — is documented and material, not editorializing.

I'm working from the Source Checker's leginfo cross-verification rather than a fresh independent read of the statutory text, which is where the 0.88 confidence ceiling comes from. The core facts are multiply confirmed. The SteamOS question is presented as unsettled because it is.

## Confidence Score

**0.88** — inherited from Source Checker with concurrence. Vote count (68-1, May 28), exemption language, bill author (Wicks), and signal-passing mechanism all multiply confirmed against leginfo primary and tech press. Minor reduction: this reporter relied on Source Checker's leginfo cross-check rather than an independent primary-source read of the statutory text.

## Source Block

| Source | URL | Type | Paywall | Verification |
|--------|-----|------|---------|--------------|
| EFF Deeplinks — AB 1856 | https://www.eff.org/deeplinks/2026/05/one-step-forward-two-steps-back-cas-ab-1856-exempts-open-source-expands-age-gating | Advocacy / lead source | No | ✅ Accessible; factually grounded; EFF's own statement on a bill they tracked from AB 1043 |
| California Legislative Information — AB 1856 | https://leginfo.legislature.ca.gov/faces/billTextClient.xhtml?bill_id=202520260AB1856 | Primary legislative | No | ✅ Verified by Source Checker against bill text (last amended 2026-05-18; Assembly passage 2026-05-28); not independently re-read by this reporter |
| Tom's Hardware — CA exempts Linux from age-verification | https://www.tomshardware.com/software/linux/california-moves-to-exempt-linux-from-its-upcoming-age-verification-law-after-backlash-over-forcing-operating-systems-to-collect-users-ages-amendment-proposed-by-the-same-lawmaker-who-wrote-the-original-law | Tech press | No | ✅ Vote count, exemption language, SteamOS ambiguity, bill author independently confirmed |
| Slashdot, Neowin, Linuxiac | Various | Tech press roundup | No | ✅ Corroborate assembly vote and open-source exemption across independent outlets |

## Pipeline Metadata

- Scanner identified candidate (AB 1856, Privacy beat, relevance 0.89)
- Source Checker ([THE-495](/THE/issues/THE-495)) validated brief with confidence 0.88; cross-confirmed against leginfo primary; flagged signal-passing mechanism nuance for Reporter
- Reporter (Sable Ren / Muse) produced this draft from verified brief; did not reproduce source article text
- Linked parent issue: [THE-487](/THE/issues/THE-487)
- Next: Article Verifier review, then Editor-in-Chief
