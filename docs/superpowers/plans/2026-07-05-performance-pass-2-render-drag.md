# Performance Pass 2 (C + D: render granularity + drag latency) — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make dragging smooth and stop one edit re-rendering all four command grids, without regressing the editor (this is the invasive render/drag surgery — every task is browser-verified).

**Architecture:** rAF-coalesce the pointer-move hit-test/follower work (at most one per frame); move each grid's `Vec<RenderedTile>` compute into a per-grid `use_memo` so Dioxus's `Memo<PartialEq>` gates re-render to only the grid whose tiles changed; memoize the header-button booleans and the unit-list catalog query.

**Tech Stack:** Rust, Dioxus 0.7.9 (`use_memo`), `web-sys` `Window::request_animation_frame` (already available — `"Window"` feature enabled), `js-sys`/`wasm-bindgen` closures.

## Global Constraints

- **⚠️ SHARED WORKING TREE:** other agents (styling only — `*/style.rs`) and the user edit this repo concurrently. This plan touches ONLY logic files (`render.rs`, `mechanics.rs`, `mod.rs`, `*hooks.rs`, `state.rs`, `logic.rs`, `drag_state.rs`) — no overlap. Discipline: commit ONLY your own files via explicit pathspec (`git -c commit.gpgsign=false commit -m "…" -- <file>…`), never `git add -A`/`git add crates/`; format ONLY your files (`rustfmt <file>`, never `cargo fmt --all`); if a build fails on a FOREIGN file, report it distinctly, don't fix foreign code.
- **Behavior must be preserved exactly:** drag/drop, selection, hotkey assignment, conflict highlighting, drag-follower overlay all unchanged. This is optimization only.
- **No new deps** (rAF is available via the enabled `"Window"` feature).
- **Verify commands (wasm crate):** compile gate `moon run hotkey-editor:rust/lint`; unit tests `cargo test -p hotkey-editor --lib`. Commit with GPG disabled.
- **MANDATORY per risky task:** a live Playwright-MCP check (see the Verification protocol at the end) — boot renders, drag works, drag-hover highlights the correct tile, undo/redo across reload. The reactive-loop class already caused a boot-freeze once; unit tests do NOT catch it — only a real browser boot does.
- **RUST_STYLE** throughout; **COMPONENTS.md** (memoization lives in hooks, bodies stay pure RSX).
- **Deferred to a follow-up (NOT this plan):** the per-tile drag-hover restructure (making each tile subscribe to `drop_target_tile` so a hover re-renders only 1–2 tiles instead of the grid). The rAF coalescing (Task 1) caps drag cost to per-frame, which is the bulk of the win; the per-tile hover is a smaller, deeper follow-up.

---

## Task 1: rAF-coalesce the grid drag `pointer_move` (Theme D)

**Files:**
- Modify: `crates/hotkey-editor/src/components/.../grid_editor/logic/drag_state.rs` (add rAF thread-locals + a scheduler)
- Modify: `crates/hotkey-editor/src/components/.../grid_editor/logic/mechanics.rs` (`pointer_move` tail + cancel paths)

**Interfaces:** no API change. `pointer_move`'s handler now stores the latest cursor position and runs the follower-update + `element_from_point` hit-test + `drop_target_tile.set` tail at most once per animation frame.

Context: `pointer_move` (`mechanics.rs:167-342`) runs, on EVERY pointermove: `drag_follower.set(...)` (`:281`, unconditional), `document.element_from_point(...)` (`:291`, forces sync layout), two `.closest()` walks (`:293`,`:300`), and a coordinate-guarded `drop_target_tile.set(...)` (`:333-339`). At 60–120 events/sec this drops frames.

- [ ] **Step 1: Add rAF state + scheduler to `drag_state.rs`**

Read `drag_state.rs` for the exact thread-local block and imports. Add beside the existing thread-locals:

