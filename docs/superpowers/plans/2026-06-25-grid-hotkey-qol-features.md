# Grid/Hotkey QOL Features Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add (1) a persisted, on-by-default toggle in the grid layout dialog that makes dragging an ability to a new cell move *position only* (keeping its hotkey), and (2) double-clicking an ability icon to open its primary hotkey picker.

**Architecture:** Feature 1 adds one boolean to the domain `MoveRequest` and gates the hotkey writes inside `assign_position`; the renderer reads a UI preference signal at drag-drop time and passes it on the request. Feature 2 is renderer-only: a `Signal<bool>` intent flag set by a `GridCell` double-click and consumed by `TileOverridePanel` via an effect that opens the existing key picker. The wall between `hotkey-editor` (renderer) and `warcraft-keybinds` (pure domain) is preserved — only Feature 1's domain task touches the crate, and all domain decisions stay in the panel/crate, never in `GridCell`.

**Tech Stack:** Rust, Dioxus (wasm), `warcraft-keybinds` (pure Rust, native `cargo test`), Playwright (e2e), `moon run :ci`.

## Global Constraints

- **The wall:** `warcraft-keybinds` is pure Rust — no `wasm-bindgen`/`web-sys`/`dioxus`/`gloo`. Renderer never computes domain decisions, never mutates `CustomKeysFile` directly, only calls named facade commands.
- **localStorage is the single source of truth** for `CustomKeys.txt` (key `warcraft-hotkey-editor.custom-keys`). UI preferences live in their own separate keys.
- **Every domain change ships with tests** (`docs/RUST_STYLE.md` rules apply to every new/edited Rust line): full semantic names, no tuples, private fields + accessors, no `as` casts, `Self` inside impls, derive what the type qualifies for.
- **Definition of done:** `moon run :ci` is green (includes the Playwright e2e gate), and UI features are manually verified in a browser.
- Existing localStorage UI-pref pattern to follow: `CustomKeysPersistence` in `crates/hotkey-editor/src/services/customkeys/persistence.rs`.

---

### Task 1: Domain — position-only move support (`warcraft-keybinds`)

Add `assign_hotkey_on_move` to `MoveRequest`, gate the hotkey writes in `assign_position`, and forward the flag from `move_slot`. Position writes always run; only hotkey writes are gated.

**Files:**
- Modify: `crates/warcraft-keybinds/src/command/move_request.rs`
- Modify: `crates/warcraft-keybinds/src/custom_keys.rs` (`assign_position` ~693-743, `move_slot` ~825-862, existing test line 2333)
- Test: `crates/warcraft-keybinds/src/custom_keys.rs` (test module, alongside `assign_position_replicates_upgrade_hotkey_per_tier` ~2326)

**Interfaces:**
- Produces: `MoveRequest::with_assign_hotkey_on_move(self, assign: bool) -> Self` (builder, defaults `true`), `MoveRequest::assign_hotkey_on_move(&self) -> bool` (accessor). `CustomKeys::assign_position` gains a trailing `assign_hotkey: bool` parameter.

- [ ] **Step 1: Write the failing tests**

Add these three tests inside the `#[cfg(test)] mod tests` block in `crates/warcraft-keybinds/src/custom_keys.rs`, next to `assign_position_replicates_upgrade_hotkey_per_tier`:

```rust
#[test]
fn move_slot_keeps_hotkey_when_reassignment_disabled() {
    // Position-only move: dragging AEah from (2,2) to (1,1) with
    // assign_hotkey_on_move=false must move Buttonpos but keep the
    // manually-set hotkey P (instead of snapping to (1,1)'s grid letter).
    use crate::identity::slot::GridSlotId;
    let input = "[AEah]\nHotkey=P\nButtonpos=2,2\n";
    let mut keys = CustomKeys::from(input);
    let layout = GridLayout::qwerty_grid();
    let moving = GridSlotId::ability("AEah");
    let slot_ids = [moving];
    let request = MoveRequest::new(layout, &slot_ids, &moving, 1, 1, false)
        .with_assign_hotkey_on_move(false);
    keys.move_slot(&request);
    let binding = keys.binding("AEah").expect("AEah exists");
    let position = binding.button_position().expect("position set");
    assert_eq!(u8::from(position.column()), 1);
    assert_eq!(u8::from(position.row()), 1);
    let hotkey = binding.hotkey().expect("hotkey set");
    let expected = Hotkey::try_from("P").expect("valid hotkey");
    assert_eq!(hotkey, &expected);
}

#[test]
fn move_slot_reassigns_hotkey_by_default() {
    // Default behavior (assign_hotkey_on_move=true): moving AEah to (1,1)
    // rebinds its hotkey to that cell's grid letter (S on QWERTY).
    use crate::identity::slot::GridSlotId;
    let input = "[AEah]\nHotkey=P\nButtonpos=2,2\n";
    let mut keys = CustomKeys::from(input);
    let layout = GridLayout::qwerty_grid();
    let moving = GridSlotId::ability("AEah");
    let slot_ids = [moving];
    let request = MoveRequest::new(layout, &slot_ids, &moving, 1, 1, false);
    keys.move_slot(&request);
    let binding = keys.binding("AEah").expect("AEah exists");
    let hotkey = binding.hotkey().expect("hotkey set");
    let expected = Hotkey::try_from("S").expect("valid hotkey");
    assert_eq!(hotkey, &expected);
}

#[test]
fn move_slot_swap_keeps_both_hotkeys_when_reassignment_disabled() {
    // Swapping AEah (at (0,0), hotkey P) with AHbz (at (1,1), hotkey K)
    // with assign_hotkey_on_move=false swaps both positions but leaves
    // both hotkeys untouched.
    use crate::identity::slot::GridSlotId;
    let input = "[AEah]\nHotkey=P\nButtonpos=0,0\n[AHbz]\nHotkey=K\nButtonpos=1,1\n";
    let mut keys = CustomKeys::from(input);
    let layout = GridLayout::qwerty_grid();
    let moving = GridSlotId::ability("AEah");
    let displaced = GridSlotId::ability("AHbz");
    let slot_ids = [moving, displaced];
    let request = MoveRequest::new(layout, &slot_ids, &moving, 1, 1, false)
        .with_assign_hotkey_on_move(false);
    keys.move_slot(&request);

    let moving_binding = keys.binding("AEah").expect("AEah exists");
    let moving_position = moving_binding.button_position().expect("position set");
    assert_eq!(u8::from(moving_position.column()), 1);
    assert_eq!(u8::from(moving_position.row()), 1);
    let moving_hotkey = moving_binding.hotkey().expect("hotkey set");
    let expected_moving = Hotkey::try_from("P").expect("valid hotkey");
    assert_eq!(moving_hotkey, &expected_moving);

    let displaced_binding = keys.binding("AHbz").expect("AHbz exists");
    let displaced_position = displaced_binding.button_position().expect("position set");
    assert_eq!(u8::from(displaced_position.column()), 0);
    assert_eq!(u8::from(displaced_position.row()), 0);
    let displaced_hotkey = displaced_binding.hotkey().expect("hotkey set");
    let expected_displaced = Hotkey::try_from("K").expect("valid hotkey");
    assert_eq!(displaced_hotkey, &expected_displaced);
}
```

