# Signal Noise Frontend Redesign — Design Spec

**Issue:** SIG-208
**Date:** 2026-04-04
**Author:** Frontend Engineer
**Status:** Approved

## Overview

Signal Noise's current frontend uses an extremely dark near-black colour scheme (`#070810`) with low-contrast secondary text (`#40435a`), scan-line overlays, and heavy monospace font usage throughout. The result reads more like a hacker terminal than an AI newspaper. Readers struggle with legibility.

This spec defines a redesign to **Option B: Warm Dark primary + Newsprint light mode**, making Signal Noise the prettiest AI newspaper on the web while preserving the transparency-first identity.

**Goals:**
- Fix readability in dark mode without abandoning the brand
- Add a warm cream "Newsprint" light mode
- Replace the fragile category button row with a scalable section nav
- Refine AI transparency components to feel editorial, not like debug output

**Non-goals:**
- Changing the information architecture (same pages, same components)
- Rewriting the Dioxus component tree structure
- Altering any API or data model

---

## Section 1: Colour System

### Dark Mode — "Warm Ink"

Shift the palette from cold blue-black to warm purple-black. Feels like a newsroom at midnight rather than a terminal.

| CSS Variable | Current | New | Rationale |
|---|---|---|---|
| `--sn-bg` | `#070810` | `#16141a` | Warmer, less oppressive |
| `--sn-bg-raised` | `#0d0f1a` | `#1d1b24` | Visible elevation lift |
| `--sn-bg-card` | `#10121e` | `#221f2c` | Clear card distinction |
| `--sn-bg-deep` | `#080a14` | `#110f17` | Deeper inset elements |
| `--sn-text` | `#e4e4f0` | `#f0eef8` | Slightly warmer white |
| `--sn-text-dim` | `#9a9db4` | `#a09ab8` | Comfortable secondary |
| `--sn-text-dimmer` | `#40435a` | `#6b658a` | **Critical fix** — was near-invisible |
| `--sn-border` | `#1c1f32` | `#2d2840` | Visible structure |
| `--sn-violet` | `#7c6bff` | `#9d8fff` | Warmer, less clinical |
| `--sn-accent` | `#00e5a0` | `#00e5a0` | Keep — works well |
| `--sn-amber` | `#ffb224` | `#ffb224` | Keep |
| `--sn-red` | `#ff4f4f` | `#ff5f5f` | Slightly softer |

**Overlay changes (dark mode):**
- `body::after` (scan-line effect) — **remove entirely**
- `body::before` (grid pattern) — reduce `opacity` from `0.3` → `0.10`

### Light Mode — "Newsprint"

A warm cream paper theme. Added via `html.theme-light` CSS class overrides.

| CSS Variable | Light Value | Notes |
|---|---|---|
| `--sn-bg` | `#f8f5ef` | Warm cream/newsprint |
| `--sn-bg-raised` | `#f0ece4` | Slightly warm off-white |
| `--sn-bg-card` | `#ffffff` | Clean white cards |
| `--sn-bg-deep` | `#ede9e0` | Inset/well elements |
| `--sn-text` | `#1a1523` | Dark warm ink |
| `--sn-text-dim` | `#5c5675` | Medium gray-purple |
| `--sn-text-dimmer` | `#9b95ae` | Visible on cream |
| `--sn-border` | `#e2ddd6` | Warm light gray |
| `--sn-border-glow` | `rgba(0,122,85,0.15)` | Teal glow on cream |
| `--sn-accent` | `#007a55` | Darker teal, readable on cream |
| `--sn-accent-dim` | `rgba(0,122,85,0.08)` | |
| `--sn-accent-mid` | `rgba(0,122,85,0.25)` | |
| `--sn-violet` | `#5b4fcf` | Darker purple |
| `--sn-violet-dim` | `rgba(91,79,207,0.08)` | |
| `--sn-violet-mid` | `rgba(91,79,207,0.25)` | |
| `--sn-amber` | `#b87a00` | Darker amber |
| `--sn-red` | `#cc3333` | Darker red |

**Overlay changes (light mode):** both `body::before` and `body::after` removed entirely. Clean paper.

### Theme Toggle

- A sun/moon `<button>` in the nav bar right side
- Toggles `class="theme-light"` on `<html>`
- Preference persisted to `localStorage` key `sn-theme`
- Default: dark
- No flash-of-wrong-theme: initial theme applied via an inline `<script>` in `<head>` before paint

Implementation: the toggle is a small icon button in the boot banner (right side, next to the build info). Label: `☀` / `☾`. Accessible with `aria-label="Switch to light mode"`.

---

## Section 2: Layout & Navigation

### Category Section Bar

Replace the current flex-wrapped `<button>` group with a dedicated `<nav>` strip between the ticker and the feed.

**Visual design:**
```
[ ALL ]  [ LINUX ]  [ TECH ]  [ PRIVACY ]  [ AI ]  [ HARDWARE ] ...
         ───────
```

- Active section indicated by a `3px` bottom border in `--sn-accent`, no background fill
- Inactive items: uppercase small caps, `--sn-text-dimmer` colour, hover → `--sn-text-dim`
- Container: `overflow-x: auto; scrollbar-width: none; -webkit-overflow-scrolling: touch`
- Scales to any number of categories — extras are horizontally scrollable
- Tap targets minimum `44px` height for mobile

**CSS class:** `.sn-section-nav` (new), `.sn-section-nav-item`, `.sn-section-nav-item.active`

