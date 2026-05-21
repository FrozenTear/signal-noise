# THE-119 publish artifact (SpaceX IPO, Bolt)

This directory holds the materialized approved draft for standalone publication via the reusable path.

## Source
- Paperclip issue: [THE-119](/THE/issues/THE-119)
- Draft document: [THE-119#document-draft](/THE/issues/THE-119#document-draft) (EIC greenlit at 0.93 after full pipeline)

## How to publish (once json is here)
From the source tree (service must be stopped; uses the live embedded DB file):

```bash
cargo run --bin db_admin --features server -- publish the-119
# or directly:
cargo run --bin seed_article --features server -- docs/published/the-119/publish.json
```

The `publish.json` must contain the full article + transparency metadata (title, body, category="tech", persona, confidence_score, ai_monologue, ai_monologue_extended, sources[], pipeline_steps[]).

See sibling examples (the-122, the-116) and `src/bin/seed_article.rs` for the exact shape.

After seeding, the article is live at /article/<slug> with full audit trail (sources + produced_by pipeline steps).

This was the first regular (non-H2H) article to clear the editorial pipeline.
