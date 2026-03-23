# Signal Noise

AI-powered transparent news site. The journalism is real, the journalists are artificial, and everyone knows it. Transparency is the product.

## Board Direction (2026-03-23)

- **Beats at launch**: Linux, Tech, Privacy (expand after stable in ~2 weeks)
- **Agent architecture**: Full Paperclip agents, no scripts. Scanner/Fact Checker/Reporter/Editor pipeline.
- **AI-first editorial**: Embrace AI fully. Board injects as editor only if AI fails completely.
- **Frontend**: Dioxus fullstack (Rust WASM + Axum). No static site fallback.
- **Database**: SurrealDB embedded (Rust SDK). Graph edges, LIVE SELECT, multi-model.
- **Revenue**: Merch-first, then newsletter sponsorship, Clipmart template, sponsored coverage.
- **Staffing**: Option 1+2 — Founding Engineer on repo reset, CEO hires full roster + dedicated frontend agent.

## Current State

- **Project**: `1d41b810` in Paperclip, workspace at `/home/pure/signal-noise`
- **Plan**: Document key `plan` on SIG-2, rev 6 (SurrealDB from day one)
- **Active tasks**: SIG-8 (repo reset, critical), SIG-9 (SurrealDB schema, high) — both assigned to Founding Engineer
- **Team**: All 7 agents active. EiC manages Scanner/Fact Checker/Reporter. CEO manages EiC/Frontend Engineer/Founding Engineer.

## Key References

- Company goal ID: `6efa1d2d-3c6a-47f1-bc88-f6fb99fb2042`
- Project ID: `1d41b810-6402-48db-b091-53778e442636`
- Pitch issue: SIG-2 (`fb0de337-dc28-4fb0-97a3-4460af1c7ae6`)
- Pitch document key: `signal-noise-pitch-1` on SIG-2
