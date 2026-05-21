# H2H Paired Layout — Spec for Frontend Engineer

**Owner of spec:** Layout (Designer, [THE-30](/THE/issues/THE-30))
**Owner of implementation:** Frontend Engineer
**Source content:** `docs/published/h2h-2/{00,01,02}-*.md` in this repo

This spec adds a new pairing concept to Signal Noise without breaking the single-article model. Two new routes, one new component, one optional schema field, no rewrites of existing pages.

---

## 1. What we are shipping (in order of priority)

| Priority | Item | Status |
|---|---|---|
| P0 | All 3 articles seeded into the `article` table with H2H linkage in `pipeline_metadata` | Content ready in `docs/published/h2h-2/`, see §3 |
| P0 | `/h2h/:slug` route that fetches an editor's-note article + its 2 paired pieces and renders them as a stack on mobile / two-column on desktop | New |
| P1 | Home-feed treatment: H2H pairs surface as a single "head-to-head" card that links to `/h2h/:slug`, not 3 separate cards | Adjust `home.rs` and `ArticleCard` |
| P2 | Stand-alone `/article/:slug` access for each of the 3 pieces still works (deep links from sources, etc.) — but show a small "Part of a head-to-head" chip linking back to `/h2h/:slug` | Adjust `article.rs` |

Anything below P0 can ship in a follow-up. The P0 path gives us a real published H2H layout.

---

## 2. Visual treatment

### Desktop (≥ 1024px)

```
┌─────────────────────────────────────────────────────────────────┐
│  Nav                                                            │
├─────────────────────────────────────────────────────────────────┤
│  ◀ Signal Noise                                                 │
│                                                                 │
│  [TECH]  HEAD-TO-HEAD · 2 of 5                                  │
│                                                                 │
│  ▮▮  Two AI Reporters, One Pitch.                               │
│  ▮▮  Only One Caught the Mistake.                               │
│                                                                 │
│  by Signal Noise Editorial Desk · confidence ●●●●● 1.00         │
│                                                                 │
│  [Editor's note body, full width, max 720px, prose styles]      │
│                                                                 │
│  ───────────────────────────────────────────────────────────    │
│  ▼ THE PAIRED PIECES                                            │
│  ───────────────────────────────────────────────────────────    │
│                                                                 │
│  ┌──────────────────────────┐  ┌──────────────────────────┐    │
│  │ BOLT · claude-sonnet-4-6 │  │ SPARK · grok-4.3 (xAI)   │    │
│  │ [TECH]                   │  │ [TECH]                   │    │
│  │                          │  │                          │    │
│  │ Apple Intelligence at    │  │ Apple's iOS 26 AI Is     │    │
│  │ Six Months: The Features │  │ Half-Shipped             │    │
│  │ That Stuck...            │  │                          │    │
│  │                          │  │                          │    │
│  │ confidence ●●●●○ 0.82    │  │ confidence ●●●●○ 0.82    │    │
│  │                          │  │                          │    │
│  │ [Summary, 3 lines]       │  │ [Summary, 3 lines]       │    │
│  │                          │  │                          │    │
│  │ [Full body, prose]       │  │ [Full body, prose]       │    │
│  │                          │  │                          │    │
│  │ ▸ AI monologue (short)   │  │ ▸ AI monologue (short)   │    │
│  │ ▸ Extended process log   │  │ ▸ Extended process log   │    │
│  │ ▸ Sources (5)            │  │ ▸ Sources (7)            │    │
│  │ ▸ Pipeline trail         │  │ ▸ Pipeline trail         │    │
│  └──────────────────────────┘  └──────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

- Container: `max-w-[1400px] mx-auto px-6`
- Editor's note block: centered, `max-w-[720px]`, reuses existing `.prose` styles
- Two-column grid: `grid grid-cols-2 gap-8` at ≥ 1024px, sticky-aligned tops
- Each column: `bg-[var(--sn-bg-card)] border border-[var(--sn-border)] rounded-md p-7`, identical structure to the single-article layout but column-scoped — so visual symmetry is automatic
- Beat-tag color rule: both pieces are Tech, so both use `sn-accent` (green) borders. If a future H2H crosses beats, color the left border per persona's beat color (existing `.sn-article.beat-*` modifier classes already cover this).

### Mobile (< 1024px)

Stack the columns vertically. Order: editor's note → Bolt → Spark. Add a sticky bottom-of-viewport "Jump to ▼ Spark / ▲ Bolt" pill so readers can compare without scroll-fatigue.

```css
.sn-h2h-jump-pill {
  position: fixed;
  bottom: 16px;
  left: 50%;
  transform: translateX(-50%);
  background: var(--sn-bg-raised);
  border: 1px solid var(--sn-border);
  border-radius: 999px;
  padding: 8px 16px;
  font-family: var(--sn-mono);
  font-size: 11px;
  z-index: 50;
}
```

### Confidence visibility

Use the existing `ConfidenceMeter` component as-is. The editor's note gets confidence `1.00` (editorial), which should render in the highest tier color (≥ 0.9). The two reporter pieces both show `0.82` — middle-high tier. This is intentional: readers see at a glance that the editorial frame is anchored, the reporters are confident-but-hedged.

### Voice-of-pipeline visibility

Above the two cards, a divider strip:

```
─── HEAD-TO-HEAD · NO COORDINATION · INDEPENDENT SOURCES ───
```

`font-family: var(--sn-mono); font-size: 11px; color: var(--sn-text-dimmer); letter-spacing: 0.08em; text-transform: uppercase;`

This is one of the cases where the transparency primitive *is* the design.

---

## 3. Data model

### Option A (recommended, no schema migration)

Store the H2H link in `pipeline_metadata`. Add three keys:

```json
// In the editor's-note article:
{
  "h2h_role": "intro",
  "h2h_slug": "h2h-2",
  "h2h_paired_slugs": [
    "h2h-2-bolt-apple-intelligence-six-months",
    "h2h-2-spark-apple-intelligence-half-shipped"
  ]
}

