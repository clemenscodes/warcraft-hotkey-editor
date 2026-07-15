# Editor navigation controls — UX consistency fix (HANDOFF / PLAN)

**Date:** 2026-07-15
**Status:** Plan. Nothing here implemented yet. Do NOT start coding before reading
`docs/ARCHITECTURE.md`, `docs/COMPONENTS.md`, `docs/RUST_STYLE.md`, `docs/AGENTS.md`
in full (Rule Zero), and consider **brainstorming the affordance choices with the
repo owner first** (they are real design decisions).

---

## 0. Where we are (context — the editor was just restructured)

The editor layout was rebuilt this session from side-by-side `[sidebar | detail]`
into a **unified stacked layout**, identical mobile→uhd:

```
race badges (carousel)            editor_tabs_bar → race_tabs_host
controls row (all buttons)        unit_list → unit_controls  (mode + 3 toggle groups)
search bar                        unit_list → unit_list_search
2-row unit carousel               unit_list → category_scroll (grid, 2 rows, h-scroll)
full-width unit detail            editor_workspace body → unit_detail
```

Design spec for that restructure:
`docs/superpowers/specs/2026-07-15-editor-layout-unified-navigation-design.md`.

**Working now** (verified in browser, compiles `moon run :check` green, NOT committed):
- 2-row uniform card grid carousel; full-width detail; footer pinned at the bottom
  (fixed via `WarcraftPage` `flex-none` on mobile/tablet — see §5).
- Buttons equal-width and filling the row on desktop; gold-unified button colors.

**This handoff is about a DIFFERENT, deeper problem the owner flagged:** the controls
look uniform but behave four different ways, and the mode buttons are in the wrong row.

---

## 1. Problem 1 — mode buttons + chunky race badges (desktop)

- **Melee/Campaign must live in the SAME ROW as the race badges** on desktop (they are
  a peer selector, not part of the unit-filter controls). Today they're in
  `unit_controls` with the unit filters.
- **The race badges should get slimmer** (shorter height) to make room and still look
  good.

## 2. Problem 2 — control affordances don't match their semantics (the big one)

Four control groups, all rendered as visually identical toggle buttons, but with four
different behaviors. A user cannot tell "pick one" from "toggle each" from "search
config." Verified against the code:

| Control | Component (dir) | TRUE semantics | Correct affordance | Current (wrong) |
|---|---|---|---|---|
| **Melee / Campaign** | `editor_tabs_bar/…/mode_tabs_host` (`ModeTabs`→`ToggleButton`) | mutually-exclusive selection (`UnitMode`) | **segmented control / radio** — pick one | identical buttons, in the wrong row |
| **Unit / Ability** | `unit_list/…/search_field_toggle` | mutually-exclusive, sets **search scope only** (`SearchField::UnitName`/`Ability`) | **inline in the search bar** (scope selector on/inside the input) | standalone buttons in the controls row |
| **No abilities / All variants** | `unit_list/…/catalog_visibility_toggle` | **two INDEPENDENT booleans** (`show_abilityless_units`, `expand_variants`) | **independent switches/checkboxes** (each on/off) | identical buttons |
| **Heroes / Units / Workers / Buildings** | `unit_list/…/mobile_category_tabs` | **independent per-category show/hide** (`collapsed_categories: HashSet<UnitKind>`; `is_active = not collapsed`) — NOT tabs; `role="tablist"` is **wrong** | **multi-select filter chips** | identical buttons + wrong ARIA `role=tablist` |

The owner's summary, verbatim and correct: mode = toggles yet mutually exclusive;
Unit/Ability = toggles, mutually exclusive, search-only (unintuitive as buttons);
No-abilities/All-variants = toggles, not mutually exclusive; Heroes/… = toggles, none
mutually exclusive. "Horrible UX and inconsistency all over the place."

## 2b. Problem 3 — the search bar is two different designs (desktop vs mobile)

`unit_list_search` ships **two entirely different search looks in one component**,
split by band. Not a tweak — a full design fork. **Owner's decision: the MOBILE look
wins on every band** ("it looks much more naturally like a search; desktop look sucks").

