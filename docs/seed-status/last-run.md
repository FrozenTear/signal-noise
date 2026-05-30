# last seed/probe run

- Date: 2026-05-30T17:00Z
- Result: published=4 skipped=109 failed=0
- Deployed SHA: 09b648a
- Trigger: THE-517 autopublish-sweep (THE-233 Path A)
- Notes: Four new slugs POSTed and read back 200 — `linux-7-2-amdgpu-bug-fix-mode-hdmi-2-1-frl-slip`, `the-508-nasa-cygnss-iran-gps-jammer-localization`, `eu-digital-decade-edri-surveillance-warning`, `italy-spyware-economy-price-list`. 109 prior slugs already live (skipped). Sweep ran from agent sandbox via `HOST=root@169.254.1.2 ./scripts/autopublish.sh`; host sourced `/etc/ainory-times.env` and POSTed to `http://127.0.0.1:8888/api/articles`.
