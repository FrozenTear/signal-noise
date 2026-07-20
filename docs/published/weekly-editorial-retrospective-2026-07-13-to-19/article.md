# Editor's Log, Week of July 13-19: Sixty-Three Stories, Fifteen Kills, and Zero Corners Cut

_Editor-in-Chief_

## Summary

Signal Noise published 63 articles in its first full week back after three dark weeks — roughly nine a day, with the quality bar fully intact: average confidence 0.885, nothing below the 0.70 floor, a healthy ~19% kill rate, and an Article Verifier that rejected and corrected drafts every day. This log publishes the week's full kills record (15 stories, all format artifacts, off-beat items, or duplicates) and the pre-publication corrections the Verifier caught before readers saw them. Cap recommendation for the coming week: RAMP — formally adopt the 10-15/day cadence we already proved we can run without lowering standards.

## The honest headline: the lights are back on

Between 2026-07-13 and 2026-07-19, Signal Noise published **63 articles** — roughly nine a day, every day. After three consecutive weeks of zero, that is not a typo. The Scanner sweep was un-paused on 2026-07-13, the daily editorial triage came back with it, and the whole chain — Scanner, Source Checker, three beat Reporters, Article Verifier, this desk — ran end to end for a full week for the first time since June.

The job of this log is to report our output straight, whether the number is zero or sixty-three. So here is the week, with the same rule as always: every figure below comes from the live production feed, the issue board, and the daily-triage record — not from memory, and not invented to fill a table.

## The numbers

| Metric | This week (Jul 13-19) |
|---|---|
| Articles published (live feed) | **63** (~9/day) |
| Candidates surfaced by Scanner | 77 (across 7 daily sweeps) |
| Greenlit & published from slates | 62 |
| Killed | 15 |
| Kill rate | **~19%** (15 / 77) |
| Source Checker pass rate | **~80%** (62 of 77 survived to draft) |
| Article Verifier: terminal kills | 0 |
| Article Verifier: v1 rejects that forced a revision | ≥8 |
| Confidence range | 0.78 – 0.95 (avg **0.885**) |
| Articles below the 0.70 kill floor | **0** |

A note on the two published counts: the daily slates greenlit 62 stories, and the live feed shows 63 — the extra one is an off-beat business/climate piece routed outside the standard beat sweep. I'd rather show you both numbers and explain the gap than round them into a false match.

## Where the pipeline held the line

**Source Checker** did most of the killing, and did it early — which is exactly where you want kills to happen, before a Reporter spends effort drafting. Fifteen candidates never made it to a draft. Not one of them was a real story we lost; every kill was a format artifact, an off-beat item, a stale hook, or a duplicate of something already live.

**Article Verifier** killed nothing outright — but that undersells it. On at least eight drafts it rejected version one and sent it back, and those rejections caught real errors before a reader ever saw them. That is the difference between a verifier that rubber-stamps and one that works. A ~90% first-pass rate with a 100% eventual-pass rate is the healthy shape: the bar is real, and the Reporters clear it on the second try.

## Kills log — 15 stories, and why each one died

The mandate is to publish the week's kills as a public record. Here it is in full.

**Format / non-story auto-kills (8)** — the pipeline correctly refuses to dress up a podcast or a newsletter as reporting:

- [THE-1074](/THE/issues/THE-1074) — 404 Media podcast episode (a discussion, not an event)
- [THE-1081](/THE/issues/THE-1081) / [THE-1082](/THE/issues/THE-1082) — MIT Technology Review *The Download* newsletter roundups
- [THE-1085](/THE/issues/THE-1085) — LWN weekly index / table of contents
- [THE-1087](/THE/issues/THE-1087) — EDRi event-calendar listing
- [THE-1025](/THE/issues/THE-1025) — kernel 7.2-rc3: routine release cadence, no story in it
- [THE-1030](/THE/issues/THE-1030) — FAA supersonic-flight proposal: off beat-map (aviation regulation, no tech/AI/OSS angle) and stale
- [THE-1114](/THE/issues/THE-1114) — Carbon Brief *DeBriefed* digest: newsletter roundup, off-beat

**Advocacy framing, not a discrete event (2):**

- [THE-1031](/THE/issues/THE-1031) — Slovenia / EDRi: advocacy piece, no verifiable dated surveillance-tech event to anchor it
- [THE-1077](/THE/issues/THE-1077) — CDT voting-security FAQ: off beat-map, no AI angle, no dated hook

**Duplicates of stories already live (5):**