(No new test for tiered fan-out: `fan_out_position` only ever copies positions, never hotkeys, so it is unaffected by this change.)

- [ ] **Step 2: Run the tests to verify they fail**

Run: `cargo test -p warcraft-keybinds move_slot_keeps_hotkey move_slot_reassigns_hotkey move_slot_swap_keeps_both`
Expected: FAIL — `with_assign_hotkey_on_move` does not exist (compile error).

- [ ] **Step 3: Add the `MoveRequest` field, builder, and accessor**

In `crates/warcraft-keybinds/src/command/move_request.rs`, add the field to the struct after `prevent_co_move: bool,`:

```rust
    prevent_co_move: bool,
    assign_hotkey_on_move: bool,
```

In `MoveRequest::new`, set the default after `prevent_co_move: false,`:

```rust
            prevent_co_move: false,
            assign_hotkey_on_move: true,
```

Add the builder after `with_prevent_co_move` (before the accessors):

```rust
    pub fn with_assign_hotkey_on_move(mut self, assign: bool) -> Self {
        self.assign_hotkey_on_move = assign;
        self
    }
```

Add the accessor after `prevent_co_move(&self)`:

```rust
    pub fn assign_hotkey_on_move(&self) -> bool {
        self.assign_hotkey_on_move
    }
```

- [ ] **Step 4: Add the `assign_hotkey` parameter to `assign_position` and gate the hotkey writes**

In `crates/warcraft-keybinds/src/custom_keys.rs`, replace the whole `assign_position` method (currently lines 693-743) with:

```rust
    pub fn assign_position(
        &mut self,
        layout: GridLayout,
        slot: &GridSlotId,
        column: u8,
        row: u8,
        is_research_context: bool,
        assign_hotkey: bool,
    ) {
        let Ok(column_index) = ColumnIndex::try_from(column) else {
            return;
        };
        let Ok(row_index) = RowIndex::try_from(row) else {
            return;
        };
        let Some(letter) = layout.letter_at(column_index, row_index) else {
            return;
        };
        let new_position = GridCoordinate::new(column_index, row_index);
        match slot {
            GridSlotId::Ability(ability_id) => {
                let is_passive = ObjectLookup::is_passive_ability(ability_id.value());
                let grid_hotkey = Self::grid_hotkey_for(*ability_id, letter);
                if let Some(binding) = self.binding_or_default_mut(*ability_id) {
                    if is_research_context {
                        binding.set_research_button_position(Some(new_position));
                        if assign_hotkey {
                            binding.set_research_hotkey(Some(grid_hotkey));
                        }
                    } else {
                        binding.set_button_position(Some(new_position));
                        if assign_hotkey && !is_passive {
                            binding.set_hotkey(Some(grid_hotkey));
                        }
                    }
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                if let Some(binding) = self.binding_or_default_mut(*ability_id) {
                    binding.set_unbutton_position(Some(new_position));
                    if assign_hotkey {
                        let unhotkey = Hotkey::from(letter);
                        binding.set_unhotkey(Some(unhotkey));
                    }
                }
            }
            GridSlotId::Command(command_name) => {
                if let Some(binding) = self.command_or_default_mut(*command_name) {
                    binding.set_button_position(Some(new_position));
                    if assign_hotkey {
                        let command_hotkey = Hotkey::from(letter);
                        binding.set_hotkey(Some(command_hotkey));
                    }
                    binding.set_unbutton_position(Some(new_position));
                }
            }
        }
    }
```

- [ ] **Step 5: Forward the flag from `move_slot`**

