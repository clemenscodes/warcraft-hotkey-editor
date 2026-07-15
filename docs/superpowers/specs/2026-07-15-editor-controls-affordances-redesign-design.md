# Editor controls — affordance redesign (DESIGN)

**Date:** 2026-07-15
**Status:** Design, approved. Not implemented. Companion to the handoff
`docs/superpowers/specs/2026-07-15-editor-controls-ux-consistency-handoff.md`
(Problems 1, 2 and the affordance half of Phase B, plus Phases A/C/D/E). Read
`docs/ARCHITECTURE.md`, `docs/COMPONENTS.md`, `docs/RUST_STYLE.md`,
`docs/AGENTS.md` in full before implementing (Rule Zero).

---

## 1. The problem

The editor's control row renders four control groups as **visually identical
gold toggle buttons**, but they carry four different behaviours. A user cannot
tell "pick one" from "toggle each" from "search config":

| Group | True semantics | Today |
|---|---|---|
| Melee / Campaign | mutually-exclusive selection (`UnitMode`) | identical buttons, wrong row |
| Unit / Ability | mutually-exclusive, sets **search scope only** (`SearchField`) | standalone buttons |
| No abilities / All variants | **two independent booleans** | identical buttons |
| Heroes / Units / Workers / Buildings | **independent per-category show/hide** (`collapsed_categories: HashSet<UnitKind>`) | identical buttons + wrong `role=tablist` |

Three of the four groups literally render the same shared `ToggleButton`
(`rounded-panel`, `text-xl`, `aria_pressed`, active = filled gold); the category
group is the lone exception with its own `role="tab"`/`tablist` markup (wrong
ARIA). **The fix is to give each behaviour its own distinct shape**, each
mirroring a pattern that already exists in the repo. State plumbing is left
untouched — every signal and handler already exists; this is presentation +
placement only.

### Already done (Phase B.1)
The search bar was unified to one design (gold input + magnifier icon) across all
bands by promoting the mobile-band values into `base` and deleting the
`mobile`/`tablet` bands in the three `unit_list_search` style files. That change
stands on its own; **B.2 below re-homes the box** it produced.

---

## 2. Decisions (locked with the owner)

- **Mode** → **connected segmented control** (filled active segment, no gaps).
- **Search scope** → **`Unit ▾` dropdown** pinned inside the search box (compact;
  best on mobile; hides the alternative until opened).
- **Catalog booleans** → **sliding track+knob switches** (strongest independent
  on/off signal).
- **Categories** → **separated, gapped multi-select chips**, **filled = shown /
  dim outline = hidden**.
- **Labels** → **microlabels + grouped rows** (`Show:` / `Categories:`), the
  handoff's explicit legibility goal.

### The differentiation invariant
Because chips were chosen as *filled = shown*, "filled" now reads as active on
**both** the segmented control and the chips. They must therefore be
distinguished by **shape, not fill**:

- Segmented control = **connected**, gapless, single outer rounding (reads "pick
  one").
- Chips = **separated**, gapped, individually rounded (reads "toggle each,
  independently").
- Switches = **track + knob** (reads "independent on/off").

Three unmistakably different silhouettes.

---

## 3. Target layout (all bands, stacked; identical mobile→uhd shape)

```
[ Melee │ Campaign ]  ← connected segmented control, in the race row (badges slim, beside it)
┌ Unit ▾ │ 🔍 Search units… ────────────────┐   ← scope dropdown inside the search box
Show:        No abilities ─○   All variants ●─   ← two independent switches
Categories:  (Heroes)(Units)·Workers··Buildings· ← separated multi-select chips, filled = shown
2-row unit carousel  /  full-width detail        ← already built (prior session)
```

---

## 4. Patterns to mirror (do not invent — improvising is forbidden)

- **Dropdown:** `HeroLevelPicker` (the `LEVEL n ▾` control) at
  `…/unit_detail/…/hero_level_picker/`. Structure: a `relative` wrapper `div` →
  a `<button type="button">` trigger (closed/open variant split with a `▾`/`▴`
  chevron) → gated by `if is_open`: an `absolute z-20` menu of `<button>`
  options **plus** a `fixed inset-0 z-10 bg-transparent` backdrop sibling for
  outside-click dismiss. Open state = a local `use_signal::<bool>` (toggle on the
  trigger, `set(false)` on backdrop click and on each option click); a
  `use_effect` auto-closes on selection-context change.
- **Dropdown ARIA:** `system_hotkeys_breadcrumbs_trigger`/`_menu` — copy
  `aria_haspopup="listbox"`, `aria_expanded`, and `role="listbox"` (HeroLevelPicker
  omits these).
- **Mutually-exclusive/state looks:** the shared `ToggleButton` and
  `MobileCategoryTab` idiom — a thin **dispatcher** that `match`es the state and
  renders one of two look-components (`Active…`/`Idle…`), each owning its classed
  root. Never a state-keyed class table.
- **No `dioxus_primitives`.** The crate declares it in `Cargo.toml` but uses it
  nowhere; every dropdown/menu/dialog is hand-rolled. Dialogs use
  `dioxus_kit::frame::Dialog` via `WarcraftDialog`. The scope selector is a
  **dropdown**, not a dialog.

---

## 5. Per-affordance component design

Every new component obeys `directory == component == class`, the render-tree ==
directory-tree law, "a component owns its look; its parent owns its size", "one
class per component", and "mutually-exclusive looks are components". New
affordances are **proper components** — never more inline `[&_button]` overrides,
and the shared `ToggleButton` is **never** restyled globally.

### Phase A — mode → connected segmented control, in the race row

- `editor_tabs_bar` renders **both** `race_tabs_host` and `mode_tabs_host` in one
  row. **Remove the `ModeTabsHost` mount and its cross-subtree
  `use crate::…::editor_tabs_bar::…::ModeTabsHost` import from `unit_controls`.**
  This deletes a render-tree == directory-tree violation (mode was defined under
  `editor_tabs_bar` but mounted under `unit_controls`).
- Inner leaf swaps from the shared `ToggleButton` to a new **connected segment**:
  ```
  mode_tabs_host/
    components/mode_tabs/            container: flex flex-row, NO gap, outer rounding + overflow-hidden
      components/mode_tab/           dispatcher: match active → Active | Idle
        components/active_mode_segment/   own markup; shared divider border; filled active
        components/idle_mode_segment/     own markup; shared divider border; resting
  ```
  Mirrors the existing `ModeTab` → Active/Idle split; only the look changes
  (connected instead of gapped pills). State unchanged: reads
  `navigation.unit_mode()` + `editor.selected_slot()`, fires
  `navigation.select_mode(mode, slot)`.
- **Race badges: reduce height per band** (shorter). Exact per-band values dialed
  in the browser; badges keep their look, only the parent-owned height shrinks.
- **Gotcha:** the segment container is a query container (`@container`) → it needs
  a **definite width from flex in a definite-width parent**; `w-auto`/`w-fit`
  collapse it.

### Phase B.2 — search scope → `Unit ▾` dropdown inside the box

- **Re-home the box:** move the framed-box styling (border, `rounded-panel`,
  `bg-panel-dark`, `shadow-bevel`) from the **input** up to the **`unit_list_search`
  container**; the input becomes borderless/transparent inside it. Add a divider
  between the scope picker and the input. (This revises B.1's placement — expected;
  B.1 de-risked it and remains valid standalone until this lands.)
- New picker under `unit_list_search`, mirroring `HeroLevelPicker`:
  ```
  unit_list_search/components/search_scope_picker/
    components/search_scope_trigger/      closed/open split, "Unit ▾"/"Ability ▾" + chevron;
                                          ARIA aria_haspopup=listbox, aria_expanded
      components/closed_search_scope_trigger/
      components/open_search_scope_trigger/
    components/search_scope_menu/          absolute z-20, role="listbox", 2 options
      components/search_scope_option/       dispatcher: active | idle option
    components/search_scope_backdrop/       fixed inset-0 z-10 bg-transparent dismiss
  ```
  Open state = local `use_signal::<bool>` owned by the picker's presentation
  builder (UI-only signal). Options call the existing
  `editor.search_field().set(SearchField::UnitName | Ability)`.
- **Delete `search_field_toggle`** from `unit_controls`.
- `unit_list_search` renders `SearchScopePicker` + the input + the magnifier icon
  as flex children of the framed container.

### Phase C — catalog booleans → two switches

- New **shared leaf** `switch_toggle` (rendered by 2 sibling wrappers → lives in a
  `shared/` grouping dir per the count rule). It is a dispatcher over two looks:
  ```
  catalog_visibility_toggle/
    components/catalog_visibility_switch/   thin per-bool wrapper (sources its Signal<bool>)
    components/shared/switch_toggle/        track+knob leaf; dispatcher:
      components/on_switch_toggle/          knob right, gold-filled track
      components/off_switch_toggle/         knob left, resting track
  ```
  Dispatcher (not a class swap) to obey "looks are components"; no knob-slide
  animation (consistent with `ToggleButton`, which also swaps). Handlers unchanged:
  each wrapper keeps `Signal::set(!current)`; the "No abilities" tooltip
  (`title`) is preserved.
- Add a `Show:` microlabel via a **shared `filter_group_label` leaf** (text as a
  prop; reused by the categories group too, so it lives in a `shared/` grouping
  dir at the **common parent** of the two filter groups — the exact path resolved
  against the real tree in the implementation plan, since `unit_controls` renders
  these groups via `super::` today and Phase E re-homes them under their renderer).

### Phase D — categories → multi-select chips + ARIA fix

- **Rename** `mobile_category_tabs` → `category_filters` and `MobileCategoryTab`
  → `CategoryFilter` (they render on every band and are not tabs). **Update
  coupled e2e selectors in the same commit** (identity classes are a coupled
  contract).
- **ARIA fix:** container `nav role="tablist"` + per-item `role="tab"` /
  `aria_selected` → container `role="group"` `aria_label="Categories"`, each chip
  a `<button aria_pressed>` (independent multi-select, not tabs).
- Look: separated/gapped, individually rounded chips; dispatcher over two looks:
  ```
  category_filters/                          role="group", renders label + N chips (for kind in tabs)
    components/category_filter/              dispatcher: shown | hidden
      components/shown_category_chip/        filled gold (= shown)
      components/hidden_category_chip/       dim outline (= hidden)
  ```
  State unchanged: reads `editor.collapsed_categories()`; the chip's onclick
  inserts/removes `kind` in the `HashSet` (active = shown = not collapsed).
- Add a `Categories:` microlabel via the shared `filter_group_label` leaf.

### Phase E — layout + polish

- After A/B.2, `unit_controls` holds only the two filter groups (mode left for the
  race row; scope moved into the search). Lay them out as the two labeled rows in
  §3 (switches row, chips row), visually distinct from the selection controls
  above.
- Verify UX + spacing mobile / tablet / laptop / desktop / qhd / uhd.

---

## 6. Judgment calls & open items

1. **`ToggleButton` likely becomes orphaned.** Its three current consumers (mode,
   search scope, catalog) all migrate off it; categories never used it. At the end
   of Phase D, verify remaining usages across the crate. If none remain, **surface
   it for removal — do not delete unilaterally** (per "mention dead code, don't
   delete"). Same check for now-unused `search_field_button` /
   `catalog_visibility_button` / `mode_tab`-via-ToggleButton wrappers replaced by
   the new leaves.
2. **Renames** (`mobile_category_tabs`→`category_filters`, mode inner leaves).
   Justified by "name a thing for what it is"; each rename updates its coupled e2e
   selector in the same commit.
3. **B.2 moves the box** from the input to the search container so scope + input
   share one framed box with a divider. B.1 stays valid until B.2 lands.
4. **Switches/chips use the dispatcher-of-two-looks pattern**, matching the repo's
   `ToggleButton`/`MobileCategoryTab` idiom, rather than animated class swaps.

---

## 7. Constraints / gotchas (hard-won; do not relearn)

- **Browser is the only signal.** After any class edit: `touch
  crates/hotkey-editor/tailwind.css` (the watcher is inotify-blind to new
  classes), then reload. `moon run :check` = compile only. **Never run `moon run
  :ci`** — the owner runs it. Do not commit unless asked. Do not start the dev
  server (the owner owns port 8123); drive the running server with Playwright.
- **Shell nukes `min-w` on mobile/tablet:** `components/shell/style` has
  `mobile:**:min-w-0` + `tablet:**:min-w-0`. Any `min-w-[…]` on descendants is
  void there — size with `flex-basis` / explicit `w-[…]` + `flex-none`, never
  `min-w`.
- **`@container` needs a definite width.** The mode segment container (and any
  `@container` leaf) has content-independent width → `w-auto`/`w-fit` collapse it;
  give it a definite width via flex in a definite-width parent.
- **`ToggleButton` is shared** — do not restyle it globally; build the new
  affordances as new components.
- **Footer/editor collapse fix (already in place — don't revert):** on
  mobile/tablet `editor-page` is `flex-none`; `warcraft_page/style` carries
  `mobile:flex-none tablet:flex-none`.
- **`RaceTheme`** wraps the unit content + detail and publishes `--race-color`;
  keep it reaching the cards + detail.

---

## 8. Phasing & verification

Order: **A → B.2 → C → D → E**. Each phase ends with `moon run :check` green and a
browser pass at mobile / tablet / laptop / desktop / qhd / uhd against the running
dev server (`http://localhost:8123/warcraft-hotkey-editor/`, trailing slash). No
commits unless the owner asks. The owner runs `moon run :ci` as the final gate.
