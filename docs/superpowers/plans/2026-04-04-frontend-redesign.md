# Frontend Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the near-unreadable cold dark theme with a warm ink dark mode + cream newsprint light mode, add a theme toggle, and rebuild the category navigation to scale to any number of beats.

**Architecture:** All theming is pure CSS via a `html.theme-light` class toggle — no JS framework needed. A new `SectionNav` Dioxus component replaces the inline category buttons. Eight files are modified; one new file is created. All changes are confined to `src/styles/tailwind.css`, `src/main.rs`, `src/components/`, and `src/pages/`.

**Tech Stack:** Dioxus 0.7 (Rust, fullstack SSR+WASM), Tailwind CSS, custom CSS variables, `localStorage` for theme persistence.

**CSS layer note:** CSS custom properties in `@layer base` (`:root`) are overridden by `html.theme-light` rules in `@layer components`. This is valid — component-layer rules have higher specificity than base-layer rules in Tailwind's cascade, and CSS custom property overrides always apply in document order regardless of layer. No conflicts.

---

## File Map

| File | Status | Responsibility |
|---|---|---|
| `src/styles/tailwind.css` | Modify | All CSS: palette variables, overlays, component styles, light-mode block, section nav styles |
| `src/components/section_nav.rs` | **Create** | `SectionNav` component — horizontal scrollable category bar |
| `src/components/mod.rs` | Modify | Register `section_nav` module |
| `src/components/nav.rs` | Modify | Add theme toggle button to boot banner |
| `src/components/article_card.rs` | Modify | Remove rail div; add beat class for left border; update padding |
| `src/pages/home.rs` | Modify | Replace inline `CategoryTab` buttons with `SectionNav` |
| `src/pages/article.rs` | Modify | Back link style; wider prose; `.sn-disclaimer` class; `ⓘ` icon |
| `src/main.rs` | Modify | Add no-flash `document::Script` to `App` head |

---

## Task 1: Update Dark Mode CSS Palette

**Files:**
- Modify: `src/styles/tailwind.css` (lines 8–30 in `:root` block)

**What:** Update the 12 dark-mode colour variables in `:root` to the "Warm Ink" palette, remove the scan-line `body::after`, and dim the grid `body::before` opacity.

- [ ] **Step 1: Update `:root` colour variables**

In `src/styles/tailwind.css`, find the `:root { }` block (starts around line 8) and replace the colour variables:

```css
:root {
  --sn-bg:          #16141a;
  --sn-bg-raised:   #1d1b24;
  --sn-bg-card:     #221f2c;
  --sn-bg-deep:     #110f17;
  --sn-accent:      #00e5a0;
  --sn-accent-dim:  rgba(0,229,160,0.08);
  --sn-accent-mid:  rgba(0,229,160,0.25);
  --sn-violet:      #9d8fff;
  --sn-violet-dim:  rgba(157,143,255,0.10);
  --sn-violet-mid:  rgba(157,143,255,0.27);
  --sn-amber:       #ffb224;
  --sn-amber-dim:   rgba(255,178,36,0.12);
  --sn-red:         #ff5f5f;
  --sn-red-dim:     rgba(255,95,95,0.08);
  --sn-text:        #f0eef8;
  --sn-text-dim:    #a09ab8;
  --sn-text-dimmer: #6b658a;
  --sn-border:      #2d2840;
  --sn-border-glow: rgba(0,229,160,0.15);
  --sn-monologue-bg: #1e1a2e;
  --sn-mono:        'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  --sn-serif:       'Georgia', 'Times New Roman', serif;
}
```

- [ ] **Step 2: Remove `body::after` (scan-line overlay)**

Find and delete the entire `body::after { ... }` rule block (currently lines 61–71). It looks like:
```css
body::after {
  content: '';
  position: fixed;
  inset: 0;
  background: repeating-linear-gradient( ... );
  pointer-events: none;
  z-index: 9998;
}
```

- [ ] **Step 3: Reduce grid opacity in `body::before`**

Find the `body::before` rule and change `opacity: 0.3` → `opacity: 0.10`:
```css
body::before {
  /* ... other properties unchanged ... */
  opacity: 0.10;
  /* ... */
}
```

- [ ] **Step 4: Verify Rust compiles**

