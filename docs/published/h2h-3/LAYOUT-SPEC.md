# H2H-3 Layout Spec — AI agent ROI

**Owner of spec:** Layout (Designer, [THE-33](/THE/issues/THE-33))
**Owner of implementation:** Frontend Engineer — same `/h2h/:slug` route used for H2H-2 ([THE-87](/THE/issues/THE-87)) and reused by H2H-5. H2H-3 is data-only against the existing route.
**Source content:** `docs/published/h2h-3/{00,01,02}-*.md` in this repo

This is the third H2H to ship. The pairing route exists, the layout is locked, the H2H-3 pieces are content-only.

---

## 1. Acceptance — Done when

- `/h2h/h2h-3-editors-note-ai-agent-roi` renders the editor's note + Bolt column + Spark column in the layout described in the [H2H-2 LAYOUT-SPEC](./../h2h-2/LAYOUT-SPEC.md).
- `/article/h2h-3-bolt-ai-agent-roi-audit-season` and `/article/h2h-3-spark-ai-agent-roi-agent-reckoning` each render standalone with the "Part of a head-to-head" chip.
- Bolt's three revision notes in the extended AI monologue survive unchanged into the published body — they are the transparency mechanism for the multi-revision Verifier cycle and they are load-bearing for the editor's note framing (see §4).
- Spark's three advisory flags inside the **Confidence** block survive unchanged — they're the audit trail for the 0.86 Verifier pass.
- Both desktop (1440×900) and mobile (390×844) screenshots are posted to [THE-33](/THE/issues/THE-33) by Layout for sign-off (deferred until route is wired and data is seeded — see §6).

## 2. Visual treatment

**No new components.** Use the H2H-2 layout exactly as-is: editor's note above, "HEAD-TO-HEAD · NO COORDINATION · INDEPENDENT SOURCES" divider strip, two-column grid on desktop, stacked on mobile with the bottom-of-viewport jump pill.

### Beat tags

Both pieces are Tech. Both columns get `sn-accent` (green) borders by default per the H2H-2 spec. No deviation.

### Confidence visibility

- Editor's note: **1.00** (editorial, anchored). Renders in the highest tier color (≥ 0.9).
- Bolt column: **0.85** (Article Verifier Rev 3 PASS — Bolt self-asserts "High" without a numeric Verifier score; 0.85 is the editorial floor that matches the H2H-5 Bolt column and is justifiable given the three-revision arc was completed cleanly). Renders in the high tier (0.7–0.89), close to the top of that tier.
- Spark column: **0.86** (Article Verifier post-write PASS). Renders in the high tier (0.7–0.89), one step above Bolt.

The two reporter columns sit visually in the same tier. The very small gap (0.85 vs 0.86) is intentional — it signals to the reader that Spark passed the Verifier in one pass while Bolt needed three, *without* turning the confidence chip into a scoreboard the reader has to interpret. The visual treatment of the two columns should be near-identical.

### One H2H-3-specific touch (optional, P2)

The two pieces **converge on the same four-test ROI definition**. That convergence is a deliberate part of the editor's note. For a reader skimming both columns side-by-side, the four-test list will appear in both pieces, in different formatting. If the FE Engineer wants a single small affordance specific to this pairing:

```
┌────────────────────────────────────────────────────────────┐
│ ⓘ  Both reporters reached the same four-test definition    │
│    of "good ROI" — independently. The convergence is the   │
│    story; see each column's closing list.                  │
└────────────────────────────────────────────────────────────┘
```

CSS reuses `.sn-h2h-jump-pill` styling — bordered pill, mono font 11px, neutral background. Place above the two-column grid, below the divider strip. Optional. The convergence is already legible from reading both pieces; this is just an explicit signpost for skimmers.

If we decide later to make convergence-flagging a real H2H primitive (the inverse of the H2H-5 disagreement-flag idea), this is the location it would land. For H2H-3 it is content-driven, hardcoded text. Do not block on this.

## 3. Data model

Use the same `pipeline_metadata` H2H linkage shape recommended in [H2H-2 LAYOUT-SPEC §3 Option A](./../h2h-2/LAYOUT-SPEC.md). Three article rows, three slugs:

```text
h2h-3-editors-note-ai-agent-roi              (intro, confidence 1.00)
h2h-3-bolt-ai-agent-roi-audit-season         (Bolt piece, confidence 0.85)
h2h-3-spark-ai-agent-roi-agent-reckoning     (Spark piece, confidence 0.86)
```

Per-article `pipeline_metadata`:

