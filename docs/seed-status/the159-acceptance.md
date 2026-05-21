# THE-189 bearer-auth acceptance report

- run: https://github.com/FrozenTear/signal-noise/actions/runs/26241127687
- when: 2026-05-21T17:08:08Z
- trigger: push
- token configured: false

```
== THE-189 bearer-auth acceptance https://news.scuffedcrew.no ==
pre-test article count: 12
T1 FAIL: unauthenticated POST -> 422 (want 401)
  T1 body: Failed to deserialize the JSON body into the target type: missing field `title` at line 1 column 2
T2 FAIL: wrong-token POST -> 422 (want 401)
  T2 body: Failed to deserialize the JSON body into the target type: missing field `title` at line 1 column 2
T1/T2 no-DB-write check PASS: count stable (12 -> 12)
T4 PASS: GET /api/articles -> 200
T3 SKIP: SEED_API_TOKEN not configured — skipping valid-bearer + seed tests
T5 SKIP: same reason
== THE-189 ACCEPTANCE: RED — see failures above ==
```
