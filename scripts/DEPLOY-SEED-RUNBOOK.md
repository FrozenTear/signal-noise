# VPS-reachable deploy + seed (THE-157)

## Problem
The agent sandbox's egress to `194.163.163.153` (news.scuffedcrew.no) is IP-blocked
(rejected in ~5 ms; every other host connects — see THE-156, re-confirmed 2026-05-21).
So the live seed (THE-153) and publish-deploy (THE-125) cannot run from the sandbox.

## Solution: seed over the public HTTP API from a GitHub runner
GitHub-hosted runners are not the agent sandbox and reach the VPS normally.

The Dioxus server nests the API at `/api` on the same public bind (`src/main.rs`),
and `POST /api/articles` is an idempotent **UPSERT-by-slug** that runs inside the
running server — so the server itself owns the single-writer SurrealKV handle.
Therefore seeding needs **no SSH and no `systemctl stop/restart`**: we just POST each
approved `publish.json` to `https://news.scuffedcrew.no/api/articles`.

`.github/workflows/deploy-seed.yml`:
- `workflow_dispatch` with `mode` = `probe` (read-only live check, default) or `seed`.
- `push` to `master` touching `docs/published/**/publish.json` → auto `seed`.
- `scripts/seed-live.sh` does the work; `probe` mode is also a **reachability bridge**.
- Each run commits its result to `docs/seed-status/last-run.md` (`[skip ci]`, a
  non-trigger path) so an agent can verify the outcome by pulling the repo — no
  Actions-API token and no VPS reachability needed.

## Auth (changed by THE-159 hardening)
Mutating `/api/*` routes are now **failure-closed bearer-gated** (`src/api/auth.rs`):
the server reads `SEED_API_TOKEN` from its environment and requires
`Authorization: Bearer <SEED_API_TOKEN>` on writes (503 if the server's token is
unset, 401 on mismatch). `seed-live.sh` sends that header from `$SEED_API_TOKEN`.

## The single irreducible operator action
**Provision one shared `SEED_API_TOKEN` value in TWO places (same string):**
1. **The live server's env** (the `signal-noise`/`ainory-times` systemd unit on the
   VPS) — this is what `auth.rs` validates against. *(Set as part of the THE-180
   gated-binary deploy if not already present.)*
2. **This repo's Actions secrets** — Settings → Secrets and variables → Actions →
   New repository secret → name `SEED_API_TOKEN`, same value.

That's the only step no sandbox-bound agent can do: the VPS is unreachable from the
sandbox, and writing repo Actions secrets needs a token with secrets scope (the
deploy key cannot). No SSH key secret is required — seeding is HTTP-only.

## Status of prerequisites
- ✅ Approved payloads on `master`: `docs/published/the-{116,119,121,122,124,132–138}/publish.json`
  (THE-158, commit `f9716aa`), incl. THE-119 `spacex-s1-biggest-ipo-musk-risk-factor`.
- ✅ Workflow + script committed (this file, `seed-live.sh`, `deploy-seed.yml`).
- ⏳ `SEED_API_TOKEN` provisioned on the VPS **and** in Actions secrets — see above.

## Verify THE-119 renders
After the token is provisioned, run the workflow (`workflow_dispatch` mode `seed`, or
push any `docs/published/**/publish.json`). Then either:
- read `docs/seed-status/last-run.md` (committed back by the run) for the per-slug
  `GET /api/articles/<slug> -> 200` lines, or
- `curl https://news.scuffedcrew.no/articles/spacex-s1-biggest-ipo-musk-risk-factor`
  from any VPS-reachable host.
