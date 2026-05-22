# last seed/probe run

- when:    2026-05-22T10:08:49Z
- mode:    autopublish
- trigger: THE-271 (THE-233 Path A on-demand sweep)
- host:    root@169.254.1.2 (deployed sha ca6cf7d)
- token configured: true (host-local bearer, never left host)

```
== autopublish sweep THE-271 ==
published=5 skipped=25 failed=0
  PUBLISH instagram-kills-opt-in-dm-encryption    (THE-258) -> POST 200, GET 200, 8 sources live
  PUBLISH the-250-drupal-postgresql-sql-injection (THE-250) -> POST 200, GET 200
  PUBLISH the-252-us-government-quantum-equity-stake (THE-252) -> POST 200, GET 200, 7 sources live
  PUBLISH the-253-github-fight-survival-microsoft (THE-253) -> POST 200, GET 200
  PUBLISH the-259-uk-national-digital-id          (THE-259) -> POST 200, GET 200

required-this-run: THE-252 + THE-258 both live; source counts verified intact (7/7, 8/8)
also-swept-live:   THE-250, THE-253, THE-259 (staged on master, missed by prior sweeps)
```
