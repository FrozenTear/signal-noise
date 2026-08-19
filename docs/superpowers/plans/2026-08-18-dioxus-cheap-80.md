# Dioxus Cheap 80% Implementation Plan

> **For agentic workers:** Implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Keep Dioxus. Make the reader honest, ship a real release WASM, and stop the site from looking live when it is not. Topcoat stays a later sidecar, not this plan.

**Architecture:** Server functions already query Surreal and fall back to fixtures when the table is empty. Default that fallback off. Keep the mock helpers behind `SN_USE_MOCKS=1` so UI work can still render cards. Poll agent status and chatter on an interval instead of implying a live WASM island. Add a `make build` that actually uses the unused `wasm-release` profile. Do not join the scanner to Surreal, do not collapse the REST/server-fn split, do not add schema for tokens/cost.

**Tech Stack:** Dioxus 0.7.3 fullstack, Axum, SurrealDB, `dx` CLI, custom `sn-*` CSS.

**Recommended scope:** Honesty + ship (Tasks 1–6). Task 7 (poll) is the optional extra if the empty newsroom still wants a pulse.

---

## Out of scope

- Topcoat crate, sidecar, or rewrite
- Scanner / Paperclip → `POST /api/articles` publish path
- Collapsing `#[server]` vs REST
- Surreal schema for model / tokens / temperature / cost
- WebSocket, EventSource, or `LIVE SELECT`
- Real markdown library (lists/links/emphasis) — leave `simple_md_to_html` unless a published body actually needs it
- Deleting mock helper functions (gate them, do not erase)

---

## File Map

| File | Change |
|---|---|
| `src/server_fns.rs` | Honest empty returns; `SN_USE_MOCKS` gate; keep helpers |
| `src/components/agent_roster.rs` | No fake economics / live badge; empty states; optional poll |
| `src/components/nav.rs` | Theme signal from `localStorage`; drop fake BUILD/live labels; empty ticker |
| `src/pages/article.rs` | Stop hardcoding `claude-sonnet-4-6` |
| `src/pages/about.rs` | Same masthead / `sn-*` system as the rest of the site |
| `src/pages/home.rs` | Empty feed already exists; confirm it is reachable |
| `Makefile` | `build` / `serve` targets that use release WASM |
| `.gitignore` | Track `Cargo.lock`; ignore `.next/` |
| `.next/` | Remove leftover Next cache from git |

---

## Task 1: Honest server-fn fallbacks

**Files:**
- Modify: `src/server_fns.rs`

**What:** Default empty DB to empty data. Mocks only when `SN_USE_MOCKS=1`.

`get_transparency_stats` already returns zeros. Match that pattern.

- [x] **Step 1: Add a server-only mock gate**

Near the mock helpers (after the server functions, before `mock_pipeline_activity`), add:

```rust
#[cfg(feature = "server")]
fn use_mocks() -> bool {
    std::env::var("SN_USE_MOCKS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}
```

- [x] **Step 2: Change the four fallbacks**

Replace the current “always mock when empty” tails:

| Function | Empty / missing DB today | Empty default after |
|---|---|---|
| `get_articles` | `Ok(mock_articles(category))` | `Ok(vec![])` unless `use_mocks()` |
| `get_article_by_slug` | `Ok(mock_article(&slug))` | `Ok(None)` unless `use_mocks()` |
| `get_agent_status` | four idle agents | `Ok(vec![])` unless `use_mocks()` |
| `get_recent_pipeline_activity` | curated editor jokes | `Ok(vec![])` unless `use_mocks()` |

Keep the mock helper functions. Update the file header comment: mocks are opt-in, not “until SIG-104”.

- [x] **Step 3: Confirm UI empty states already exist**

These must become reachable without new copy unless the wording still implies a live newsroom:

