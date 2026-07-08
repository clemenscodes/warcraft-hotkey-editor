# HANDOFF — eradicate every `Element`-as-prop / `Element`-from-logic

## The rule (authoritative: `docs/COMPONENTS.md` → "`Element` is never a prop and never a value")

`Element` is produced by exactly one thing — a `#[component] fn` — and is **never**
passed around as data. Compose UIs out of well-typed components nested by name.

**Forbidden, everywhere, no exceptions:**
1. `children: Element` (and `Option<Element>`, `Vec<Element>`, any `Element`-typed
   prop/struct field/enum variant, or a `#[component] fn Foo(children: Element)` param).
2. `fn … -> Element` that is not itself a `#[component]` — no trait method returning
   `Element` (`trait XKind { fn tile/cells/scroll/header(…) -> Element }`), no
   free/`logic.rs`/`hooks.rs` fn returning `Element`. Logic returns **data**.
3. Binding markup and threading it as a prop: `let children = rsx!{…}; Foo { children }`.

**Compose instead:** a parent *names* its children (specific typed components, or a
`for` over typed **data** rendering one specific typed component); it never *receives*
them. Share a *look* by writing the same utility classes in each `style.rs` (sharing a
value is not coupling); reuse a *piece* as a typed leaf nested by name. A generic base
selects typed **data** via a marker `B` and renders a **fixed** typed component via
`From`; where variants render different components, there is no base — separate components.

## ⚠️ State of the tree — do NOT reset to HEAD

**IGNORE any advice to `git checkout HEAD -- .` / reset to `5ed3cfbc`.** That was verified WRONG:
the current working tree has **fewer** `Element` violations than HEAD (HEAD is itself a messy
mid-refactor `wip`), and it holds real, wanted work (the `hotkey_*`→`conflict_*` renames, the
+77-line `docs/COMPONENTS.md` rule this task depends on, a11y `kb-focus`, and several completed
eradications below). Resetting is strictly negative — more violations to fix, real work destroyed.
Work forward from the current tree. Re-grep the three "Done" greps to get the live count.

## What the previous (fired) session did — 2026-07-08

**DONE CORRECTLY, keep (all gated green earlier):**
- **Stat panel** (`unit_stats_panel/`): rebuilt per [[per-data-point-components]] — 4 column
  components, ~16 per-stat row components, shared leaves (`StatValue`/`StatGain`/`RegenQualifier`/
  `StatFigure`). `StatRow<B>`/`StatRowKind`/`StatColumn`/`StatRows`/`MatchupGrid` deleted. Design
  approved + visually verified. **Caveat:** rows still inline `span.LABEL` / (HP/Mana) `span.VALUE`
  — those are TWO extra classes → per the cardinal rule they must become `StatLabel` / value
  components. Minor, but not yet done.
- **`page_state`** dissolved (ClearState/EmptyState now = one `section.CLASS` + component children ✓).
- **`toasts`**: provider moved into `Shell`; gallery gets a bin-local `ToastMount` decorator
  (`crates/gallery/src/stories/toast_mount.rs`) — `StoryFrame`/gallery LIB stays domain-agnostic
  ([[gallery-generic-framework]] — do NOT put app types in `frame.rs`).
- `CLAUDE.md` gained the dev-URL base path (`http://localhost:8123/warcraft-hotkey-editor/`, NOT `/`)
  and the "don't wait on the stuck rebuild overlay — refresh" notes. e2e firefox full-suite flakes
  under machine load: **never run a `tail -f` monitor during the e2e phase**, light-poll only, re-run clean.

**DONE WRONG — must be REDONE per the cardinal rule (they INLINED wrappers → multi-class components):**
The previous session dissolved these by deleting the wrapper and inlining its classed element into the
consumer (multiple `class:` per component = the exact violation). Each must be **converted to a
typed-props component** instead (see the corrected Fix pattern below). Clusters affected:
- **resolve `plan_body`**: `move_row`, `unresolved_row`, `anchor_column`, `active_move_list`,
  `unresolved_section` — inlined `move_card`, `fight_row`, `fight_column`, `move_transition`,
  `transition_column`, `move_list`, `panel_card` (all deleted; need restoring as typed-props components).
- **collisions conflict cards**: `island_conflict_card`, `hotkey_conflict_card`,
  `unit_position_conflict_card` — inlined `conflict_card` + `panel_card`.
- **unit_list**: inlined `unit_category_tabs`, `unit_list_scroll`, `unit_list_track`.
- **tile_override**: inlined `tile_override_panel` (in `unit_detail_row`) + `alt_state_container`/
  `alt_state_header`/`alt_state_header_text` (in `alt_state_section`, `upgrade_section`).
- **collisions detail panes**: `hotkey_unit_detail`, `island_detail`, `unit_position_detail` —
  inlined `detail`, `detail_header`, `conflict_grid`.