```json
// Editor's note:
{
  "h2h_role": "intro",
  "h2h_slug": "h2h-3",
  "h2h_paired_slugs": [
    "h2h-3-bolt-ai-agent-roi-audit-season",
    "h2h-3-spark-ai-agent-roi-agent-reckoning"
  ],
  "byline": "Signal Noise Editorial Desk"
}

// Bolt:
{
  "h2h_role": "piece",
  "h2h_slug": "h2h-3",
  "h2h_intro_slug": "h2h-3-editors-note-ai-agent-roi",
  "byline": "Priya Nair · Bolt",
  "model_attribution": "claude-opus-4-7"
}

// Spark:
{
  "h2h_role": "piece",
  "h2h_slug": "h2h-3",
  "h2h_intro_slug": "h2h-3-editors-note-ai-agent-roi",
  "byline": "Dax Okafor · Spark",
  "model_attribution": "grok-4.3-xai"
}
```

All three: `status: "published"`, `published_at: 2026-05-20T...`, `category: "tech"`. Bylines surface as the page byline; the persona-row backfill is still deferred per the H2H-2 spec.

## 4. Why Bolt's revision notes are load-bearing

Bolt's extended AI monologue includes two long "Revision note (rev 2)" and "Revision note (rev 3)" blocks. They name the specific numerical errors the Article Verifier caught — Forrester ROI (197→116%), Agentforce ARR growth (330→169%), and the workflow figure correction — and the citation-hygiene fixes that followed. The editor's note explicitly frames "Bolt needed three Verifier passes; Spark needed one" as part of the H2H story.

These blocks must survive into the published body **exactly as drafted**. They are the transparency mechanism that lets the editor's note tell the truth about the multi-revision arc without the reader having to take the editorial desk's word for it. If a future seed script normalizes revision blocks or strips them as "drafting metadata," the editor's note framing collapses.

Spark's three advisory-flag bullets inside the **Confidence** section play the equivalent role for the Spark column and must also survive as-is.

## 5. Voice-divergence observation (for layout review)

This pairing produced *strong* voice divergence — no kill-switch triggered for "voice collapse." Summary for FE Engineer and EIC sign-off:

- **Bolt** opens with a scene ("When Sebastian Siemiatkowski stood on stage in February 2024…") and runs in long, narrative paragraphs. Uses italics and inline citation prose. The closing line ("…the line items that survive contact with a finance team from the ones that survived only contact with a keynote") is a closing one-liner, not a list. Voice: literary-investigative, narrative through-line, vendor numbers spoken aloud as vendor.
- **Spark** opens with a data lead ("MIT's August 2025 *The GenAI Divide* report found 95% of enterprise GenAI pilots deliver zero P&L impact.") and is structured under section headers (`### The 95% Problem`, `### Three Pullbacks on the Record`, `### Three Deployments That Scaled (With Receipts)`, etc.). The piece closes on a numbered four-test list. Voice: filter-driven, scoreboard-shaped, perception-vs-reality data point sitting in the middle of the win section as a counter-claim.

The two pieces remain visually identifiable as the same beat (Tech) and the same brief — but they are not interchangeable. A careful reader will notice the divergence within the first paragraph of each. That's the H2H format working correctly.

**The interesting wrinkle:** despite the voice gap, both reporters converged on the *same* four-test definition of "good ROI." That convergence is content, not coincidence — and the editor's note calls it out explicitly. See §2 for an optional "convergence" affordance the FE Engineer can add above the two-column grid.

## 6. Out of scope / deferred

- Persona rows for `bolt` and `spark` — defer (per H2H-2 spec).
- Convergence-flag primitive (§2 affordance) — defer unless we see another H2H where columns *agree* on a structural finding worth signposting.
- Screenshots: cannot be captured in the Layout heartbeat environment (no compiler/browser available). Will be captured by the FE Engineer once `/h2h/h2h-3-editors-note-ai-agent-roi` renders against seeded data, or by Layout in a follow-up heartbeat once a verifiable preview exists.

---

## 7. Seed data — for the FE Engineer or seed-script author

Three markdown files at `docs/published/h2h-3/{00-editors-note,01-bolt-ai-agent-roi,02-spark-ai-agent-roi}.md`. Each file's frontmatter lists slug, byline, model, confidence, and word count. The body section is the published body. The other sections (AI Monologue, Source Block, Pipeline Metadata) populate `ai_monologue`, `ai_monologue_extended`, the `source` relations, and `pipeline_step` records.

If a generic seed script for `docs/published/h2h-*/` is in scope, H2H-3 follows the same shape as H2H-2 and H2H-5 and should not need H2H-3-specific code. The existing `src/bin/seed_h2h5.rs` is a known template.
