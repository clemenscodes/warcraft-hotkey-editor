# Frontend Refactor Plan — Composable Tailwind Components

> **Status**: In progress. Component decomposition (WT-A through WT-F) is
> largely complete — the major components now live in sub-directories. CSS
> consolidation (WT-G) is not yet complete.

## Goal

Replace the current 21-file global CSS architecture with Tailwind utility classes
collocated in RSX, break down the four oversized components into sub-100-line
pieces, and collapse the per-component CSS files into a single consolidated
`@layer components` block. Behavior must be **byte-for-byte identical** after
each worktree lands.

---

## Current State (what is wrong)

| File | Lines | Problem |
|---|---|---|
| `command_grid.rs` | 1,001 | Drag state, grid rendering, event handlers, CSS logic all in one place |
| `unit_detail.rs` | 1,122 | Stats data model, slot computation, header, stats panel, grids, all mixed |
| `tile_override.rs` | 854 | Key capture, position pickers, description display tangled together |
| `app_header.rs` | 336 | Brand, toolbar, burger drawer, 3 dialogs all in one component |
| `system_hotkeys/inventory.rs` | 389 | Reasonable, but styles are split from component |
| CSS total | 8,479 | 21 files; mobile-foundation.css alone is 2,279 lines |

---

## Hard Rules for Every Agent

These come from `CLAUDE.md` and `docs/RUST_STYLE.md`. Breaking them is not
acceptable — check them before submitting.

1. **The wall holds.** No domain logic crosses into the renderer. No cascade
   lookups, no collision resolution, no `binding.set_*` calls in UI code.
2. **`moon run :ci` must be green** when your branch is done.
3. **`window :hot`**: open the app in a browser and verify the feature you
   touched before marking done.
4. **Rust style**: full semantic names, no abbreviations, no tuples, no `pub`
   fields, no `as` casts outside `From`/`TryFrom`, no inline struct construction
   in argument position, no `verb_noun` free functions.
5. **No `#[ignore]` on tests** — failing tests are the release gate.
6. **Touch only what the task requires.** Do not clean up unrelated code.
   Mention any violations you see; do not fix them.
7. **Tailwind utilities go in RSX class attributes.** Do not add new per-
   component CSS files. Complex selectors that cannot be expressed as utilities
   (pseudo-elements, data-attribute combinators, deep descendant state like
   `.has-ability.selected`) belong in `@layer components` inside
   `tailwind.input.css` or the single `styles/components.css` target.
8. **Do not touch `crates/warcraft-database/src/db.rs`.**

---

## CSS Strategy

### What moves to Tailwind inline classes

All structural layout styles: `display`, `flex`, `grid`, `gap`, `padding`,
`margin`, `width`, `height`, `overflow`, `position`, `border-radius`, `color`
where the color maps to a Tailwind custom token (`text-warcraft-gold`,
`bg-warcraft-bg-panel`, etc.).

### What stays in `@layer components`

- State-class selectors: `.has-ability.selected`, `.grid-tile.drag-over`,
  `.toolbar-icon-button.active`
- Data-attribute selectors: `[data-race="human"] .grid-tile`,
  `[data-race="orc"] .race-tab`
- Pseudo-elements: `::before`, `::after` used for decorative borders / overlays
- Complex CSS custom-property chains that feed child elements
- Any `@keyframes` animation
- `cursor: grab`, `cursor: grabbing`, `touch-action`, `user-select` overrides
  that can't be composed via Tailwind modifiers

### Target file structure

```
tailwind.input.css          ← only @import "tailwindcss" + @theme + @font-face
styles/components.css       ← single @layer components block (replaces all 20 per-component files)
styles/mobile.css           ← single @layer mobile block (replaces mobile-foundation.css)
```

`tailwind.input.css` changes from 22 `@import` lines to:
```css
@import "tailwindcss";
@import "./styles/components.css" layer(components);
@import "./styles/mobile.css" layer(mobile);
@source "./src/**/*.rs";
/* @theme and @font-face stay here */
```

---

## Worktree Tasks (run in parallel)

Each worktree is independent. File ownership is listed to avoid conflicts.

---

### WT-A — AppHeader decomposition

**Owns**: `src/components/app_header.rs`, `styles/app-header.css`
**Does not touch**: any other component file, `styles/mobile-foundation.css`

**Current**: 336 lines, 3 dialogs, burger drawer, brand, toolbar all in one `fn`.

**Target structure** (`src/components/app_header/`):
```
mod.rs            ← AppHeader orchestrator, <100 lines
brand.rs          ← AppHeaderBrand: logo image + title + mirrored image
toolbar.rs        ← HeaderToolbar: upload, templates, system-hotkeys, export buttons
burger.rs         ← BurgerMenu + BurgerDrawer + backdrop (all burger state lives here)
```

