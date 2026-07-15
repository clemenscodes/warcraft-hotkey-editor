# Mobile editor redesign — locked design

**Date:** 2026-07-15
**Status:** Mobile design **locked** (converged in discussion). Tablet → UHD still to be
brainstormed. This is the target UI (information architecture) only; the
header-quality `cqi` fluid-scaling pass (per `docs/CQI_CASCADE_HANDOFF.md`) is a
**later stage** applied after this IA is right — not now.

Companion visual: the wireframe artifact (v1) — this doc supersedes its mobile frame.

---

## 0. The reframe (why we're redoing it)

The app is a **hotkey editor**: the point is editing the command-card hotkeys/grid
positions for a unit. Every selector — race, mode, search, categories, show — is a
single input worth *a few bits*: **which unit am I editing?** In the old UI that
selection apparatus + a broken stats panel ate ~80% of the viewport and the actual
editing surface was buried at the bottom.

**Invert it:** the command card is the screen; everything else is either a one-line
identity or a summoned surface.

## 1. The rule for mobile

> **The persistent screen is only *identity + command card*. Everything else is
> summoned (a dialog).** No persistent selector, no arrows, no collapsibles — so
> there is zero layout shift, ever.

## 2. Mobile layout (persistent screen), top to bottom

```
┌───────────────────────────────┐
│ Header  … actions …      [⌕]   │  ← existing header + a search toolbar button
├───────────────────────────────┤
│ [icon]  Archmage         ⓘ    │  ← unit strip: icon + name only, + info button
├───────────────────────────────┤
│                               │
│      COMMAND CARD  (canvas)    │  ← the star; a carousel (see §3)
│      [Q][W][E][R]              │     tap a tile → override dialog
│      [A][S][D][F]              │
│      [Z][X][C][V]              │
│         grid label · • •       │  ← current grid's label + dots if >1 grid
│                               │
└───────────────────────────────┘
```

- **Header** — unchanged, gains a **⌕ search button**. There is **no separate
  "navbar"**; the ⌕ button *is* the entry to switching units.
- **Unit strip** — **icon + name only** (no description). Carries a small **ⓘ** to the
  right of the name that opens the stats dialog.
- **Command card canvas** — fills the rest. It is the only thing you interact with
  directly.

## 3. Interaction model (the gesture map) — **Option C, perpendicular axes**

| Gesture / tap | Result |
|---|---|
| **Horizontal swipe** on the card | move between **this unit's grids** (they already have labels; each pane shows its grid name; dots for >1). Supports *n* grids cleanly (most units have 1–2 — main card, build menu, etc.). |
| **Vertical swipe** | move to **prev / next unit** (the whole card is a pager). Walks the **current filtered working set** defined by the last advanced search. |
| **Tap a tile** | open the **override dialog** for that button (hotkey / unhotkey / grid position). |
| **⌕** (header) | open the **advanced search dialog** (switch units). |
| **ⓘ** (unit strip) | open the **stats dialog** (stats + description). |

Snappy pager feel = CSS scroll-snap + momentum (native), not a JS carousel.

## 4. The three summoned surfaces (all dialogs on mobile)

- **Search dialog (⌕)** — the full unit switcher: a search field + the filters
  (race · mode · category · show) + the result list. Keyboard-first. This is where the
  affordance controls already built (segmented mode, scope, switches, category chips)
  **live now** — relocated into the dialog instead of a persistent row. Selecting a
  unit sets the current unit **and** defines the filtered working set the vertical
  swipe then walks.
- **Override dialog (tap a tile)** — one button's full override. Mobile = **one button
  at a time** (tap tile → edit → close), matching the tap gesture.
- **Stats dialog (ⓘ)** — unit stats + the description. Reusing `WarcraftDialog`, the
  same infra the app already uses for grid-layout / templates / help / system hotkeys.

## 5. Decisions & rationale (so we remember *why*)

- **Whole-card carousel replaces prev/next arrows.** Swiping the card *is* the arrow →
  arrows dropped on mobile.
- **Perpendicular axes (C) chosen** over the alternatives:
  - *A — single axis (horizontal = units) + grid tabs:* rejected here because the owner
    wants both to feel like swipes and the grids already carry labels.
  - *B — region-based (swipe-on-grid vs swipe-elsewhere, both horizontal):* rejected —
    the grid/not-grid boundary is finicky and a 1-grid unit makes the grid-swipe a dead
    zone.
- **Stats = dialog, not a bottom collapsible.** A collapsible causes layout shift; a
  dialog doesn't, and it also *removed the only conflict* Option C had (no bottom
  element left to fight the vertical unit-swipe).
- **Description lives in the stats dialog**, not the unit strip → the strip stays just
  icon + name.
- **Override = dialog, one button at a time on mobile** (grid stays clean, full focus).

## 6. How it scales up (noted, not yet designed)

Consistent responsive story: on mobile the three surfaces are **dialogs**; on wider
bands, where there's room, the *same components* become **persistent panels/rails**
(search → left rail, stats → right panel, override → side panel) instead of modals —
placement varies by band, components don't. Then Stage 2 applies the header-quality
`cqi` scaling so it resizes smoothly.

## 7. Still open (next: tablet 768–1279, then laptop → uhd)

- **Tablet fork:** hold the pure mobile model (everything still a dialog, just larger),
  **or** promote the **override** from a modal to a **side panel** (tap a tile → panel
  beside the card, so grid + override are visible together) while search/stats stay
  dialogs. (Leaning: promote the override on tablet.)
- Laptop/desktop/qhd/uhd layouts (three-pane, persistent picker rail, side panels,
  scaled-up card) — to brainstorm after tablet.