- Home / beat: “No articles yet. The pipeline is warming up.”
- Article: existing “Article not found”
- Roster: “Agent status unavailable.” / chatter empty
- Nav ticker: currently falls through to “Loading pipeline activity…” for both loading *and* empty `Ok([])`. Split that: empty `Ok([])` should say the pipeline is idle, not that it is loading.

- [x] **Step 4: Verify**

```bash
# default: empty feed, no fake kernel story
# SN_USE_MOCKS=1: old fixtures still render
```

---

## Task 2: Stop labeling fiction as live

**Files:**
- Modify: `src/components/agent_roster.rs`
- Modify: `src/components/nav.rs`
- Modify: `src/pages/article.rs`

**What:** If the number is not in Surreal, do not print a number.

- [x] **Step 1: Agent roster badge**

In `AgentRoster`, drop the default `"live"` badge. Show `"{n} active"` only when `n > 0`. Otherwise show nothing, or `"idle"`.

When `agents` is `Ok([])`, render a one-line empty state (“No agent heartbeats yet.”) instead of an empty card with a live badge.

When chatter is `Ok([])`, say “No recent pipeline steps.” Do not keep the comment “falls back to mock”.

- [x] **Step 2: Hide Model Economics**

The card is labeled illustrative and prints `$0.84` / `1.24M` / `$0.060`. Remove the whole card. Schema has no spend/token fields; inventing `—` rows still looks like a dashboard.

Keep Transparency Report. It is real counts. Leave the `0%` human-involvement row only if you treat it as a product claim, not a measured metric. Prefer deleting that row too — it is the same class of hardcoded theater as the nav chip.

- [x] **Step 3: Nav chrome**

- Replace `BUILD 0.9.4` with `env!("CARGO_PKG_VERSION")` (today `0.1.0`) or drop the build string.
- Ticker label: “Pipeline activity”, not “Live Activity”.
- `SYSTEM ● ONLINE` can stay (the process is up). Do not add a fake LIVE SELECT hint.
- `HUMAN INVOLVEMENT 0%` is a slogan. Move it to About or delete it from the sys strip.

- [x] **Step 4: Article gen-bar**

`src/pages/article.rs` hardcodes `claude-sonnet-4-6`. `ArticleDetail` has no model field. Remove the model pill until publish writes one. Keep persona + timestamp.

Same for `AgentCommandRow`: stop defaulting missing model to `"claude-sonnet-4-6"`. If `model` is `None`, omit the line.

---

## Task 3: Theme signal matches the no-flash script

**Files:**
- Modify: `src/components/nav.rs`

**What:** `App` already applies `theme-light` from `localStorage` before paint. `Nav` starts `is_light` at `false`, so the first click can no-op or invert.

- [x] **Step 1: Initialize from the document class**

Read the class on first render via `eval` / a tiny inline check, or assume WASM can read `web_sys` if already in tree. Smallest fix that works in this crate:

```rust
let mut is_light = use_signal(|| {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
            .map(|e| e.class_list().contains("theme-light"))
            .unwrap_or(false)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
});
```

`web_sys` is already pulled in by Dioxus web. Do not add a new crate unless the compile fails.

If SSR renders Nav on the server, keep the server branch `false` (no localStorage). After hydrate the signal must match the class the boot script already set.

- [x] **Step 2: Verify**

Set `localStorage.sn-theme = 'light'`, reload, click the toggle once: it must go dark, not stay light. Browser click not run; wasm + server both compile with the `web-sys` init.

---

## Task 4: About page on the design system

**Files:**
- Modify: `src/pages/about.rs`

**What:** `/about` is leftover utility Tailwind and has no `Nav`. Beats listed there do not match shipped routes.

- [x] **Step 1: Wrap with `Nav` + `sn-*` layout**

Use the same masthead/footer pattern as `beat.rs`: `Nav {}`, single-column `sn-layout`, `sn-headline`, `prose` only inside the existing design-system `.prose` rules (already in `tailwind.css`).

- [x] **Step 2: Align beats with routes**