In `crates/warcraft-keybinds/src/custom_keys.rs`, update all four `self.assign_position(...)` calls inside `move_slot` to pass `request.assign_hotkey_on_move()` as the new trailing argument.

The moving-slot call (~825):
```rust
        self.assign_position(
            request.layout(),
            request.moving_slot(),
            request.target_column(),
            request.target_row(),
            request.is_research_context(),
            request.assign_hotkey_on_move(),
        );
```

The moving off-state co-move call (~833):
```rust
            self.assign_position(
                request.layout(),
                &GridSlotId::AbilityOff(*moving_id),
                request.target_column(),
                request.target_row(),
                false,
                request.assign_hotkey_on_move(),
            );
```

The displaced-slot call (~847):
```rust
            self.assign_position(
                request.layout(),
                &displaced,
                old_column,
                old_row,
                request.is_research_context(),
                request.assign_hotkey_on_move(),
            );
```

The displaced off-state co-move call (~855):
```rust
                self.assign_position(
                    request.layout(),
                    &GridSlotId::AbilityOff(*displaced_id),
                    old_column,
                    old_row,
                    false,
                    request.assign_hotkey_on_move(),
                );
```

- [ ] **Step 6: Fix the existing `assign_position` test call**

In `crates/warcraft-keybinds/src/custom_keys.rs`, in `assign_position_replicates_upgrade_hotkey_per_tier` (line ~2333), add the new argument:

```rust
        keys.assign_position(layout, &slot, 1, 1, false, true);
```

- [ ] **Step 7: Run the tests to verify they pass**

Run: `cargo test -p warcraft-keybinds`
Expected: PASS — all three new tests pass, and the whole crate's suite (including `assign_position_replicates_upgrade_hotkey_per_tier`) is green.

- [ ] **Step 8: Commit**

```bash
git add crates/warcraft-keybinds/src/command/move_request.rs crates/warcraft-keybinds/src/custom_keys.rs
git commit -m "feat(keybinds): support position-only moves via MoveRequest.assign_hotkey_on_move"
```

---

### Task 2: Toggle persistence helper (`hotkey-editor`)

Add a localStorage-backed boolean preference (default ON) with a pure, unit-tested parse helper, mirroring `OnboardingPersistence`.

**Files:**
- Modify/Test: `crates/hotkey-editor/src/services/customkeys/persistence.rs`

**Interfaces:**
- Produces: `CustomKeysPersistence::load_update_hotkeys_on_move() -> bool` and `CustomKeysPersistence::save_update_hotkeys_on_move(enabled: bool)`.

- [ ] **Step 1: Write the failing tests**

Add a test module at the bottom of `crates/hotkey-editor/src/services/customkeys/persistence.rs` (after `onboarding_tests`):

```rust
#[cfg(test)]
mod update_hotkeys_on_move_tests {
    use super::CustomKeysPersistence;

    #[test]
    fn absent_value_defaults_to_enabled() {
        let stored = None;
        let result = CustomKeysPersistence::update_hotkeys_on_move_from_stored(stored);
        assert!(result);
    }

    #[test]
    fn explicit_false_is_disabled() {
        let stored = Some(String::from("false"));
        let result = CustomKeysPersistence::update_hotkeys_on_move_from_stored(stored);
        assert!(!result);
    }

    #[test]
    fn explicit_true_is_enabled() {
        let stored = Some(String::from("true"));
        let result = CustomKeysPersistence::update_hotkeys_on_move_from_stored(stored);
        assert!(result);
    }
}
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `cargo test -p hotkey-editor update_hotkeys_on_move`
Expected: FAIL — `update_hotkeys_on_move_from_stored` does not exist.

- [ ] **Step 3: Add the storage key and methods**

In `crates/hotkey-editor/src/services/customkeys/persistence.rs`, add the key constant after `GRID_LAYOUT_STORAGE` (line 5):

```rust
const UPDATE_HOTKEYS_ON_MOVE_STORAGE: LocalStorage =
    LocalStorage::new("warcraft-hotkey-editor.update-hotkeys-on-move");
```

Inside `impl CustomKeysPersistence`, after `save_grid_layout` (line 29), add:

```rust
    pub(crate) fn load_update_hotkeys_on_move() -> bool {
        let stored = UPDATE_HOTKEYS_ON_MOVE_STORAGE.get();
        Self::update_hotkeys_on_move_from_stored(stored)
    }

    pub(crate) fn save_update_hotkeys_on_move(enabled: bool) {
        let value = if enabled { "true" } else { "false" };
        UPDATE_HOTKEYS_ON_MOVE_STORAGE.set(value);
    }

    fn update_hotkeys_on_move_from_stored(stored: Option<String>) -> bool {
        let stored_value = stored.as_deref();
        stored_value != Some("false")
    }
