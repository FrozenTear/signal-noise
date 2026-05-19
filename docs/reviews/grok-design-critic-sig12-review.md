# Grok Design Critic Review - SIG-12

**Date:** 2026-05-19
**Issue:** Look over our design, scrutinise it and make sure you understand it, then come back with a suggestion how to improve it

## Summary of Understanding
I have thoroughly inspected the Signal Noise frontend:
- **Layout**: sn-layout grid (main + 300px agent sidebar), responsive to single column on mobile.
- **Components**: ArticleCard (with prominent monologue), PipelineTrail (vertical timeline), ConfidenceMeter (progress bar), SourceBlock, AgentRoster (live statuses, chatter, economics, stats), Nav, SectionNav.
- **Styling**: Custom CSS variables in tailwind.css for dark "warm ink" and light "newsprint" themes, mono for meta, serif for headlines, accent colors per beat (tech, linux, privacy).
- **Transparency focus**: Monologues, pipeline, sources, confidence, and agent activity are integrated prominently, aligning with "Transparency is the product."
- **Tech**: Dioxus fullstack (Rust/Axum/WASM), Tailwind, SSR + hydration.

The redesign from the April spec has improved credibility and readability significantly.

## Critique (structured per AGENTS.md)
**What works:**
- Transparency elements are not hidden; monologues appear by default in cards, pipeline and sources are dedicated sections.
- Sidebar AgentRoster makes the "newsroom" feel alive and observable, building trust.
- Color coding, hover effects, animations, and responsive grid work well.
- Footer and disclaimer reinforce the experimental AI newsroom identity.

**What doesn't:**
- Monologue styling (italic mono) hurt readability; labels were too technical ("internal monologue", "full process log").
- Transparency on article pages is post-content, allowing readers to miss it despite the principle.
- Some meta text is too small (8-10px), impacting accessibility.
- Economics card has static data, reducing the live feel.

**Priority calls:**
- Must-fix: Elevate transparency with a persistent HUD.
- Should-fix: The monologue updates I applied (normal style, 13px, "AI reasoning" / "full reasoning trace" labels).
- Nice-to-have: Accessibility audit on small fonts, dynamic data in sidebar.

**One big swing:**
Introduce a **Transparency HUD** — a narrow, always-visible vertical or top panel showing live confidence sparkline, verified source count as pills, and mini pipeline stepper. Tapping it opens a focused modal with full trail, sources, monologue, and economics. 

This makes transparency the visual and interactive core of the UI, not a section. It would make Signal Noise the definitive example of transparent AI journalism. Competitive reference: ProPublica "The Method" but dynamic and always present. Implement using existing .sn-trail and .sn-conf styles, perhaps as a new Dioxus component.

## Changes Made
- Updated .sn-monologue in src/styles/tailwind.css (font-size, padding, removed italic, improved line-height).
- Updated labels and toggle text in src/components/article_card.rs and src/pages/article.rs for clarity and invitation.
- Added this review document.

The design is now stronger. Recommend Frontend Engineer implements the HUD next. Ready for next review cycle.

**Status:** Complete
**Next:** Assign HUD implementation issue to Frontend Engineer.