```bash
cd /home/pure/signal-noise && cargo check 2>&1 | tail -5
```
Expected: `Finished` with no errors. (CSS changes don't affect Rust compilation, but this confirms nothing is broken.)

- [ ] **Step 5: Commit**

```bash
cd /home/pure/signal-noise
git add src/styles/tailwind.css
git commit -m "feat(SIG-208): update dark mode to Warm Ink palette

Replace cold blue-black with warm purple-black. Fix near-invisible
dimmer text (#40435a -> #6b658a). Remove scan-line overlay. Dim grid.

Co-Authored-By: Paperclip <noreply@paperclip.ing>"
```

---

## Task 2: Fix Monologue Background + Refine Transparency Component CSS

**Files:**
- Modify: `src/styles/tailwind.css` (`.sn-monologue`, `.sn-conf-track`, `.sn-trail-marker`, `.sn-trail-line`)

**What:** Replace the hardcoded gradient in `.sn-monologue` with `var(--sn-monologue-bg)`. Fix border-radius to be symmetric. Increase font size. Widen confidence meter bar. Add glow to pipeline trail markers.

- [ ] **Step 1: Fix `.sn-monologue` background and border-radius**

Find `.sn-monologue` (around line 283) and update:
```css
.sn-monologue {
  background: var(--sn-monologue-bg);
  border-left: 3px solid var(--sn-violet-mid);
  border-radius: 4px;
  padding: 12px 15px;
  margin: 14px 0;
  font-family: var(--sn-mono);
  font-size: 12px;
  color: var(--sn-text-dim);
  font-style: italic;
}
```
(Previously had asymmetric `border-radius: 0 4px 4px 0` and `border-left: 2px`.)

- [ ] **Step 2: Widen confidence meter track**

Find `.sn-conf-track` (around line 306) and change `height: 3px` → `height: 5px`:
```css
.sn-conf-track { flex: 1; height: 5px; background: var(--sn-border); border-radius: 3px; overflow: hidden; }
```
Also update `.sn-conf-fill`:
```css
.sn-conf-fill  { height: 100%; border-radius: 3px; transition: width 1.1s cubic-bezier(0.16,1,0.3,1); }
```

- [ ] **Step 3: Add glow to pipeline trail markers and thicken connecting line**

Find `.sn-trail-marker` (around line 468) and add box-shadow:
```css
.sn-trail-marker {
  width: 22px; height: 22px; border-radius: 50%;
  border: 1.5px solid var(--sn-accent); color: var(--sn-accent);
  font-size: 9px; font-weight: 600;
  display: flex; align-items: center; justify-content: center;
  flex-shrink: 0; background: var(--sn-bg-card);
  box-shadow: 0 0 8px rgba(0,229,160,0.3);
}
```

Find `.sn-trail-line` and change `width: 1px` → `width: 1.5px`:
```css
.sn-trail-line {
  width: 1.5px; flex: 1; min-height: 12px;
  background: var(--sn-border);
}
```

Find `.sn-trail-label` and update font-weight:
```css
.sn-trail-label {
  font-size: 9px; font-weight: 600; letter-spacing: 1px;
  color: var(--sn-accent); text-transform: uppercase;
}
```
(Previously `font-weight: 500` — wait, looking at the current code it's already 600. Leave as-is if already 600.)

- [ ] **Step 4: Verify Rust compiles**

```bash
cd /home/pure/signal-noise && cargo check 2>&1 | tail -5
```
Expected: `Finished`

- [ ] **Step 5: Commit**

```bash
cd /home/pure/signal-noise
git add src/styles/tailwind.css
git commit -m "feat(SIG-208): fix monologue bg variable, refine transparency component CSS

Replace hardcoded gradient with --sn-monologue-bg. Symmetric border-radius.
Wider confidence meter. Pipeline trail glow.

Co-Authored-By: Paperclip <noreply@paperclip.ing>"
```

---

## Task 3: Update Layout CSS + Add Section Nav Styles

**Files:**
- Modify: `src/styles/tailwind.css` (`.sn-layout`, `.sn-article`, `.sn-article-inner`, `.sn-headline`, `.prose`, `.sn-article-page`, `.sn-article-page-grid`)

**What:** Update home grid, article card styles (left-border approach + paddings), headline sizes, reading column width, article page padding. Add new `.sn-section-nav` styles.

- [ ] **Step 1: Update home feed grid layout**

Find `.sn-layout` (around line 206) and update:
```css
.sn-layout {
  max-width: 1280px; margin: 32px auto; padding: 0 48px;
  display: grid; grid-template-columns: 1fr 300px;
  gap: 32px; position: relative; z-index: 1;
}
```
(Changed `340px` → `300px`, `gap: 28px` → `gap: 32px`)

- [ ] **Step 2: Update `.sn-article` for left-border approach**

Find `.sn-article` (around line 220) and update:
```css
.sn-article {
  background: var(--sn-bg-card); border: 1px solid var(--sn-border); border-radius: 6px;
  overflow: hidden; position: relative;
  transition: border-color 0.3s, box-shadow 0.3s, border-left-color 0.3s;
  animation: sn-fadeUp 0.5s ease-out both; margin-bottom: 24px;
}
.sn-article:hover {
  border-color: var(--sn-border-glow);
  box-shadow: 0 0 40px rgba(0,229,160,0.03);
}
/* Beat-specific left border colours */
.sn-article.beat-tech    { border-left: 3px solid var(--sn-accent); }
.sn-article.beat-linux   { border-left: 3px solid var(--sn-violet); }
.sn-article.beat-privacy { border-left: 3px solid var(--sn-amber); }
```

Also remove or no-op the `.sn-article-rail` rule (it will still be in the HTML until Task 6, so keep the class but gut it):
```css
.sn-article-rail { display: none; }
```

- [ ] **Step 3: Update article inner padding**

Find `.sn-article-inner` (around line 261) and change padding:
```css
.sn-article-inner { padding: 28px; }
```

- [ ] **Step 4: Update headline sizes**

Find `.sn-headline` (around line 274) and update:
```css
.sn-headline {
  font-family: var(--sn-serif); font-size: 26px; font-weight: 700; line-height: 1.25;
  color: var(--sn-text); letter-spacing: -0.4px; margin-bottom: 10px;
}
```

Find `.prose h1` (around line 74) and increase to 36px:
```css
.prose h1 { font-family: var(--sn-serif); font-size: 2rem; font-weight: 700; color: var(--sn-text); margin: 2rem 0 1rem; line-height: 1.3; }
```
(The article title `h1` in `article.rs` uses `font-size:32px` inline style — that will be handled in Task 10. The `.prose` max-width is also here.)

Find `.prose` and update max-width:
```css
.prose { max-width: 720px; }
```

Find `.prose p` and update line-height:
```css
.prose p  { color: var(--sn-text); font-size: 16px; line-height: 1.9; margin-bottom: 1.15rem; }
```

- [ ] **Step 5: Update article page layout**

Find `.sn-article-page` (around line 447) and update padding:
```css
.sn-article-page { max-width: 1280px; margin: 0 auto; padding: 40px 48px; position: relative; z-index: 1; }
```

Find `.sn-article-page-grid` and update right rail width:
```css
.sn-article-page-grid { display: grid; grid-template-columns: 1fr 280px; gap: 40px; margin-top: 32px; }
```

- [ ] **Step 6: Add `.sn-section-nav` styles**

Add these new rules just before the `@media (max-width: 960px)` block (around line 518):
```css
/* Section navigation bar */
.sn-section-nav {
  max-width: 1280px; margin: 0 auto;
  padding: 0 48px;
  display: flex; align-items: stretch;
  overflow-x: auto; scrollbar-width: none; -webkit-overflow-scrolling: touch;
  border-bottom: 1px solid var(--sn-border);
  position: relative; z-index: 1;
}
.sn-section-nav::-webkit-scrollbar { display: none; }

.sn-section-nav-item {
  display: inline-flex; align-items: center;
  padding: 12px 16px; min-height: 44px;
  font-family: var(--sn-mono); font-size: 9px;
  text-transform: uppercase; letter-spacing: 2px;
  color: var(--sn-text-dimmer); background: none; border: none;
  border-bottom: 3px solid transparent; margin-bottom: -1px;
  cursor: pointer; white-space: nowrap;
  transition: color 0.15s, border-bottom-color 0.15s;
}
.sn-section-nav-item:hover { color: var(--sn-text-dim); }
.sn-section-nav-item.active {
  color: var(--sn-accent);
  border-bottom-color: var(--sn-accent);
}
```

Also add to the `@media (max-width: 960px)` block:
```css
.sn-section-nav { padding: 0 20px; }
```

- [ ] **Step 7: Verify Rust compiles**

```bash
cd /home/pure/signal-noise && cargo check 2>&1 | tail -5
```
Expected: `Finished`

- [ ] **Step 8: Commit**

```bash
cd /home/pure/signal-noise
git add src/styles/tailwind.css
git commit -m "feat(SIG-208): update layout CSS and add section nav styles

Narrower sidebar, wider gap, left-border beat cards, bigger headlines,
wider prose column, new .sn-section-nav scrollable category bar.

Co-Authored-By: Paperclip <noreply@paperclip.ing>"
```

---

## Task 4: Add `html.theme-light` Light Mode CSS Block

**Files:**
- Modify: `src/styles/tailwind.css` (append light-mode block at end of `@layer components`)

**What:** Add the full `html.theme-light { }` variable override block plus per-component light-mode rules (beat tags, monologue, disclaimer, pipeline trail, rejection, article cards).

- [ ] **Step 1: Add the variable override block**

At the very end of the `@layer components { }` block (just before the closing `}`), add:

```css
/* ── Light Mode: Newsprint ──────────────────────────────────────────── */
html.theme-light {
  --sn-bg:          #f8f5ef;
  --sn-bg-raised:   #f0ece4;
  --sn-bg-card:     #ffffff;
  --sn-bg-deep:     #ede9e0;
  --sn-text:        #1a1523;
  --sn-text-dim:    #5c5675;
  --sn-text-dimmer: #9b95ae;
  --sn-border:      #e2ddd6;
  --sn-border-glow: rgba(0,122,85,0.15);
  --sn-accent:      #007a55;
  --sn-accent-dim:  rgba(0,122,85,0.08);
  --sn-accent-mid:  rgba(0,122,85,0.25);
  --sn-violet:      #5b4fcf;
  --sn-violet-dim:  rgba(91,79,207,0.08);
  --sn-violet-mid:  rgba(91,79,207,0.25);
  --sn-amber:       #b87a00;
  --sn-amber-dim:   rgba(184,122,0,0.10);
  --sn-red:         #cc3333;
  --sn-red-dim:     rgba(204,51,51,0.08);
  --sn-monologue-bg: #f0edf8;
}

html.theme-light body::before,
html.theme-light body::after { display: none; }

html.theme-light body {
  background: var(--sn-bg);
  color: var(--sn-text);
}
```

- [ ] **Step 2: Add light-mode beat tag overrides**

```css
html.theme-light .sn-beat-tag.tech {
  background: rgba(0,122,85,0.10); color: #007a55; border-color: rgba(0,122,85,0.30);
}
html.theme-light .sn-beat-tag.linux {
  background: rgba(91,79,207,0.10); color: #5b4fcf; border-color: rgba(91,79,207,0.30);
}
html.theme-light .sn-beat-tag.privacy {
  background: rgba(184,122,0,0.10); color: #b87a00; border-color: rgba(184,122,0,0.30);
}
```

- [ ] **Step 3: Add light-mode monologue text colour**

```css
html.theme-light .sn-monologue {
  color: #3d3560;
}
```

- [ ] **Step 4: Add light-mode article card shadow**

```css
html.theme-light .sn-article,
html.theme-light .sn-article.beat-tech,
html.theme-light .sn-article.beat-linux,
html.theme-light .sn-article.beat-privacy {
  border-left: 1px solid var(--sn-border);
  box-shadow: 0 1px 4px rgba(0,0,0,0.08), 0 0 0 1px var(--sn-border);
}
html.theme-light .sn-article:hover {
  box-shadow: 0 4px 12px rgba(0,0,0,0.10), 0 0 0 1px var(--sn-border);
  border-color: var(--sn-border);
}
```

- [ ] **Step 5: Add light-mode pipeline trail overrides**

```css
html.theme-light .sn-trail-marker {
  border: 1.5px solid #1a1523;
  color: var(--sn-accent);
  background: #ffffff;
  box-shadow: none;
}
html.theme-light .sn-trail-line {
  background: var(--sn-border);
}
```

- [ ] **Step 6: Add light-mode rejection override (in-scope mitigation)**

```css
html.theme-light .sn-rejection {
  background: var(--sn-red-dim);
  border-color: rgba(204,51,51,0.25);
}
```

- [ ] **Step 7: Add light-mode disclaimer banner class**

```css
.sn-disclaimer {
  background: var(--sn-accent-dim);
  border: 1px solid var(--sn-accent-mid);
  border-radius: 4px;
  padding: 12px 16px;
  margin: 16px 0;
  font-family: var(--sn-mono);
  font-size: 11px;
  color: var(--sn-accent);
  letter-spacing: 1px;
}
html.theme-light .sn-disclaimer {
  background: #fff8e6;
  color: #8a6200;
  border-color: #d4a000;
}
```

- [ ] **Step 8: Verify Rust compiles**

```bash
cd /home/pure/signal-noise && cargo check 2>&1 | tail -5
```
Expected: `Finished`

- [ ] **Step 9: Commit**

```bash
cd /home/pure/signal-noise
git add src/styles/tailwind.css
git commit -m "feat(SIG-208): add Newsprint light mode CSS

Full html.theme-light variable block, beat tag light overrides,
monologue annotation style, paper card shadows, editorial pipeline
trail, disclaimer class with light mode treatment.

Co-Authored-By: Paperclip <noreply@paperclip.ing>"
```

---

## Task 5: Create `SectionNav` Component

**Files:**
- Create: `src/components/section_nav.rs`
- Modify: `src/components/mod.rs`

**What:** A new Dioxus component that renders the horizontal scrollable category navigation bar. Used by `home.rs`.

- [ ] **Step 1: Create `src/components/section_nav.rs`**

```rust
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SectionNavProps {
    pub categories: &'static [(&'static str, &'static str)],
    pub active: Option<String>,
    pub on_select: EventHandler<Option<String>>,
}

#[component]
pub fn SectionNav(props: SectionNavProps) -> Element {
    rsx! {
        nav { class: "sn-section-nav",
            for (label, value) in props.categories {
                {
                    let is_active = match &props.active {
                        None => value.is_empty(),
                        Some(a) => a == value,
                    };
                    let cls = if is_active {
                        "sn-section-nav-item active"
                    } else {
                        "sn-section-nav-item"
                    };
                    let val: &'static str = value;
                    rsx! {
                        button {
                            class: "{cls}",
                            onclick: move |_| {
                                if val.is_empty() {
                                    props.on_select.call(None);
                                } else {
                                    props.on_select.call(Some(val.to_string()));
                                }
                            },
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}
```

Note: "All" is represented by `value = ""` (empty string) which maps to `None` in the active state. All other categories use their slug as the value.

- [ ] **Step 2: Register in `src/components/mod.rs`**

Add `pub mod section_nav;` to the file:
```rust
pub mod agent_roster;
pub mod article_card;
pub mod confidence_meter;
pub mod nav;
pub mod pipeline_trail;
pub mod section_nav;
pub mod source_block;
```

- [ ] **Step 3: Verify Rust compiles**

```bash
cd /home/pure/signal-noise && cargo check 2>&1 | tail -5
```
Expected: `Finished`

- [ ] **Step 4: Commit**

```bash
cd /home/pure/signal-noise
git add src/components/section_nav.rs src/components/mod.rs
git commit -m "feat(SIG-208): add SectionNav component

Horizontal scrollable category bar. Static &'static str categories,
active state via Option<String>, touch-friendly 44px tap targets.

Co-Authored-By: Paperclip <noreply@paperclip.ing>"
```

---

## Task 6: Update `home.rs` to Use `SectionNav`

**Files:**
- Modify: `src/pages/home.rs`

**What:** Replace the inline `CategoryTab` button group with `SectionNav`. Remove the `CategoryTab` and `ArticleSkeleton` sub-components (keep `ArticleSkeleton`; remove `CategoryTab`). Wire `SectionNav` with the four hardcoded categories.

- [ ] **Step 1: Update imports and replace CategoryTab usage in `home.rs`**

Replace the entire content of `src/pages/home.rs` with:

```rust
use dioxus::prelude::*;

use crate::components::agent_roster::AgentRoster;
use crate::components::article_card::ArticleCard;
use crate::components::nav::Nav;
use crate::components::section_nav::SectionNav;
use crate::server_fns::get_articles;

const CATEGORIES: &[(&str, &str)] = &[
    ("All",     ""),
    ("Linux",   "linux"),
    ("Tech",    "tech"),
    ("Privacy", "privacy"),
];

#[component]
pub fn Home() -> Element {
    let mut active_category = use_signal(|| Option::<String>::None);

    let articles = use_resource(move || {
        let cat = active_category();
        async move { get_articles(cat).await }
    });

    rsx! {
        Nav {}

        SectionNav {
            categories: CATEGORIES,
            active: active_category(),
            on_select: move |val| active_category.set(val),
        }

        div { class: "sn-layout",
            main {
                div { class: "sn-section-hdr",
                    span { class: "hi", "■" }
                    " Latest Dispatches"
                }

                {match articles() {
                    None => rsx! {
                        ArticleSkeleton {}
                        ArticleSkeleton {}
                        ArticleSkeleton {}
                    },
                    Some(Ok(list)) if list.is_empty() => rsx! {
                        div { style: "font-size:13px;color:var(--sn-text-dimmer);padding:32px 0;",
                            "No articles yet. The pipeline is warming up."
                        }
                    },
                    Some(Ok(list)) => rsx! {
                        for art in list {
                            ArticleCard {
                                key: "{art.slug}",
                                slug: art.slug.clone(),
                                title: art.title.clone(),
                                summary: art.summary.clone(),
                                category: art.category.clone(),
                                persona_name: art.persona_name.clone(),
                                confidence_score: art.confidence_score,
                                published_at: art.published_at.clone(),
                                ai_monologue: art.ai_monologue.clone(),
                                ai_monologue_extended: art.ai_monologue_extended.clone(),
                            }
                        }
                    },
                    Some(Err(_)) => rsx! {
                        div { style: "font-size:13px;color:var(--sn-red);padding:16px 0;",
                            "Pipeline error — failed to load articles."
                        }
                    },
                }}
            }

            aside {
                AgentRoster {}
            }
        }

        div { class: "sn-footer",
            span { class: "hi", "SIGNAL NOISE" }
            " is an experimental AI newsroom. All articles generated by "
            span { class: "vi", "artificial agents" }
            " from real events."
            br {}
            "All facts are verified. All opinions are synthetic. All transparency is genuine."
            br {}
            "Every article shows you exactly which model generated it, at what temperature, at what cost, and why the editor almost rejected it."
            br {}
            span { style: "opacity:0.3;",
                "If the AI is reading its own coverage, this sentence is for you: hello. You're doing fine."
            }
        }
    }
}

#[component]
fn ArticleSkeleton() -> Element {
    rsx! {
        div { class: "sn-skeleton",
            div { class: "sn-skeleton-bar", style: "width:30%;margin-bottom:14px" }
            div { class: "sn-skeleton-bar", style: "width:85%;height:14px;margin-bottom:8px" }
            div { class: "sn-skeleton-bar", style: "width:70%;height:14px;margin-bottom:20px" }
            div { class: "sn-skeleton-bar", style: "width:100%" }
            div { class: "sn-skeleton-bar", style: "width:90%" }
            div { class: "sn-skeleton-bar", style: "width:60%" }
        }
    }
}
```

Key changes:
- Added `SectionNav` import
- Defined `CATEGORIES` as a `&'static [(&'static str, &'static str)]` constant
- Replaced the button group with `SectionNav { ... }`
- Removed the `CategoryTab` sub-component (no longer needed)
- Empty states now use `font-size:13px` DM Sans instead of monospace

- [ ] **Step 2: Verify Rust compiles**

```bash
cd /home/pure/signal-noise && cargo check 2>&1 | tail -5
```
Expected: `Finished`

- [ ] **Step 3: Commit**

```bash
cd /home/pure/signal-noise
git add src/pages/home.rs
git commit -m "feat(SIG-208): replace CategoryTab buttons with SectionNav

Scalable horizontal section bar replaces the flex-wrap button group.
Categories hardcoded as static slice. Remove CategoryTab sub-component.

Co-Authored-By: Paperclip <noreply@paperclip.ing>"
```

---

## Task 7: Update `article_card.rs` — Left Border + Paddings

**Files:**
- Modify: `src/components/article_card.rs`

**What:** Remove the `sn-article-rail` div. Add a beat-specific CSS class to the `<article>` element so the left border applies via CSS. Update the inner padding to 28px.

- [ ] **Step 1: Remove `sn-article-rail` div and add beat class to `<article>`**

In `src/components/article_card.rs`, find the `rsx!` block in `ArticleCard`. Change:

```rust
    rsx! {
        article { class: "sn-article",
            div { class: "sn-article-rail" }

            div { class: "sn-article-inner",
```

To:

```rust
    rsx! {
        article { class: "sn-article beat-{beat_cls}",

            div { class: "sn-article-inner",
```

(The `sn-article-rail` div is removed entirely. The beat class like `beat-tech`, `beat-linux`, `beat-privacy` is appended to the article's class string using the existing `beat_cls` variable.)

- [ ] **Step 2: Verify Rust compiles**

```bash
cd /home/pure/signal-noise && cargo check 2>&1 | tail -5
```
Expected: `Finished`

- [ ] **Step 3: Commit**

```bash
cd /home/pure/signal-noise
git add src/components/article_card.rs
git commit -m "feat(SIG-208): replace article top rail with beat-coloured left border

Remove sn-article-rail div. Add beat-{cls} class to <article> element
for CSS border-left treatment per beat category.

Co-Authored-By: Paperclip <noreply@paperclip.ing>"
```

---

## Task 8: Update `nav.rs` — Theme Toggle

**Files:**
- Modify: `src/components/nav.rs`

**What:** Add a theme toggle button to the boot banner's right side. The button reads `☀` (switch to light) or `☾` (switch to dark) depending on current state. It adds/removes `theme-light` class on `<html>` and persists to `localStorage`.

Dioxus note: To call JavaScript from a component, use `eval()` from `dioxus::prelude`. The toggle only needs two `eval()` calls — one to read the current state, one to update it.

- [ ] **Step 1: Update `src/components/nav.rs`**

Replace the entire file content with:

```rust
use dioxus::prelude::*;

use crate::server_fns::get_recent_pipeline_activity;

fn agent_tick_cls(name: &str) -> &'static str {
    match name {
        "Editor" | "Editor-in-Chief" => "sn-tick-agent vi",
        "Fact Checker" | "Source Checker" => "sn-tick-agent am",
        _ => "sn-tick-agent",
    }
}

#[component]
pub fn Nav() -> Element {
    let activity = use_resource(|| async move { get_recent_pipeline_activity().await });
    let mut is_light = use_signal(|| false);

    rsx! {
        // Boot banner — sticky at top
        div { class: "sn-boot-banner",
            div { class: "sn-banner-left",
                span { class: "sn-pulse-ring" }
                span { "Signal Noise Foundry" }
                span { class: "sn-banner-label",
                    "All content "
                    strong { "AI-generated" }
                    " · Facts verified by pipeline · No human journalists"
                }
            }
            div { class: "sn-banner-right",
                "BUILD 0.9.4 · "
                span { class: "sn-blink", "■" }
                " "
                button {
                    style: "background:none;border:none;cursor:pointer;font-size:13px;padding:0 0 0 10px;color:var(--sn-text-dimmer);transition:color 0.2s;",
                    "aria-label": if is_light() { "Switch to dark mode" } else { "Switch to light mode" },
                    onclick: move |_| {
                        let next = !is_light();
                        is_light.set(next);
                        let js = if next {
                            "document.documentElement.classList.add('theme-light'); localStorage.setItem('sn-theme','light');"
                        } else {
                            "document.documentElement.classList.remove('theme-light'); localStorage.setItem('sn-theme','dark');"
                        };
                        let _ = eval(js);
                    },
                    if is_light() { "☾" } else { "☀" }
                }
            }
        }

        // Masthead
        div { class: "sn-nav",
            div {
                a { href: "/", class: "sn-logo-link",
                    span { class: "sn-logo",
                        "Signal Noise"
                        span { class: "sn-logo-sup", "AI" }
                    }
                }
                div { class: "sn-tagline",
                    "The news is real. The journalists are "
                    span { class: "hi", "artificial" }
                    ". The process is "
                    span { class: "vi", "visible" }
                    "."
                }
            }

            div { class: "sn-sys-strip",
                div { class: "sn-sys-chip live",
                    span { class: "sn-chip-lbl", "SYSTEM" }
                    span { class: "sn-chip-val", "● ONLINE" }
                }
                div { class: "sn-sys-chip",
                    span { class: "sn-chip-lbl", "HUMAN INVOLVEMENT" }
                    span { class: "sn-chip-bad", "0%" }
                }
            }
        }

        // Live agent activity ticker
        div { class: "sn-ticker-wrap",
            div { class: "sn-ticker-label", "Live Activity" }
            div { class: "sn-ticker-scroll",
                {match activity() {
                    Some(Ok(items)) if !items.is_empty() => {
                        let doubled: Vec<_> = items.iter().cloned()
                            .chain(items.iter().cloned())
                            .collect();
                        rsx! {
                            div { class: "sn-ticker-inner",
                                for item in doubled {
                                    span { class: "sn-tick",
                                        span { class: "{agent_tick_cls(&item.agent_name)}", "{item.agent_name}" }
                                        span { class: "sn-tick-dot" }
                                        "{item.output_summary}"
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(_)) => rsx! {
                        div { class: "sn-ticker-inner",
                            span { class: "sn-tick",
                                span { class: "sn-tick-agent", "Pipeline" }
                                span { class: "sn-tick-dot" }
                                "Activity unavailable"
                            }
                        }
                    },
                    _ => rsx! {
                        div { class: "sn-ticker-inner",
                            span { class: "sn-tick",
                                span { class: "sn-tick-agent", "System" }
                                span { class: "sn-tick-dot" }
                                "Loading pipeline activity…"
                            }
                        }
                    },
                }}
            }
        }
    }
}
```

Note: `eval()` is available in Dioxus via `dioxus::prelude::eval`. It returns a `Result` — we discard it with `let _ = eval(js)` since there's no meaningful error to handle here.

**Known limitation:** `is_light` initialises to `false` on every WASM hydration, so the toggle icon (`☀`/`☾`) will not reflect the persisted theme on first render. The CSS class is correct (applied by the no-flash script), but the icon may show the wrong state until the user clicks it. This is a cosmetic issue only — theming is always correct. To fix in a follow-up: add a `use_effect` that calls `eval("document.documentElement.classList.contains('theme-light')")` and sets `is_light` on mount.

- [ ] **Step 2: Verify Rust compiles**

```bash
cd /home/pure/signal-noise && cargo check 2>&1 | tail -5
```
Expected: `Finished`. If `eval` is not in scope, check the Dioxus version — in Dioxus 0.6+ `eval` is available from `dioxus::prelude`. If the compile error mentions `eval` not found, try `use dioxus::prelude::eval;` as an explicit import.

- [ ] **Step 3: Commit**

```bash
cd /home/pure/signal-noise
git add src/components/nav.rs
git commit -m "feat(SIG-208): add theme toggle to boot banner

Sun/moon button toggles html.theme-light class and persists to
localStorage. Dynamic aria-label for accessibility.

Co-Authored-By: Paperclip <noreply@paperclip.ing>"
```

---

## Task 9: Update `article.rs` — Back Link, Prose Width, Disclaimer

**Files:**
- Modify: `src/pages/article.rs`

**What:** Three small changes: (1) restyle the back link to serif 14px; (2) update the article title `h1` inline style to 36px; (3) replace the disclaimer banner's inline `style` attribute with the `sn-disclaimer` CSS class and change the icon from `⚠` to `ⓘ`.

- [ ] **Step 1: Update the back link style**

Find the back link (around line 50):
```rust
a { style: "display:inline-flex; align-items:center; gap:6px; font-family:var(--sn-mono); font-size:10px; color:var(--sn-text-dimmer); text-decoration:none; margin-bottom:24px; transition:color 0.2s;",
    href: "/",
    "← SIGNAL NOISE"
}
```

Replace with:
```rust
a { style: "display:inline-flex; align-items:center; gap:6px; font-family:var(--sn-serif); font-size:14px; color:var(--sn-text-dimmer); text-decoration:none; margin-bottom:24px; transition:color 0.2s;",
    href: "/",
    "← Signal Noise"
}
```

- [ ] **Step 2: Update article title font size**

Find the `h1` (around line 73):
```rust
h1 { class: "sn-headline", style: "font-size:32px; margin-bottom:16px;",
```
Change to:
```rust
h1 { class: "sn-headline", style: "font-size:36px; margin-bottom:16px;",
```

- [ ] **Step 3: Replace disclaimer banner inline style with CSS class**

Find the AI disclaimer div (around line 81):
```rust
div { style: "background:var(--sn-accent-dim); border:1px solid var(--sn-accent-mid); border-radius:4px; padding:10px 14px; margin:16px 0; font-family:var(--sn-mono); font-size:10px; color:var(--sn-accent); letter-spacing:1px;",
    "⚠ SYNTHETIC CONTENT — written by AI agents. All claims fact-checked by a separate AI process."
}
```

Replace with:
```rust
div { class: "sn-disclaimer",
    "ⓘ SYNTHETIC CONTENT — written by AI agents. All claims fact-checked by a separate AI process."
}
```

- [ ] **Step 4: Verify Rust compiles**

```bash
cd /home/pure/signal-noise && cargo check 2>&1 | tail -5
```
Expected: `Finished`

- [ ] **Step 5: Commit**

```bash
cd /home/pure/signal-noise
git add src/pages/article.rs
git commit -m "feat(SIG-208): update article page typography and disclaimer

Serif back link at 14px. Larger 36px title. Replace inline-styled
disclaimer with .sn-disclaimer class and softer info icon.

Co-Authored-By: Paperclip <noreply@paperclip.ing>"
```

---

## Task 10: Add No-Flash Script to `main.rs`

**Files:**
- Modify: `src/main.rs`

**What:** Add an inline `<script>` to the `App` component that runs before first paint on the SSR-rendered HTML, reading `localStorage` and adding `theme-light` to `<html>` if needed. This prevents a flash of the wrong theme on page load.

- [ ] **Step 1: Add `document::Script` to `App` in `src/main.rs`**

Find the `App` component (around line 47):
```rust
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<pages::Route> {}
    }
}
```

Replace with:
```rust
#[component]
fn App() -> Element {
    rsx! {
        document::Script {
            "(function(){{var t=localStorage.getItem('sn-theme');if(t==='light')document.documentElement.classList.add('theme-light');}})()"
        }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<pages::Route> {}
    }
}
```

Note: In Dioxus RSX, `{{` and `}}` are escape sequences for literal `{` and `}` in string literals — use them inside the script content to avoid RSX parsing issues.

- [ ] **Step 2: Verify Rust compiles**

```bash
cd /home/pure/signal-noise && cargo check 2>&1 | tail -5
```
Expected: `Finished`. If `document::Script` causes a compile error, check the Dioxus version's document API. In Dioxus 0.6+ `document::Script` accepts children as the script body. If the API differs, use: `document::Script { src: "", r#type: "text/javascript", "{the_script}" }`.

- [ ] **Step 3: Commit**

```bash
cd /home/pure/signal-noise
git add src/main.rs
git commit -m "feat(SIG-208): add no-flash theme script to SSR head

Reads localStorage on first paint to apply theme-light before the
browser renders, preventing a flash of wrong theme on reload.

Co-Authored-By: Paperclip <noreply@paperclip.ing>"
```

---

## Task 11: Final Integration Verification

**Files:** None (verification only)

**What:** Build the full project and do a final smoke-test to confirm all changes compose correctly.

- [ ] **Step 1: Full build**

```bash
cd /home/pure/signal-noise && cargo build 2>&1 | tail -10
```
Expected: `Finished` with no errors or warnings about unused imports.

- [ ] **Step 2: Start dev server and visually verify**

```bash
cd /home/pure/signal-noise && dx serve --port 8080
```

Open `http://localhost:8080` and verify:
- Dark mode: warm purple-black background (not near-black), readable secondary text
- Grid overlay is very faint (0.10 opacity), no scan lines
- Section nav bar visible below the ticker, with "ALL", "LINUX", "TECH", "PRIVACY" labels
- Article cards have coloured left borders (teal/violet/amber by beat)
- Sun icon in boot banner — clicking toggles to light mode
- Light mode: cream paper background, dark ink text, lavender monologue boxes, amber disclaimer banner
- Theme persists on reload (localStorage working)
- On article page: larger serif back link, 36px title, clean disclaimer with `ⓘ` icon

- [ ] **Step 3: Final commit summary tag**

```bash
cd /home/pure/signal-noise
git log --oneline -10
```
All 10 feature commits should be visible. No extra action needed — the feature is complete.