```

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test -p hotkey-editor update_hotkeys_on_move`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/hotkey-editor/src/services/customkeys/persistence.rs
git commit -m "feat(editor): persist update-hotkeys-on-move preference (default on)"
```

---

### Task 3: Toggle UI — signal, persistence wiring, and the dialog checkbox

Create the app-level signal, load/save it, thread it to the `LayoutEditor` dialog via the `Header`, and render the checkbox. After this task the checkbox exists and persists, but it does not yet affect moves (Task 4 wires that).

**Files:**
- Modify: `crates/hotkey-editor/src/app.rs` (signal + effect ~after line 92; `Header` call ~488)
- Modify: `crates/hotkey-editor/src/components/shell/header/mod.rs` (`HeaderProps` ~49-63; `LayoutEditor` call ~201-207)
- Modify: `crates/hotkey-editor/src/components/dialogs/layout_editor/mod.rs` (`LayoutEditorProps` ~69-76; render ~239-244)
- Test: `crates/hotkey-editor/e2e/tests/update-hotkeys-toggle.spec.ts` (new)

**Interfaces:**
- Consumes: `CustomKeysPersistence::load_update_hotkeys_on_move` / `save_update_hotkeys_on_move` (Task 2).
- Produces: app signal `update_hotkeys_on_move: Signal<bool>`, now a prop on `Header` and `LayoutEditor`.

- [ ] **Step 1: Create the signal and persistence effect in `app.rs`**

In `crates/hotkey-editor/src/app.rs`, immediately after the `grid_layout` persistence effect (the `use_effect` ending at line 92), add:

```rust
    let update_hotkeys_on_move =
        use_signal::<bool>(CustomKeysPersistence::load_update_hotkeys_on_move);
    use_effect(move || {
        let enabled = *update_hotkeys_on_move.read();
        CustomKeysPersistence::save_update_hotkeys_on_move(enabled);
    });
```

- [ ] **Step 2: Pass the signal to `Header`**

In `crates/hotkey-editor/src/app.rs`, in the `Header { ... }` call (line ~488), add the field after `dragging_layout_cell,`:

```rust
                dragging_layout_cell,
                update_hotkeys_on_move,
```

- [ ] **Step 3: Add the prop to `HeaderProps` and forward it to `LayoutEditor`**

In `crates/hotkey-editor/src/components/shell/header/mod.rs`, add to `HeaderProps` after `dragging_layout_cell` (line ~55):

```rust
    pub(crate) dragging_layout_cell: Signal<Option<EditingCell>>,
    pub(crate) update_hotkeys_on_move: Signal<bool>,
```

In the same file, in the function body where props are destructured (after `let grid_layout = props.grid_layout;`, line ~70), add:

```rust
    let update_hotkeys_on_move = props.update_hotkeys_on_move;
```

In the `LayoutEditor { ... }` call (line ~201), add after `dragging_layout_cell,`:

```rust
                            dragging_layout_cell,
                            update_hotkeys_on_move,
```

- [ ] **Step 4: Add the prop to `LayoutEditorProps` and render the checkbox**

In `crates/hotkey-editor/src/components/dialogs/layout_editor/mod.rs`, add to `LayoutEditorProps` after `loaded_keys` (line ~74):

```rust
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) update_hotkeys_on_move: Signal<bool>,
```

In the function body, after `let mut loaded_keys = props.loaded_keys;` (line ~83), add:

```rust
    let mut update_hotkeys_on_move = props.update_hotkeys_on_move;
```

Add a toggle handler next to the other handlers (after `let handle_picker_close = ...`, line ~159):

```rust
    let toggle_update_hotkeys_on_move = move |_| {
        let current = *update_hotkeys_on_move.read();
        update_hotkeys_on_move.set(!current);
    };
```

In the `rsx!`, insert a checkbox row between the grid `div` and the apply `button`. Replace the apply button block (lines 239-244) with:

```rust
            {
                let is_checked = *update_hotkeys_on_move.read();
                rsx! {
                    label {
                        class: "layout-move-hotkey-toggle flex items-center gap-[0.6rem] \
                            font-friz-quadrata uppercase tracking-[0.06em] text-warcraft-gold \
                            text-[1.6rem] cursor-pointer [text-shadow:1px_1px_0_#000] \
                            max-[1099px]:text-[13px]",
                        input {
                            r#type: "checkbox",
                            "aria-label": "Update hotkeys when moving abilities",
                            checked: is_checked,
                            onchange: toggle_update_hotkeys_on_move,
                        }
                        "Update hotkeys when moving abilities"
                    }
                }
            }
            button {
                class: LAYOUT_APPLY_BUTTON,
                r#type: "button",
                onclick: apply_grid,
                "Apply grid to all hotkeys"
            }
```

- [ ] **Step 5: Verify it compiles and type-checks**

Run: `moon run hotkey-editor:check` (or the project's wasm typecheck task; fall back to `cargo check -p hotkey-editor`)
Expected: compiles with no errors.

- [ ] **Step 6: Write the e2e persistence test**

Create `crates/hotkey-editor/e2e/tests/update-hotkeys-toggle.spec.ts`:

```ts
import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const PREF_KEY = "warcraft-hotkey-editor.update-hotkeys-on-move";

