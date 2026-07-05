# Performance pass — design

Date: 2026-07-05
Status: approved (design), pending implementation plan

## Goal

Make the editor feel snappy and non-blocking, and harden it against the class
of reactivity/performance footguns that already produced one boot-freeze. This
is a targeted pass, not a rewrite — the audits confirmed most async/debounce/
memoization discipline is already correct.

Scope (user-approved): themes **A, B, E, C, D** below. The Web Worker offload
(theme F) and further architectural work are explicitly out of scope for this
pass.

## The problem, in numbers (from the audit)

- Live document is ~257 KB / ~1031 sections; 1742 game objects behind it.
- **Every edit** currently triggers: 3× full `normalize()` + 3× serialize of the
  whole document; a full ~850-unit collision scan (always-mounted badge) with
  ~1700 static-but-recomputed `UnitGrids::for_unit` rebuilds and thousands of
  allocating `by_id` lowercase lookups; 2 synchronous 257 KB localStorage writes
  (one redundant).
- **Dragging** re-renders the whole command grid (re-runs domain
  `rendered_command_grid` + ~96 fresh closures) on every tile crossing;
  `pointer_move` does a synchronous `element_from_point` hit-test on every
  pointer event with no rAF throttle.
- **Safety:** no `console_error_panic_hook`; several `.expect()`/`panic!` on the
  render/drag hot path — a wasm panic aborts → silent white freeze.

Already-correct (verified, leave alone): undo debounce, search debounce,
session-query timer, collisions/resolve `use_memo`s, resolve-apply yielding,
layout-editor HTML5 drag, `handle_keyboard_request` peek discipline.

## Hard constraints

- **R1 (localStorage is source of truth, written synchronously same-tick):** the
  primary `save_text` write stays synchronous — it is NOT debounced. Only the
  *derived* collision badge (not source-of-truth) may be debounced.
- **The wall / R8:** domain-crate changes (theme B) stay pure Rust, no browser
  deps, and ship with native tests (R9).
- **R2 / always-normalized invariant:** `loaded_keys` always holds a normalized
  aggregate; theme A relies on this to drop redundant `normalize()` calls.
- **COMPONENTS.md:** hooks return data, bodies stay pure RSX; memoization goes in
  hooks, not component bodies.

## Sequencing principle

Domain-crate + clearly-safe changes first (native-testable, low risk);
rendering/drag surgery (C, D) LAST, each increment browser-verified. Never
bundle a risky render change with anything else. Order: **B(domain) → A → E →
C → D**.

## Theme A — Kill redundant per-edit work (and fix a latent R1 violation)

Two shell `use_effect`s re-do work `CustomKeysService::commit` already did:
- `shell/hooks.rs:61-68` — re-normalize + serialize + `save_text` on every
  `loaded_keys` change (redundant with commit's save; also the R1-violating
  trailing write flagged in ARCHITECTURE.md §5).
- `shell/hooks.rs:69-72` — grid persist effect (kept in the DDD pass as the
  persistence path for direct grid sets).
- `shell/hooks.rs:88-97` — undo-capture effect: `file.normalize().to_string()`
  again to build the `EditorSnapshot`.

The expensive part of each pass is `normalize()` (two full-aggregate clones + ~5
O(n) passes); the serialize + `setItem` of 257 KB is ~1–2 ms. So the big,
low-risk win is eliminating the redundant *normalize* calls; deleting the effects
outright is a second, higher-care step because the effects are also the
persistence path for *direct* `loaded_keys.set(...)` writers.

**Design — two levels:**