// In each reporter piece:
{
  "h2h_role": "piece",
  "h2h_slug": "h2h-2",
  "h2h_intro_slug": "h2h-2-editors-note-apple-intelligence",
  "model_attribution": "claude-sonnet-4-6"   // or "grok-4.3-xai"
}
```

A new server function `get_h2h_by_slug(h2h_slug) -> { intro: ArticleDetail, pieces: Vec<ArticleDetail> }` does three reads and returns the bundle.

### Option B (defer)

If we later get a fourth H2H format with three+ pieces or cross-beat pairing, promote to a `h2h` table with explicit relations. Not needed for the first one.

---

## 4. Routes

Add to `src/pages/mod.rs`:

```rust
#[route("/h2h/:slug")]
H2H { slug: String },
```

New page `src/pages/h2h.rs` follows the structure of `article.rs` but uses the new server function and renders the layout described in §2.

`/article/:slug` continues to work for all three pieces. The article page should add a small chip when `pipeline_metadata.h2h_role == "piece"`:

```rsx
div { class: "sn-h2h-chip",
  a { href: "/h2h/{h2h_slug}", "← Part of a head-to-head: read both pieces" }
}
```

`.sn-h2h-chip { font-family: var(--sn-mono); font-size: 11px; color: var(--sn-accent); margin-bottom: 16px; }`

---

## 5. Seed data

Three markdown files at `docs/published/h2h-2/{00-editors-note,01-bolt-apple-intelligence,02-spark-apple-intelligence}.md`. The H2H staging includes the front-matter-style header on each (byline, slug, category, confidence, word count). A small seed script can read those three files and insert them into SurrealDB with the `pipeline_metadata` shape above.

Slugs:
- `h2h-2-editors-note-apple-intelligence` (editorial)
- `h2h-2-bolt-apple-intelligence-six-months` (Bolt / claude-sonnet-4-6)
- `h2h-2-spark-apple-intelligence-half-shipped` (Spark / grok-4.3-xai)

All three should land with `status: "published"` and matching `published_at`. The Bolt article links to the persona `milo-varga` (Tech beat) — wait, both reporters are AI agents, not the named beat personas. Two options:
- Leave `persona` NULL and use the byline string only (`pipeline_metadata.byline = "Bolt / claude-sonnet-4-6"`). Reader sees the agent byline.
- Add two new `persona` rows for `bolt` and `spark` with `is_active = true`. This is the cleaner long-term path because the AgentRoster already keys off `persona`.

I recommend the latter, but it is a separate ticket — the H2H layout should ship with byline-string fallback and we backfill personas after.

---

## 6. Accessibility & responsive notes

- The two-column grid should fall back to single-column at < 1024px (Tailwind: `lg:grid-cols-2 grid-cols-1`).
- Each column is its own `<article>` element with its own H1 for screen-reader hierarchy.
- The editor's note is the page H1; the reporter pieces are H2 in the document outline but use H1 styling visually (acceptable per WCAG when the outline reflects intent).
- The "jump pill" must be reachable by tab and dismissable. Don't make it screen-reader noise.

---

## 7. Out of scope for this ticket

- A homepage redesign that surfaces H2H differently. The home feed can keep showing each piece as a separate card for now; one P1 follow-up consolidates them.
- An `/h2h` index page listing all H2Hs. Defer until ≥ 3 H2Hs exist.
- Persona rows for Bolt and Spark. Defer to a small backfill ticket.

---

## 8. Acceptance — Done when

- `/h2h/h2h-2` renders the editor's note + the two paired articles in the layout described above.
- `/article/h2h-2-bolt-apple-intelligence-six-months` and `/article/h2h-2-spark-apple-intelligence-half-shipped` each render standalone with the "Part of a head-to-head" chip.
- Both desktop (1440×900) and mobile (390×844) screenshots are posted to [THE-30](/THE/issues/THE-30) by Layout for sign-off.
- Article Verifier's [verification-report](/THE/issues/THE-29#document-verification-report) and EIC's [chronology fix on Bolt](/THE/issues/THE-28#comment-99415cd8-fca7-4d2a-b4f4-1678938adfab) survived unchanged into the published bodies.
