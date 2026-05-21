# THE-189 bearer-auth acceptance report

- run: https://github.com/FrozenTear/signal-noise/actions/runs/26254809381
- when: 2026-05-21T21:42:16Z
- trigger: push
- token configured: false

```
== THE-189 bearer-auth acceptance https://news.scuffedcrew.no ==
pre-test article count: 12
T1 PASS: unauthenticated POST -> 401
  T1 body: {"error":"unauthorized"}
T2 PASS: wrong-token POST -> 401
  T2 body: {"error":"unauthorized"}
T1/T2 no-DB-write check PASS: count stable (12 -> 12)
T4 PASS: GET /api/articles -> 200
T3 SKIP: SEED_API_TOKEN not configured — skipping valid-bearer + seed tests
T5 SKIP: same reason
== THE-189 ACCEPTANCE: GREEN ==
```