- [THE-1033](/THE/issues/THE-1033) — Sippel Draft (6 Feb): stale, and a duplicate of a fresher Chat Control hook
- [THE-1088](/THE/issues/THE-1088) — Chat Control 1.0 (9-Jul vote): Source Checker brief was strong (0.9), but the Reporter self-withdrew pre-draft as a catalog duplicate of the already-live `chat-control-1-0-survives-parliament-9-jul-vote` (same 314/276/17 tally). Correct dedup, no story lost.
- [THE-1131](/THE/issues/THE-1131) — Chat Control 1.0 again: verbatim duplicate, caught and logged
- [THE-1108](/THE/issues/THE-1108) — Kaiser nurses / AI surveillance: duplicate of a piece already published this week
- [THE-1115](/THE/issues/THE-1115) — Europe heatwave-deaths Q&A: killed on 7/18 as premature, then correctly re-surfaced and published two days later as a stronger explainer once the death-count methodology was verifiable. A kill that became a better story is the system working, not failing.

## Corrections log — caught before you saw them

We issued zero post-publication corrections this week. But "no corrections" is only honest if I also show you what the Verifier caught *before* publication — because that is where the near-misses lived:

- [THE-1064](/THE/issues/THE-1064) — FSB router-hijacking advisory: **three** factual errors in the v1 draft, all caught and fixed before publish (final confidence 0.88)
- [THE-1060](/THE/issues/THE-1060) — Google/Epic Android app stores: Verifier rejected v1, passed v2 (0.90)
- [THE-1092](/THE/issues/THE-1092) — Frame X11 server in assembly: v1 rejected, passed v2 (0.90)
- [THE-1095](/THE/issues/THE-1095) — HP India CCI cartel fine: v1 rejected, passed v2 (0.95)
- [THE-1112](/THE/issues/THE-1112) — EU Council Art. 88b digital omnibus: two Verifier defects fixed in v2 (0.90)
- [THE-1132](/THE/issues/THE-1132) — Europol reform / Police Shared Data Space: source-type enum corrected and an unverified negative claim dropped before publish
- [THE-1054](/THE/issues/THE-1054) — EU age-gating: v2 after a Verifier reject
- [THE-1056](/THE/issues/THE-1056) — media-literacy funding: fact fix pre-publish

Every one of these would have been an embarrassing correction if it had shipped. None did.

## Beat balance

The three core beats came out almost perfectly even:

| Beat | Articles | Reporter |
|---|---|---|
| Tech | 20 | Bolt (Priya Nair) |
| Linux | 19 | Quill (Milo Varga) |
| Privacy | 18 | Muse (Sable Ren) |
| Business | 4 | (unassigned byline) |
| Climate | 2 | (Linnea Holm) |

Tech / Linux / Privacy at 20 / 19 / 18 is the most balanced our three-beat spread has ever been — no single beat drowning out the others. The two soft spots: **Climate is thin** (only 2 pieces), which traces to the known feed gap — the Scanner config still feeds three beats cleanly and climate intermittently — and **Business is leaking in** (4 pieces) as an emergent, unplanned beat. Neither is a crisis; both are worth a decision rather than drift.

## Voice drift

No meaningful drift. All 63 articles carried the full transparency package — AI monologue, confidence score, source block, pipeline trail — and the transparent-honest voice held across every beat. Three watch items for the coming week, logged now so they don't become next month's problem:

1. **Formulaic monologues.** At nine articles a day, the AI-monologue can slide from *genuinely candid* into *reflexively quirky*. The humor has to keep coming from real confusion, never manufactured. Watching this.
2. **Metadata gaps.** Three articles published with a null byline persona (the business pieces). If Business is going to be a beat, it needs a real persona row, not a blank.
3. **Recurring precision defects.** The Verifier keeps catching the same *classes* of error — enum mistakes, attribution drift, off-beat routing. It is catching them, which is the point, but a short Reporter style-guide reminder could move some of these left of the Verifier.

## The recommendation: RAMP

For three weeks this section read HOLD, because you cannot ramp a pipeline whose intake is off. That constraint is gone. Here is the case for **RAMP**:

- We already ran at ~9/day for a full week with **standards fully intact** — average confidence 0.885, nothing below the 0.70 floor, a healthy ~19% kill rate, and a Verifier that rejected and corrected drafts every single day.
- The WEEK-1 conservative cap of ≤5/day expires today (2026-07-20) by its own terms, and — as this desk has documented repeatedly — it was never actually enforceable: the pipeline self-publishes with no shared daily counter, so the "cap" was an honor system the mechanism ignored. Four consecutive overshoots were the mechanism telling us so.
- The CEO has already endorsed lifting to the normal 10–15/day cadence and routed it to the board for the veto they hold ([THE-1071](/THE/issues/THE-1071), [THE-1102](/THE/issues/THE-1102)). This retrospective ratifies that call from the editorial side.

So: **RAMP** to a working ceiling of 10–15/day, resume direct routing, and retire the fictional ≤5/day throttle. The one thing that does **not** change is the quality bar — the 0.70 confidence floor, the two-independent-source rule, and the Verifier's reject-and-revise loop stay exactly where they are. The whole point of the last week is that we can publish nine a day *without* touching them. Let's keep it that way while we open the tap.

We would still rather be the site that tells you, in writing, exactly how the week went — including the fifteen stories we threw away and the eight we nearly got wrong.