```rust
    /// Latest pointer coords awaiting an animation-frame flush (client x, y).
    pub(crate) static LATEST_DRAG_MOVE: Cell<Option<DragMovePoint>> = const { Cell::new(None) };
    /// Handle of the pending requestAnimationFrame, so it can be cancelled.
    pub(crate) static DRAG_RAF_HANDLE: Cell<Option<i32>> = const { Cell::new(None) };
    /// The rAF callback closure, kept alive while a frame is pending.
    pub(crate) static DRAG_RAF_CLOSURE: RefCell<Option<Closure<dyn FnMut(f64)>>> =
        const { RefCell::new(None) };
```

Use a NAMED struct (no tuples — RUST_STYLE): add near the top of `drag_state.rs`:
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DragMovePoint {
    pub(crate) client_horizontal: f64,
    pub(crate) client_vertical: f64,
}
```

Add a cancel to `DragThreadState` (and call it from `reset`):
```rust
    pub(crate) fn cancel_drag_raf() {
        if let Some(handle) = DRAG_RAF_HANDLE.with(|cell| cell.replace(None))
            && let Some(window) = web_sys::window()
        {
            let _ = window.cancel_animation_frame(handle);
        }
        LATEST_DRAG_MOVE.with(|cell| cell.set(None));
        DRAG_RAF_CLOSURE.with(|cell| cell.borrow_mut().take());
    }