*A1 (safe core, primary win):* rely on the always-normalized invariant to drop
the redundant `normalize()` calls. The two effects keep persisting but only
**serialize** the already-normalized aggregate (no `normalize()`): rewrite
`shell/hooks.rs:61` and the undo-capture `:88` to `file.to_string()` instead of
`file.normalize().to_string()`. This takes 3× normalize → **1×** (commit's) with
no change to which code persists — lowest risk. Guard with a debug assertion that
the signal's content is already normalized (`file.clone().normalize() == file`)
so a future denormalized writer is caught in tests, not silently mis-persisted.

*A2 (fuller, R1-correctness — do only after enumerating all writers):* make every
direct writer of `loaded_keys`/`grid_layout` persist synchronously, then delete
the two trailing persist effects (removing the R1 trailing-write wart). This
requires the plan to first enumerate EVERY `loaded_keys.set(...)` /
`grid_layout.set(...)` site — known ones: `UndoHistory::apply`
(`undo/mod.rs:124-128`), the resolve-page Apply
(`resolve_page/hooks.rs:102`), boot (already persists), `import_overlay`
(already write-throughs) — and give each a synchronous persist (a service
`restore`/`replace_and_persist` path, or a direct `CustomKeysPersistence::save_text`
+ `GridLayoutPersistence::save_grid_layout`). If enumeration is not confidently
complete, A2 is deferred and only A1 ships.

Result: A1 → **1 normalize** per commit (was 3); serialize/write counts unchanged
(commit save, trailing-effect save, undo snapshot). A2 → additionally removes the
trailing-effect serialize + write and completes synchronous write-through.

Verification: native reasoning + browser — edit persists across reload; undo/redo
persists across reload (the exact behavior the effects guaranteed); resolve-Apply
persists across reload. Watch the undo/grid interaction: the capture effect must
still no-op when the snapshot equals present (preserved), so no re-entrancy.

## Theme B — Tame the collision badge (dominant per-edit cost)

The always-mounted collisions badge (`collisions_button_host/hooks.rs:17`)
recomputes `CollisionSummary::compute` on every edit: two whole-DB scans over
~850 units, each calling the static-but-uncached `UnitGrids::for_unit`, whose
inner `by_id` lookups each allocate a lowercased `String`.

**Design, two parts (B2 dropped — see below):**
1. **Cache `UnitGrids::for_unit` per unit id** (domain crate,
   `warcraft-keybinds/src/unit/grids.rs`). It depends only on the static
   `WARCRAFT_DATABASE`, so memoize in a process-lifetime cache
   (`LazyLock<HashMap<WarcraftObjectId, UnitGrids>>`) exposed via a cached
   accessor; switch the collision hot-path callers
   (`collision/cross_unit.rs`, `collision/unit_report.rs`) to it. `UnitGrids`
   currently derives no `Clone` — the plan either adds `Clone` (and to
   `NamedCommandGrid`) to clone out of the cache, or returns a `&'static`
   reference from the cache. Native test: same unit id returns equal grids; the
   expensive build runs once. Eliminates the ~1700 per-edit rebuilds — **and,
   because the allocating `by_id` lookups happen inside `for_unit`, this also
   removes essentially all of the per-edit `by_id` allocations** (they now run
   once at cache-fill).
2. **[DROPPED] Non-allocating `by_id`.** `WarcraftDatabase::by_id`
   (`to_ascii_lowercase()` per call) lives in the **external git-pinned
   `warcraft-data` crate** (not this workspace), so it cannot be edited here;
   changing it needs an upstream change + new tag. And B1's caching already moves
   those lookups off the per-edit path. Out of scope for this pass.
3. **Debounce the badge compute** (frontend). The badge is a derived display, not
   source-of-truth. Recompute ~150 ms after edits settle (generation-guarded,
   mirroring the search debounce), so the scan is off the per-edit critical path.
   The count still reflects the settled state. Caveat: the `view-switcher` e2e
   asserts the badge count after edits — it uses `waitFor`, so a debounce is
   compatible, but confirm those tests still pass (extend their wait if needed).

(1) and (2) make even a non-debounced compute far cheaper; (3) removes it from
the edit's paint-blocking path entirely. All three are independent.

## Theme E — Safety / footgun hardening

1. **`console_error_panic_hook`** installed in `main.rs` (`main.rs:1-5` currently
   just `dioxus::launch(App)`). Turns a wasm panic from a silent white freeze into
   a readable console error. Add the dependency.
2. **Downgrade hot-path panics to graceful fallbacks.** `render.rs:91`
   `.expect("loaded_keys is always Some after boot")` and `render.rs:186-192` the
   grid `try_into().unwrap_or_else(|_| panic!(...))` run on every grid render
   (incl. every drag crossing). Convert to early-return / fallback render so an
   invariant violation degrades a single grid rather than aborting the app. The
   panic hook makes any such violation visible in the console. (Do NOT silently
   swallow — log via the panic hook / a warn; the goal is "no full-app freeze,"
   not "hide bugs.")
3. **`.read()`→`.peek()` hygiene sweep.** Convert point-in-time/callback reads
   the audit found to `.peek()`: `CustomKeysService::snapshot` +
   `GridLayoutService::snapshot` (the deferred cleanup), plus the callback sites
   (`download_info_dialog_host/mod.rs:19`, `resolve_page/hooks.rs:88`,
   `tile_override/hooks.rs:276-278`, `grid_editor/logic/handlers.rs:27,104,134`,
   and the page-body booleans `collisions_page/hooks.rs:131`,
   `resolve_page/hooks.rs:72`). These are correctness-hygiene against the class we
   already hit.
4. **Fix the touch-drag closure leak** (`mechanics.rs:161` `cb.forget()` on a
   per-touch long-press `Closure::once` that leaks when the press is aborted).

## Theme C — Rendering granularity (riskier; small increments, browser-verified)

Root cause: the whole-document `Signal<Option<CustomKeys>>` is read in ~28
places with no slice granularity, so one edit re-runs every subscriber. On the
editor page this rebuilds all four command grids per edit.

**Design:**
1. **Per-grid derived reads.** Each `GridEditor` should re-render only when *its
   own* slots change. Memoize the per-grid `rendered_command_grid` (in a hook, per
   COMPONENTS.md) keyed on the grid's inputs so an edit to one container's slots
   doesn't rebuild the other three grids. Exact selector mechanism (per-container
   `use_memo` over a derived slice vs. a service-provided sliced read) decided in
   the plan; the commitment is "a grid re-renders only on its own slot changes."