test.describe("Update-hotkeys-on-move toggle", () => {
  test("defaults to checked and persists when unchecked", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".layout-editor-shell").waitFor();

    const toggle = page.locator('input[aria-label="Update hotkeys when moving abilities"]');
    await expect(toggle).toBeChecked();

    await toggle.uncheck();
    const storedAfter = await page.evaluate((key) => localStorage.getItem(key), PREF_KEY);
    expect(storedAfter).toBe("false");

    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".layout-editor-shell").waitFor();
    await expect(
      page.locator('input[aria-label="Update hotkeys when moving abilities"]'),
    ).not.toBeChecked();
  });
});
```

- [ ] **Step 7: Run the e2e test**

Run: `cd crates/hotkey-editor && npx playwright test update-hotkeys-toggle`
Expected: PASS (the dev server / build harness used by the existing e2e suite must be running per the project's `playwright.config.ts`).

- [ ] **Step 8: Commit**

```bash
git add crates/hotkey-editor/src/app.rs crates/hotkey-editor/src/components/shell/header/mod.rs crates/hotkey-editor/src/components/dialogs/layout_editor/mod.rs crates/hotkey-editor/e2e/tests/update-hotkeys-toggle.spec.ts
git commit -m "feat(editor): add persisted update-hotkeys-on-move checkbox to layout dialog"
```

---

### Task 4: Wire the toggle into drag-moves

Thread `update_hotkeys_on_move` from `app.rs` down to `GridCell`, and pass it on the `MoveRequest` at drop time.

**Files:**
- Modify: `crates/hotkey-editor/src/app.rs` (`UnitDetailPanel` call ~515)
- Modify: `crates/hotkey-editor/src/components/unit_detail/mod.rs` (`UnitDetailPanelProps` ~24-36; `UnitCommandGrids` call ~195)
- Modify: `crates/hotkey-editor/src/components/unit_detail/grids/mod.rs` (`UnitCommandGridsProps` ~11-27; body destructure ~31-44; four `CommandGridSectionProps` literals)
- Modify: `crates/hotkey-editor/src/components/command_grid/mod.rs` (`CommandGridSectionProps` ~18-53; `GridTile` call ~116)
- Modify: `crates/hotkey-editor/src/components/command_grid/grid_tile/mod.rs` (`GridTileProps` ~15-36; `cell_props` literal ~280)
- Modify: `crates/hotkey-editor/src/components/command_grid/grid_cell/mod.rs` (`GridCellProps` ~29-56; body destructure ~85; `MoveRequest` build ~501)
- Test: `crates/hotkey-editor/e2e/tests/update-hotkeys-toggle.spec.ts` (extend)

**Interfaces:**
- Consumes: app signal `update_hotkeys_on_move: Signal<bool>` (Task 3); `MoveRequest::with_assign_hotkey_on_move` (Task 1).
- Produces: `update_hotkeys_on_move` as a prop on `UnitDetailPanel`, `UnitCommandGrids`, `CommandGridSection`, `GridTile`, `GridCell`.

- [ ] **Step 1: `app.rs` → `UnitDetailPanel`**

In `crates/hotkey-editor/src/app.rs`, in the `UnitDetailPanel { ... }` call (line ~515), add after `grid_layout,`:

```rust
                            grid_layout,
                            update_hotkeys_on_move,
```

- [ ] **Step 2: `UnitDetailPanel` props + `UnitCommandGrids` call**

In `crates/hotkey-editor/src/components/unit_detail/mod.rs`, add to `UnitDetailPanelProps` after `grid_layout` (line ~35):

```rust
    pub(crate) grid_layout: Signal<GridLayout>,
    pub(crate) update_hotkeys_on_move: Signal<bool>,
```

Wherever the body destructures props (mirror the existing `let grid_layout = props.grid_layout;`), add:

```rust
    let update_hotkeys_on_move = props.update_hotkeys_on_move;
```

In the `UnitCommandGrids { ... }` call (line ~195), add after `grid_layout,`:

```rust
                        grid_layout,
                        update_hotkeys_on_move,
```

- [ ] **Step 3: `UnitCommandGrids` props + four section literals**

In `crates/hotkey-editor/src/components/unit_detail/grids/mod.rs`, add to `UnitCommandGridsProps` after `grid_layout` (line ~26):

```rust
    pub(crate) grid_layout: Signal<GridLayout>,
    pub(crate) update_hotkeys_on_move: Signal<bool>,
```

In the body, after `let grid_layout = props.grid_layout;` (line ~44):

```rust
    let update_hotkeys_on_move = props.update_hotkeys_on_move;
```

Add `update_hotkeys_on_move,` after `grid_layout,` in **all four** `CommandGridSectionProps { ... }` literals (`command_card_props` ~56, `build_menu_props` ~79, `uprooted_props` ~102, `research_props` ~125). Each becomes:

```rust
        grid_layout,
        update_hotkeys_on_move,
```

- [ ] **Step 4: `CommandGridSection` props + `GridTile` call**

In `crates/hotkey-editor/src/components/command_grid/mod.rs`, add to `CommandGridSectionProps` after `grid_layout` (line ~30):

```rust
    pub(crate) grid_layout: Signal<GridLayout>,
    pub(crate) update_hotkeys_on_move: Signal<bool>,
```

In the `GridTile { ... }` call (line ~116), add after `grid_layout,`:

```rust
                            grid_layout,
                            update_hotkeys_on_move,
```

Note: `CommandGridSection` receives `update_hotkeys_on_move` via `props` and forwards it; if the body destructures every prop into a local before use, add `let update_hotkeys_on_move = props.update_hotkeys_on_move;` next to the existing `grid_layout` local. If the `GridTile` call references `grid_layout` directly through `props`, reference `props.update_hotkeys_on_move` the same way — match the file's existing style.

- [ ] **Step 5: `GridTile` props + `cell_props`**

In `crates/hotkey-editor/src/components/command_grid/grid_tile/mod.rs`, add to `GridTileProps` after `grid_layout` (line ~29):

```rust
    pub(super) grid_layout: Signal<GridLayout>,
    pub(super) update_hotkeys_on_move: Signal<bool>,
```

In the `GridCellProps { ... }` literal `cell_props` (line ~280), add `update_hotkeys_on_move: props.update_hotkeys_on_move,` near the other signal fields, e.g. after `keys_signal,`:

```rust
        keys_signal,
        update_hotkeys_on_move: props.update_hotkeys_on_move,