`AppHeaderBrand` renders the two gold-decoration images and the `<h1>`.
`HeaderToolbar` renders the action buttons; it receives the same signals as
today but delegates dialog state to its children.
`BurgerMenu` owns `burger_open`, `burger_upload_info_open`,
`burger_download_info_open` signals internally; it calls `UploadInfoDialog` and
`DownloadInfoDialog`.
The layout-dialog and templates-dialog stay in `AppHeader` (the mod.rs
orchestrator) since they are opened from both desktop toolbar and burger.

**CSS target**: inline Tailwind for layout; keep state selectors
(`.toolbar-icon-button.active`, `.layout-pill`, burger drawer animation) in the
consolidated `styles/components.css`. Delete `styles/app-header.css`.

**Acceptance**:
- All header buttons work (upload, templates, system hotkeys, export, preview)
- Burger drawer opens/closes on mobile
- `moon run :ci` green

---

### WT-B — UnitDetailPanel decomposition

**Owns**: `src/components/unit_detail.rs`, `styles/unit-detail.css`,
`styles/stats-panel.css`, `styles/matchups.css`
**Does not touch**: `command_grid.rs`, `tile_override.rs`

**Current**: 1,122 lines mixing slot-data computation, hero level picker, stats
columns, grid orchestration.

**Target structure** (`src/components/unit_detail/`):
```
mod.rs              ← UnitDetailPanel: <120 lines; computes slot data, delegates to children
slot_data.rs        ← UnitSlotData struct + impl (compute, empty, all accessors)
leveled_stats.rs    ← LeveledStats struct + impl + for_hero
stat_icon.rs        ← StatIcon struct + From<AttackType> + From<DefenseType> + From<PrimaryAttribute>
header.rs           ← UnitDetailHeader: portrait + name + HeroLevelPicker
stats_panel.rs      ← UnitStatsPanel: the vitality/combat/defense/attributes columns
                      includes DamageMatchupRow, DefenseMatchupRow, AttributeRow
grids.rs            ← UnitCommandGrids: the 4 CommandGridSection instantiations
```

`UnitDetailPanel` (mod.rs) reads its signals, calls `UnitSlotData::compute`,
constructs `command_card_props` and similar, then renders:
```
section.unit-detail
  UnitDetailHeader { ... }
  UnitStatsPanel { ... }
  div.unit-detail-body
    div.unit-detail-row
      div.unit-detail-grids
        UnitCommandGrids { ... }
      aside.tile-override-panel
        TileOverridePanel { ... }
```

**CSS target**: structural layout (flex columns, gaps, portrait sizing) → Tailwind.
State classes for matchup cells (`.matchup-cell.strong`, `.matchup-cell.weak`)
and stat-row color accents stay in `styles/components.css`. Delete
`styles/unit-detail.css`, `styles/stats-panel.css`, `styles/matchups.css`.

**Acceptance**:
- Hero level picker works and updates stats
- Matchup rows display for attack and defense
- All stat rows show correct values
- `moon run :ci` green

---

### WT-C — CommandGridSection decomposition

**Owns**: `src/components/command_grid.rs`, `styles/command-grid.css`,
`styles/drag-follower.css`
**Does not touch**: `unit_detail.rs`, `tile_override.rs`

**Current**: 1,001 lines. The drag machinery is entangled with the render loop.

**Target structure** (`src/components/command_grid/`):
```
mod.rs          ← re-exports; CommandGridSection orchestrator, <120 lines
drag_state.rs   ← all thread_local! statics + DragOrigin + PendingDragData
                   + DragThreadState impl + TouchScrollLock type alias
grid_cell.rs    ← GridCell component: renders one tile; owns all pointer/click
                   event handlers for that tile
tile_class.rs   ← tile_class() free function (stays as-is)
```

`CommandGridSection` becomes the outer shell:
- computes `conflicting_hotkeys`
- iterates rows/columns
- for each cell computes occupant + class + drag flags
- renders `GridCell` for each position

`GridCell` receives all the per-cell derived data as props and owns
`onpointerdown`, `onpointermove`, `onpointerup`, `onpointercancel`,
`onlostpointercapture`, `onclick`, `onkeydown`. It reads/writes the drag signals
passed in as props.