- **collisions ability**: `conflict_ability`, `island_conflict_ability` inlined `conflict_ability_trigger`
  (⚠️ `conflict_ability_trigger` was JUST restored as a typed-props component, but the two consumers
  still reference a now-gone `TRIGGER` `ClassList` const → **tree is RED right now**);
  `conflict_marker_view` inlined `conflict_hotkey_badge`; `conflict_multi_stack`, `conflict_pair_row`,
  `island_conflict_card` inlined `conflict_ability_row`.

**Recovery:** it is uncommitted, so the cleanest path for each wrong cluster is `git checkout -- <files>`
to restore the original `children: Element` wrappers, then convert each wrapper `children` → typed props
(NOT inline). Gate after each cluster. `git status`/`git diff` show exactly which files each cluster touched.

## The full inventory (current tree — re-grep, counts have moved)

Only allowed commands: **`moon run :ci`** (the one gate) and **`moon run :dev`**. Nothing else
(no bare cargo/dx/playwright, no individual moon targets).

### Category 1 — `fn … -> Element` in logic (~23 sites; grep `-> Element` then drop `#[component]`)
Path root: `crates/hotkey-editor/src/components/app/components/shell/components/`
- **Stat panel** (`…/editor_page/…/unit_stats_panel/`): `shared/stat_row/kind.rs` (`fn cells(Self::Value)->Element`)
  and its impls in `attributes_column/kinds.rs` (×3), `combat_column/kinds.rs` (×5), `vitality_column/kinds.rs`
  (×4), `defense_column/kinds.rs` (×4). **The entire stat panel is unsanctioned — refactor it wholesale.**
  Decide: do all stat cells render the SAME leaf with different props? If yes → the kind returns typed
  **props/data**, `StatRow` renders the fixed cell; if the cells differ → dissolve into separate row components.
- **Grid** (`…/unit_detail_row/components/shared/grid_editors/grid_editor/…/grid/`): `grid_tile_kind.rs`
  (trait `fn tile(Self::Tile)->Element`) + impls `grid_editor_tile/kind.rs`, `…/grid/components/grid_tile/kind.rs`,
  `grid_editor_tile/components/tile_face/kind.rs`. The two kinds render **different** tile components
  (interactive `GridEditorTile` vs read-only `GridTile`) → there is no valid generic `Grid<B>`; dissolve it,
  each user renders its own tiles into its own grid `div` (shared class value). `GridEditor/mod.rs`'s
  `pub(crate) fn GridEditor<B>` is a `#[component]` — NOT a violation, leave the fn signature.
- **Sidebar** (`…/collisions_page/…/sidebars/sidebar/`): `list_scroll_kind.rs` (`fn scroll(children:Element)->Element`)
  + `components/collision_list_scroll/mod.rs` impl. Dissolve `ListScrollKind` + the generic `Sidebar<B>`; the
  `aside`+scroll shell becomes markup each sidebar variant writes itself, rendering its cards directly.

### Category 2 — `Element`-typed props (41 sites; grep `: Element` in props.rs / component params)
Every one is a wrapper that must be dissolved (its single/typed content rendered by the consumer directly,
shared class written per consumer). By area:
- **Dialogs** (`…/toolbar_actions/components/shared/dialogs/`): `dialog/props.rs`, `dialog/components/dialog_body`,
  `layout_editor/components/layout_editor_content` (inline `children` param), `system_hotkeys_dialog/…/system_hotkeys_section`.
- **Sidebars** (collisions): `sidebar` (`children` + kind), `collision_list_scroll`, `…/collision_list_track`.
- **Details** (collisions): `shared/detail`, `shared/detail_header`, `shared/conflict_grid`, `shared/conflict_card`,
  `shared/conflict_ability_row`, `shared/conflict_ability_trigger`, `shared/conflict_marker_view/…/conflict_hotkey_badge`,
  `shared/row_meta`. Plus `collisions_shell`, `body/…/content`.
- **Resolve** (`…/resolve_page/…/plan_body/`): `move_card`, `move_transition`, `fight_row`, `fight_column`,
  `active_move_list/…/move_row/components/transition_column`, `shared/move_list`.
- **Editor tile-override** (`…/unit_tile_override/tile_override/…`): `tile_override_card/components/shared/alt_state_header`,
  `…/alt_state_header_text`, `…/alt_state_container`, `…/shared/alt_position_picker_body`,
  `…/shared/alt_position_picker_grid_anchor`, `…/tile_override_panel`.
- **Unit list**: `unit_list_scroll`, `unit_list_scroll/…/unit_list_track`, `unit_category_tabs`.
- **Stats**: `shared/stat_rows`, `shared/stat_column`, `shared/matchup_grid` (this session added the last as a leaf).
- **Session-added shared wrappers (delete these):** `shared/page_state`, `shared/panel_card`, `shared/selectable_entity_card`.
- **Toasts**: `toasts`.