```

- [ ] **Step 6: `GridCell` consumes the flag in the `MoveRequest`**

In `crates/hotkey-editor/src/components/command_grid/grid_cell/mod.rs`, add to `GridCellProps` after `keys_signal` (line ~53):

```rust
    pub(super) keys_signal: Signal<Option<CustomKeys>>,
    pub(super) update_hotkeys_on_move: Signal<bool>,
```

In the body, after `let mut keys_signal = props.keys_signal;` (line ~85), add:

```rust
    let update_hotkeys_on_move = props.update_hotkeys_on_move;
```

In the drop handler where the `MoveRequest` is built (line ~501-510), read the preference and add the builder call:

```rust
                    let assign_hotkey_on_move = *update_hotkeys_on_move.read();
                    let move_request = MoveRequest::new(
                        layout_snapshot,
                        &slot_ids_for_drop,
                        dragging.slot_id(),
                        drop.column(),
                        drop.row(),
                        is_research_grid,
                    )
                    .with_prevent_swap(prevent_swap_on_drop)
                    .with_prevent_co_move(is_uprooted_grid)
                    .with_assign_hotkey_on_move(assign_hotkey_on_move);
```

- [ ] **Step 7: Verify it compiles**

Run: `moon run hotkey-editor:check` (or `cargo check -p hotkey-editor`)
Expected: compiles with no errors or unused-variable warnings.

- [ ] **Step 8: Extend the e2e test with move behavior**

Append this test to `crates/hotkey-editor/e2e/tests/update-hotkeys-toggle.spec.ts` inside the existing `describe`:

```ts
  test("with the toggle off, moving an ability keeps its hotkey", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator(".unit-card").first().click();
    await page.locator(".grid-tile.has-ability").first().waitFor();

    // Give the first ability a known manual hotkey (Q).
    await page.locator(".grid-tile.has-ability").first().click();
    await page.locator(".override-key-cell").waitFor();
    await page.locator(".override-key-cell").click();
    await page.locator(".key-picker-shell").waitFor();
    await page.locator('.key-picker-key[data-label="Q"]').click();
    await expect(page.locator(".override-key-cell")).toContainText("Q");

    // Turn the toggle off.
    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".layout-editor-shell").waitFor();
    await page.locator('input[aria-label="Update hotkeys when moving abilities"]').uncheck();
    await page.locator(".layout-editor-shell [aria-label='Close']").click().catch(() => {});
    await page.keyboard.press("Escape");
    await expect(page.locator(".layout-editor-shell")).toHaveCount(0);

    // Move the ability to a different cell.
    const tiles = page.locator(".grid-tile.has-ability");
    if ((await tiles.count()) < 2) {
      test.skip();
      return;
    }
    await tiles.first().dragTo(tiles.nth(1));

    // The moved ability still shows Q (hotkey not snapped to the new cell).
    await page.locator(".grid-tile.has-ability").first().click();
    await page.locator(".override-key-cell").waitFor();
    await expect(page.locator(".override-key-cell")).toContainText("Q");
  });
```

If the layout dialog's close affordance differs, adjust the close step to match `layout-editor.spec.ts` conventions. Manually verify the move-behavior in a browser regardless, per the Definition of Done.

- [ ] **Step 9: Run the e2e test**

Run: `cd crates/hotkey-editor && npx playwright test update-hotkeys-toggle`
Expected: PASS.

- [ ] **Step 10: Commit**

```bash
git add crates/hotkey-editor/src/app.rs crates/hotkey-editor/src/components/unit_detail/mod.rs crates/hotkey-editor/src/components/unit_detail/grids/mod.rs crates/hotkey-editor/src/components/command_grid/mod.rs crates/hotkey-editor/src/components/command_grid/grid_tile/mod.rs crates/hotkey-editor/src/components/command_grid/grid_cell/mod.rs crates/hotkey-editor/e2e/tests/update-hotkeys-toggle.spec.ts
git commit -m "feat(editor): apply update-hotkeys-on-move toggle to drag-moves"
```

---

### Task 5: Double-click an icon to open its hotkey picker

Add an app-level intent flag set by a `GridCell` double-click and consumed by `TileOverridePanel` via an effect that opens the primary hotkey picker only when that field applies (the panel owns the domain decision via its existing `show_hotkey_field`).

**Files:**
- Modify: `crates/hotkey-editor/src/app.rs` (signal ~after line 160; `UnitDetailPanel` call ~515)
- Modify: `crates/hotkey-editor/src/components/unit_detail/mod.rs` (`UnitDetailPanelProps`; `UnitCommandGrids` call ~195; `TileOverridePanel` call ~214)
- Modify: `crates/hotkey-editor/src/components/unit_detail/grids/mod.rs` (`UnitCommandGridsProps`; four section literals)
- Modify: `crates/hotkey-editor/src/components/command_grid/mod.rs` (`CommandGridSectionProps`; `GridTile` call)
- Modify: `crates/hotkey-editor/src/components/command_grid/grid_tile/mod.rs` (`GridTileProps`; `cell_props`)
- Modify: `crates/hotkey-editor/src/components/command_grid/grid_cell/mod.rs` (`GridCellProps`; body; `ondblclick` on the tile div ~571)
- Modify: `crates/hotkey-editor/src/components/tile_override/mod.rs` (`TileOverridePanelProps` ~45-61; consume effect)
- Test: `crates/hotkey-editor/e2e/tests/hotkey-edit.spec.ts` (extend)

**Interfaces:**
- Produces: app signal `hotkey_assign_request: Signal<bool>`, threaded to `GridCell` (write on double-click) and `TileOverridePanel` (read + clear in an effect that sets the panel's local `editing_target` to `Hotkey`).

- [ ] **Step 1: Create the intent signal in `app.rs`**

In `crates/hotkey-editor/src/app.rs`, after the selection signals (line ~160), add:

```rust
    let selected_from_uprooted = use_signal::<bool>(|| false);
    let hotkey_assign_request = use_signal::<bool>(|| false);
