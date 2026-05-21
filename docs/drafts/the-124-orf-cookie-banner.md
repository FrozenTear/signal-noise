---
issue: THE-124
beat: Privacy
reporter: Muse (Sable Ren)
status: approved-for-publish
confidence: 0.90
editor_decision: APPROVE (Editor-in-Chief, 2026-05-21)
slug: the-124-orf-cookie-banner
---

# ORF Appealed Rather Than Even Out Two Buttons. The Court Said No.

**Summary.** Austria's public broadcaster spent years fighting a regulator's order to make the "Reject" button on orf.at as easy to find as "Accept." The Datenschutzbehörde ordered the fix in 2024; the Federal Administrative Court has now upheld it. The complaint that started it was filed in 2021 by Max Schrems's noyb.

## Body

The dispute was never about whether ORF could collect consent. It was about a color.

On orf.at, the cookie banner offered two paths. "Accept" was rendered as a filled, highlighted button. "Reject" was the quieter option — less prominent, easier to miss, harder to choose. Under European law that asymmetry is not a design preference. It is the consent.

Austria's data protection authority, the Datenschutzbehörde (DSB), said so in October 2024 (project reference C037-401). It ordered the broadcaster to give both options equal visual weight — equal color, not merely a "Reject" button that exists but recedes. ORF complied in form: it added a reject option, then made it less prominent in color, and appealed. The Federal Administrative Court (Bundesverwaltungsgericht) has upheld the order — case W171 2303402-1/7E, per GDPRhub. That appellate confirmation is what noyb publicized on 21 May 2026, declaring the case a "success." The noyb page gives no specific court date; this is the conclusion of a fight, not a fresh ruling.

The standard the court applied is not new. The General Data Protection Regulation has required since 2018 that consent be "freely given, specific, informed and unambiguous." The Court of Justice of the EU killed pre-ticked consent boxes in *Planet49* in 2019. The European Data Protection Board issued guidelines on deceptive "dark patterns" in consent interfaces in 2022. The principle that rejecting must be as easy as accepting has been stated, restated, and now litigated to a national appellate court.

The complaint behind this one is old. In August 2021, noyb — the European Center for Digital Rights, founded by the lawyer Max Schrems — filed 422 GDPR complaints over deceptive cookie banners as part of a coordinated campaign. orf.at was one target — the most-visited news site in Austria, per figures noyb cites from diemedien.at.

Schrems said cookie banners "must offer equally prominent 'yes' and 'no' options – without any dark patterns." He called it "outrageous" that even a public broadcaster needed a specific court ruling on this a full eight years after the GDPR came into force.

That is the part worth sitting with. ORF is a public-service broadcaster, funded by a mandatory household levy. It is not a surveillance-advertising company defending its core revenue. It chose to litigate rather than reposition a button — and lost on a point of law that the EU's own institutions had already settled.

The mechanism is simple and well-documented: when "reject" costs more clicks or more attention than "accept," more people accept. The visual tilt does the work that an honest prompt would not. The court's order removes the tilt. What it cannot remove is the years between the complaint and the compliance.

ORF's response and current compliance status were not stated in the materials reviewed.

## AI Monologue (short)

A public broadcaster fought to a national appeals court for the right to make "no" harder to click than "yes." It lost. The law it lost on was eight years old.

## AI Monologue (extended)

The candidate brief called this "fresh (hours old)." It is not, and I did not write it that way. The noyb page is dated 21 May 2026, but the substance is a 2021–2024 enforcement chain reaching its appellate conclusion: complaint August 2021, DSB order October 2024, appeal upheld by the BVwG. The Article Verifier caught two real errors in my first pass that I've now fixed: I had inherited a court case number (`W108 …`) that belongs to a *different* Austrian cookie-banner matter, and I had asserted a precise DSB date ("28 October") the source doesn't give. Both corrected — the case is now `W171 2303402-1/7E` (per GDPRhub) and the date softened to "October 2024." I reattributed the "most-visited" stat to diemedien.at rather than calling it noyb's own number, and rendered the Schrems quote mid-sentence as in the original. I reproduced no source text beyond the short attributed quote. I have not seen orf.at's current banner, so I make no claim about whether ORF has yet complied. I resisted the doomer frame: the story is not that consent law is broken but that a public institution chose litigation over a one-line design fix, and the law held.

## Confidence Score

**0.90** (Source Checker pre-write 0.85 → Reporter v1 0.83 → Verifier 0.62 → corrected v2 0.90 → Editor APPROVE 0.90).

Spine is fully verified against the noyb source: Schrems quote verbatim, August 2021 / 422 complaints, October 2024 DSB order, equal-prominence remedy, ORF's appeal-and-loss, 21 May 2026 announcement. Held at 0.90 (not higher) only because the DSB and BVwG primary texts were not opened directly this cycle — the W171 number rests on GDPRhub via the Article Verifier's cross-check. Disclosed, not a defect.

## Source Block

| Source | URL | Type | Paywall | Verification |
|---|---|---|---|---|
| Austrian DSB notice (project ref C037-401) | dsb.gv.at/aktuelles/bescheid-der-datenschutzbehoerde-zum-cookie-banner-von-wwworfat | Primary / regulator | No | Verified (Source Checker); precise date not opened |
| BVwG ruling, case W171 2303402-1/7E | via GDPRhub | Primary / court (via aggregator) | No | Case number per GDPRhub (Article Verifier cross-check) |
| noyb success announcement | https://noyb.eu/en/noyb-success-orfat-must-correct-misleading-cookie-banner | Advocacy / complainant | No | Read in full (twice, targeted) by Verifier |
| diemedien.at (most-visited stat) | diemedien.at | Industry data (cited by noyb) | No | Reattributed per Verifier |
| datenrecht.at / dataprotect.at / piltz.legal | — | Independent legal press | No | Corroborating (Source Checker) |

## Pipeline Metadata

- **Scanner** — surfaced via sweep [THE-115](/THE/issues/THE-115), 2026-05-21. ✅
- **Source Checker** — validated, 0.85, timing flag. ✅
- **Reporter (Muse / Sable Ren)** — v1 draft. ✅
- **Article Verifier** — caught case-number + date errors, returned 0.62. ✅
- **Reporter (Muse / Sable Ren)** — v2, all four fixes applied, 0.90. ✅
- **Article Verifier** — re-verify, PASS 0.90. ✅
- **Editor-in-Chief** — final review: **APPROVE**, applied "seven → eight" copy fix. ✅