**Spacing:** `padding: 0 48px; margin: 16px 0 0;` with a `1px` bottom border on the container in `--sn-border`.

### Home Feed Grid

| Property | Current | New |
|---|---|---|
| `grid-template-columns` | `1fr 340px` | `1fr 300px` |
| `gap` | `28px` | `32px` |
| Article card `margin-bottom` | `22px` | `24px` |
| Article card `padding` | `24px` | `28px` |

### Article Cards

Dark mode: replace the `3px` top accent rail with a `3px` left border spanning full card height, coloured by beat:
- `tech`: `--sn-accent`
- `linux`: `--sn-violet`
- `privacy`: `--sn-amber`

Light mode: replace border approach with a subtle `box-shadow: 0 1px 4px rgba(0,0,0,0.08), 0 0 0 1px var(--sn-border)`. Feels like paper cards.

### Article Page Layout

| Property | Current | New |
|---|---|---|
| `max-width` | `680px` (prose) | `720px` |
| Right rail width | `300px` | `280px` |
| Article page padding | `32px 48px` | `40px 48px` |

The back link ("← SIGNAL NOISE") is styled with the serif font at `14px`, not monospace at `10px`. More prominent, more editorial.

### Typography

| Element | Current | New |
|---|---|---|
| Card headline | `24px` | `26px`, `font-weight: 700` |
| Article title | `32px` | `36px`, `font-weight: 700` |
| Body line-height | `1.85` | `1.9` |
| Error/empty states | monospace | DM Sans |

Monospace font reserved for: timestamps, beat tags, labels, AI metadata panels. Not for general UI copy.

---

## Section 3: AI Transparency Components

### AI Monologue Box

**Dark mode:** Background `#1e1a2e` (warm purple), left border `3px` solid `--sn-violet`, border-radius `4px` on all corners (currently asymmetric). Font size `11px` → `12px`. Pulse dot on label header retained.

**Light mode:** Background `#f0edf8` (soft lavender), left border `3px` solid `--sn-violet`, text `#3d3560`. Styled like an editor's annotation in the margin — familiar, not technical.

### Confidence Meter

**Both modes:** Bar height `3px` → `5px`. Track becomes `6px` tall total with `3px` border-radius. Labels get `2px` more vertical padding.

**Light mode:** Track background `#e2ddd6`. Score value colours use the light-mode accent variants.

### Beat Tags

**Dark mode:** No structural change; colours update with the new palette variables automatically.

**Light mode:** Solid background tints with darker text:
- `tech`: `rgba(0,122,85,0.10)` bg, `#007a55` text/border
- `linux`: `rgba(91,79,207,0.10)` bg, `#5b4fcf` text/border
- `privacy`: `rgba(184,122,0,0.10)` bg, `#b87a00` text/border

### Pipeline Trail

**Dark mode:** Step markers gain `box-shadow: 0 0 8px rgba(0,229,160,0.3)`. Connecting line `1px` → `1.5px`. Step type labels `font-weight: 500` → `600`.

**Light mode:** Markers become dark ink circles (`border: 1.5px solid #1a1523`) with white fill, accent-coloured step number. Connecting line is `--sn-border`. Reads like an editorial version history.

### Source Block

**Dark mode:** No structural changes beyond colour token updates.

**Light mode:** Source URLs in `--sn-accent` (`#007a55`). Verified dot stays green. Paywall badge: amber on cream background. Row hover: `--sn-bg-raised`. No card background needed — list floats on cream.

### AI Disclaimer Banner

**Dark mode:** Padding `10px 14px` → `12px 16px`. Font size `10px` → `11px`. Icon `⚠` → `ⓘ` (less alarm, more transparency).

**Light mode:** Background `#fff8e6`, text `#8a6200`, border `#d4a000`. Reads like a newspaper's standard syndication/disclosure note.

---

## Implementation Notes

### CSS Architecture

All light-mode overrides live under `html.theme-light { }` in `tailwind.css`. No separate stylesheet. The toggle adds/removes the class on `<html>`.

Variable redeclarations follow this pattern:
```css
html.theme-light {
  --sn-bg: #f8f5ef;
  --sn-bg-card: #ffffff;
  /* ... all overrides ... */
}
```

Components use CSS variables throughout — no hardcoded colours in component files. The theme switch is therefore zero-JS beyond toggling the class.

### Section Nav Component

New `SectionNav` Dioxus component replacing the inline `CategoryTab` buttons in `home.rs`. Props: `categories: Vec<(String, String)>` (label, value), `active: Option<String>`, `on_select: EventHandler<Option<String>>`.

### Theme Toggle Placement

Added to the `Nav` component's boot banner right side, replacing the static "SYS.UP" text or sitting alongside it.

### No-Flash Script

Inline `<script>` added to the `App` component's `<head>` (via `document::Script`):
```js
(function(){
  var t = localStorage.getItem('sn-theme');
  if (t === 'light') document.documentElement.classList.add('theme-light');
})();
```

---

## Files Changed

| File | Change |
|---|---|
| `src/styles/tailwind.css` | Palette update, overlay removal, new section nav styles, light-mode overrides |
| `src/components/nav.rs` | Add theme toggle button + `SectionNav` component |
| `src/pages/home.rs` | Use `SectionNav` instead of inline `CategoryTab` buttons |
| `src/pages/article.rs` | Updated back link style, wider prose |
| `src/components/article_card.rs` | Left border instead of top rail, updated paddings |
