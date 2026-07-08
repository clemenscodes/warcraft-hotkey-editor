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

## ⚠️ State of the tree RIGHT NOW — do not trust it

The working tree is **mid-refactor, uncommitted, and almost certainly does not compile.**
This session ran a large "reuse dedup" campaign that (wrongly) created many
`children: Element` wrapper components, then began ripping them out and was interrupted:
- `details/` subtree: a DetailKind→typed rewrite was **killed mid-flight** (partial).
- `matchup_grid`: rewritten to a `children: Element` leaf (still a violation).
- Batch-6 focus/tracking edits applied but **never gated**.
- Nothing is committed — **GPG signing is locked** (`git commit` times out; a hook forbids
  `--no-gpg-sign`; the user must unlock the agent to commit).

**Recommended start:** reset to a clean, compiling baseline and do the eradication from
there. Two options:
- `git checkout HEAD -- .` — HEAD is `5ed3cfbc` (pristine, pre-session, compiles). **Recommended.**
  The session's dedup wrappers are themselves violations you'd have to remove anyway, so
  starting clean shrinks the surface. Note: the pre-existing violations (dialogs, sidebars,
  stats, grid_editors) are all still present at HEAD — they are the real job.
- Restore point `62decbeb` (a `git stash create` object = "waves 1-2 green"): keeps the
  session's dedup structure, but that structure is full of the violations to remove. Not recommended.
Only the legit, spec-aligned session bits (the `hotkey_*`→`conflict_*` rename, the
`LayoutTileProps`→`GridCoordinate` typing, the a11y `kb-focus` additions) would be lost by a
HEAD reset — cherry-pick them later if wanted; they are minor next to a clean base.

## The full inventory (current tree; re-grep after you pick a baseline)

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

## Fix pattern (concrete)
- **Wrapper `Foo { children }` consumed once with a fixed shape** → delete `Foo`; the consumer renders `Foo`'s
  classed element + its specific typed children directly (move `Foo`'s class into the consumer's `style.rs`).
- **Wrapper consumed by N siblings with the SAME shape** → it is a real leaf, but it must take **typed props**,
  not `children`: give it the typed fields and have it render its own specific children.
- **`XKind { fn f() -> Element }` where variants render the same leaf** → change `f` to return typed **props/data**;
  the base renders the fixed leaf from them. **Different leaves** → delete the base+trait; separate components.
- **`let children = rsx!{}`** → never; inline the markup at its destination once the wrapper is gone.

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
