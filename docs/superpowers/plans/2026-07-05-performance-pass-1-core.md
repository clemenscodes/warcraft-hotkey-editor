# Performance Pass 1 (core: B + A1 + E) — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make edits non-blocking and harden the app against reactivity/panic footguns, without touching the risky render/drag surgery (that is Plan 2).

**Architecture:** Cache the static per-unit command grids in the domain crate (kills the ~1700-rebuild collision scan per edit); collapse the 3× redundant `normalize()` per edit to 1× using the always-normalized invariant; install a wasm panic hook and convert point-in-time `.read()`s to `.peek()`; debounce the derived collision badge.

**Tech Stack:** Rust, Dioxus (wasm), `warcraft-keybinds` (pure domain), `console_error_panic_hook`, `gloo-timers`. Build/test: `cargo test -p warcraft-keybinds`, `cargo test -p hotkey-editor --lib`, `moon run hotkey-editor:rust/lint` (wasm clippy), `cargo fmt --all`.

## Global Constraints

- **R1:** localStorage is written synchronously, same-tick, on every mutation. Do NOT debounce the primary `save_text` write. Only the *derived* collision badge may be debounced.
- **Always-normalized invariant:** `loaded_keys` always holds a normalized `CustomKeys` (every writer produces normalized text: commit/import normalize, `from_text` normalizes, resolve-Apply calls `.normalize()`, template-apply uses `import_overlay`'s normalized output). Task 2 relies on this and guards it with a debug assertion.
- **R8/R9:** `warcraft-keybinds` stays pure Rust (no browser deps) and every domain change ships with native tests.
- **RUST_STYLE:** full semantic names, no tuples, `Self` in impls, derive every qualifying trait, no `verb_noun` free functions, split files over section comments, no `as` casts outside From/TryFrom.
- **Verify commands for `hotkey-editor` (a wasm crate):** compile gate is `moon run hotkey-editor:rust/lint` (NOT `cargo check`); unit tests `cargo test -p hotkey-editor --lib`; always run `cargo fmt --all` before committing.
- **Commit with signing disabled** (GPG hangs non-interactively): `git -c commit.gpgsign=false commit -m "…"`. Stage only source (`git add crates/`), never the untracked `docs/superpowers/`.
- **Out of scope this plan:** Theme A2 (deleting the trailing persist effects), Theme B2 (external-crate `by_id`), the render.rs hot-path panic downgrade (→ Plan 2 with the render rework), Themes C/D.

---

## Task 1: Cache static per-unit command grids (Theme B1)

**Files:**
- Modify: `crates/warcraft-keybinds/src/unit/grids.rs`
- (Read for wiring: `crates/warcraft-keybinds/src/collision/cross_unit.rs`, `crates/warcraft-keybinds/src/collision/unit_report.rs`, and the `NamedCommandGrid` definition to determine `Clone`-ability.)

**Interfaces:**
- Produces: a cached path for `UnitGrids::for_unit(WarcraftObjectId)` that builds each unit's grids at most once for the process lifetime. Public call sites keep working (either `for_unit` keeps returning an owned `UnitGrids` via `Clone`-from-cache, or a new `for_unit` returns `&'static UnitGrids` and callers are updated).

Context: `UnitGrids::for_unit` (`grids.rs:79-93`) rebuilds a unit's command/research/build/uprooted grids from the static `WARCRAFT_DATABASE` on every call, doing dozens of allocating `WARCRAFT_DATABASE.by_id(...)` lookups. `CollisionSummary::compute` calls it for ~850 units, twice, on every edit. The result depends ONLY on `unit_id` + the static DB, so it is safe to memoize process-wide.

- [ ] **Step 1: Write the failing test**

Add to `crates/warcraft-keybinds/src/unit/grids.rs` (in its `#[cfg(test)] mod tests`, or add one):

```rust
#[cfg(test)]
mod cache_tests {
    use super::UnitGrids;
    use warcraft_database::WARCRAFT_DATABASE;

    fn first_unit_id() -> warcraft_api::WarcraftObjectId {
        // any real unit id from the DB; take the first command-card-bearing object
        *WARCRAFT_DATABASE
            .into_iter()
            .map(|(object_id, _object)| object_id)
            .next()
            .expect("database is non-empty")
    }

    #[test]
    fn for_unit_is_stable_across_calls() {
        let unit_id = first_unit_id();
        let first = UnitGrids::for_unit(unit_id);
        let second = UnitGrids::for_unit(unit_id);
        assert_eq!(first.grid_count(), second.grid_count());
        assert_eq!(first.unit_id(), second.unit_id());
    }

    #[test]
    fn cached_grids_match_a_fresh_build() {
        let unit_id = first_unit_id();
        let cached = UnitGrids::for_unit(unit_id);
        let fresh = UnitGrids::build_for_unit(unit_id); // uncached builder (added below)
        assert_eq!(cached.grid_count(), fresh.grid_count());
        assert_eq!(cached.unit_id(), fresh.unit_id());
    }
}
```

(`assert_eq!` on grids requires `UnitGrids`/`NamedCommandGrid` to derive `PartialEq`; if adding that is heavy, compare `grid_count()` + `unit_id()` + each grid's role/slot count instead. Keep the test asserting real equivalence, not a tautology.)

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test -p warcraft-keybinds cache_tests`
Expected: FAIL to compile — `build_for_unit` doesn't exist yet.

- [ ] **Step 3: Implement the cache**

In `crates/warcraft-keybinds/src/unit/grids.rs`:
1. Rename the current `pub fn for_unit` body to a private `fn build_for_unit(unit_id: WarcraftObjectId) -> Self` (the uncached builder — identical body).
2. Add a process-lifetime cache and a cached `for_unit`. Prefer the lowest-ripple form that compiles:
   - **If `UnitGrids` + `NamedCommandGrid` (and their fields) can derive `Clone`:** derive `Clone` on both, keep `for_unit -> Self`, and clone out of the cache:
     ```rust
     use std::collections::HashMap;
     use std::sync::LazyLock;

     static UNIT_GRIDS_CACHE: LazyLock<HashMap<WarcraftObjectId, UnitGrids>> =
         LazyLock::new(|| {
             let mut cache = HashMap::new();
             for entry in WARCRAFT_DATABASE.into_iter() {
                 let object_id = *entry.0;
                 let grids = UnitGrids::build_for_unit(object_id);
                 cache.insert(object_id, grids);
             }
             cache
         });

     pub fn for_unit(unit_id: WarcraftObjectId) -> Self {
         if let Some(cached) = UNIT_GRIDS_CACHE.get(&unit_id) {
             return cached.clone();
         }
         Self::build_for_unit(unit_id)
     }
     ```
     Cloning a `UnitGrids` (a `Vec` of a few `NamedCommandGrid`s) is far cheaper than `build_for_unit` (dozens of allocating DB lookups).
   - **If some inner type cannot derive `Clone`:** instead make `for_unit` return `&'static UnitGrids`:
     ```rust
     pub fn for_unit(unit_id: WarcraftObjectId) -> &'static Self {
         UNIT_GRIDS_CACHE
             .get(&unit_id)
             .unwrap_or_else(|| panic!("unit id not in grids cache: {unit_id:?}"))
     }
     ```
     and update the collision callers (`cross_unit.rs`, `unit_report.rs`) — method calls (`grids.grids()`, etc.) work unchanged on a reference; only `let grids = UnitGrids::for_unit(id);` bindings that then move the value need a `&` / adjust. Read those two files and adjust minimally.

   Pick ONE approach (the `Clone` one if it compiles). The cache is keyed by `WarcraftObjectId` (which is `Copy + Eq + Hash`).

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test -p warcraft-keybinds cache_tests`
Expected: PASS.

- [ ] **Step 5: Full crate tests + no browser deps**

Run: `cargo test -p warcraft-keybinds`
Expected: PASS (all existing collision/grid tests still green — this is a memoization, behavior is identical).
Run: `grep -n 'wasm\|web_sys\|dioxus\|gloo' crates/warcraft-keybinds/src/unit/grids.rs`
Expected: no output.

- [ ] **Step 6: Commit**

```bash
cargo fmt --all
git add crates/warcraft-keybinds/
git -c commit.gpgsign=false commit -m "perf(keybinds): cache static per-unit command grids"
```

---

## Task 2: Collapse 3× normalize per edit to 1× (Theme A1)

**Files:**
- Modify: `crates/hotkey-editor/src/components/app/components/shell/hooks.rs` (the re-normalize effect at 61-68 and the undo-capture effect at 88-97)

**Interfaces:**
- Consumes: the always-normalized invariant (Global Constraints).
- Produces: no API change; per-edit `normalize()` count drops from 3 to 1 (only `CustomKeysService::commit` normalizes).

Context: `commit` already normalizes + persists. Two shell effects then re-`normalize()` the already-normalized aggregate: the persist effect (`hooks.rs:66` `file.normalize().to_string()`) and the undo-capture effect (`hooks.rs:92` `.map(|file| file.normalize().to_string())`). Since the signal always holds normalized content, both `normalize()` calls are redundant — replace with plain `to_string()`.

- [ ] **Step 1: Add a debug-assert guard + drop the redundant normalize in the persist effect**

Replace `crates/hotkey-editor/src/components/app/components/shell/hooks.rs:61-68`:

```rust
    use_effect(move || {
        let read_guard = loaded_keys.read();
        let Some(file) = read_guard.as_ref() else {
            return;
        };
        let canonical_text = file.normalize().to_string();
        CustomKeysPersistence::save_text(&canonical_text);
    });
```

with:

```rust
    use_effect(move || {
        let read_guard = loaded_keys.read();
        let Some(file) = read_guard.as_ref() else {
            return;
        };
        // Invariant: every writer of `loaded_keys` stores a normalized aggregate
        // (commit/import normalize, from_text normalizes, resolve-Apply normalizes,
        // template-apply uses import_overlay's normalized output). So re-normalizing
        // here is redundant work; just serialize. The debug assertion catches any
        // future writer that violates the invariant.
        debug_assert_eq!(
            file.clone().normalize().to_string(),
            file.to_string(),
            "loaded_keys held a non-normalized aggregate; a writer must normalize before set()",
        );
        let canonical_text = file.to_string();
        CustomKeysPersistence::save_text(&canonical_text);
    });
```

- [ ] **Step 2: Drop the redundant normalize in the undo-capture effect**

Replace `crates/hotkey-editor/src/components/app/components/shell/hooks.rs:88-97`'s body:

```rust
        let keys_text = loaded_keys
            .read()
            .as_ref()
            .map(|file| file.normalize().to_string())
            .unwrap_or_default();
```

with:

```rust
        let keys_text = loaded_keys
            .read()
            .as_ref()
            .map(|file| file.to_string())
            .unwrap_or_default();
```

(Leave the rest of that effect — `grid_layout_text`, `EditorSnapshot::new`, `undo_history.record` — unchanged.)

- [ ] **Step 3: Verify compile + tests**

Run: `cargo fmt --all`
Run: `moon run hotkey-editor:rust/lint`  → 0 warnings.
Run: `cargo test -p hotkey-editor --lib`  → pass.

- [ ] **Step 4: Behavior check (reasoning)**

Confirm in the diff: the only change is `normalize().to_string()` → `to_string()` in the two effects, plus the debug assertion. Persistence still writes on every change (R1 preserved). Undo snapshots still capture the same canonical text (the aggregate is already normalized, so the text is identical).

- [ ] **Step 5: Commit**

```bash
git add crates/hotkey-editor/
git -c commit.gpgsign=false commit -m "perf(hotkey-editor): drop redundant normalize() in persist + undo-capture effects"
```

---

## Task 3: Install `console_error_panic_hook` (Theme E)

**Files:**
- Modify: `crates/hotkey-editor/Cargo.toml` (add dependency), `crates/hotkey-editor/src/main.rs`

**Interfaces:**
- Produces: a wasm panic now prints a readable message + stack to the browser console instead of a silent module abort (white freeze).

- [ ] **Step 1: Add the dependency**

In `crates/hotkey-editor/Cargo.toml` `[dependencies]` (after line 36 `base64 = "0.22"`), add:

```toml
console_error_panic_hook = "0.1"
```

- [ ] **Step 2: Install the hook in `main.rs`**

Replace `crates/hotkey-editor/src/main.rs` in full:

```rust
use hotkey_editor::components::app::App;

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}
```

- [ ] **Step 3: Verify compile**

Run: `cargo fmt --all`
Run: `moon run hotkey-editor:rust/lint`  → 0 warnings (the crate builds for wasm with the new dep).
Expected: builds clean. (`console_error_panic_hook` is a tiny, wasm-standard crate.)

- [ ] **Step 4: Commit**

```bash
git add crates/hotkey-editor/
git -c commit.gpgsign=false commit -m "feat(hotkey-editor): install console_error_panic_hook so wasm panics surface"
```

---

## Task 4: `.read()` → `.peek()` hygiene in callbacks + service snapshots (Theme E)

**Files:**
- Modify: `crates/hotkey-editor/src/services/customkeys/service.rs:78-81`
- Modify: `crates/hotkey-editor/src/services/grid_layout/service.rs:50-52`
- Modify: `crates/hotkey-editor/src/components/.../download_info_dialog_host/mod.rs:19`
- Modify: `crates/hotkey-editor/src/components/.../resolve_page/hooks.rs:88` (the `handle_apply` closure read ONLY — NOT the `:72` top-level `has_file`)
- Modify: `crates/hotkey-editor/src/components/.../tile_override/hooks.rs:276,278`
- Modify: `crates/hotkey-editor/src/components/.../grid_editor/logic/handlers.rs:27,104,134`

**Interfaces:** no API change; these reads become non-subscribing (correct for point-in-time reads inside callbacks / trait methods invoked outside reactive scopes).

**IMPORTANT — do NOT touch these (they are top-level render reads that MUST keep their subscription):** `resolve_page/hooks.rs:72`, `collisions_page/hooks.rs:131`, and any `loaded_keys.read()` directly in a component/hook body used to drive rendering. Converting those would drop reactivity. Only convert reads inside `EventHandler::new`/`Callback::new` closures and the two `Service::snapshot` methods.

- [ ] **Step 1: Convert the two service snapshots**

`services/customkeys/service.rs:78-81` — change `let read_guard = self.keys.read();` to `let read_guard = self.keys.peek();`:
```rust
    fn snapshot(&self) -> CustomKeys {
        let read_guard = self.keys.peek();
        read_guard.clone().unwrap_or_default()
    }
```

`services/grid_layout/service.rs:50-52` — change `*self.layout.read()` to `*self.layout.peek()`:
```rust
    fn snapshot(&self) -> GridLayout {
        *self.layout.peek()
    }
```

- [ ] **Step 2: Convert the callback-site reads**

In each of these, change the `.read()` to `.peek()` (the surrounding lines stay the same):
- `download_info_dialog_host/mod.rs:19` — `let read_guard = keys.peek();`
- `resolve_page/hooks.rs:88` — `let read_guard = loaded_keys.peek();` (inside `handle_apply`; leave `:72` alone)
- `tile_override/hooks.rs:276` — `let layout_snapshot_for_check = *grid_layout.peek();`
- `tile_override/hooks.rs:278` — `let read_guard = loaded_keys.peek();`
- `grid_editor/logic/handlers.rs:27` — `let read_guard = loaded_keys.peek();`
- `grid_editor/logic/handlers.rs:104` — `let read_guard = loaded_keys.peek();`
- `grid_editor/logic/handlers.rs:134` — `let read_guard = loaded_keys.peek();`

- [ ] **Step 3: Verify compile + tests**

Run: `cargo fmt --all`
Run: `moon run hotkey-editor:rust/lint`  → 0 warnings.
Run: `cargo test -p hotkey-editor --lib`  → pass.

- [ ] **Step 4: Behavior check**

These are callbacks/trait-methods invoked from event handlers, not reactive scopes, so dropping the subscription changes nothing observable — but it removes latent footguns of the class that caused the boot loop. Confirm the diff touched only `.read()`→`.peek()` and nothing structural.

- [ ] **Step 5: Commit**

```bash
git add crates/hotkey-editor/
git -c commit.gpgsign=false commit -m "refactor(hotkey-editor): peek instead of read for point-in-time callback/snapshot reads"
```

---

## Task 5: Fix the touch-drag long-press closure leak (Theme E)

**Files:**
- Modify: `crates/hotkey-editor/src/components/.../grid_editor/logic/mechanics.rs:121-161`
- Modify: `crates/hotkey-editor/src/components/.../grid_editor/logic/drag_state.rs` (add a thread-local to hold the live closure)

**Interfaces:** no API change; the per-touch long-press `Closure::once` is retained and dropped on cancel/fire instead of leaked via `forget()`.

Context: `mechanics.rs:161` calls `cb.forget()` on the per-`pointerdown` long-press `Closure::once`, leaking it on every touch drag start. `cancel_long_press` (`drag_state.rs:79-85`) clears the timer handle but cannot drop the forgotten closure.

- [ ] **Step 1: Add a thread-local to own the live closure**

In `drag_state.rs`, beside `TOUCH_LONG_PRESS_TIMER_ID` (65-68), add:

```rust
    /// Holds the live long-press callback so it is dropped (not leaked) when the
    /// timer fires or is cancelled. Replaces the previous `Closure::forget()`.
    pub(crate) static TOUCH_LONG_PRESS_CLOSURE: RefCell<Option<Closure<dyn FnMut()>>> =
        const { RefCell::new(None) };
```

(Add `use wasm_bindgen::closure::Closure;` and `use std::cell::RefCell;` to `drag_state.rs` if not already imported.)

Then in `DragThreadState::cancel_long_press` (79-85), after clearing the timer handle, drop the closure:
```rust
        pub(crate) fn cancel_long_press() {
            if let Some(id) = TOUCH_LONG_PRESS_TIMER_ID.with(|cell| cell.replace(None))
                && let Some(window) = web_sys::window()
            {
                window.clear_timeout_with_handle(id);
            }
            TOUCH_LONG_PRESS_CLOSURE.with(|cell| cell.borrow_mut().take());
        }
```

- [ ] **Step 2: Store the closure instead of forgetting it**

In `mechanics.rs`, replace the `cb.forget();` at line 161 with storing the closure in the thread-local, and have the `Closure::once` body clear itself on fire. Since `Closure::once` frees itself after firing, storing it in the thread-local and taking it out is the clean owner. Change the timer-install block so that after `set_timeout` succeeds, the closure is moved into `TOUCH_LONG_PRESS_CLOSURE`:

```rust
        if let Some(window) = web_sys::window()
            && let Ok(timer_id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                LONG_PRESS_MS,
            )
        {
            TOUCH_LONG_PRESS_TIMER_ID.with(|c| c.set(Some(timer_id)));
            TOUCH_LONG_PRESS_CLOSURE.with(|c| *c.borrow_mut() = Some(cb));
        }
```

Remove the `cb.forget();` line entirely. Note `cb` is now a `Closure<dyn FnMut()>` — if it was typed as `Closure::once` producing `Closure<dyn FnOnce()>`, change it to `Closure::<dyn FnMut()>::new(...)` OR keep `once` and store as `Option<Closure<dyn FnOnce()>>` in the thread-local (match the type). Read the exact `Closure::once` type and make the thread-local's type match; the goal is: the closure is owned by the thread-local, dropped in `cancel_long_press` and replaced/cleared on the next `pointerdown`.

- [ ] **Step 3: Verify compile**

Run: `cargo fmt --all`
Run: `moon run hotkey-editor:rust/lint`  → 0 warnings (type of the thread-local matches the closure).

- [ ] **Step 4: Commit**

```bash
git add crates/hotkey-editor/
git -c commit.gpgsign=false commit -m "fix(hotkey-editor): own+drop the touch long-press closure instead of leaking it"
```

---

## Task 6: Debounce the collision badge compute (Theme B3)

**Files:**
- Modify: `crates/hotkey-editor/src/components/.../collisions_button_host/hooks.rs`

**Interfaces:** no prop API change; `CollisionsButtonProps { summary, onclick }` still produced. `summary` now updates ~150 ms after edits settle instead of synchronously per edit.

Context: `use_collisions_button` (`hooks.rs:13-34`) recomputes `CollisionSummary::compute` in a `use_memo` that re-fires on every `keys`/`grid_layout` change — a full scan on the paint-blocking path of every edit. The badge is a derived display (not source-of-truth), so debouncing it is R1-safe. With Task 1's cache the scan is much cheaper, but debouncing takes it off the edit's critical path entirely.

- [ ] **Step 1: Replace the eager memo with a debounced signal**

Rewrite `use_collisions_button` to hold the summary in a `use_signal`, and recompute it in a generation-guarded 150 ms debounce driven by a `use_effect` that subscribes to `keys`/`grid_layout` (mirror the search debounce in `unit_list/hooks.rs:109-123`). Concretely:

```rust
pub(super) fn use_collisions_button() -> CollisionsButtonProps {
    let custom_keys_service = use_custom_keys_service();
    let keys = custom_keys_service.keys();
    let grid_layout = use_grid_layout();

    let mut summary = use_signal(CollisionSummary::default);
    let mut debounce_generation = use_signal(|| 0_u32);

    use_effect(move || {
        // Subscribe to the inputs so this effect re-runs on each edit...
        let _keys_subscribe = keys.read();
        let _layout_subscribe = grid_layout.read();
        // ...but do the expensive scan only after a 150 ms quiet period,
        // guarded by a generation counter so superseded runs no-op.
        let next_generation = debounce_generation.peek().wrapping_add(1);
        debounce_generation.set(next_generation);
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(150).await;
            if *debounce_generation.peek() != next_generation {
                return;
            }
            let read_guard = keys.peek();
            let computed = match read_guard.as_ref() {
                Some(file) => {
                    let layout = *grid_layout.peek();
                    CollisionSummary::compute(file, layout)
                }
                None => CollisionSummary::default(),
            };
            drop(read_guard);
            summary.set(computed);
        });
    });

    let navigation = use_view_navigation();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let target = AppView::Collisions {
            kind: CollisionKind::Positions,
        };
        navigation.apply(target);
    });

    // Subscribing read: the button must re-render when the debounced summary lands.
    let summary_value = *summary.read();
    CollisionsButtonProps { summary: summary_value, onclick }
}
```

Adjust imports (`use dioxus::prelude::*;` already present; ensure `spawn`, `gloo_timers`, `CollisionSummary` are in scope — they are used elsewhere in the crate). Fix the `CollisionSummary::default(); return;` block to just `return;` after setting default if that reads cleaner — the point is: no file → leave summary at default. Match the None handling to current behavior (current returns `CollisionSummary::default()`).

Note `CollisionSummary` is `Copy` (`summary.rs:11` derives `Copy`), so `summary.peek().clone()` / `*summary.peek()` both work — use `*summary.peek()`.

- [ ] **Step 2: Verify compile + tests**

Run: `cargo fmt --all`
Run: `moon run hotkey-editor:rust/lint`  → 0 warnings.
Run: `cargo test -p hotkey-editor --lib`  → pass.

- [ ] **Step 3: Note the e2e coupling**

The `view-switcher.spec.ts` tests assert the badge count after edits (e.g. "Collisions count tracks position, command-card hotkey, and system-hotkey collisions", "Collisions button reaches the clean state after Resolve + Apply Grid"). They use Playwright `expect(...).toHaveText`/`waitFor`, which retry until the debounced value settles, so a 150 ms debounce should be compatible. This is verified in Task 7's e2e run — if any of those specs flake on timing, increase their `expect` timeout rather than removing the debounce.

- [ ] **Step 4: Commit**

```bash
git add crates/hotkey-editor/
git -c commit.gpgsign=false commit -m "perf(hotkey-editor): debounce the derived collision badge compute"
```

---

## Task 7: Verification — full CI + browser + measurement

**Files:** none (verification only).

- [ ] **Step 1: Full workspace tests + lint + fmt**

Run: `cargo test -p warcraft-keybinds`  → pass.
Run: `cargo test -p hotkey-editor --lib`  → pass.
Run: `moon run hotkey-editor:rust/lint`  → 0 warnings.
Run: `cargo fmt --all -- --check`  → exit 0.

- [ ] **Step 2: Full e2e gate (monitored)**

Run: `moon run hotkey-editor:playwright/test` (with the Bash sandbox disabled; run it MONITORED — do not treat a stale exit code as success; confirm the real "N passed" summary line and a 0 exit). Expected: the full suite passes, including the `view-switcher` badge-count specs (which now settle after the 150 ms debounce) and `undo.spec.ts` (undo survives reload — unaffected).
Do NOT start any competing server on port 8123/8200.

- [ ] **Step 3: Browser sanity via Playwright MCP**

Serve the built release bundle on a NON-dev scratch port (e.g. 8199) with `node crates/hotkey-editor/e2e/server.mjs <staticDir> 8199 /warcraft-hotkey-editor`, navigate Playwright MCP to `http://127.0.0.1:8199/warcraft-hotkey-editor/`, confirm: units render (`.unit-card` count > 0), no console errors beyond favicon 404, make an edit (assign a hotkey) and confirm the collision badge updates within a moment. Kill the scratch server immediately after.

- [ ] **Step 4: Commit (if any verification-only notes/fixtures added)**

Nothing to commit if all green; otherwise commit any test-timeout adjustments made in Step 2.

---

## Self-review notes

- **Spec coverage:** B1 (Task 1), B2 dropped (external crate — documented), B3 (Task 6), A1 (Task 2), A2 deferred (documented), E panic hook (Task 3), E peek hygiene (Task 4), E closure leak (Task 5); render.rs panic downgrade deferred to Plan 2 (documented). C/D are Plan 2.
- **R1 preserved:** only the derived badge is debounced; the primary `save_text` stays synchronous (Task 2 keeps the persist effect writing every change).
- **Always-normalized invariant** is guarded by a `debug_assert_eq!` (Task 2) so a future violating writer fails tests rather than silently mis-persisting.
- **Risk:** all tasks are domain-crate or contained frontend changes with no render-tree restructuring; the only behavior-timing change is the badge debounce (Task 6), covered by the e2e in Task 7.