```

- [ ] **Step 2: Pass it to `UnitDetailPanel`**

In the `UnitDetailPanel { ... }` call (line ~515), add after `update_hotkeys_on_move,` (added in Task 4):

```rust
                            update_hotkeys_on_move,
                            hotkey_assign_request,
```

- [ ] **Step 3: `UnitDetailPanel` props, then forward to `UnitCommandGrids` and `TileOverridePanel`**

In `crates/hotkey-editor/src/components/unit_detail/mod.rs`, add to `UnitDetailPanelProps` after `update_hotkeys_on_move`:

```rust
    pub(crate) update_hotkeys_on_move: Signal<bool>,
    pub(crate) hotkey_assign_request: Signal<bool>,
```

Destructure it in the body next to `update_hotkeys_on_move`:

```rust
    let hotkey_assign_request = props.hotkey_assign_request;
```

In the `UnitCommandGrids { ... }` call (line ~195) add after `update_hotkeys_on_move,`:

```rust
                        update_hotkeys_on_move,
                        hotkey_assign_request,
```

In the `TileOverridePanel { ... }` call (line ~214) add after `active_container_slots: active_container_slots.clone(),`:

```rust
                                active_container_slots: active_container_slots.clone(),
                                hotkey_assign_request,
```

- [ ] **Step 4: Thread through `UnitCommandGrids` (props + four literals)**

In `crates/hotkey-editor/src/components/unit_detail/grids/mod.rs`, add to `UnitCommandGridsProps` after `update_hotkeys_on_move`:

```rust
    pub(crate) update_hotkeys_on_move: Signal<bool>,
    pub(crate) hotkey_assign_request: Signal<bool>,
```

Destructure in the body:

```rust
    let hotkey_assign_request = props.hotkey_assign_request;
```

Add `hotkey_assign_request,` after `update_hotkeys_on_move,` in all four `CommandGridSectionProps` literals.

- [ ] **Step 5: Thread through `CommandGridSection` (props + `GridTile` call)**

In `crates/hotkey-editor/src/components/command_grid/mod.rs`, add to `CommandGridSectionProps` after `update_hotkeys_on_move`:

```rust
    pub(crate) update_hotkeys_on_move: Signal<bool>,
    pub(crate) hotkey_assign_request: Signal<bool>,
```

In the `GridTile { ... }` call add after `update_hotkeys_on_move,`:

```rust
                            update_hotkeys_on_move,
                            hotkey_assign_request,
```

Forward via `props` / local in the same style as `update_hotkeys_on_move`.

- [ ] **Step 6: Thread through `GridTile` (props + `cell_props`)**

In `crates/hotkey-editor/src/components/command_grid/grid_tile/mod.rs`, add to `GridTileProps` after `update_hotkeys_on_move`:

```rust
    pub(super) update_hotkeys_on_move: Signal<bool>,
    pub(super) hotkey_assign_request: Signal<bool>,
```

In the `cell_props` literal, after `update_hotkeys_on_move: props.update_hotkeys_on_move,`:

```rust
        update_hotkeys_on_move: props.update_hotkeys_on_move,
        hotkey_assign_request: props.hotkey_assign_request,
```

- [ ] **Step 7: `GridCell` — prop, local, and the double-click handler**

In `crates/hotkey-editor/src/components/command_grid/grid_cell/mod.rs`, add to `GridCellProps` after `update_hotkeys_on_move`:

```rust
    pub(super) update_hotkeys_on_move: Signal<bool>,
    pub(super) hotkey_assign_request: Signal<bool>,
```

In the body, after `let update_hotkeys_on_move = props.update_hotkeys_on_move;`, add:

```rust
    let mut hotkey_assign_request = props.hotkey_assign_request;
```

Add an occupant binding for the handler next to the others (line ~90):

```rust
    let occupant_for_dblclick = occupant_slot;
```

Add the double-click handler next to `handle_click` (after line ~566):

```rust
    let handle_double_click = move |_| {
        let Some(slot) = occupant_for_dblclick else {
            return;
        };
        select_slot.set(Some(slot));
        select_from_research.set(is_research_grid);
        select_from_uprooted.set(is_uprooted_grid);
        hotkey_assign_request.set(true);
    };
```

Wire it onto the tile `div` next to `onclick: handle_click,` (line ~584):

```rust
                onclick: handle_click,
                ondoubleclick: handle_double_click,
```

(Note: Dioxus names the double-click handler `ondoubleclick`. If the compiler reports an unknown attribute, use `ondblclick` to match the actual Dioxus event name in this version.)

- [ ] **Step 8: `TileOverridePanel` — prop + consume effect**

In `crates/hotkey-editor/src/components/tile_override/mod.rs`, add to `TileOverridePanelProps` after `active_container_slots` (line ~60):

```rust
    pub(crate) active_container_slots: Rc<[GridSlotId]>,
    pub(crate) hotkey_assign_request: Signal<bool>,
