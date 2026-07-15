# Editor layout restructure — unified carousel navigation + full-width detail

**Date:** 2026-07-15
**Status:** Design — direction approved in conversation; pending spec review.
**Supersedes:** `2026-07-15-editor-navbar-restructure-design.md` (the mode/race
"match the sidebar/detail widths" approach is abandoned — there is no sidebar
column any more).

---

## 1. Goal

Replace the current **side-by-side** editor layout (`[nav-sidebar | unit-detail]`)
with a **stacked** layout: one full-width navigation panel of horizontal
**carousels** over a full-width unit detail — and make the structure **identical
across every band** (mobile → uhd). Mobile and desktop stop being two different
layouts; wider viewports simply show more items per carousel row.

## 2. Why (the payoff)

- Dissolves the alignment problem entirely — no sidebar column, so nothing needs
  to match a sidebar/detail width.
- One interaction pattern for *all* navigation (mode, race, unit) — the mobile
  race-carousel we already built becomes the pattern everywhere.
- The unit detail gains **horizontal space** (full width) instead of being squeezed
  beside a 34rem sidebar.
- Everything in the top panel answers "*what am I editing?*"; the bottom panel is
  "*edit it*."

## 3. Current structure

```
editor_page  → Page frame { header: editor_tabs_bar,  body: editor_workspace }
editor_tabs_bar        → ModeTabsHost | RaceTabsHost           (the navbar)
editor_workspace (grid 34rem|1fr) → RaceTheme → <Race>RaceTheme → UnitList | UnitDetail
UnitList (aside, absolute w-136, h-full) → search + category toggles + CategoryScroll
CategoryScroll         → vertical, infinite-scroll sections (Hero/Soldier/Worker/Building)
```

## 4. Target structure

```
editor_page  (flex-col / grid-rows [auto  1fr])
├── EditorNavigation            full-width top panel, SAME on every band
│     ├── ModeTabs              Melee | Campaign
│     ├── RaceTabs              5 race badges — horizontal carousel (scroll+snap), ALL bands
│     └── UnitCarousel          category-tabbed; the active category's units in a
│                               2–3 row horizontal carousel (scroll+snap). No infinite scroll.
└── UnitDetail                  full width, below — gains horizontal space
```

- **RaceTabs:** the mobile carousel becomes the base for every band (drop the
  per-band 2-col / portrait / width-alignment styling).
- **UnitCarousel:** the existing category tabs (Heroes / Units / Workers / Buildings)
  select the category; that category's unit cards fill a 2–3 row horizontal
  carousel. Search still filters. The unit *cards* already exist inside
  `CategoryScroll` — they get re-laid-out into a carousel, not rebuilt.
- **Identical across bands:** the only per-band difference is how many cards are
  visible before you scroll.

## 5. Key decisions (approved)

- Category-tabbed unit carousel (not one big all-units grid).
- Same stacked-carousel structure on mobile and desktop.
- Detail trades vertical space for full width — accepted.

## 6. Structural considerations

- **`RaceTheme`** (the per-race `--race-color` themer + race dispatcher) currently
  wraps `UnitList | UnitDetail`. In the new tree it must wrap the **race-specific**
  content — the `UnitCarousel` and the `UnitDetail` — while `ModeTabs` and the
  `RaceTabs` *selector* (always all 5) sit above/outside it. Exact placement is an
  implementation detail for the plan; the `--race-color` var must reach the unit
  cards and the detail.
- **`editor_page` frame:** the `Page { header, body }` split is replaced by a plain
  `flex-col` (or `grid-rows-[auto_1fr]`): `EditorNavigation` then `UnitDetail`.
- **`editor_tabs_bar` and `editor_workspace`** (the two-column grids) are superseded
  and removed/repurposed; `EditorNavigation` takes over the top.
- The in-progress navbar work (mode/race grid alignment, aspect height, per-band
  widths) is **reverted** — superseded by this design. The race-carousel styling is
  kept and generalized to all bands.

## 7. Build phases (checkpointed)

1. **Revert** the superseded navbar alignment/aspect changes to a clean base.
2. **`EditorNavigation` shell:** new component; restructure `editor_page` to
   `EditorNavigation` (mode + race) over full-width `UnitDetail`; move `RaceTheme`
   to wrap the race-specific content. Verify detail is full-width, races still work.
3. **RaceTabs carousel on all bands:** generalize the mobile carousel; drop the
   2-col/portrait styling. Verify scroll/snap at every band.
4. **UnitCarousel:** turn `CategoryScroll` into the category-tabbed 2–3 row
   horizontal carousel; wire the category tabs + search. Verify no infinite scroll,
   cards scan at a glance.
5. **Cross-band verification** (mobile → uhd): consistent structure, carousels
   scroll/snap, detail full-width. `moon run :check`; user runs `moon run :ci`.

## 8. Follow-up (NOT this phase)

Command-card / grids restructure inside `UnitDetail`: split **stats + command cards
to the right**, **grids on the left stacked above**. A separate spec + plan after
this lands.

## 9. Verification

Browser (Playwright MCP) at all bands. The browser is the only real signal;
`cqi`/sizes measured live per the CQI handoff. Same structure mobile → uhd.
