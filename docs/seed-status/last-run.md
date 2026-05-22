# last seed/publish run

- when:    2026-05-22T05:56Z
- mode:    autopublish (THE-233 Path A — agent-side SSH → host-local bearer)
- trigger: agent (scripts/autopublish.sh)
- driver:  fully agent-side; no GitHub secret, no operator, no human
- token:   sourced on-host from /etc/ainory-times.env, never transmitted/logged

## last successful publish

- slug:   autopublish-selftest-20260522T055552Z  (self-test; PATCHed status=rejected after proof)
- result: POST /api/articles -> 200, GET /api/articles/<slug> -> 200 (went live)
- idempotency: immediate re-run SKIPPED the slug (already live) — no double-publish
- cleanup: self-test PATCHed status=rejected; live feed restored to 15 articles
- also cleaned: stray `deploy-gate-probe` row (leftover from redeploy gate check) PATCHed rejected

## how it runs

1. `scripts/autopublish.sh` (agent side): ssh root@169.254.1.2 -i ~/.ssh/ainory_deploy
   - SYNC=1 (default): `git fetch && git reset --hard origin/master` in /opt/ainory-times/src
   - pipes `scripts/autopublish-host.sh` over the same ssh and runs it on the host
2. `scripts/autopublish-host.sh` (host side): sources SEED_API_TOKEN from /etc/ainory-times.env,
   sweeps docs/published/*/publish.json, and for each slug NOT already live POSTs it to
   http://127.0.0.1:8888/api/articles (UPSERT-by-slug), then verifies GET 200.
   - idempotent: live-feed presence is the gate; FORCE=1 re-POSTs; ONLY=<slug> scopes to one.

## recurring trigger

Driven by the Paperclip routine `autopublish-sweep` (Console), which wakes to run
`scripts/autopublish.sh` and publish any newly-merged docs/published/<slug>/publish.json.