### Category 3 — `let <x> = rsx!{…}` threaded as a prop (17 sites; grep `let .* = rsx!`)
Every dialog `logic.rs` (help/info/key_picker/layout_editor/preview/system_hotkeys/system_key_picker/templates),
the pickers (`alt_position_picker`, `upgrade_position_picker`), the system-hotkeys views (`control_groups`,
`hero_selection`, `inventory`), both sidebars (`island_sidebar`, `unit_cards_sidebar`), `collision_card`, `unit_card`.
These build a `children` variable and pass it to a Category-2 wrapper — they get fixed together with their wrapper:
delete the wrapper, inline the rsx where it belongs.

## ⚠️⚠️ THE CARDINAL RULE THE LAST SESSION GOT WRONG — READ THIS FIRST ⚠️⚠️

**ONE classed root element per component. Full stop.** (`docs/COMPONENTS.md`: *"The one piece of
markup a Host owns is a single classed root that wraps its leaf."*) A component's body is exactly:
**one element with a `class`, containing typed child components nested by name (or a `for` over data
rendering one typed component).** Nothing else carries a class.

**If a component's markup assigns `class:` to two or more elements, that is PROOF you failed to
extract a component.** `div.A { div.B { … } }` is illegal — `div.B` must be its own component. This
includes raw `ClassList` consts (`const PANEL: ClassList = …; div { class: PANEL, … }`) — those count
as a class. There is no "leaf span" or "inline scaffold" loophole: `span.LABEL` + `span.VALUE` inside
one row is already two extra classes → they are `StatLabel` / `StatValue` **components**.

**So you DO NOT "dissolve" a `children: Element` wrapper by deleting it and inlining its classed
element into the parent.** That is the exact mistake that produced multi-class components. You
**CONVERT** the wrapper into a component that takes **typed props** instead of `children`, keeping its
single classed root and naming its children by type. The wrapper stays a component; only its
`children: Element` prop is replaced by the typed props of the specific children it wraps.

## Fix pattern (concrete — CORRECTED)
- **`Foo { children }` wrapping one typed component** (e.g. `Foo { Bar { ..x } }`) → `Foo` keeps its
  single classed root; give it a **typed prop** `bar: BarProps` and render `root.CLASS { Bar { ..bar } }`.
  Consumer passes `Foo { bar: x }`. **Never inline `Foo`'s root into the consumer.**
- **`Foo { children }` wrapping several fixed typed components** → `Foo` takes one typed prop per child
  (`a: AProps, b: BProps`) and renders `root.CLASS { A { ..a } B { ..b } }`.
- **`Foo { children }` wrapping a `for` over data** → `Foo` takes the data (`items: Vec<ItemProps>` /
  a `Signal`) and renders `root.CLASS { for it in items { Item { ..it } } }`.
- **`Foo`'s children genuinely DIFFER per consumer** (a shared scaffold class worn by structurally
  different content) → there is no single `Foo`; each consumer gets its **own** component that owns
  the classed root and names its own children, and they **share the class VALUE** (same utilities
  written in each `style.rs` — sharing a value is not coupling). Still one classed root each.
- **`XKind { fn f() -> Element }`** → change `f` to return typed **props/data**; the base renders the
  fixed leaf from them. Different leaves → separate components.
- **`let children = rsx!{}`** → never; the markup becomes typed props on the (now typed) child component.

## e2e-coupled selectors to preserve (grep `crates/hotkey-editor/e2e` before renaming any class/attr)
`.unit-card`, `data-unit-kind`, `data-selected`, `.collision-card`?/`data-collision-key`, `.conflict-detail-unit`,
`.island-conflict-unit`, `.conflict-card`, `.conflict-more`, `.island-conflict-ability`, `data-action="apply-cascade"`,
`.apply-button`, `[data-layout-col]`/`[data-layout-row]`, `[data-inventory-slot]`, `.system-slot-key`, `.filled-tile`/
`.empty-tile`/`data-grid-*`, `data-collision-kind`/`data-resolve-state`, `.hotkey-editor-grid` etc. Keep every one; if a
class must change, update the e2e spec in the SAME change (the gate runs Playwright).

## Done = all three greps clean AND `moon run :ci` green
```
grep -rn "fn .*-> Element" crates/hotkey-editor/src | grep -vE "pub(\(crate\))? fn [A-Z]"   # → empty
grep -rn ": Element" crates/hotkey-editor/src --include=props.rs                             # → empty (no Element props)
grep -rn "let [a-z_]* = rsx!" crates/hotkey-editor/src                                       # → none threaded into a prop
moon run :ci                                                                                  # → green (fmt/clippy/test/wasm/e2e)
```
The `.claude/hooks/spec-lint.sh` PreToolUse hook already blocks new `children: Element` — good; do not route around it
(do not heredoc past it). If it fires, the code is wrong, not the hook.
