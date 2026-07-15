# Editor Navbar Restructure Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Reshape the editor navbar (mode tabs + race tabs) so it's taller and well-proportioned on every band — a compact-width, full-height mode block; tall portrait race hero cards on tablet+; and a horizontal-scroll carousel on mobile.

**Architecture:** Pure per-band styling via the project's host + `cqi` fill cascade. `editor_tabs_bar` owns ONE per-band height (the only absolute size at the top); `mode_tabs_host` and `race_tabs_host` are `@container` box-owners that fill it; leaves draw their interior in `cqi` measured live off the real host width. No render-tree / directory-tree changes, no domain logic — presentation only.

**Tech Stack:** Rust + Dioxus (wasm), Tailwind via the `tw!` / `classes!` macros, `moon run :check` (compile), Playwright MCP (browser — the only real signal).

## Global Constraints

- **The browser is the test.** There are no unit tests for Tailwind styling. After every edit: `touch crates/hotkey-editor/tailwind.css` (the watcher is inotify-blind to new classes) → `moon run :check` (compile) → Playwright MCP measure + screenshot at the affected bands. **NEVER run `moon run :ci`** — the human runs that ~7-min gate.
- **Dev server is already running** (the human's) at `http://localhost:8123/warcraft-hotkey-editor/` (trailing slash; bare `/` is 404). Do not start one. Refresh to clear any "being rebuilt" overlay; don't wait on it.
- **Exact `cqi` values are MEASURED LIVE:** `cqi = round(100 * px / hostContentWidth, 2)`, hostContentWidth = the nearest `@container` ancestor's content-box width. The class lists below carry **starting** values (estimates); confirm/adjust each in the browser.
- **No `clamp`.** Per-band absolute sizes at the top; `cqi` / fill below. Keep `text-*` / `leading-*` / `rounded-*` / `border` / colors as design tokens; only gaps, padding, and box sizes become `cqi`.
- **Do NOT restyle the shared `ToggleButton`** (`editor_page/components/shared/toggle_button`, used beyond mode tabs) unless a change is verified safe for every consumer.
- **Render-tree == directory-tree unchanged;** the `super::` test must stay clean (`grep 'use super::' … | grep -v 'super::(props|state|logic|style|hooks|data|model|view|presentation|components)'` → empty).
- **Commits are human-triggered** (`develop` is throwaway, squashed on merge). The "Checkpoint" step in each task marks a reviewable point; batch or commit per the human's direction.
- **Bands:** mobile `<768` · tablet `768–1279` · laptop `1280–1919` · desktop `1920–2559` · qhd `2560–3839` · uhd `≥3840`. `base` (unprefixed) applies everywhere unless a band overrides it.
- **Design doc:** `docs/superpowers/specs/2026-07-15-editor-navbar-restructure-design.md`.

## File Structure

All paths under `crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_tabs_bar/` (abbreviated `…/editor_tabs_bar/` below). Only `style/mod.rs` files change; no `.rs` logic, no new components.

- `…/editor_tabs_bar/style/mod.rs` — owns the per-band navbar **height** (Task 1).
- `…/components/mode_tabs_host/style/mod.rs` — `contents` → `@container` box-owner with the compact per-band **width** (Task 2).
- `…/components/mode_tabs_host/components/mode_tabs/style/mod.rs` — fills the host, `cqi` gap (Task 2).
- `…/components/race_tabs_host/style/mod.rs` — fills the taller bar; drop interim `h-40` (Tasks 1 & 4).
- `…/components/race_tabs_host/components/race_tabs/style/mod.rs` — carousel behavior on mobile (Task 4).
- `…/…/race_tabs/components/race_tab_banner/components/{human,orc,nightelf,undead,neutral}_race_tab/style/mod.rs` — the 5 wrappers: center on tablet+, carousel-item on mobile (Tasks 3 & 4).
- `…/…/shared/race_tab_state/components/shared/race_tab/style/mod.rs` — the base card (`RaceTab`, used by both active+inactive): portrait on tablet+, carousel size on mobile (Tasks 3 & 4).
- `…/…/race_tab_state/components/active_race_tab/style/mod.rs` — the active wrapper (holds `RaceTab` + the `inset-0` accent): must shrink-wrap the portrait card so the ring matches (Tasks 3 & 4).
- `…/…/race_tab/components/race_tab_label/style/mod.rs` — label proportions in the taller/ carousel card (Tasks 3 & 4).

**Active vs inactive asymmetry (important):** `RaceTabState` renders either `InactiveRaceTab` (which renders `RaceTab` **directly**) or `ActiveRaceTab` (a `relative size-full` div holding `RaceTab` **plus** an `absolute inset-0` `ActiveAccent` ring). So the portrait box lives on **`RaceTab`** (both use it), and **`ActiveRaceTab` must shrink-wrap it (`w-fit`)** so its accent ring is portrait-shaped too — editing only `RaceTab` would leave the active card's ring full-width. `InactiveRaceTab` needs no change (it has no style; the wrapper's `justify-center` centers the portrait `RaceTab`).

