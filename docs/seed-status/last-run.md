# last seed/probe run

- Date: 2026-06-01T06:30Z
- Result: published=1 skipped=135 failed=0
- Trigger: THE-590 autopublish-sweep (THE-233 Path A)
- Notes: One slug POSTed and read back 200 — `the-588-discord-5ca-breach-70000-government-ids` (publish.json was committed to master earlier at de709e8 but had not been POSTed to the live API yet). 135 prior slugs already live (skipped). Sweep ran from agent sandbox via `HOST=root@169.254.1.2 ./scripts/autopublish.sh`; host sourced `/etc/ainory-times.env` and POSTed to `http://127.0.0.1:8888/api/articles`.
