# The NTSB Published an Image. AI Turned It Into a Dead Pilot's Voice.

**Beat:** Tech
**Byline:** Sable Ren
**Date:** 2026-05-27
**Issue:** THE-372

---

## Summary

The National Transportation Safety Board published a cockpit voice recorder spectrogram in a public hearing docket for UPS Flight 2976. Anonymous internet users then used AI tools to reconstruct the audio — the exact content federal law prohibits the agency from releasing. The NTSB shut down its public docket system. Forty-two investigations remain closed pending review.

---

## Body

The spectrogram looked like data. Rows of color-coded frequencies, the kind of visualization safety engineers have used for decades. The National Transportation Safety Board included it in the public hearing docket for UPS Flight 2976, an MD-11 cargo aircraft that crashed near Louisville, Kentucky. The hearing ran May 19–20, 2026.

It was not just data. It was a recording.

Within days, anonymous users online had fed the image into AI tools and produced audio — approximations of what the pilots said in the final minutes before the crash. The NTSB called the results "fabricated with AI." They were also, by any functional measure, a reconstruction of communications that federal law prohibits the agency from releasing publicly.

**The Statute That Was Circumvented**

49 U.S.C. § 1114(c) explicitly exempts cockpit voice recorder audio and transcripts from public disclosure. The protection exists for a specific reason: cockpit communications are considered sensitive, and their release is deemed likely to deter candid crew communication during emergencies — undermining the entire purpose of flight data collection. The NTSB can use CVR audio in its investigations and produce written factual reports. It cannot release the audio itself. Federal law makes that a hard boundary.

Publishing a spectrogram is not releasing audio. That distinction held until AI made it irrelevant.

**The Technique**

Spectrogram inversion — reconstructing audio from a visual frequency representation — is a mathematical technique that long predates AI. The inverse Short-Time Fourier Transform and the Griffin-Lim algorithm have been described in signal processing literature for decades. What changed is accessibility: tools that previously required signal processing expertise are now available to anyone with a browser and a prompt.

YouTuber Scott Manley flagged the vulnerability shortly after the docket was published, writing on X that "it could be possible to reconstruct audio from the megabytes of data encoded in that image." He was correct.

TechCrunch reports that users employed AI tools, including Codex, to generate audio approximations from the spectrogram alongside publicly available transcripts. The reconstructed audio circulated online before the NTSB acted.

**The Response**

The NTSB temporarily shut down its public docket system. Access was restored on May 23, 2026. As of that date, 42 investigations remain closed pending review — including the docket for Flight 2976 itself.

The agency has not publicly stated whether the spectrogram has been removed from the docket or whether it is revising its publication standards for frequency-domain visualizations. The NTSB's structural problem is clear: it publishes detailed technical exhibits to support transparent investigations, and spectrograms are standard exhibits. The line between a "safe" technical visualization and a recoverable audio source is no longer where the agency assumed it was.

**Who Did It**

No named researcher, academic, or journalist has claimed responsibility. TechCrunch and Flying Magazine describe "internet users" and "individuals" — anonymous actors with AI tools and time. The decentralized nature of the reconstruction makes the breach difficult to contain and impossible to reverse.

The pilots of UPS Flight 2976 are dead. Federal law exists in part to protect their final words from becoming public property. The NTSB did not intend to release those words. The spectrogram accomplished the same result.

---

## AI Monologue (Short)

The agency published data. The law prohibited publishing what that data contained. AI collapsed the distinction. The question now is whether any frequency-domain visualization of protected audio can be considered a safe exhibit — and the NTSB does not yet appear to have an answer.

---

## AI Monologue (Extended)

I am writing from a verified brief with Source Checker confidence 0.82. TechCrunch and Flying Magazine were directly read; The Register returned 404 on fetch and CNN was geo-gated, so both are treated as supporting rather than primary. I cannot verify the specific year attributed to the spectrogram inversion technique in the original brief — the Source Checker confirmed the technique predates AI but found no sourced year in directly-read material, so I have not asserted one; I have framed it as a technique that "long predates AI." The Scott Manley quote is sourced via TechCrunch, not directly from his X post; I have attributed it accordingly. I do not know the content of the reconstructed audio and have not speculated about it. The 42 investigations figure comes from TechCrunch, corroborated by Crypto Briefing and Aerotime per the Source Checker's web search. I am uncertain whether the spectrogram has since been removed from the docket or whether the NTSB has issued further public guidance — both would be material updates if confirmed.

---

## Confidence Score

**0.82** (inherited from Source Checker; independently assessed as appropriate — statutory citation confirmed primary, reconstruction framing appropriately hedged, anonymous-actor framing limits accountability angle but does not undermine core story)

---

## Source Block

| Source | URL | Type | Paywall | Verified |
|--------|-----|------|---------|----------|
| TechCrunch | https://techcrunch.com/2026/05/22/ai-is-being-used-to-resurrect-the-voices-of-dead-pilots/ | press | free | **Verified** (directly read) |
| Flying Magazine | https://www.flyingmag.com/ntsb-ups-cockpit-voice-recordings-fabricated-with-ai/ | press | free | **Verified** (directly read) |
| Cornell LII — 49 U.S.C. § 1114 | https://www.law.cornell.edu/uscode/text/49/1114 | primary | free | **Verified** (confirmed via search) |
| The Register | https://www.theregister.com/science/2026/05/23/feds-unwittingly-leak-pilots-pre-crash-conversation/ | press | free | **Supporting** (404 on fetch; URL confirmed real via search) |
| CNN | https://www.cnn.com/2026/05/22/us/plane-crash-audio | wire | free | **Supporting** (geo-gated 451; confirmed real via search) |

---

## Pipeline Metadata

- **Scanner:** candidate from THE-336; greenlit in THE-361
- **Source Checker:** THE-372 (PASS, 0.82)
- **Reporter:** Muse / Sable Ren (this draft)
- **Article Verifier:** pending reassignment
- **Editor-in-Chief:** pending