**GridCell props sketch**:
```rust
struct GridCellProps {
    class_name: String,
    tabindex_value: &'static str,
    column: u8,
    row: u8,
    heading_text: &'static str,
    is_focusable: bool,
    tile_is_draggable: bool,
    icon_src: Option<IconUrl>,
    label_text: String,
    displayed_letter: Option<String>,
    hotkey_overlay_class: &'static str,
    occupant_slot: Option<GridSlotId>,
    is_research_grid: bool,
    is_uprooted_grid: bool,
    is_passive_on_command_grid: bool,
    is_command_cell: bool,
    layout_snapshot: GridLayout,
    restrict_draggable_to: Rc<[GridSlotId]>,
    // signals (all passed by value, Copy):
    selected_slot: Signal<Option<GridSlotId>>,
    selected_from_research: Signal<bool>,
    selected_from_uprooted: Signal<bool>,
    dragging_slot: Signal<Option<DraggingSlot>>,
    drop_target_cell: Signal<Option<DropTargetCell>>,
    drag_follower: Signal<Option<DragFollower>>,
    keys_signal: Signal<Option<CustomKeys>>,
    slot_ids_for_drop: Rc<[GridSlotId]>,
    prevent_swap_on_drop: bool,
    toast_api: ToastHandle,
}
```

Do **not** change any of the drag logic itself — only move it into a sub-component
and sub-module. Behavior must be identical.

**CSS target**: `.grid-tile.*` state selectors, `.grid-tiles` grid layout, hotkey
overlay badge → keep in `styles/components.css` (they are complex enough to
require CSS). Remove `styles/command-grid.css`, `styles/drag-follower.css`.
Structural padding/color on `.command-section` → Tailwind inline.

**Acceptance**:
- Drag and drop works on mouse and touch (long-press → drag)
- Click to select works; Escape cancels drag
- Hotkey conflict badges appear
- `moon run :ci` green

---

### WT-D — TileOverridePanel decomposition

**Owns**: `src/components/tile_override.rs`, `styles/override-panel.css`,
`styles/key-picker.css`
**Does not touch**: `command_grid.rs`, `unit_detail.rs`

**Current**: 854 lines mixing key capture cells, position pickers, upgrade tier
UI, description display.

**Target structure** (`src/components/tile_override/`):
```
mod.rs              ← TileOverridePanel orchestrator, <120 lines
key_field.rs        ← OverrideKeyField: one key-capture row (label + KeyPickerCell)
position_picker.rs  ← AltPositionPicker + UpgradePositionPicker (already separate fns,
                       extract into dedicated module)
description.rs      ← AbilityDescription: renders ubertip + tip text
upgrade_tier.rs     ← UpgradeTierSelector: the tier cycling button, if applicable
```

`TileOverridePanel` (mod.rs) reads `detail`, derives display values, and
composes the sub-components. It continues to own the `editing_target`,
`alt_position_picker_open`, `upgrade_position_picker_open` signals because they
are tightly coupled to its own layout.

**CSS target**: key cell appearance (`.override-key-cell.editing`), card chrome
(`.tile-override-card`) stay as state selectors in `styles/components.css`.
Layout padding, row/column structure → Tailwind. Delete `styles/override-panel.css`,
`styles/key-picker.css`.

**Acceptance**:
- Clicking a key cell opens the key picker
- Alt-state position picker opens and saves correctly
- Upgrade tier cycling works
- Ability description and tip render
- `moon run :ci` green

---

### WT-E — UnitList + Tabs decomposition

**Owns**: `src/components/unit_list.rs`, `src/components/mode_and_race_tabs.rs`,
`src/components/race_tabs.rs`, `styles/unit-list.css`, `styles/mode-tabs.css`,
`styles/race-tabs.css`
**Does not touch**: anything under `system_hotkeys/`

**Current**: `UnitListPanel` (175L) is borderline acceptable but its CSS is
large (329L). The tabs (68L+54L) are fine in size but use heavy CSS.

**Target**:

`UnitListPanel` → split into:
```
unit_list/
  mod.rs          ← UnitListPanel: search input + scrollable list + category collapse
  unit_card.rs    ← UnitCard: single unit row (icon + name + selected state)
  category.rs     ← UnitCategorySection: heading + collapse toggle + list of UnitCards
```

`ModeAndRaceTabs` stays as one file (68L is fine) but converts layout to Tailwind.
`RaceTabs` stays as one file (54L is fine) but converts layout to Tailwind.

For tabs: `.race-tab`, `.race-tab.active`, `[data-race="*"] .race-tab` selectors
stay in `styles/components.css`. The layout wrapping (flex, gap, overflow-x) →
Tailwind. Delete `styles/unit-list.css`, `styles/mode-tabs.css`,
`styles/race-tabs.css`.

**Acceptance**:
- Search filters units correctly
- Category collapse/expand works
- Race tabs highlight the active race with correct color
- Mode toggle (melee/campaign) works
- `moon run :ci` green

