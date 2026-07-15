# Editor navbar restructure — compact mode block + tall race hero cards

**Date:** 2026-07-15
**Status:** Design — composition + height approved; pending spec review.
**Scope:** `editor_page/components/editor_tabs_bar` and its two subtrees
(`mode_tabs_host`, `race_tabs_host`). **Re-proportioning only — no render-tree /
directory-tree changes.**

---

## 1. Problem

The editor navbar (mode tabs on the left, race tabs on the right) behaves badly on
every band. Measured live in the browser (Human unit selected):

| band | navbar height | mode button | race banner | issue |
|---|---|---|---|---|
| mobile 390 | 196 | 184 × **26** | 70 × 152 | mode row is a **26px sliver** vs 152px banners |
| tablet 1024 | 160 | 288 × 76 | 131 × 152 | passable but short |
| desktop 1920 | **144** | **544 × 68** | 254 × 144 | mode buttons wide + squat, big dead space |
| uhd 3840 | **144** | **992 × 68** | **535 × 144** | thin ribbon on a 4K screen; banners squished ~3.7:1 |

**Root cause (one flaw, cascading):** the navbar height is frozen at `min-h-36`
(144px) from laptop through UHD. Width keeps growing, height does not, so:

- **Race banners never grow taller** — base has no height; only mobile/tablet pin
  `h-40`. From desktop up they stretch **wide-and-short** instead of staying
  portrait "banner" shaped.
- **Mode column uses fixed rem widths** (`w-136`→`w-248` = 544→992px) → ~26% of a
  4K screen for two buttons, buttons stay 68px short → **wide, empty, squat bars**.
- **Mobile mode row collapses to 26px** — the "way too thin" complaint.

## 2. Goal

A grander, balanced navbar from mobile to UHD: taller per band (≥2× at tablet+),
race banners as **tall portrait hero cards**, mode as a **compact-width,
full-height stacked toggle** — built with the project's host + `cqi` fill cascade
(one per-band absolute height at the top, fill all the way down, `cqi` leaves),
and **browser-verified at every band**. This mirrors `shell/header` / `shell/footer`
and the just-completed `race_tabs` conversion.

## 3. Approved decisions

- **Composition:** side-by-side (`mode | races`) at tablet+, stacked (`flex-col`)
  at mobile. **Structure unchanged** — this is already the layout.
- **Height** — the one per-band knob, a `min-h` step on `editor_tabs_bar` (no
  `clamp`):

  | band | new height | was |
  |---|---|---|
  | mobile | 190 | 196 (mode row 26→~52) |
  | tablet | 290 | 160 |
  | laptop | 300 | 144 |
  | desktop | 320 | 144 |
  | qhd | 380 | 144 |
  | uhd | 440 | 144 |

- **Race hero cards:** fill the full bar height; hold a **portrait aspect (~3:4)**
  with a `max-w` so they stay banner-shaped and **center in their flex slot**
  (extra width becomes gap) instead of stretching wide. Bigger shield art, label
  pinned bottom, active glow unchanged. Interior spacing stays the `cqi` already
  wired in the race_tabs conversion, re-tuned for the taller card.
- **Mode block:** **compact width** (~13–15rem per band, down from 34–62rem) but
  **full height** — Melee over Campaign, each button ≈ half the bar height, label
  centered. This trades the wide-squat look for two tall narrow blocks with no
  dead space.
- **Mobile:** stacked. **Mode** = a **horizontal** Melee|Campaign segmented pair at
  a real height (~52px, up from 26px) — unchanged and fine. **Races** = a
  **horizontal-scroll carousel**: the row scrolls sideways with scroll-snap, each
  hero card large (~40% of viewport width, so ~2–3 visible with a peek of the next
  card signalling more), portrait art. Replaces the cramped 5-across (~70px each,
  labels wrapping).

## 4. Component changes (design level — exact class values MEASURED LIVE at implementation)

Per the CQI handoff playbook, every `cqi` value and each per-band size is measured
against the real rendered host width in the browser during implementation, not
guessed here.

1. **`editor_tabs_bar/style`** — add the per-band `min-h` steps (the height knob).
   Keep `flex items-stretch flex-none gap-*`; keep `mobile:flex-col`.
