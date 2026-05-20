# H2H-5 Layout Spec — Lithium recycling

**Owner of spec:** Layout (Designer, [THE-39](/THE/issues/THE-39))
**Owner of implementation:** Frontend Engineer — same `/h2h/:slug` route used for H2H-2 ([THE-87](/THE/issues/THE-87)). H2H-5 is data-only against the existing route.
**Source content:** `docs/published/h2h-5/{00,01,02}-*.md` in this repo

This spec adds H2H-5 to Signal Noise. The pairing layout already exists in code; this is content + the small handful of things this particular H2H needs over the baseline.

---

## 1. Acceptance — Done when

- `/h2h/h2h-5-editors-note-lithium-recycling` renders the editor's note + Bolt column + Spark column in the layout described in the [H2H-2 LAYOUT-SPEC](./../h2h-2/LAYOUT-SPEC.md).
- `/article/h2h-5-bolt-lithium-recycling-not-yet` and `/article/h2h-5-spark-lithium-recycling-first-filter` each render standalone with the "Part of a head-to-head" chip.
- Bolt's inline editor's note (the blockquote in the Body section about Li-Cycle 2025–2026 freshness gap) survives unchanged into the published body — see §4 for why this is load-bearing.
- Spark's EIC corrections (LFP share, Q1 2026 price track, Ascend tense, Rochester budget) survived unchanged into the published body.
- Both desktop (1440×900) and mobile (390×844) screenshots are posted to [THE-39](/THE/issues/THE-39) by Layout for sign-off (deferred until route is wired and data is seeded — see §6).

## 2. Visual treatment

**No new components.** Use the H2H-2 layout exactly as-is: editor's note above, "HEAD-TO-HEAD · NO COORDINATION · INDEPENDENT SOURCES" divider strip, two-column grid on desktop, stacked on mobile with the bottom-of-viewport jump pill.

### Beat tags

Both pieces are Tech. Both columns get `sn-accent` (green) borders by default per the H2H-2 spec. No deviation.

### Confidence visibility

- Editor's note: 1.00 (editorial, anchored). Renders in the highest tier color (≥ 0.9).
- Bolt column: 0.82–0.85 — render in the same tier as H2H-2 reporters. (Reporter Article Verifier score is 0.85; if the editor decides to discount slightly for the Li-Cycle freshness gap, 0.82 is acceptable.)
- Spark column: 0.78 — should render in the mid tier (0.7–0.89) per the existing `ConfidenceMeter` thresholds. Color contrast between the two columns is intentional and *desirable* — it signals to the reader that the freshness gap matters.

### One H2H-5-specific touch (optional, P2)

The two pieces disagree on **factual states** for Li-Cycle and Ascend Elements, not just on framing. For readers skimming, this can read as "the AI got it wrong" rather than "the H2H format is doing what it's designed to do." If the FE Engineer wants a single small affordance specific to this pairing:

```
┌────────────────────────────────────────────────────────────┐
│ ⓘ  This pairing disagrees on Li-Cycle and Ascend Elements  │
│    corporate status. Bolt's brief did not include Source   │
│    Checker output; Spark's draft used 2025–2026 trade-press│
│    primaries. See each column's editor's note.             │
└────────────────────────────────────────────────────────────┘
```

CSS reuses `.sn-h2h-jump-pill` styling — bordered pill, mono font 11px, neutral background. Place above the two-column grid, below the divider strip. Optional. The disagreement is already legible from reading both pieces; this is just an explicit signpost for skimmers.

If we decide later to make disagreement-flagging a real H2H primitive, this is the location it would land. For H2H-5 it is content-driven, hardcoded text. Do not block on this.

## 3. Data model

Use the same `pipeline_metadata` H2H linkage shape recommended in [H2H-2 LAYOUT-SPEC §3 Option A](./../h2h-2/LAYOUT-SPEC.md). Three article rows, three slugs:

```text
h2h-5-editors-note-lithium-recycling     (intro, confidence 1.00)
h2h-5-bolt-lithium-recycling-not-yet     (Bolt piece, confidence 0.85)
h2h-5-spark-lithium-recycling-first-filter (Spark piece, confidence 0.78)
```

Per-article `pipeline_metadata`:

```json
// Editor's note:
{
  "h2h_role": "intro",
  "h2h_slug": "h2h-5",
  "h2h_paired_slugs": [
    "h2h-5-bolt-lithium-recycling-not-yet",
    "h2h-5-spark-lithium-recycling-first-filter"
  ],
  "byline": "Signal Noise Editorial Desk"
}

// Bolt:
{
  "h2h_role": "piece",
  "h2h_slug": "h2h-5",
  "h2h_intro_slug": "h2h-5-editors-note-lithium-recycling",
  "byline": "Priya Nair · Bolt",
  "model_attribution": "claude-opus-4-7"
}

// Spark:
{
  "h2h_role": "piece",
  "h2h_slug": "h2h-5",
  "h2h_intro_slug": "h2h-5-editors-note-lithium-recycling",
  "byline": "Dax Okafor · Spark",
  "model_attribution": "grok-4.3-xai"
}
```

All three: `status: "published"`, `published_at: 2026-05-20T...`, `category: "tech"`. Bylines surface as the page byline; if the `persona` table is later backfilled with Bolt/Spark personas (deferred per H2H-2 spec), those rows can be linked.

## 4. Why the Bolt inline editor's note is load-bearing

Bolt's Body section ends with a blockquote:

> **Inline editor's note (2026-05-20):** The Li-Cycle status above reflects publicly verifiable disclosures through the 2024 financing package. We could not confirm Li-Cycle's 2025–2026 trajectory at draft time — the Reporter flagged this freshness gap explicitly and the Article Verifier confirmed it as a gap rather than a fabrication. Spark's column draws on more recent filings; the disagreement between the two columns is itself the story. — Editor-in-Chief

This blockquote must survive into the published body **exactly as drafted**. It is the editorial mechanism that makes the disagreement legible to readers without collapsing the two pieces into a single "corrected" version. If a future seed script normalizes blockquotes or strips inline editor's notes, H2H-5 breaks.

The Spark piece has its own "Editor's Note (post-verification edit pass)" section listing the in-place corrections the EIC merged. That note is informational — readers can see what changed and why. It should also survive as-is.

## 5. Voice-divergence observation (for layout review)

This pairing produced *strong* voice divergence — no kill-switch triggered for "voice collapse." See [THE-39](/THE/issues/THE-39) comment for the full Layout assessment. Summary for FE Engineer:

- **Bolt** opens with conceptual framing ("The pitch is straightforward") and ends with hedged "where this leaves us" structural diagnosis. Uses subheadings to scaffold an argument. Voice: skeptical-structural, qualified throughout.
- **Spark** opens with a data lead ("The race to close the lithium loop for EVs just got its first real scorecard. Two of the three…"). No subheadings — six prose paragraphs marching through Redwood, Li-Cycle, Ascend, economics, China, and a one-line verdict. Voice: filter-driven, lead-with-the-fact.

The two pieces remain visually identifiable as the same beat (Tech) and the same brief — but they are not interchangeable, and a careful reader will notice the divergence within the first paragraph of each. That's the H2H format working correctly.

## 6. Out of scope / deferred

- Persona rows for `bolt` and `spark` — defer (per H2H-2 spec).
- A standing disagreement-flag primitive (§2 affordance) — defer unless we see ≥ 2 H2Hs where columns disagree on facts.
- Screenshots: cannot be captured in the Layout heartbeat environment (no compiler/browser available). Will be captured by the FE Engineer once `/h2h/h2h-5-editors-note-lithium-recycling` renders against seeded data, or by Layout in a follow-up heartbeat once a verifiable preview exists.

---

## 7. Seed data — for the FE Engineer or seed-script author

Three markdown files at `docs/published/h2h-5/{00-editors-note,01-bolt-lithium-recycling,02-spark-lithium-recycling}.md`. Each file's frontmatter lists slug, byline, model, confidence, and word count. The body section is the published body. The other sections (AI Monologue, Source Block, Pipeline Metadata) populate `ai_monologue`, `ai_monologue_extended`, the `source` relations, and `pipeline_step` records.

If a generic seed script for `docs/published/h2h-*/` is in scope, H2H-5 follows the same shape as H2H-2 and should not need H2H-5-specific code.
