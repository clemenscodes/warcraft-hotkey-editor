# Search Filter Panel UI Implementation Plan (Phase 2b)

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Replace the mobile search dialog's stacked filter buttons with the approved design. The `UNIT` scope picker becomes a filter icon that opens one floating filter panel over the results with no layout shift. Inside it live the search field choice, the mode choice, two include switches, and a races row that opens a further floating panel of race banner badges.

**Approved visual reference:** the mockup at `scratchpad/search-filter-mockup.html` (published as an artifact and signed off). It is the source of truth for layout, control forms, labels, and copy. Translate its plain CSS into the project's `tw_macro` classes and `@theme` tokens during execution, mirroring the existing dialog components.

**Architecture:** The dialog stops using the shared `UnitListSearch` and composes its own search bar from the shared leaves `UnitListSearchIcon` and `UnitListSearchInput`, plus a filter trigger. The filter panel and the races sub-panel float with `absolute` inside a `relative` container and dismiss through a scrim styled like the burger menu. `RaceTabBanner` is relocated to a shared location and reused, fed with scope membership instead of navigation.

## Global Constraints

- All paths relative to `/home/clemens/.local/src/warcraft-hotkey-editor`. The `[patch]` on `warcraft-data` stays; do not tag or bump.
- `docs/COMPONENTS.md` is law. Directory equals component equals class. The render tree is the directory tree, a component you render lives under your own `components/`. One classed element per component. `Element` is never a prop, compose typed components. Reuse a look only by composing the component that owns it, never by sharing styles. Mutually exclusive looks are separate components with a dispatcher, never a state table.
- `docs/RUST_STYLE.md` is law. Full semantic names, no tuples, `Self` in impls, derive everything, private fields.
- Punctuation in all copy follows the owner's rule. Only the period and the comma, plus apostrophes and quotation marks. No colon, no semicolon, no dash of any kind. This applies to every user-facing string and every comment.
- Verify with `moon run :check --force` while iterating, the full gate is `moon run :ci` at the end. The dev server is the user's, never start it, drive the existing one with Playwright and reload once.
- End commit messages with `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`. Branch `feature/mobile-redesign`.

## The approved control forms and copy

- Search bar, one row. A filter trigger on the left (a filter icon, with a small count badge of active filters), then the search icon, then the input. Placeholder is exactly `Search...`.
- Floating filter panel, opened from the trigger, floats over the results, dismissed by a scrim like the burger menu. Groups top to bottom.
  - `Find units by`, a segmented control of two, `Name` and `Ability`. Exactly one selected.
  - `Mode`, a segmented control of three, `Melee`, `Campaign`, `Both`. Exactly one selected. This maps to the mode selection, Melee to melee only, Campaign to campaign only, Both to both.
  - `Also include`, two switches, each with a small info button that opens a popover.
    - `Units with no special abilities`. Popover, `Units that bring no ability of their own. Some have no abilities at all, others carry only the generic ones like Move, Attack and Stop that hundreds of units share. Both are hidden by default. Turn on to list them too.`
    - `Separate variants`. Popover, `Some units exist as several tiers or forms of the same unit, like leveled summons, upgrade swaps such as Headhunter to Berserker, and hero forms. By default they collapse into one entry and an edit there applies to all of them. On lists each one.`
  - `Races`, a trigger row with the five race colour swatches and a summary that reads `All races` or the selected race names, opening the races sub-panel.
- Races sub-panel, floats, dismissed by its own backdrop. The five races as vertically stacked banner badges, the real `RaceTabBanner` look. Active means included in the search, dimmed means excluded, the last active race stays on.

---

### Task 0: Relocate `RaceTabBanner` to a shared location

The dialog and the desktop tabs both render race banners, so per COMPONENTS.md the leaf lives once in a `shared/` grouping at their nearest common parent, which is `shell/components`. Move the whole `race_tab_banner/` subtree there and move the `RaceTabBinding` prop struct with it.

**Files:**
- Move: the entire directory `.../editor_tabs_bar/components/race_tabs_host/components/race_tabs/components/race_tab_banner/` to `crates/hotkey-editor/src/components/app/components/shell/components/shared/race_tab_banner/`.
- Move: `RaceTabBinding` (currently `race_tabs/model/mod.rs`) into `shared/race_tab_banner/` as its own model so the subtree is self-contained. The banner already has `RaceTabBannerModel`, which is `RaceTabBinding` plus a `race` field, so make the per-race wrappers take `RaceTabBinding` from the banner's own module.
- Modify: `race_tabs/mod.rs` (the desktop parent) to import `RaceTabBanner` from the new shared path.
- Modify: every `use crate::...race_tab_banner...` and every `use crate::...race_tabs::RaceTabBinding` inside the moved subtree to the new shared path.

- [ ] **Step 1: Move the directory**

```bash
cd /home/clemens/.local/src/warcraft-hotkey-editor
git mv crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_tabs_bar/components/race_tabs_host/components/race_tabs/components/race_tab_banner crates/hotkey-editor/src/components/app/components/shell/components/shared/race_tab_banner
```

- [ ] **Step 2: Re-home `RaceTabBinding` inside the shared subtree**

Create `shared/race_tab_banner/binding/mod.rs` holding the `RaceTabBinding` struct (moved verbatim from the old `race_tabs/model/mod.rs`), add `pub mod binding;` to `shared/race_tab_banner/mod.rs`, and delete the struct from `race_tabs/model/mod.rs`. Every per-race wrapper and `RaceTabState` changes its import from `...race_tabs::RaceTabBinding` to `crate::components::app::components::shell::components::shared::race_tab_banner::binding::RaceTabBinding`.