| | Desktop (`base` band) — REMOVE | Mobile/tablet bands — KEEP, promote to base |
|---|---|---|
| outer container | blue **tile box** (`bg-warcraft-bg-mid/85`, `border-warcraft-blue-deep`, `rounded-tile`, `p-2`) | **no box** (`bg-transparent border-0 rounded-none p-0`, just `relative` positioning) |
| input | **blue** (`bg-warcraft-bg-base/70`, `border-warcraft-blue`, `rounded-control`), `text-lg`, `px-4 py-3` | **gold** (`bg-panel-dark`, `border-warcraft-gold/45`, `rounded-panel`), `shadow-bevel`, `h-11`, `text-base`, `pl-10`, italic placeholder, `tracking-label` |
| search icon | **hidden** | **shown** — magnifier `absolute left-3`, inside the input |

**The fix (mechanical, 3 style files under `unit_list_search/`):** promote the
mobile/tablet band values into `base` and delete the `mobile:`/`tablet:` override bands,
so all bands render the single gold+icon design. Concretely:

- `unit_list_search/style/mod.rs` → `base` becomes the mobile look: `relative flex
  items-center gap-2 min-w-0 flex-none` (drop `p-2 bg-* border-* rounded-tile`); delete
  the `mobile`/`tablet` bands.
- `unit_list_search_input/style/mod.rs` → `base` becomes the gold input (all the
  `mobile:` values without the prefix: `h-11 pl-10 pr-3.5 text-base rounded-panel
  bg-panel-dark border-warcraft-gold/45 shadow-bevel` + italic placeholder + the
  `kb-focus`/`-webkit-search-cancel-button` rules); keep `flex-1 min-w-0 w-full text-white`;
  drop the old blue `border-warcraft-blue rounded-control px-4 py-3 text-lg`; delete the
  `mobile`/`tablet` bands.
- `unit_list_search_icon/style/mod.rs` → `base` becomes `block absolute left-3 top-1/2
  -translate-y-1/2 w-4.5 h-4.5 text-warcraft-gold pointer-events-none` + the `[&_svg]`
  rules (drop the base `hidden`); delete the `mobile`/`tablet` bands.

Watch: after promoting to `base`, the parent must still give the input its width via
flex — and remember the shell's `**:min-w-0` nukes any `min-w` on these descendants at
mobile/tablet (§5). `touch tailwind.css`, then verify the search looks identical at
mobile / tablet / laptop / desktop / qhd / uhd.

---

## 3. Proposed design (confirm with owner before building)

**Desktop row structure:**
1. **Race badges (slim) + Melee/Campaign** on one row (mode as a segmented control).
2. **Search bar** with the **Unit/Ability scope selector inlined** (e.g. a small
   segmented toggle or a `Unit ▾` dropdown at the left of the input). Remove
   `SearchFieldToggle` from the controls row entirely.
3. **Filters row**, visually distinct from selection controls:
   - No abilities / All variants → **switches** (independent on/off).
   - Heroes / Units / Workers / Buildings → **multi-select chips** (independent).
   - Consider light labels ("Show:", "Categories:") to make the grouping legible.
4. **2-row unit carousel** (already built).
5. **Full-width detail** (already built).

**Affordance rules (the point):**
- Mutually-exclusive selection → **segmented control** (one connected control, active
  segment filled). Reads as "pick one."
- Independent on/off → **switch/checkbox** (distinct checked state). Reads as "toggle each."
- Search-scope → **belongs to the search input**, not a peer button.
- Category multi-select → **chips** with an active/inactive dot or fill; fix the ARIA
  (a group of toggle buttons / checkboxes, not `role=tablist`).

Do these as **proper new components** per `COMPONENTS.md` (a `SegmentedControl`, a
`FilterChip`, a `SwitchToggle`), not more inline `[&_button]` overrides.

---

## 4. Plan (phased; each phase ends with a browser check at mobile+desktop)

- **Phase A — mode into the race row + slim badges.** `editor_tabs_bar` renders
  race + mode; restyle race badges shorter; **remove `ModeTabsHost` from
  `unit_controls`** (kills the cross-subtree import debt, see §5).
