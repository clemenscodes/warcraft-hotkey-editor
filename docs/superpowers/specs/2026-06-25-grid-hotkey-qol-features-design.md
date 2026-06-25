# Grid/Hotkey QOL features — design

Date: 2026-06-25
Status: approved, pending implementation plan

Two independent quality-of-life improvements driven by user feedback.

1. A toggle that decides whether dragging an ability to a new grid cell also
   re-assigns its hotkey, or only moves its position.
2. Double-clicking an ability icon opens its hotkey picker directly, instead of
   selecting the icon and then clicking the small key-field button.

Both features respect the wall between the `hotkey-editor` renderer and the
pure-Rust `warcraft-keybinds` domain crate (see `docs/ARCHITECTURE.md`). Only
Feature 1 touches the domain crate; Feature 2 is renderer-only and reuses the
existing hotkey-assignment command path unchanged.

---

## Feature 1 — Toggle "update hotkeys when moving" in the grid layout dialog

### Behavior

A checkbox lives in the `LayoutEditor` dialog. It is **ON by default** and its
state **persists across sessions**.

- **ON** (current behavior): dragging an ability to a new grid cell updates both
  its `Buttonpos` and its hotkey to the grid letter of the destination cell.
- **OFF**: a drag updates **position only**. The ability keeps its existing
  hotkey — including a hotkey the user set manually. Example: assign Patrol to
  `P`, move it to a different cell, and it stays `P` instead of snapping back to
  the cell's grid hotkey.

The checkbox governs **drag-moves only**. The explicit "Apply grid to all
hotkeys" button is unaffected — re-assigning every hotkey from the grid is its
entire purpose, so it always assigns regardless of the checkbox.

### Deliberate consequence: conflicts stay visible

A position-only move can leave an ability's manual hotkey colliding with another
ability's hotkey. We do **not** auto-resolve this. The conflict remains visible
exactly the way manually-assigned hotkey conflicts already are surfaced in the
key picker. Opting out of re-assignment means opting out of the cascade that
would have prevented the collision; that is the intended trade-off.

### Domain change (`warcraft-keybinds`)

`MoveRequest` (`crates/warcraft-keybinds/src/command/move_request.rs`):

- Add a private `assign_hotkey_on_move: bool` field, defaulting to `true` in
  `MoveRequest::new`.
- Add a `with_assign_hotkey_on_move(bool)` builder method and an
  `assign_hotkey_on_move()` accessor, matching the existing
  `with_prevent_swap` / `with_prevent_co_move` pattern (private fields, explicit
  accessors, no tuples — per `docs/RUST_STYLE.md`).

`CustomKeys::move_slot` (`crates/warcraft-keybinds/src/custom_keys.rs`):

- Read `request.assign_hotkey_on_move()` and forward it into `assign_position`.

`CustomKeys::assign_position`:

- Gains an `assign_hotkey: bool` parameter.
- When `false`, skip **only** the hotkey setters — `set_hotkey`,
  `set_research_hotkey`, `set_unhotkey`, and the command-context `set_hotkey`.
- All position setters (`set_button_position`, `set_unbutton_position`,
  `set_research_button_position`) always run.
- `fan_out_position` is unchanged — it already copies positions only and never
  touches hotkeys.

All **other** callers of `assign_position` pass `assign_hotkey: true`, including
the "Apply grid to all hotkeys" path (`Positions::apply_grid_to_all_known_objects`).
Only the `move_slot` drag path forwards the request's flag.

### Domain tests (required — every domain change ships with tests)

- Move with flag `false`: destination `Buttonpos` updated, hotkey unchanged.
- Swap with flag `false`: both abilities' positions swap, both hotkeys preserved.
- Move/swap with flag `true`: identical to current behavior (regression guard).
- Tiered ability move with flag `false`: position fans out to tiered siblings,
  hotkeys preserved.

### UI / persistence wiring (`hotkey-editor`)

- New app-level signal `update_hotkeys_on_move: Signal<bool>` in `app.rs`,
  created alongside the other selection/layout signals.
- Persisted via `CustomKeysPersistence`
  (`crates/hotkey-editor/src/services/customkeys/persistence.rs`):
  - New storage key constant
    `warcraft-hotkey-editor.update-hotkeys-on-move`.
  - `load_update_hotkeys_on_move() -> Option<bool>` and
    `save_update_hotkeys_on_move(bool)`, following the `grid_layout` pattern.
  - Load on boot (default `true` when absent); a `use_effect` in `app.rs` saves
    on change.
- Threaded to:
  - the `LayoutEditor` dialog, which renders the checkbox bound to the signal;
  - `GridCell`, which reads it at drag-drop time when it constructs the
    `MoveRequest`, calling `.with_assign_hotkey_on_move(value)`. This follows the
    same prop path the existing `selected_slot` signal already travels
    (`UnitDetailPanel → UnitCommandGrids → CommandGridSection → GridTile →
    GridCell`) and the dialog's existing prop set.

---

## Feature 2 — Double-click an icon to open its hotkey picker

### Behavior

- **Single-click** an ability icon: select only (unchanged).
- **Double-click** an ability icon: select it **and** open the key picker for its
  **primary** hotkey field (`OverrideEditTarget::Hotkey`). The picker already
  grabs focus and accepts a keypress to assign the hotkey, so a double-click
  followed by pressing a key is the full quick-assign flow.
- For abilities with no primary hotkey field (e.g. passives), double-click just
  selects — no empty picker opens.

This is **renderer-only**. It reuses the existing key picker and the existing
`HotkeyOverride::apply` → `CustomKeys::set_hotkey` facade path verbatim. No
domain change.

### Wiring (`hotkey-editor`)

- Lift the `editing_target` signal out of `TileOverridePanel`
  (`crates/hotkey-editor/src/components/tile_override/mod.rs`) up to `app.rs`,
  alongside `selected_slot`. Make the `OverrideEditTarget` enum `pub(crate)`.
  The panel reads `editing_target` from props instead of owning it; all of its
  existing internal sets and selection-change resets continue to operate on the
  same lifted signal.
- Thread `editing_target` through the existing 5-layer path
  (`UnitDetailPanel → UnitCommandGrids → CommandGridSection → GridTile →
  GridCell`), the same path `selected_slot` already uses.
- `GridCell` (`crates/hotkey-editor/src/components/command_grid/grid_cell/mod.rs`)
  gets a double-click handler (`ondblclick`, or `event.data().detail() == 2` on
  the existing click handler). On double-click of an occupied cell:
  1. set `selected_slot` to the occupant (and the research/uprooted selection
     flags, as the single-click handler does);
  2. set `editing_target = Some(OverrideEditTarget::Hotkey)`.
- The panel **guards** opening: the picker only appears when the primary hotkey
  field actually applies to the selected ability (the panel already computes
  `show_hotkey_field`). So `picker_open` requires both
  `editing_target == Some(Hotkey)` **and** the field being shown — a
  double-click on a passive is a harmless select. This keeps the "is this
  passive?" domain decision inside the panel, never in the grid cell.

### Verification (no pure-Rust unit test applies)

- Extend the Playwright e2e smoke suite: double-click an ability icon → picker
  is open → press a key → hotkey is assigned.
- Manually verify in-browser (project rule: type-checking/tests verify code
  correctness, not feature correctness).

---

## Out of scope

- Single-click-opens-picker and global keyboard capture on selection were
  considered and rejected in favor of double-click (explicit, no accidental
  picker on normal selection/drag).
- Auto-resolving hotkey collisions created by position-only moves (see Feature 1
  "conflicts stay visible").
- Touch/long-press equivalent of double-click (desktop keyboard workflow).