```

In the body, after `let mut editing_target = use_signal::<Option<OverrideEditTarget>>(|| None);` (line ~76), add the consuming effect. `show_hotkey_field` is computed at line ~112; place this effect *after* that line (it depends on it):

```rust
    let mut hotkey_assign_request = props.hotkey_assign_request;
    let hotkey_field_available = show_hotkey_field;
    use_effect(move || {
        if !*hotkey_assign_request.read() {
            return;
        }
        if hotkey_field_available {
            editing_target.set(Some(OverrideEditTarget::Hotkey));
        }
        hotkey_assign_request.set(false);
    });
```

This sets the picker target only when the primary hotkey field applies (so double-clicking a passive just selects), and always clears the flag so no stuck state can leak into the next selection.

- [ ] **Step 9: Verify it compiles**

Run: `moon run hotkey-editor:check` (or `cargo check -p hotkey-editor`)
Expected: compiles with no errors or unused-variable warnings.

- [ ] **Step 10: Extend the e2e test**

Append to `crates/hotkey-editor/e2e/tests/hotkey-edit.spec.ts` inside the existing `describe` (the `beforeEach` already selects the first unit and first ability tile):

```ts
  test("double-clicking an ability icon opens the key picker and assigns a key", async ({ page }) => {
    await page.locator(".grid-tile.has-ability").first().dblclick();
    await page.locator(".key-picker-shell").waitFor();
    await page.locator('.key-picker-key[data-label="W"]').click();
    await expect(page.locator(".key-picker-shell")).not.toBeVisible();
    await expect(page.locator(".override-key-cell")).toContainText("W");
    const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(stored).toContain("hotkey=W");
  });
```

- [ ] **Step 11: Run the e2e test**

Run: `cd crates/hotkey-editor && npx playwright test hotkey-edit`
Expected: PASS.

- [ ] **Step 12: Commit**

```bash
git add crates/hotkey-editor/src/app.rs crates/hotkey-editor/src/components/unit_detail/mod.rs crates/hotkey-editor/src/components/unit_detail/grids/mod.rs crates/hotkey-editor/src/components/command_grid/mod.rs crates/hotkey-editor/src/components/command_grid/grid_tile/mod.rs crates/hotkey-editor/src/components/command_grid/grid_cell/mod.rs crates/hotkey-editor/src/components/tile_override/mod.rs crates/hotkey-editor/e2e/tests/hotkey-edit.spec.ts
git commit -m "feat(editor): double-click an ability icon to open its hotkey picker"
```

---

### Task 6: Final verification

**Files:** none (verification only)

- [ ] **Step 1: Full CI gate**

Run: `moon run :ci`
Expected: all Rust tests, type checks, and the four+ Playwright e2e smoke/regression tests pass (including the two new specs).

- [ ] **Step 2: Manual browser verification**

Open the app and confirm:
- Layout dialog shows the "Update hotkeys when moving abilities" checkbox, checked by default; unchecking it survives a reload.
- With the toggle ON, dragging an ability to a new cell changes its hotkey to the cell's grid letter (unchanged behavior).
- With the toggle OFF, dragging an ability to a new cell keeps its existing hotkey and only moves the icon.
- Double-clicking an ability icon opens its hotkey picker; pressing a key assigns it. Double-clicking a passive ability just selects it (no picker).

- [ ] **Step 3: Architecture self-check**

Confirm no new code in `hotkey-editor/` imports from `warcraft_keybinds::cascade` or calls `binding.set_*`, and that `GridCell` makes no domain passivity decision (the panel's `show_hotkey_field` is the only gate). Re-read the diff against `docs/RUST_STYLE.md`.

---

## Self-Review

**Spec coverage:**
- Feature 1 behavior (toggle, ON default, position-only move, Apply-grid unaffected) → Tasks 1, 3, 4. The "Apply grid to all hotkeys" path (`apply_grid_to_all_bindings`) is untouched and never calls `assign_position`, so it always assigns — covered by construction.
- Feature 1 "conflicts stay visible" (no auto-resolve) → guaranteed because position-only moves skip only hotkey writes; no collision pass is added.
- Feature 1 persistence (separate key, default ON) → Task 2 + Task 3.
- Feature 1 domain tests (move false, swap false, default true) → Task 1. Tiered fan-out intentionally has no new test (fan_out_position is unchanged; documented).
- Feature 2 behavior (double-click opens primary picker; passive = select only; renderer-only) → Task 5, with the intent-flag wiring (refined from the spec's lift-`editing_target` approach to avoid stuck-state and keep the enum private — see spec update note).
- Verification (e2e + manual + CI gate) → Tasks 3, 4, 5, 6.

**Placeholder scan:** No TBD/TODO; every code step shows complete code. Two intentional adaptive notes (Dioxus `ondoubleclick` vs `ondblclick`; layout-dialog close affordance) point at concrete fallbacks rather than leaving work undefined.

**Type consistency:** `update_hotkeys_on_move: Signal<bool>` and `hotkey_assign_request: Signal<bool>` are used identically across every prop struct and call site. `MoveRequest::with_assign_hotkey_on_move` / `assign_hotkey_on_move` and `assign_position(..., assign_hotkey: bool)` match between definition (Task 1) and use (Task 4). `CustomKeysPersistence::load_update_hotkeys_on_move` / `save_update_hotkeys_on_move` match between Task 2 and Task 3.