---

### WT-F — SystemHotkeys CSS cleanup

**Owns**: `src/components/system_hotkeys/`, `styles/system-hotkeys.css`
**Does not touch**: any component outside `system_hotkeys/`

The system hotkeys sub-components are already reasonably sized. This task:

1. Converts structural layout in all 8 files to Tailwind utilities.
2. Keeps state selectors (`.key-cell.active`, `.inventory-cell.drop-target`,
   `.slot-button.dragging`) in `styles/components.css`.
3. Deletes `styles/system-hotkeys.css`.

No component splitting is required here unless a file exceeds 200 lines after
conversion (inventory.rs at 389L needs one pass: extract `InventoryGrid` from
`InventoryHotkeysView`).

**Acceptance**:
- System hotkeys dialog opens and closes
- Inventory drag-and-drop works
- Key picker dialog captures keys correctly
- `moon run :ci` green

---

### WT-G — CSS Consolidation (runs last; depends on WT-A through WT-F)

**Owns**: `tailwind.input.css`, `styles/primitives.css`, `styles/preflight.css`,
`styles/app-root.css`, `styles/dialog-shared.css`, `styles/templates.css`,
`styles/preview.css`, `styles/toasts.css`, `styles/layout-editor.css`,
`styles/mobile-foundation.css`

This worktree should be spawned only after WT-A through WT-F are merged, because
it references class names from all components and validates nothing was missed.

**Tasks**:

1. Create `styles/components.css`: aggregate all `@layer components` content that
   WT-A through WT-F left behind (state selectors, pseudo-elements, data-attribute
   combinators, keyframe animations). This file should be the **only** non-Tailwind
   stylesheet after the refactor.

2. Create `styles/mobile.css`: migrate `styles/mobile-foundation.css` (2,279L)
   class-by-class. For each rule:
   - If the element now carries Tailwind responsive classes (e.g. `max-[700px]:px-4`),
     the CSS rule is redundant → delete it.
   - If the rule references a state selector or data-attribute combination that
     cannot be expressed as Tailwind → move it to `styles/components.css` inside
     a `@media (max-width: ...)` block.
   - The target for `styles/mobile.css` is ≤300 lines (rules that genuinely
     cannot be expressed as Tailwind responsive utilities).

3. Migrate remaining small stylesheets (`primitives.css`, `preflight.css`,
   `app-root.css`, `dialog-shared.css`, `templates.css`, `preview.css`,
   `toasts.css`, `layout-editor.css`) the same way: layout → Tailwind, state
   → `styles/components.css`. Delete the source files.

4. Update `tailwind.input.css` to import only the two new files:
   ```css
   @import "tailwindcss";
   @import "./styles/components.css" layer(components);
   @import "./styles/mobile.css" layer(mobile);
   @source "./src/**/*.rs";
   /* @theme and @font-face unchanged */
   ```

**Acceptance**:
- Visual diff against `main` is zero (no pixel regressions)
- `styles/` contains only `components.css` and `mobile.css`
- All old per-component CSS files are deleted
- `moon run :ci` green

---

## Merge Order

```
WT-A  ─┐
WT-B  ─┤
WT-C  ─┼──▶ merge to develop ──▶ WT-G
WT-D  ─┤
WT-E  ─┤
WT-F  ─┘
```

WT-A through WT-F are independent and can merge in any order. WT-G must be last.

---

## What Each Agent Must NOT Do

- Do not touch `crates/warcraft-keybinds/` (domain crate — not part of this refactor)
- Do not touch `crates/warcraft-database/src/db.rs`
- Do not add new feature behavior; this is a structural refactor only
- Do not reformat unrelated code or rename symbols outside your owned files
- Do not introduce new CSS files; consolidate only

---

## Quick Reference: Tailwind Custom Tokens

These are available via `@theme` in `tailwind.input.css`:

| Token | Usage |
|---|---|
| `text-warcraft-gold` | Gold accent text |
| `text-warcraft-text-primary` | Main body text |
| `text-warcraft-text-secondary` | Secondary labels |
| `text-warcraft-text-muted` | Dimmed labels |
| `bg-warcraft-bg-base` | Darkest background |
| `bg-warcraft-bg-mid` | Mid-tone panel background |
| `bg-warcraft-bg-panel` | Translucent panel fill |
| `bg-warcraft-blue` | Standard blue |
| `text-race-human` / `bg-race-human` | Human race color |
| `text-race-orc` / `bg-race-orc` | Orc race color |
| `text-race-nightelf` | Night Elf color |
| `text-race-undead` | Undead color |
| `text-race-neutral` | Neutral color |
