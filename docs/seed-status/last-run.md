# last seed/probe run

- when:    2026-05-21T15:56:45Z
- mode:    probe
- trigger: push
- run:     https://github.com/FrozenTear/signal-noise/actions/runs/26237331730
- token configured: false

```
== probe https://news.scuffedcrew.no ==
GET /            -> 200
GET /api/articles -> 1 article(s)
   - orf-at-misleading-cookie-banner | ORF Appealed Rather Than Even Out Two Buttons. The Court Said No.
write-gate: OPEN (POST /api/articles -> 422); THE-159 gate not deployed live yet
```