- [ ] **Step 3: Rewrite the absolute import paths in the moved subtree**

Run `rg -n "editor_tabs_bar::components::race_tabs_host::components::race_tabs::components::race_tab_banner" crates/hotkey-editor/src/components/app/components/shell/components/shared/race_tab_banner` and replace every hit's prefix with `crate::components::app::components::shell::components::shared::race_tab_banner`. Do the same for any `...::race_tabs::RaceTabBinding` reference, pointing it at the new `binding` module.

- [ ] **Step 4: Point the desktop parent at the shared banner**

In `race_tabs/mod.rs`, change `use components::race_tab_banner::RaceTabBanner;` to `use crate::components::app::components::shell::components::shared::race_tab_banner::RaceTabBanner;`, and remove the now empty `race_tab_banner` entry from `race_tabs/components/mod.rs`.

- [ ] **Step 5: Verify the desktop still builds and looks unchanged**

Run `moon run :check --force`. Expected PASS. Then reload the dev server and confirm the desktop race tabs still render with their banners. Commit.

```bash
git add -A
git commit -m "Relocate RaceTabBanner to a shared shell leaf

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 1: Reusable segmented control leaf

A segmented control of N options where exactly one is active, used by both `Find units by` and `Mode`. Mirror the mockup's `.segment`/`.seg` look with `tw_macro` classes.

**Files:** create `.../search_dialog/components/search_dialog_body/components/shared/segmented_control/` with `mod.rs`, `model/mod.rs`, `view/mod.rs`, `style/mod.rs`, and a child `segment_option/` (the individual segment, one class). Per COMPONENTS.md, exactly one look, active and idle are one component with a state modifier is not allowed, so the segment is a dispatcher rendering `ActiveSegmentOption` xor `IdleSegmentOption`, each its own directory. The control takes a `Vec` of option data (a value key, a label, an is_active flag, an on_pick handler) as named-field domain data, never `Element`.

- [ ] Write the component tree, then `moon run :check --force`, then commit. (Full code authored during execution against the mockup and the existing `toggle_button` dispatcher pattern.)

---

### Task 2: Switch leaf with an info popover

**Files:** create `shared/filter_switch/` (the on and off switch, a dispatcher over `on_filter_switch` and `off_filter_switch`, mirroring `mockup .switch`), and `shared/info_popover/` (the small info button plus its popover, one classed root, the popover text arrives as a named `&'static str` prop from `data.rs`). The popover opens on click and on hover, closes on outside click, and is anchored to its row so it never overflows.

- [ ] Write, check, commit.

---

### Task 3: The races sub-panel, reusing `RaceTabBanner`

**Files:** create `search_dialog_filters/components/race_scope_menu/` with a trigger (`race_scope_trigger`, showing the swatches and the summary), a floating `race_scope_panel` that renders the five `RaceTabBanner` leaves from the shared location, and a `race_scope_backdrop` (mirror `SearchScopeBackdrop`). The panel's presentation reads `editor.search_race_scope()`, builds one banner per `AllRaces::ALL` with `is_active = scope.admits(Some(race))` and an `on_pick` that toggles the race in the scope with the last one staying on (the same logic already in `race_chip_row/presentation`, moved here). The banner badges stack vertically, the panel scrolls if it exceeds the height.

- [ ] Write, check, verify in browser, commit.

---

### Task 4: The filter panel and the dialog search bar

**Files:**
- `search_dialog_filters/` becomes the floating panel, rendering the segmented `Find units by`, the segmented `Mode`, the two `filter_switch` rows under `Also include`, and the `race_scope_menu`. Its presentation reads and writes the four state sources, the search field (`editor.search_field()`), the mode selection (`navigation.unit_modes()` mapped to and from the three way choice), and the two booleans.
- New `search_dialog_bar/` in the dialog, composing `UnitListSearchIcon` and `UnitListSearchInput` from their shared paths, plus a `filter_trigger` (the filter icon with the active count badge) that toggles the panel. Root is `relative` so the panel and scrim anchor to it. It does not render `SearchScopePicker`.
- New `search_dialog_scrim/` mirroring `burger_backdrop` (`fixed inset-0 bg-warcraft-shadow/65`), dismisses the panel.
- `search_dialog_body/mod.rs` renders `SearchDialogBar` then `CategoryScroll`, and owns the panel open signal in its presentation.

- [ ] Write, check, verify in browser, commit.

---

### Task 5: Remove the superseded disclosure

Delete the interim `search_config_button/` and the old inline `SearchDialogFilters` disclosure wiring committed earlier in this branch, and the `open` guard prop, now replaced by the floating panel. Ensure no dead references remain.

- [ ] Remove, `moon run :check --force`, commit.

---

### Task 6: Full verification

- [ ] `moon run :ci` green (with `--force` where the out of workspace patch defeats the cache).
- [ ] In a real mobile viewport in the browser, confirm the search field is first, the filter icon opens the floating panel over the results with no layout shift, the scrim dismisses it, `Find units by` and `Mode` are single choice segments with no sticky buttons, the two switches and their popovers read correctly and stay in view, the races row opens the banner badges, toggling a race narrows the search and the last race stays on, and the desktop race tabs are unchanged.

## Out of scope

The tag of `warcraft-data` and the `Cargo.toml` bump, which happen once the whole initiative is ready to ship, not in this plan.