---

### Task 1: Per-band bar height (tablet+); race host fills it

Make `editor_tabs_bar` tall per band. Race host already fills via `self-stretch`; just remove the `tablet:h-40` cap so tablet races fill the new height. Mobile is left untouched here (fixed in Task 4).

**Files:**
- Modify: `…/editor_tabs_bar/style/mod.rs`
- Modify: `…/components/race_tabs_host/style/mod.rs`

**Interfaces:**
- Produces: a per-band-tall `.editor-tabs-bar`; `.race-tabs-host` fills it on tablet+ (still `@container grow self-stretch min-w-0`).

- [ ] **Step 1: Baseline screenshots (so "before" is captured)**

Resize + element-screenshot `.editor-tabs-bar` at tablet (1024×768), desktop (1920×1080), uhd (3840×2160) via Playwright MCP. Keep for comparison.

- [ ] **Step 2: Set the per-band height on the bar**

`…/editor_tabs_bar/style/mod.rs`:

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "items-stretch",
        "flex-none",
        "gap-10",
        "min-h-[300px]",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:min-h-0",
        "mobile:gap-2.5",
    ],
    tablet: tw![
        "tablet:min-h-[290px]",
    ],
    desktop: tw![
        "desktop:min-h-[320px]",
    ],
    qhd: tw![
        "qhd:min-h-[380px]",
    ],
    uhd: tw![
        "uhd:min-h-[440px]",
    ],
}
```

(`base` `min-h-[300px]` covers laptop; `mobile:min-h-0` keeps mobile content-sized; the other bands override per the spec table.)

- [ ] **Step 3: Let tablet races fill the taller bar**

`…/components/race_tabs_host/style/mod.rs` — remove the `tablet` block (keep `mobile:h-40` for now so mobile stays the current 5-across until Task 4):

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "grow",
        "self-stretch",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:h-40",
    ],
}
```

- [ ] **Step 4: Rebuild CSS + compile**

Run: `touch crates/hotkey-editor/tailwind.css && moon run :check 2>&1 | tail -5`
Expected: `Finished`, no errors.

- [ ] **Step 5: Browser-verify height per band**

Reload the app. At tablet/desktop/qhd/uhd measure `.editor-tabs-bar` height (Playwright `getBoundingClientRect`). Expected ≈ 290 / 320 / 380 / 440. Confirm `.race-tabs` and `.mode-tabs` both fill the height and nothing collapses (`width:0`). Element-screenshot each. Mobile unchanged (still ~196). Cards will look tall-and-wide / mode buttons tall — that's expected here; fixed in Tasks 2–3. Adjust the px values if a band looks wrong.

- [ ] **Step 6: Checkpoint**

`super::` test still clean (no structural change). Reviewable point: "navbar height grows per band (tablet+)."

---

### Task 2: Compact-width, full-height mode block

Turn `mode_tabs_host` from a no-box `contents` wrapper into the `@container` box-owner carrying a compact per-band width (down from 34–62rem); `mode_tabs` fills it and its inter-button gap becomes `cqi`. Two `ToggleButton`s (`flex-1`) then fill to ~half the bar height each.

**Files:**
- Modify: `…/components/mode_tabs_host/style/mod.rs`
- Modify: `…/components/mode_tabs_host/components/mode_tabs/style/mod.rs`

**Interfaces:**
- Consumes: the tall bar from Task 1.
- Produces: `.mode-tabs-host` = `@container` with a definite compact width; `.mode-tabs` fills it.

- [ ] **Step 1: Make the host the box-owner (compact per-band width)**

