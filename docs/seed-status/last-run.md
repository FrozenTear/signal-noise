# last seed/probe run

- when:    2026-06-05T08:38:05Z
- mode:    autopublish-sweep
- trigger: THE-785 heartbeat
- run:     scripts/autopublish.sh against root@169.254.1.2
- token configured: true (host-local /etc/ainory-times.env)

```
== summary: published=1 skipped=184 failed=0 ==
  published: the-772-german-data-center-gas-greenwashing-enefg
```

Notes:
- the-772 (German data-center gas/greenwashing — ENEFG) went live this sweep
  after the persona reassignment in b7e3997 unblocked autopublish.
- All other 184 candidates already live; sweep was otherwise an idempotent no-op.
- Supersedes prior CI probe run 26999995174 at 2026-06-05T06:45:59Z.