2. **De-subscribe header buttons.** `export_button/hooks.rs:21` and
   `resolve_button/hooks.rs:15` read the whole document for a boolean that is
   constant after load — switch to a `use_memo(|| keys.read().is_some())` (subscribe
   to the boolean) or `.peek()`.
3. **Memoize the unit list.** `UnitListState::new` /
   `UnitListing::resolve` / per-section `entries_for` run per render (and per
   selection). Wrap in `use_memo` keyed on `(race, mode, query, search_field,
   visibility)` so selecting a unit or an unrelated re-render doesn't re-walk the
   full catalog.

Each item is independently shippable and browser-verified (units render, correct
grid updates on edit, selection still highlights).

## Theme D — Drag / pointer latency (riskier; browser-verified)

`pointer_move` (`grid_editor/logic/mechanics.rs:166-341`) runs a synchronous
`element_from_point` + two `closest()` walks + a `drag_follower` clone/set on
every pointer event.

**Design:**
1. **rAF-coalesce** pointer handling so at most one hit-test + follower update per
   animation frame.
2. **Rect early-out:** cache the current tile's bounding rect; skip
   `element_from_point` while the cursor stays inside it (only re-hit-test on tile
   exit).
3. Apply the same to the duplicated inventory-hotkeys drag
   (`inventory_cell/hooks.rs:165-230`); dedupe the two implementations if it falls
   out cleanly (don't force it).

Combined with C's per-tile hover (drag highlight driven per-tile, not by
re-rendering the whole grid), a drag stops doing domain recompute + whole-grid
rebuild per crossing.

## Verification strategy

- Themes A, B: native tests (domain) + behavior reasoning; browser check that
  edit/undo/redo persist across reload.
- Themes C, D: each increment ends with a live Playwright-MCP pass (boot renders,
  units present, drag works, drag-hover highlights the right tile, undo/redo
  across reload) run against a controlled server on a NON-dev port (never squat
  8123/8200), cleaned up immediately.
- Add/extend an e2e test that exercises command-grid drag-and-drop + hover
  highlight, so the `moon :ci` gate actually covers the drag path (it currently
  does not, which is why the earlier boot-loop only showed as a global-setup
  hang).
- Final `moon run hotkey-editor:ci` (full Playwright suite) green, monitored, with
  the sandbox disabled, before the pass is considered done.

## Out of scope (future passes)

- Theme F: Web Worker offload of collision scan / cascade / normalize.
- Incremental collision counting (vs. debounced full rescan).
- `db.rs` wasm-bloat / lazy DB-init reduction.
- Templates-dialog first-open re-parse cost (theme audit P6) and cascade-resolve
  per-edit cost on the Resolve page (only mounted there).

## Rules honored

- R1 (sync write preserved; only derived badge debounced); R2/always-normalized
  (theme A relies on it); R8/R9 (domain changes pure + tested); COMPONENTS.md
  (memoization in hooks, pure bodies); RUST_STYLE throughout.