`…/components/mode_tabs_host/style/mod.rs`:

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "self-stretch",
        "min-w-0",
        "w-[224px]",
    ],
    mobile: tw![
        "mobile:w-full",
    ],
    tablet: tw![
        "tablet:w-[208px]",
    ],
    qhd: tw![
        "qhd:w-[260px]",
    ],
    uhd: tw![
        "uhd:w-[300px]",
    ],
}
```

(`base` `w-[224px]` covers laptop+desktop; mobile is full-width for the stacked layout; tune each live in Step 4.)

- [ ] **Step 2: Fill the host from `mode_tabs`; gap → `cqi`**

`…/components/mode_tabs_host/components/mode_tabs/style/mod.rs` — drop the old `flex-[0_0_…]` / `w-…` per-band widths (the host owns width now), fill, keep `flex-col`, gap in `cqi` (starting from `gap-2` = 8px off the ~224px host: `8/224 = 3.57`):

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "size-full",
        "min-w-0",
        "gap-[3.57cqi]",
    ],
    mobile: tw![
        "mobile:flex-row",
        "mobile:gap-[2.13cqi]",
    ],
}
```

(mobile `gap-2` = 8px off the full-width mobile host — recompute live in Step 4.)

- [ ] **Step 3: Rebuild + compile**

Run: `touch crates/hotkey-editor/tailwind.css && moon run :check 2>&1 | tail -5`
Expected: `Finished`, no errors.

- [ ] **Step 4: Browser-verify + measure the gap `cqi`**

Reload. At desktop measure `.mode-tabs-host` width (expect ~224) and `.mode-tabs` computed `gap` (expect ~8px). Recompute `cqi = round(100*8/hostWidth,2)` from the REAL host width and correct `gap-[…cqi]` if off. Confirm: mode column is now a narrow strip (no 26% dead space), the two buttons are tall (≈ half the bar height), the reclaimed width went to the race region. Check the button label is still vertically centered in the taller button — if it now sits at the top, DO NOT edit `ToggleButton`; instead note it and raise it (see Step 5). Screenshot desktop + uhd + tablet + mobile.

- [ ] **Step 5: (Conditional) label centering audit**

Only if Step 4 shows the mode label mis-centered in the tall button: `grep -rl "ToggleButton" crates/hotkey-editor/src` to list consumers, confirm a `flex items-center justify-center` on `ToggleButton`'s active/idle roots is safe for all, and if so add it in the same pass; otherwise leave alignment and record it as a follow-up. Re-verify in browser.

- [ ] **Step 6: Checkpoint**

`super::` test clean. Reviewable point: "mode block is compact + full-height."

---

### Task 3: Race hero cards → portrait (tablet+)

Stop the cards stretching wide-and-short. Each card fills the bar height and takes a portrait aspect (width derived from height), centered in its flex slot. Mobile keeps the current 5-across until Task 4.

**Files:**
- Modify: `…/…/shared/race_tab_state/components/shared/race_tab/style/mod.rs`
- Modify (×5): `…/race_tab_banner/components/{human,orc,nightelf,undead,neutral}_race_tab/style/mod.rs`
- Modify (if needed): `…/…/race_tab/components/race_tab_label/style/mod.rs`

**Interfaces:**
- Consumes: tall bar (Task 1), race region width reclaimed from the mode block (Task 2).
- Produces: portrait `.race-tab` cards centered within each `.race-tab` wrapper on tablet+.

- [ ] **Step 1: Make the base card (`RaceTab`) portrait on tablet+**

In `race_tab/style/mod.rs`, replace the fill sizing with fill-height + portrait aspect (width derives from height). Keep every existing look class (border, rounded-card, bg, `before:`/`after:`, hover/kb-focus, `text-shadow-tab`, `after:bg-scrim-bottom`). In the `base` list:
- Remove: `"size-full"`
- Add: `"h-full"`, `"w-auto"`, `"aspect-[3/4]"`

(`h-full` + `w-auto` + `aspect-[3/4]` ⇒ card width = height × 0.75, e.g. desktop 320→240, uhd 440→330 — banner-shaped, not 535-wide. Tune the ratio live if `3/4` reads too narrow/wide.)

- [ ] **Step 2: Shrink-wrap the active accent to the portrait card**