```
Call `Self::cancel_drag_raf();` inside `DragThreadState::reset()` (so pointer_up/cancel/lost-capture, which route through reset, cancel any pending frame). Add `use wasm_bindgen::closure::Closure;` if missing.

- [ ] **Step 2: Split `pointer_move` into an event handler + an rAF-flushed tail**

In `mechanics.rs`, refactor the `pointer_move` handler so:
- The synchronous part that MUST stay per-event (the pending-drag threshold promotion / touch long-press / cancel logic, `mechanics.rs:184-276`) runs immediately as today.
- The follower-update + hit-test tail (`mechanics.rs:277-341`) is extracted into a function `flush_drag_move(dragging_slot, drop_target_tile, drag_follower, grid_id, point: DragMovePoint)` that takes the latest coords instead of reading them from the event.
- The handler, instead of running the tail inline, stores `LATEST_DRAG_MOVE = Some(point)` and, if `DRAG_RAF_HANDLE` is `None`, schedules a frame: build a `Closure::<dyn FnMut(f64)>::new(move |_timestamp| { let Some(point) = LATEST_DRAG_MOVE.take() else { return }; DRAG_RAF_HANDLE.set(None); flush_drag_move(...captured signals..., grid_id, point); })`, call `window.request_animation_frame(closure.as_ref().unchecked_ref())`, store the returned handle in `DRAG_RAF_HANDLE`, and move the closure into `DRAG_RAF_CLOSURE` (kept alive; dropped/replaced next schedule or on `cancel_drag_raf`). If a frame is already pending, just update `LATEST_DRAG_MOVE` (coalesce — the pending frame will read the newest point).

The signals captured by the closure (`dragging_slot`, `drop_target_tile`, `drag_follower`) are `Copy` Dioxus signal handles, so they move into the closure cleanly.

- [ ] **Step 3: Add a sub-pixel guard on the follower write**

Inside `flush_drag_move`, keep the existing coordinate-compared `drop_target_tile.set` guard, and add: only `drag_follower.set(...)` if the new cursor position differs from the current follower's stored position by ≥ ~0.5px (read the current follower via `.peek()`, compare, skip if unchanged) — avoids a redundant overlay re-render when the frame coalesced but the cursor barely moved. Use `.peek()` (non-subscribing) for the compare.

- [ ] **Step 4: Verify compile**

Run: `rustfmt` on both files.
Run: `moon run hotkey-editor:rust/lint` → 0 warnings (closure/thread-local types line up).
Run: `cargo test -p hotkey-editor --lib` → pass.

- [ ] **Step 5: BROWSER verification (mandatory)**

Follow the Verification protocol at the end. Specifically: start a drag on a command-grid tile, move across several tiles, confirm: the drag-follower tracks the cursor smoothly, the drop-target highlight follows, dropping on a new tile performs the move (and swap where applicable), and Escape/pointer-up/leaving the grid cancels cleanly with no stuck follower. Confirm no console errors and no stuck rAF (drag then stop — CPU should idle).

- [ ] **Step 6: Commit**

```bash
git -c commit.gpgsign=false commit -m "perf(hotkey-editor): rAF-coalesce grid drag pointer_move + follower sub-pixel guard" -- <drag_state.rs> <mechanics.rs>
```

---

## Task 2: rAF-coalesce the inventory drag `pointer_move` (Theme D)

**Files:**
- Modify: `crates/hotkey-editor/src/components/.../inventory_hotkeys_view/components/inventory_grid/components/inventory_cell/hooks.rs`
- (Possibly) Modify: its sibling drag-state module (the inventory drag's own thread-locals, imported at `hooks.rs:1-4`).

**Interfaces:** no API change; same rAF-coalescing applied to the inventory `on_pointermove` (`hooks.rs:165-227`), which duplicates the grid drag pattern (`element_from_point` + `.closest(".inventory-cell")` + unconditional `drag_follower.set` per move).

- [ ] **Step 1: Apply the same rAF pattern**

The inventory drag uses SEPARATE thread-locals from the grid drag (different module root). Either (a) add an rAF handle/latest-point/closure trio to the inventory drag's own state module and coalesce `on_pointermove`'s tail exactly as Task 1, or (b) if the two drag systems can share a small rAF helper without entangling their state, extract `flush`-scheduling into a shared util — but do NOT force the extraction; option (a) is fine and lower-risk. Read `inventory_cell/hooks.rs` and its state module to decide.

- [ ] **Step 2: Verify compile + browser**

Run `rustfmt` on changed files, `moon run hotkey-editor:rust/lint`, `cargo test -p hotkey-editor --lib`.
BROWSER (Verification protocol): open the System Hotkeys dialog → inventory view, drag an inventory key across cells, confirm the follower + highlight track smoothly and the drop assigns correctly; cancel paths clean.

- [ ] **Step 3: Commit**

```bash
git -c commit.gpgsign=false commit -m "perf(hotkey-editor): rAF-coalesce inventory drag pointer_move" -- <files>
```

---

## Task 3: Per-grid `use_memo` for rendered tiles (Theme C — the core restructure)

**Files:**
- Modify: `crates/hotkey-editor/src/components/.../grid_editor/mod.rs` (`GridEditor` body — add the memo)
- Modify: `crates/hotkey-editor/src/components/.../grid_editor/logic/render.rs` (accept pre-computed tiles instead of reading `loaded_keys` in the `From`)

**Interfaces:**
- Produces: `GridEditor` re-renders (and re-runs its DOM diff) only when ITS grid's `Vec<RenderedTile>` changes on an edit — not when a sibling grid's slots change.

Context (from investigation): the `Vec<RenderedTile>` compute lives INSIDE `From<&GridEditorProps<B>>` (`render.rs:87-105`), which runs in `GridEditor`'s reactive scope — so all four `GridEditor`s subscribe to the shared `loaded_keys` and one edit re-renders all four. `RenderedTile` is `Clone + PartialEq` and the Vec is exactly 12 entries, so a `Memo<Vec<RenderedTile>>` PartialEq-gates propagation: all four memos recompute (cheap), only the changed grid notifies.

- [ ] **Step 1: Compute rendered tiles in a `use_memo` in the `GridEditor` body**

In `grid_editor/mod.rs`, before the `rsx!`, add a `use_memo` that computes this grid's `Vec<RenderedTile>` by moving the block at `render.rs:88-105` here. The closure reads `loaded_keys`, `tier_overrides`, `grid_layout`, `selected_slot`, `selected_from_research` (so the MEMO subscribes, not the component), and captures the per-grid constants `slot_ids`/`restrict_draggable_to`/`behavior` by value:

```rust
#[component]
pub(crate) fn GridEditor<B: GridBehavior>(props: GridEditorProps<B>) -> Element {
    let config = props.config.clone();
    let behavior = props.behavior.clone();
    let loaded_keys = config.loaded_keys;
    let tier_overrides = config.tier_overrides;
    let grid_layout = config.grid_layout;
    let selected_slot = config.selected_slot;
    let selected_from_research = config.selected_from_research;
    let slot_ids = config.slot_ids.clone();
    let restrict_draggable_to: Rc<[GridSlotId]> = Rc::from(config.restrict_draggable_to.as_slice());
    let behavior_for_memo = behavior.clone();
    let rendered_tiles = use_memo(move || {
        let read_guard = loaded_keys.read();
        let Some(file) = read_guard.as_ref() else {
            return Vec::<RenderedTile>::new();
        };
        let tier_guard = tier_overrides.read();
        let layout_snapshot = *grid_layout.read();
        let selected_snapshot = *selected_slot.read();
        let selected_research_snapshot = *selected_from_research.read();
        let input = CommandGridRenderInput {
            slots: &slot_ids,
            layout: layout_snapshot,
            selected: selected_snapshot,
            selected_is_research: selected_research_snapshot,
            tier_overrides: &tier_guard,
            restrict_draggable_to: &restrict_draggable_to,
        };
        file.rendered_command_grid(&behavior_for_memo, &input)
    });
    rsx! {
        div { class: CLASS, "data-grid-id": props.config.heading,
            HeadedGrid { ..HeadedGridProps::<EditorTileKind>::from_parts(&props, rendered_tiles.read().clone()) }
            DragFollowerOverlay { ..DragFollowerOverlayProps::from(&props) }
        }
    }
}
```

Note: the `.expect("loaded_keys is always Some after boot")` at `render.rs:91` is replaced here by a graceful `else { return Vec::new(); }` (this also discharges the deferred Theme-E hot-path panic downgrade for this site — no panic, empty grid instead of a freeze, and the panic hook from Plan 1 surfaces any real issue).

- [ ] **Step 2: Change the `From` into a `from_parts` that takes the tiles**

In `render.rs`, replace the `impl From<&GridEditorProps<B>>` with an associated constructor `HeadedGridProps::<EditorTileKind>::from_parts(props: &GridEditorProps<B>, rendered_tiles: Vec<RenderedTile>)` that does everything the current `From` does EXCEPT the `render.rs:87-105` block (which now comes in as the `rendered_tiles` argument). The tile-assembly loop (`render.rs:114-184`), the drag-signal reads (`dragging_slot`/`drop_target_tile` at `:106-107` — keep these; they are drag-time, Theme D), the handlers, and the `try_into` stay. Also downgrade the `try_into().unwrap_or_else(panic!)` at `render.rs:185-192`: if the tile count is wrong (e.g. an empty memo during a transient), fall back to rendering whatever tiles exist padded/truncated to `COMMAND_GRID_TILE_COUNT` with default tiles, or return an empty grid — do NOT `panic!` (graceful fallback; the panic hook surfaces the real cause). Read `GridEditorTileProps`/`GridProps` to construct a safe default tile array.

- [ ] **Step 3: Verify compile + tests**

Run `rustfmt` on both files, `moon run hotkey-editor:rust/lint` (0 warnings), `cargo test -p hotkey-editor --lib` (pass).

- [ ] **Step 4: BROWSER verification (MANDATORY — highest-risk task)**

Follow the Verification protocol. Confirm ALL of:
1. Boot renders (33+ unit cards, grids visible) — no white freeze / loop.
2. Select a unit, edit a hotkey in one grid → that grid updates; the OTHER grids visually unchanged and still correct.
3. Selecting a slot highlights it; the selection is exclusive across grids.
4. Drag/drop still works (Task 1 behavior intact).
5. Undo/redo (Ctrl+Z / Ctrl+Shift+Z) updates the grids correctly, and survives a reload.
6. No console errors beyond favicon.
If ANY of these fail, STOP and diagnose before committing — this is the render-restructure task.

- [ ] **Step 5: Commit**

```bash
git -c commit.gpgsign=false commit -m "perf(hotkey-editor): memoize per-grid rendered tiles so one edit re-renders only its grid" -- <mod.rs> <render.rs>
```

---

## Task 4: Memoize the header button booleans (Theme C)

**Files:**
- Modify: `crates/hotkey-editor/src/components/.../inline_actions/components/export_button/hooks.rs`
- Modify: `crates/hotkey-editor/src/components/.../inline_actions/components/resolve_button/hooks.rs`

**Interfaces:** no API change; the export/resolve buttons subscribe to a `bool` memo instead of the whole `CustomKeys`, so they stop re-rendering on every edit.

- [ ] **Step 1: Wrap the boolean in a `use_memo`**

In `export_button/hooks.rs`, replace `let visible = keys.read().is_some();` with:
```rust
    let visible_memo = use_memo(move || keys.read().is_some());
    let visible = visible_memo();
