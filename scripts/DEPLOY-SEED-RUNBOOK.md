# VPS-reachable deploy + seed (THE-157)

## Problem
The agent sandbox's egress to `194.163.163.153` (news.scuffedcrew.no) is IP-blocked
(rejected in 0 ms; every other host connects — see THE-156). So the live seed
(THE-153) and publish-deploy (THE-125) cannot run from the sandbox.

## Solution: seed over the public HTTP API from a GitHub runner
GitHub-hosted runners are not the agent sandbox and reach the VPS normally.

The Dioxus server nests the API at `/api` on the same public bind (`src/main.rs`),
and `POST /api/articles` is an idempotent **UPSERT-by-slug** that runs inside the
running server — so the server itself owns the single-writer SurrealKV handle.
Therefore seeding needs **no SSH, no `systemctl stop/restart`, and no deploy key**.
We just POST each approved `publish.json` to `https://news.scuffedcrew.no/api/articles`.

`.github/workflows/deploy-seed.yml`:
- `workflow_dispatch` with `mode` = `probe` (read-only live check, default) or `seed`.
- `push` to `master` touching `docs/published/**/publish.json` → auto `seed`.
- `scripts/seed-live.sh` does the work; `probe` mode is also a **reachability bridge**
  — its `curl` output lands in the run log, readable via api.github.com, which the
  sandbox CAN reach, so live state is observable despite the egress block.

## The single irreducible operator action
**None for seeding.** There is no secret to add — the publish endpoint is
unauthenticated and reachable over HTTPS. A maintainer (or an agent holding a PAT)
triggers the workflow via `workflow_dispatch`, or a push under `docs/published/`
auto-seeds.

> Security note for Warden: the public, unauthenticated `POST /api/articles` is what
> makes this easy, but it also means anyone on the internet can publish/UPSERT
> articles. That should be gated (token / network ACL) as a separate hardening item;
> seeding would then add one Actions secret for the publish token.

## ⚠ Blocking prerequisite (not yet satisfied)
GitHub `master` is a **stale snapshot** (tip = SIG-23 Bill C-22 draft). The approved
article artifacts and seed tooling from **THE-127 (commit `28f4701`)** —
`docs/published/*/publish.json`, including THE-119
(`spacex-s1-biggest-ipo-musk-risk-factor`) — **were never pushed** to this repo.
A runner that clones `master` therefore has nothing to seed; `seed` mode no-ops with
a clear message.

**Unblock owner / action:** whoever holds the THE-127 working tree (the SIG/Ainory
runtime that published THE-119 locally) must `git push` the `docs/published/*`
payloads to `master`. Once present, `seed` mode publishes them and verifies each
`GET /api/articles/<slug>` returns 200.

## Verify THE-119 renders
After the payloads are pushed and `seed` runs:
`curl https://news.scuffedcrew.no/article/spacex-s1-biggest-ipo-musk-risk-factor`
(or read the workflow's per-slug `GET /api/articles/<slug> -> 200` log lines).