In `active_race_tab/style/mod.rs` (currently `["relative","size-full","[--label-color:var(--race-color)]"]`), make it shrink to the portrait card so the `inset-0` accent ring is portrait-shaped, not full-slot. In the `base` list:
- Remove: `"size-full"`
- Add: `"h-full"`, `"w-fit"`

(`w-fit` = `width: fit-content` ⇒ `ActiveRaceTab` shrinks to the portrait `RaceTab` inside it, so `ActiveAccent`'s `absolute inset-0` ring matches the card. `relative` and `[--label-color…]` stay. `InactiveRaceTab` needs no change.)

- [ ] **Step 3: Center each card in its slot**

Each of the 5 `*_race_tab/style/mod.rs` wrappers is `["flex","flex-1","min-w-0", …vars]`. Add `"justify-center"` to the `base` list of all five so the portrait card — active (`ActiveRaceTab`, `w-fit`) or inactive (`RaceTab`, `w-auto`) — centers, extra slot width becoming gutter. (Leave the `--race-color` / `--banner-*` custom-property lines untouched.)

- [ ] **Step 4: Rebuild + compile**

Run: `touch crates/hotkey-editor/tailwind.css && moon run :check 2>&1 | tail -5`
Expected: `Finished`, no errors.

- [ ] **Step 5: Browser-verify portrait + accent + art (active AND inactive)**

Reload. At desktop / qhd / uhd measure a `.race-tab`: width ≈ height × 0.75 (portrait), not the full slot. Confirm: **inactive** cards are centered portrait with even gutters; the **active** card (Human by default) is the same portrait shape AND its glow ring hugs the card (not a full-slot rectangle) — this proves the `w-fit` fix. Shield art (`before:bg-contain`) reads big and isn't cut/pixelated; label stays bottom-pinned. Screenshot each band; compare to Task-1 "before". Click another race to move the active state and re-check the ring. If art floats or leaves dead space, adjust `before:bg-contain` sizing or the aspect ratio; if labels feel cramped, re-tune `race_tab_label` `cqi` (`cqi = round(100*px/cardContentWidth,2)`).

- [ ] **Step 6: Checkpoint**

`super::` test clean. Reviewable point: "race cards are portrait hero banners (active ring matches) on tablet+."

---

### Task 4: Mobile race carousel

Replace the cramped mobile 5-across with a horizontal-scroll, snap carousel of big portrait cards (~2–3 visible + peek). Only `mobile:` overrides change; tablet+ (Tasks 1–3) is untouched.

**Files:**
- Modify: `…/components/race_tabs_host/style/mod.rs` (drop `mobile:h-40`)
- Modify: `…/…/race_tabs/style/mod.rs` (mobile scroll+snap)
- Modify (×5): the `*_race_tab` wrappers (mobile carousel item)
- Modify: `race_tab/style/mod.rs` (mobile card fills item)
- Modify: `active_race_tab/style/mod.rs` (mobile `w-full` so `w-fit` doesn't collapse)
- Modify (if needed): `editor_tabs_bar/style/mod.rs` (mobile fit) and `race_tab_label` (mobile `cqi`)

**Interfaces:**
- Consumes: portrait card (Task 3).
- Produces: `.race-tabs` scrolls horizontally on mobile; cards are `shrink-0` snap items.

- [ ] **Step 1: Baseline mobile screenshot**

Resize 390×844, element-screenshot `.editor-tabs-bar` (the "before" 5-across).

- [ ] **Step 2: Make the mobile row a snap carousel**

`race_tabs/style/mod.rs` `mobile` block — add scroll + snap (keep `flex-nowrap`, keep the `cqi` gap/padding already there):

- Add to `mobile`: `"mobile:overflow-x-auto"`, `"mobile:snap-x"`, `"mobile:snap-mandatory"`
- Remove from `mobile`: `"mobile:overflow-visible"` (conflicts with the scroll)

- [ ] **Step 3: Make each card a fixed-width snap item on mobile**

Each of the 5 `*_race_tab` wrappers `mobile` behavior: drop the fill-to-fit and give a carousel width (~40% of viewport so ~2–3 show + peek). Add a `mobile:` block to each wrapper's `classes!`:

- Add: `"mobile:shrink-0"`, `"mobile:basis-[40vw]"`, `"mobile:snap-start"`
- (The base `flex-1` still applies on tablet+; `mobile:shrink-0` + `mobile:basis-[40vw]` overrides the fit-to-width on mobile so the row overflows and scrolls.)

- [ ] **Step 4: Size the mobile card (fill the carousel item)**

The carousel item (wrapper) is now `40vw` with a definite box, so the card fills it instead of deriving a portrait width. Two files:
- `race_tab/style/mod.rs` — add to its `mobile` block: `"mobile:w-full"`, `"mobile:h-full"`, `"mobile:aspect-auto"` (overrides the tablet+ `w-auto aspect-[3/4]` so the item's box governs).
- `active_race_tab/style/mod.rs` — add a `mobile` block with `"mobile:w-full"` (overrides the tablet+ `w-fit`: a `w-fit` wrapper around a `w-full` mobile card would collapse to zero; on mobile it must fill the item so the accent still matches).

Tune `basis-[40vw]` so ~2–3 cards + a peek show and the card isn't overly tall for the mobile band.

- [ ] **Step 5: Free the mobile height + fit the band**

`race_tabs_host/style/mod.rs`: remove `mobile:h-40` (carousel height now comes from the card/item). If the mobile navbar no longer reads ~190 or the mode row and carousel don't both fit, set an explicit `mobile:min-h-[190px]` on `editor_tabs_bar` and/or a `mobile:h-[…]` on the race host — decide live from the measured result.

- [ ] **Step 6: Rebuild + compile**

Run: `touch crates/hotkey-editor/tailwind.css && moon run :check 2>&1 | tail -5`
Expected: `Finished`, no errors.

- [ ] **Step 7: Browser-verify the carousel**

Reload at 390×844. Confirm: the race row scrolls horizontally, ~2–3 large portrait cards show with a clear peek of the next, `scroll-snap` snaps cleanly, the mode row above is a real-height (~52px) Melee|Campaign pair, and the whole navbar is ~190px. Scroll the row (`page.mouse.wheel` / drag) and screenshot start + mid-scroll. Also spot-check 360px and 430px widths. Re-measure the label `cqi` off the mobile card width and correct if cramped. Adjust `basis-[40vw]` for the best peek.

- [ ] **Step 8: Checkpoint**

`super::` test clean. Reviewable point: "mobile races are a scroll-snap carousel."

---

### Task 5: Full cross-band verification + cleanup

Confirm the whole navbar end-to-end and that no compliance rule regressed.

**Files:** none (verification), unless a fix is needed.

- [ ] **Step 1: Six-band sweep**

For mobile 390 / tablet 1024 / laptop 1440 / desktop 1920 / qhd 2560 / uhd 3840: element-screenshot `.editor-tabs-bar` and eyeball against the spec — taller bar, compact full-height mode block, portrait race cards (tablet+) / carousel (mobile), no wide-short banners, no dead space. Fix any band that looks off (adjust its per-band value), then re-verify that band.

- [ ] **Step 2: Compliance greps**

Run, over the `editor_tabs_bar` subtree:
```bash
S=crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_tabs_bar
grep -rn 'use super::' $S --include=mod.rs | grep -viE 'super::(props|state|logic|style|hooks|data|model|view|presentation|components)\b' || echo "super:: clean"
grep -rnoE '\b(p[trblxy]?|gap|m[trblxy]?|inset)-\[[0-9.]+(px|rem|vw)\]' $S --include='*/style/mod.rs' | grep -v editor_tabs_bar/style || echo "no stray px/rem/vw spacing in leaves"
grep -rn 'clamp' $S --include='*/style/mod.rs' || echo "no clamp"
```
Expected: `super:: clean`, `no clamp`. (Per-band absolute `min-h`/`w` on `editor_tabs_bar`/`mode_tabs_host` are the sanctioned top-of-tree scaffolds; leaf spacing should be `cqi`/`em`/tokens.)

- [ ] **Step 3: Final compile**

Run: `touch crates/hotkey-editor/tailwind.css && moon run :check 2>&1 | tail -5`
Expected: `Finished`, no errors.

- [ ] **Step 4: Handoff**

Report the six before/after screenshots. Remaining optional polish (deferred, per spec §7): auto-scroll the active race into view in the mobile carousel (a presentation-only `use_effect` + node ref). The human runs `moon run :ci` as the final gate.