Ship three beats that exist (`/linux`, `/tech`, `/privacy`). Mention AI Policy / Society as upcoming, not as live sections.

- [x] **Step 3: Verify**

`/about` has the boot banner, masthead, and theme toggle. No `max-w-3xl` / `text-3xl` utility page.

---

## Task 5: Release WASM is a real command

**Files:**
- Modify: `Makefile`
- Modify: `Cargo.toml` only if the profile name `dx` expects differs

**What:** Current `make serve` is `dx serve --port 8888` (debug). Checked-in debug WASM is **54 MB**. `wasm-release` exists and is unused.

- [x] **Step 1: Add Makefile targets**

```make
.PHONY: serve serve-release build

serve:
	dx serve --port 8888 --fullstack true

serve-release:
	dx serve --port 8888 --fullstack true --release --profile wasm-release

build:
	dx build --web --fullstack true --release --profile wasm-release
```

Confirm `--profile wasm-release` applies to the **client** (`@client`) if `dx` only honors it on one side. If the server also gets `opt-level = "z"` + `panic = "abort"`, that is acceptable for this binary.

- [x] **Step 2: Measure**

Skipped `make build` (release WASM compile is long). Targets exist; measure on first `make build`.

After `make build`, record gzipped `dist` / `target/dx/.../public/wasm/*.wasm` size in the PR description. Target: well under the 54 MB debug artifact; expect roughly 1–4 MB uncompressed for this app.

- [x] **Step 3: Confirm SSR path**

`src/main.rs` only SSRs under `feature = "server"`. `dx serve --fullstack true` must enable that. A client-only `dioxus::launch` is not the production path.

---

## Task 6: Repo hygiene

**Files:**
- Modify: `.gitignore`
- Delete from git: `.next/` (leave untracked via gitignore)
- Stop ignoring: `Cargo.lock`

**What:** This is a binary app. Lockfile should be committed. `.next/` is leftover Next cache and is currently tracked.

- [x] **Step 1: `.gitignore`**

Remove the `Cargo.lock` line. Add:

```
# Leftover Next.js cache — this app is Dioxus, not Next
/.next/
```

- [x] **Step 2: Git index**

```bash
git rm -r --cached .next
git add Cargo.lock .gitignore
```

Do not commit unless the user asks. Leave the working tree ready.

---

## Task 7 (optional extra): Cheap “live”

**Files:**
- Modify: `src/components/agent_roster.rs`
- Modify: `src/components/nav.rs`

**What:** One-shot `use_resource` never refreshes. Do not add websockets. Poll every 15s.

Use a `use_signal` tick + `gloo_timers` / `TimeoutFuture` / Dioxus `use_future` loop. Do not add a new JS framework. If a timer crate is not already in the WASM graph, prefer `wasm_bindgen_futures` + `gloo_timers` only if Dioxus already exposes an interval helper.

When the tab is hidden, skip ticks if cheap (`document.visibilityState`). If that is awkward in RSX, skip it.

Empty polls must stay empty. This task is useless before Task 1.

---

## Verification

1. `SN_USE_MOCKS` unset: home shows the warming-up empty state, not Linux 6.14.
2. `SN_USE_MOCKS=1`: fixtures still useful for card/HUD work.
3. `/article/missing` is not-found, not a synthetic article.
4. No `$0.84`, no `1.24M`, no hardcoded `claude-sonnet-4-6`, no `"live"` badge on an idle roster.
5. Light theme persisted → toggle works on first click.
6. `/about` uses `Nav` and lists only shipped beats.
7. `make build` produces a release WASM; note the byte size.
8. `Cargo.lock` is no longer gitignored; `.next/` is.

---

## Later, not this plan

1. First real `POST /api/articles` from the editorial pipeline.
2. Persist model / tokens / cost, then put them back on the gen-bar.
3. Topcoat sidecar: `/about` then roster shard, measure HTML vs this release WASM.
4. Collapse REST vs server functions once only one client remains.
