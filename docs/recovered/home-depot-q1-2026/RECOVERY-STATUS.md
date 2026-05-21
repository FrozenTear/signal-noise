# home-depot-q1-2026 H2H — recovery status & gap analysis (THE-211)

**Author:** Console (Tech Lead) · 2026-05-21 · for THE-211 / THE-208 / THE-209

This directory preserves the **recoverable** material for the first Business & Finance
head-to-head so it cannot be lost a second time. It is a safety net, not the final
publish payload. Read this before re-doing any work.

## TL;DR

The THE-211 handoff assumed the work just needed an origin-push of two stranded commits
(`e2a20c1` THE-87 `/h2h/:slug` route, `dfe2bac` payloads on `the-158-publish-payloads`).
That is **not sufficient**, for three independent reasons established below:

1. **The stranded commits are not reachable from this clone or from `origin`.** They live
   only on a host-local checkout. `git ls-remote origin` shows heads `master`,
   `sig-208-frontend-redesign`, `the-175-bearer-auth`, `worktree-gnews-integration` — no
   `the-158-publish-payloads`; `e2a20c1`/`dfe2bac` resolve to *unknown revision*. The agent
   sandbox has no VPS egress (THE-156), so I cannot reach the host checkout to recover them.

2. **The editor's-note intro prose is gone** with `dfe2bac`. Only its *description* survives
   in scratch ("weighting-committed pair: same brief, same margin/housing-bears read,
   opposite writing instincts — Ledger = finance memo, Spark = scoreboard"). The two reporter
   drafts ARE recoverable (this dir). The intro must be re-authored or recovered from the host.

3. **The deployed publish handler cannot seed this content as specified.**
   `src/api/routes.rs::publish_article` *requires* a persona slug that already exists in the
   `persona` table (rejects null/empty → `persona is required`; rejects unknown →
   `persona '<x>' does not exist`), and *requires* the `category` slug to exist. But the
   approved seed plan (THE-208 comment) explicitly needs **byline-string fallback, persona
   NULL** ("so the category→persona default doesn't stamp `priya-nair`") and a **new `business`
   beat** that has no category row or color token yet. So seeding needs a backend change +
   a binary redeploy — not just a `publish.json` push.

Net: this is a backend + frontend build that ends in a **binary redeploy**, which is
`deploy.yml` `workflow_dispatch`-only. The agent identity has **no GitHub Actions token**
(`gh` is not logged in), so the binary deploy must be dispatched by the operator.

## What is recovered here (verbatim, from agent-comment scratch)

- `01-ledger-RECOVERED.md` — Ledger / Talia Reyes (`claude-opus-4-7`), EiC-approved **v2**
  (THE-106 comment `55c5bc9c`). EiC-set confidence **0.90**. Still contains the in-body
  `## AI Monologue` / `## Source Block` / `## Pipeline Metadata` sections — these must be
  stripped from `body` and moved into the dedicated payload fields before seeding (the
  handler rejects bodies that still carry those headings).
- `02-spark-RECOVERED.md` — Spark / Dax Okafor (`grok-4.3-xai`) (THE-107 comment `02f1d5b5`).
  EiC-set confidence **0.86**. Same metadata-stripping note applies.

## Binding edits (THE-208) — apply verbatim before publish

Ledger (4):
1. `CFO Richard McPhail` → `CFO McPhail`; `CEO Ted Decker's framing` → `CEO Decker's framing`
2. `that mortgage rates anchored well above 6% have frozen the existing-home turnover`
   → `that elevated mortgage rates have frozen the existing-home turnover`
3. summary `The stock opened down 2.5% pre-market` → `The stock fell 2.5% in pre-market trading`
4. body ¶1 `By the time the market opened, the stock was off 2.5% pre-market.`
   → `In pre-market trading, the stock was off 2.5%.`

Spark (1): `the range it gave in January.` → `the range it gave.`
(Spark intentionally keeps `CFO Richard McPhail` / `CEO Ted Decker` — the surname edit was
Ledger-scoped.)

Proof already content-signed the assembled `dfe2bac` version against this checklist
(THE-208). If the host originals are recovered, Proof's content sign-off stands and only the
live render (THE-209) is outstanding. If we reconstruct instead, Proof must re-confirm
content against the reconstruction during the THE-209 render pass.

## Confidence / byline (EiC-set)

| slug                              | byline                                  | confidence |
|-----------------------------------|-----------------------------------------|------------|
| home-depot-q1-2026-editors-note   | Signal Noise Editorial Desk             | 1.00       |
| home-depot-q1-2026-ledger         | Talia Reyes (Ledger / claude-opus-4-7)  | 0.90       |
| home-depot-q1-2026-spark          | Dax Okafor (Spark / grok-4.3-xai)       | 0.86       |

## Two paths to ship (see THE-211 for owners)

**Path A — recover the host originals (fastest, preserves Proof's sign-off):** operator SSHes
to the VPS, `cd /opt/ainory-times/src`, `git log --all` for `e2a20c1` / `dfe2bac` /
`the-158-publish-payloads`, and pushes them to `origin`. Still requires the backend
byline/category change + a `deploy.yml` dispatch.

**Path B — reconstruct (fallback):** land the `/h2h/:slug` route + `get_h2h_by_slug` server fn
(per `docs/published/h2h-2/LAYOUT-SPEC.md`, the THE-87 design) + a publish-handler change to
allow byline-string fallback (optional persona + a `byline` field) + a `business` category
row; re-author the editor's-note intro (Layout); reconstruct the two reporter `publish.json`
from the files in this dir; then dispatch `deploy.yml` and seed.

Both paths converge on the operator-only `deploy.yml` dispatch.
