# last seed/probe run

- when:    2026-05-22T11:30:00Z
- mode:    autopublish (on-demand, EiC handoff)
- trigger: THE-279 Article Verifier → Editor-in-Chief final review
- scope:   ONLY=the-279-ubuntu-core-26-ota-arm64-livepatch
- published: 1
- skipped:   0
- failed:    0

```
== autopublish-host: src=/opt/ainory-times/src base=http://127.0.0.1:8888 ==
deployed sha: 3d37ab4
PUBLISH the-279-ubuntu-core-26-ota-arm64-livepatch -> POST 200, GET 200
== summary: published=1 skipped=0 failed=0 ==
```

Verification:
- Host-local idempotency GET: 200
- Live read confirms: confidence 0.93, sources=3 (no paywall-enum drop), pipeline=5 steps, ai_monologue + ai_monologue_extended populated, category=linux
