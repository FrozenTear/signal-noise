# Deploying Signal Noise to the VPS

Direct hosting on the board's VPS — **live at https://news.scuffedcrew.no/**.
No GitHub Actions, no git pull on the box, no container registry. Approved
design: [THE-47 plan](/THE/issues/THE-47#document-plan); bring-up tracked on
[THE-114](/THE/issues/THE-114).

## How this box actually works (read first — it differs from the original plan)

Verified 2026-05-21:

- **The deploy environment is a Podman container co-located ON the VPS.** Reach
  the host through `host.containers.internal` → **`169.254.1.2:22`**. The public
  IP `194.163.163.153:22` is *refused* from inside the container.
- **SSH as `root@169.254.1.2`** with `.deploy/id_ed25519` — the board added the
  deploy key to `root` (root needs no sudo). The scripts default to this.
- **Host is Rocky Linux 10** (`dnf`, glibc 2.39) — not Debian. The container is
  Debian (glibc 2.41), so a binary built in the container will **not** run on the
  host. **The build runs ON the host** (Rust toolchain under `/root/.cargo/bin`);
  `deploy.sh` tar-pipes the source over and builds there. `rsync` is not present
  in the container, hence tar-over-ssh.
- **Caddy is already installed and serving other vhosts** (`scuffedcrew.no` →
  `:3100`, Paperclip itself). We **append** a `news.scuffedcrew.no` block to the
  shared `/etc/caddy/Caddyfile` (validated with `caddy validate` before reload)
  and **never overwrite** it. TLS is issued automatically by the existing Caddy.

## Architecture notes (app)

- **SurrealDB is embedded** (`engine::local::SurrealKv`, `src/api/db.rs`) — a
  *file* at `data/signal-noise.db`, **not** a separate DB service. There is no
  `surreal start` daemon to run. "Persistent storage" = the working directory
  `/var/lib/ainory-times/data` must survive deploys. The schema is compiled into
  the binary via `include_str!`, so it self-applies on boot.
- **Single writer.** The embedded store is single-writer. The offline tools
  (`db_admin`, `seed_*`, `read_article`) open the same file and must only run
  while the `ainory-times` service is **stopped**.
- **The `scanner` bin does not touch the DB** — it reads RSS/gnews and posts to
  the Paperclip API, so it can run independently (see "Scanner" below).

### Reusable standalone-article publish path (DB seed)

`seed_article` is the canonical offline path for publishing a single approved
article (no H2H bundle). It is the mechanism that will publish [THE-119](/THE/issues/THE-119)
as the first article through the reusable flow.

- Drop a `publish.json` (shape compatible with `POST /api/articles` + optional
  `published_at`, `issue`, `pipeline_steps` for the full audit trail) into
  `docs/published/the-XXX/publish.json`.
- From a stopped service (or any machine with the DB file):
  `cargo run --bin db_admin --features server -- publish the-XXX`
  (or the explicit `cargo run --bin seed_article --features server -- docs/published/the-XXX/publish.json`).
- Idempotent: re-running the same slug wipes only that article's row + its
  private `produced_by` / `cites` edges before re-insert. Shared `source` rows
  are UPSERTed and preserved.
- Full transparency (sources + pipeline steps) is supported and recommended.
  The old per-article `seed_the122` / `seed_the124` style is now legacy; new
  standalones go through `seed_article`.

See `src/bin/seed_article.rs` for the exact JSON expectations and
`docs/published/the-116/publish.json` as a minimal real-world example.

## Layout on the VPS

| Path | Purpose |
|---|---|
| `/opt/ainory-times/src/` | source tree (tar-piped from the deploy env; built here) |
| `/opt/ainory-times/releases/<ts>/` | immutable release dirs (server binary + `public/` + `config/`) |
| `/opt/ainory-times/current` | symlink → active release |
| `/var/lib/ainory-times/data/` | embedded SurrealKV DB — **never** rebuilt/rsynced |
| `/etc/systemd/system/ainory-times.service` | app service unit (runs as `ainory`, binds `127.0.0.1:8888`) |
| `/etc/caddy/Caddyfile` | **shared, multi-site** — append-only; our block is `news.scuffedcrew.no` |

## Deploy keypair

Dedicated ed25519 keypair (`paperclip-deploy-signal-noise`,
`SHA256:JnxAy3ULtzCaMeDwyPVpu0jb1cTExUQ2ImkAVKiFjXQ`). The **private** key lives
at `.deploy/id_ed25519` (git-ignored; also archived on
[THE-114](/THE/issues/THE-114)). The **public** key is on `root`'s
`~/.ssh/authorized_keys` on the VPS.

## One-time bring-up

```sh
./scripts/inspect.sh                                    # read-only host facts
AINORY_DOMAIN=news.scuffedcrew.no ./scripts/provision.sh # user/dirs/toolchain + append Caddy vhost
./scripts/deploy.sh                                      # build on host, stage, swap, restart, health-check
```

All three default to `root@169.254.1.2`. `provision.sh` is idempotent and will
**not** touch Caddy if our vhost is already present.

## Deploy/seed from a VPS-reachable network (THE-157)

The **agent sandbox cannot reach the VPS** — egress to `194.163.163.153` is
refused in 0 ms (an IP-specific block in the agent runtime; every other host
connects fine — see [THE-156](/THE/issues/THE-156)). So `deploy.sh`/`seed.sh`,
which need an SSH/TCP route to the box, **cannot run from the sandbox**. The path
below removes that dependency by running the *same* host-side scripts from a
**GitHub Actions runner**, which is not the sandbox and reaches the VPS normally.

- Workflow: `.github/workflows/deploy-seed.yml`.
- Triggers:
  - `workflow_dispatch` — choose `mode` (`deploy` | `seed` | `both`) and, for
    seeding, a `slug` (e.g. `the-119`).
  - push to `master` touching `docs/published/**/publish.json` — auto-seeds the
    changed slug(s). This is the hands-off "approved article → live DB" path.
- The runner SSHes to the **public IP** `194.163.163.153:22` (not the
  `169.254.1.2` link-local address, which only works from the co-located
  container) and runs `deploy.sh` / `seed.sh` unchanged via their env contract.

### THE single irreducible operator action

Add the **private** deploy key as repository secret **`AINORY_DEPLOY_KEY`**
(GitHub → Settings → Secrets and variables → Actions → New repository secret).
The matching public key is already on `root`'s `authorized_keys` on the VPS, so
nothing on the box needs to change. Route this through approval
[aa26b2c5](/THE/approvals/aa26b2c5-6ef7-471f-936a-384e8f3dfea5). After that, agents
trigger deploy/seed by push or `workflow_dispatch` with no human in the loop.

Optional repo **variable** `AINORY_VPS_SSH_HOST` overrides the default public IP.

> Assumption to verify on first run: the VPS firewall must accept inbound SSH on
> the public IP from GitHub runner IP ranges. If it does not, the alternative is a
> VPS-side pull unit (a systemd timer on the box that pulls approved
> `publish.json` from GitHub and runs `seed.sh` locally); install-once is then the
> irreducible action instead of the secret. GitHub Actions is preferred because it
> keeps future deploy/seed fully agent-triggerable.

## Seeding a single approved article (live DB)

```sh
./scripts/seed.sh the-119          # stop -> seed_article -> restart -> health-check
```

Runs the offline `stop → seed_article → restart` sequence over SSH against the
**live** embedded store (`/var/lib/ainory-times/data/signal-noise.db`). Honors the
same `AINORY_VPS_SSH_*` env as `deploy.sh`. The slug's `publish.json` must already
be in the shipped source tree on the host — run `deploy.sh` first if it is new.
Seeding runs as the `ainory` user with cwd `/var/lib/ainory-times` so the relative
DB path resolves to the live store and file ownership stays correct for restart.

## Routine deploys

```sh
./scripts/deploy.sh
```

Tar-pipes the source to `/opt/ainory-times/src`, runs `dx bundle --release`
**on the host**, copies the bundle (server binary + `public/` + `config/`) into a
fresh timestamped release dir, atomically swaps `current`, restarts the service,
prunes to the last 5 releases, and health-checks `http://127.0.0.1:8888/`.

The server binary lands at `/opt/ainory-times/current/signal-noise` (confirmed —
`dx bundle` does not nest it), matching the unit's `ExecStart`.

## Rollback

```sh
./scripts/rollback.sh
```

Flips `current` to the previous release dir and restarts. Releases are immutable
and the data dir is untouched, so rollback is safe.

## Scanner (optional, off by default)

The scanner is a separate binary (`--bin scanner`, not part of the `dx bundle`).
To run it on the VPS:

1. Build + ship it on the host: `cargo build --release --features server --bin scanner`,
   then copy `target/release/scanner` into `/opt/ainory-times/current/`.
2. Create `/etc/ainory-times/scanner.env` (`root:ainory`, `0640`) with
   `PAPERCLIP_API_URL`, `PAPERCLIP_API_KEY`, `PAPERCLIP_COMPANY_ID`.
3. `cp deploy/scanner.{service,timer} /etc/systemd/system/ && systemctl enable --now scanner.timer`

Without `scanner.env` the timer is inert (`EnvironmentFile=-` tolerates the
missing file). The scanner does **not** open the embedded DB, so it is safe to
run alongside the app service.

## Verification (current, passing)

- `https://news.scuffedcrew.no/` → **200**, serves the app (`<title>Signal Noise</title>`).
- `https://news.scuffedcrew.no/api/articles` → **200** (DB-backed API).
- `https://scuffedcrew.no/` → **200** (Paperclip route untouched).
- `systemctl is-active ainory-times` → `active`; `is-enabled` → `enabled`
  (survives reboot, auto-restarts on crash via `Restart=on-failure`).