2. **`mode_tabs_host/style`** — `contents` → the `@container` box-owner, mirroring
   the `race_tabs_host` fix: per-band **compact width** (~13–15rem, moved here from
   `mode_tabs`) + fill height (`self-stretch min-w-0`). This is the definite-width
   container the mode block's `cqi` resolves against.
3. **`mode_tabs/style`** — fill the host (`size-full` / `w-full h-full`), keep
   `flex-col min-w-0`; the inter-button gap → `cqi` off the host. Buttons (via
   `ToggleButton`, `flex-1`) fill to ≈ half the bar height.
4. **`race_tabs_host/style`** — already the `@container` box-owner
   (`grow self-stretch min-w-0`); it now fills the taller bar via `self-stretch`.
   **Drop the interim `mobile:h-40` / `tablet:h-40`** (the bar owns height now).
5. **`race_tabs/style` + `race_tab` (+ the 5 `*_race_tab` wrappers)** — two per-band
   treatments:
   - **tablet+ (side-by-side):** cards fill height, portrait
     `h-full aspect-[3/4] max-w-[…]`, centered in the slot (`justify-center` on each
     wrapper, which are already `flex flex-1`).
   - **mobile (carousel):** `race_tabs` gets `mobile:overflow-x-auto
     mobile:snap-x mobile:snap-mandatory` (keeps `flex-nowrap`); each `*_race_tab`
     wrapper drops `mobile:flex-1`, gains `mobile:shrink-0` + a card width (~40cqi
     of the carousel / vw) + `mobile:snap-start`; the card stays portrait. Scroll
     is native touch; no domain logic.
6. **`race_tab_label` / art proportions** — verify the bottom-pinned label and the
   `before:` shield art read well at the taller (desktop) and carousel (mobile)
   sizes; adjust `cqi` if needed.

**Shared-component caveat:** `ToggleButton`
(`editor_page/components/shared/toggle_button`) is used beyond the mode tabs.
Re-proportion the mode tabs **only** via the mode column width + the bar height —
**do not restyle `ToggleButton`.** If the tall buttons need the label
vertically centered, first audit ToggleButton's other consumers and prefer a
change that is safe for all of them; otherwise leave alignment as-is.

## 5. Spec compliance

- **One per-band absolute size at the top** (`editor_tabs_bar min-h`); every layer
  below fills (`self-stretch` / `h-full` / `size-full` / `min-w-0`).
- **Race card portrait via `aspect` + `max-w`** = intrinsic shape, explicitly
  allowed by COMPONENTS.md "Fill the container" (shape belongs to the component;
  scale flows from the parent).
- **Gaps / padding in `cqi`** off measured host widths; **type / radius / border
  stay design tokens**; **no `clamp`** (band steps + `cqi` only).
- **Render-tree == directory-tree unchanged**; the `super::` test stays clean.
- No domain logic, no wall crossings — pure presentation.

## 6. Verification

Browser (Playwright MCP) at **mobile / tablet / laptop / desktop / qhd / uhd**:
confirm the height targets, portrait cards (not wide-short at qhd/uhd), the compact
full-height mode block, and the mobile mode row at real height. Before/after
element screenshots of `.editor-tabs-bar` at each band. `moon run :check` green;
the **user** runs `moon run :ci` as the final gate.

## 7. Risks / open items

- **Tall (~150px) mode buttons with centered small text may look sparse** — revisit
  label size / treatment in the browser during implementation; this is the one spot
  the approved choice (full-height mode block) trades balance for presence.
- Exact **portrait aspect + `max-w`** and per-band **mode width** are tuned live for
  visual balance across bands.
- **Shield art** (`before:bg-contain`) in the taller card — verify it fills nicely
  and does not pixelate or float; adjust `bg-size`/position if needed.
- Total navbar height vs editor content: at 1080p desktop, 320px is ~30% of the
  viewport — confirmed acceptable with the user (Balanced 2×+ tier chosen).
- **Mobile carousel:** card width and portrait aspect tuned live so ~2–3 cards show
  with a clear peek, and mode row (~52px) + carousel fit the ~190px mobile band.
  Optional polish — scroll the active race into view when it changes (a
  presentation-only `use_effect` + node ref; UI state, no domain). Decide whether
  it's worth the complexity during implementation; the base carousel works without
  it.