```
In `resolve_button/hooks.rs`, replace `let disabled = keys.read().is_none();` with:
```rust
    let disabled_memo = use_memo(move || keys.read().is_none());
    let disabled = disabled_memo();
```
(The memo's closure subscribes to `keys`, but the `Memo<bool>` only notifies when the bool flips — which is essentially never after boot — so the buttons stop re-rendering per edit.)

- [ ] **Step 2: Verify + browser**

`rustfmt` both, `moon run hotkey-editor:rust/lint`, `cargo test -p hotkey-editor --lib`. BROWSER: confirm the download button and resolve button still appear/enable correctly (load a file → both active). Quick check within the Task 3/5 browser passes is fine.

- [ ] **Step 3: Commit**

```bash
git -c commit.gpgsign=false commit -m "perf(hotkey-editor): memoize export/resolve button visibility booleans" -- <export_button/hooks.rs> <resolve_button/hooks.rs>
```

---

## Task 5: Memoize the unit-list catalog query (Theme C)

**Files:**
- Modify: `crates/hotkey-editor/src/components/.../unit_list/hooks.rs` and/or `unit_list/state.rs`
- Modify: `crates/hotkey-editor/src/components/.../unit_category_section/logic.rs`

**Interfaces:** no API change; `UnitListing::resolve` / `UnitCategoryListing::resolve` run only when `(race, mode, query, search_field, visibility)` change, not on every render (e.g. unit selection).

Context: `UnitListState::new` (`state.rs:40-44`) calls `UnitListing::resolve` unconditionally per render, and each `UnitCategorySection` calls `UnitCategoryListing::resolve` again (`logic.rs:66`). None depend on `loaded_keys`, yet they re-run when unrelated state (selection) changes.

- [ ] **Step 1: Memoize the listing resolve**

In `unit_list/hooks.rs` (or `state.rs`), wrap the `UnitListing::resolve` call in a `use_memo` keyed implicitly on its inputs — read `active_race`, `unit_mode`, `search_query`, and pass `search_field`/`visibility`; the memo recomputes only when those change. `UnitListing`/its `category_kinds()`/`first_result()` outputs must be cloneable into the memo value (read the types; if the whole `UnitListing` isn't `Clone`, memoize the derived `Vec<UnitCategoryKind>` + first-result that the state actually consumes). Do the analogous `use_memo` for `UnitCategoryListing::resolve` in `unit_category_section/logic.rs` keyed on `(race, mode, category_kind, query, search_field, visibility)`.

Note: `logic.rs`'s `unit_cards()` is called from a plain `From<&UnitCategorySectionProps>` that runs every render — moving the resolve into a `use_memo` requires the resolve to happen in a hook (COMPONENTS.md: memoization in hooks). Read how `UnitCategorySection` is structured (does it have a `hooks.rs`?) and place the memo in the hook layer, passing the resolved entries down as data. If the component has no hook seam, the minimal safe change is to memoize inside the existing hook that builds the section props. Keep behavior identical (same cards, same order).

- [ ] **Step 2: Verify + browser**

`rustfmt`, `moon run hotkey-editor:rust/lint`, `cargo test -p hotkey-editor --lib`. BROWSER: type in the unit search (results filter correctly + debounced), switch race/mode tabs (list updates), toggle catalog visibility, select a unit (list does NOT flicker/rebuild — selection is instant). Confirm the same units appear as before.

- [ ] **Step 3: Commit**

```bash
git -c commit.gpgsign=false commit -m "perf(hotkey-editor): memoize unit-list catalog resolves on their real inputs" -- <files>
```

---

## Task 6: Final verification + drag e2e coverage

**Files:**
- Create/Modify: an e2e spec under `crates/hotkey-editor/e2e/tests/` exercising command-grid drag-and-drop + hover highlight (there is currently no drag e2e, which is why an earlier render regression only showed as a global-setup hang).

- [ ] **Step 1: Add a drag e2e test**

Read an existing spec (e.g. `e2e/tests/undo.spec.ts`, `view-switcher.spec.ts`) for the harness/selectors (`.filled-tile`, `data-grid-id`, `data-grid-row`/`data-grid-col` per memory). Add a test that: loads the editor, selects a unit with a draggable ability, performs a drag from one tile to an empty/other tile (Playwright `dragTo` or manual pointer events), and asserts the ability moved (the target tile now shows it / export reflects it). Keep it in the style of the existing specs.

- [ ] **Step 2: Full workspace verification**

`cargo test -p warcraft-keybinds` (pass), `cargo test -p hotkey-editor --lib` (pass), `moon run hotkey-editor:rust/lint` (0 warnings), `rustfmt --check` on all files this plan touched.

- [ ] **Step 3: Full e2e gate (monitored)**

Run `moon run hotkey-editor:playwright/test` (Bash sandbox disabled, MONITORED — confirm the real "N passed" summary + 0 exit, do NOT trust a stale exit code). Includes the new drag spec. Do NOT squat port 8123/8200 if a dev server is running — coordinate.

- [ ] **Step 4: Commit the e2e spec**

```bash
git -c commit.gpgsign=false commit -m "test(e2e): cover command-grid drag-and-drop" -- <spec file>
```

---

## Verification protocol (for the mandatory browser checks)

Because the shared tree churns, verify against a controlled build, not the user's dev server, unless coordinated:
1. Build the release bundle if needed (`moon run hotkey-editor:dx/build`) OR use the user's running dx (read-only) if they OK it.
2. If serving yourself: `node crates/hotkey-editor/e2e/server.mjs <target/dx/hotkey-editor/release/web/public> 8199 /warcraft-hotkey-editor` on a NON-dev scratch port (8199), navigate Playwright MCP to `http://127.0.0.1:8199/warcraft-hotkey-editor/`, do the checks, then KILL the scratch server immediately.
3. Never bind 8123 (dx) or 8200 (gallery).
4. Check: `.unit-card` count > 0 (boot ok), console errors only favicon 404, plus the task-specific interactions.

## Self-review notes

- **Spec coverage:** D rAF-coalesce grid (Task 1) + inventory (Task 2); C per-grid memo (Task 3), header buttons (Task 4), unit list (Task 5); drag e2e + final gate (Task 6). Deferred (documented): per-tile drag-hover restructure; the render.rs hot-path panic downgrades are folded into Task 3 (graceful fallbacks replace the two `panic!`/`expect` there).
- **Risk order:** Task 1 (self-contained drag) and Task 3 (render restructure) are the risky ones — both end with a mandatory live browser pass; Task 3 additionally checks the exact reactive-loop failure mode (boot renders).
- **Shared-tree safe:** all files are logic (not `*/style.rs`); pathspec commits only.