- **Phase B — inline search scope + unify the search look.** (1) Collapse the search
  bar to ONE design across all bands = the gold input + magnifier icon (Problem 3 / §2b);
  promote the mobile-band values into `base`, delete the `mobile`/`tablet` bands in the
  3 `unit_list_search` style files. (2) Move Unit/Ability into `unit_list_search` as an
  inline scope selector; delete `search_field_toggle` from `unit_controls`. Do (1) first
  (independent, mechanical) — it de-risks (2), which restructures the same component.
- **Phase C — switches.** Reskin No-abilities/All-variants as independent switches.
- **Phase D — chips + ARIA.** Reskin Heroes/…/Buildings as multi-select chips; fix
  `role`. (Component is misnamed `mobile_category_tabs` but used on all bands — rename
  when touching it.)
- **Phase E — layout + polish.** Re-lay-out the controls with clear grouping/labels;
  verify UX + spacing at mobile / tablet / laptop / desktop / qhd / uhd.

## 5. Gotchas / constraints (hard-won this session — do not relearn the hard way)

- **Browser is the only signal.** `moon run :check` = compile only; **NEVER
  `moon run :ci`** (owner runs it). After any class edit: `touch
  crates/hotkey-editor/tailwind.css` (watcher is inotify-blind to new classes), then
  reload.
- **Shell nukes `min-w` on mobile/tablet:** `components/shell/style` has
  `mobile:**:min-w-0` + `tablet:**:min-w-0`. Any `min-w-[…]` on descendants is void on
  those bands. For wrapping/sizing use `flex-basis` or an explicit `w-[…]` + `flex-none`,
  never `min-w`.
- **The mode `ToggleButton` is `@container`** (`contain: inline-size`) → its width is
  independent of content, so `w-auto`/`w-fit` **collapse** it. It needs a definite width
  (from flex in a definite-width parent).
- **`ToggleButton` is SHARED** (`editor_page/components/shared/toggle_button`). Don't
  restyle it globally. Make new components for the new affordances.
- **`unit_controls` architectural debt:** it currently renders `ModeTabsHost` via a
  cross-subtree `crate::…::editor_tabs_bar::…::ModeTabsHost` import (violates
  render-tree == dir-tree). Phase A removes mode from here — good, do it.
- **Footer/editor collapse (already fixed — don't revert):** on mobile/tablet
  `editor-page` is `flex-none`; `WarcraftPage` was collapsing to 0 (its `flex-[1_1_0]`
  in a content-height parent), stacking the footer under the header. Fixed by adding
  `mobile:flex-none tablet:flex-none` to `warcraft_page/style`.
- **`RaceTheme`** (per-race `--race-color` + dispatcher) wraps the unit content + detail;
  keep the var reaching the cards + detail.
- **Button text was forced `text-md`** on `unit_controls` for size consistency (the mode
  `ToggleButton` is `text-xl` at qhd/uhd otherwise). Revisit when redesigning.

## 6. Key files

```
editor_tabs_bar/                         (currently: race row only)
  components/race_tabs_host/             race badges (carousel all bands) — SLIM these (Phase A)
  components/mode_tabs_host/             ModeTabs → ToggleButton (mode) — move to race row (Phase A)
editor_workspace/…/unit_list/
  mod.rs                                 renders UnitControls + UnitListSearch + CategoryScroll
  components/unit_controls/              the merged controls row (mode + 3 toggle groups) — shrinks as controls move
  components/unit_list_search/           search — UNIFY to gold+icon on all bands (§2b) + inline scope selector (Phase B)
    components/unit_list_search_input/   the <input> — blue(base) vs gold(mobile) fork lives here → promote gold to base
    components/unit_list_search_icon/    the magnifier — hidden(base) vs shown(mobile) fork → promote shown to base
  components/search_field_toggle/        Unit/Ability scope (mutually excl., search-only) — MOVE into search (Phase B)
  components/catalog_visibility_toggle/  No-abilities/All-variants (2 independent bools) — → switches (Phase C)
  components/mobile_category_tabs/       category show/hide (independent multi-select) — → chips + fix ARIA (Phase D)
  components/category_scroll/            the 2-row unit carousel (grid) — DONE
editor_workspace/…/unit_detail/          full-width detail — DONE
shared/warcraft_page/                    the Page frame (footer fix lives here)
shell/style                             mobile **:min-w-0 gotcha lives here
editor_page/components/shared/toggle_button/  SHARED gold toggle — don't restyle globally
```
