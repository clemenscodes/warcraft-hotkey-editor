# Race Scoped Search, Renderer Data and State Implementation Plan (Phase 2a)

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Thread a search race scope from a new editor-state signal through `UnitFilterQuery` into the domain `UnitListingRequest`, so the renderer can restrict a search to a set of races.

**Architecture:** The scope is a `Signal<RaceSelection>` in `EditorState`, defaulting to `RaceSelection::All`. `UnitFilterQuery` gains a matching field, and its conversion to `UnitListingRequest` passes it through the domain's `searching_within` builder. No behaviour changes yet, because nothing sets the scope away from `All` until the UI phase.

**Tech Stack:** Rust, Dioxus signals, the `hotkey-editor` renderer crate, `warcraft_api::RaceSelection` from the domain change.

## Global Constraints

- All paths are relative to `/home/clemens/.local/src/warcraft-hotkey-editor`.
- The `[patch]` in the workspace `Cargo.toml` points `warcraft-api` at the local `warcraft-data` working copy where `RaceSelection` and `searching_within` already exist. Do not remove the patch, do not tag, do not bump pins in this phase.
- Because the patched crate is out of the moon workspace, moon's cache can give a false green. Run the compile check with `moon run :check -- --force` semantics; if moon does not forward the flag, run `moon run :check` and then a direct `cargo check` is not the gate. The real gate stays `moon run :ci`, run once after the UI phase, not here.
- Rust style, from `docs/RUST_STYLE.md`. Full semantic names. No tuples. `Self` inside impls. Derive what qualifies. Private fields with accessors.
- `RaceSelection` implements `Default` as `All`, so a signal is created with `use_signal(RaceSelection::default)`. `RaceSelection` is not `Copy` (it holds a `BTreeSet`), so read it with `.read().clone()`.
- Do not change `services/navigation/default_unit.rs`. Its `UnitListingRequest::new` builds a browse with an empty query, so the search scope never applies to it, and it must keep landing on the one active race.
- End every commit message with the trailer `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`. Commit in the `hotkey-editor` repository, on the current branch `feature/mobile-redesign`.

---

### Task 1: Add the search race scope signal to `EditorState`

**Files:**
- Modify: `crates/hotkey-editor/src/services/editor_state/mod.rs`
- Modify: `crates/hotkey-editor/src/services/editor_state/context.rs`

**Interfaces:**
- Produces: `EditorState::search_race_scope(&self) -> Signal<RaceSelection>`, a signal defaulting to `RaceSelection::All`.

- [ ] **Step 1: Add the field and accessor in `mod.rs`**

In `crates/hotkey-editor/src/services/editor_state/mod.rs`, add `RaceSelection` to the `warcraft_api` import (it currently imports `SearchField, UnitKind, WarcraftObjectId`), add the struct field after `expand_variants`, and add the accessor.

Field, added to the `EditorState` struct after `expand_variants: Signal<bool>,`:

```rust
    search_race_scope: Signal<RaceSelection>,
```

Accessor, added next to `expand_variants`:

```rust
    pub fn search_race_scope(&self) -> Signal<RaceSelection> {
        self.search_race_scope
    }
```

Import line, extend the existing `use warcraft_api::{...};` in this file to include `RaceSelection`.

- [ ] **Step 2: Create and provide the signal in `context.rs`**

In `crates/hotkey-editor/src/services/editor_state/context.rs`, add `RaceSelection` to the `use warcraft_api::{...};` import, create the signal alongside the others, and add it to the struct literal.

Create it after `let expand_variants = use_signal::<bool>(|| false);`:

```rust
    let search_race_scope = use_signal(RaceSelection::default);
```

Add `search_race_scope,` to the `EditorState { ... }` struct literal, after `expand_variants,`.

- [ ] **Step 3: Verify it compiles**

Run: `moon run :check`
Expected: PASS, the crate type-checks with the new signal wired in.

- [ ] **Step 4: Commit**

```bash
cd /home/clemens/.local/src/warcraft-hotkey-editor
git add crates/hotkey-editor/src/services/editor_state/mod.rs crates/hotkey-editor/src/services/editor_state/context.rs
git commit -m "Add a search race scope signal to editor state

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 2: Thread the scope through `UnitFilterQuery` into the domain request

**Files:**
- Modify: `crates/hotkey-editor/src/services/unit_catalog/queries/unit_filter_query.rs`
- Modify: `crates/hotkey-editor/src/services/unit_catalog/context.rs`

**Interfaces:**
- Consumes: `EditorState::search_race_scope` from Task 1, `warcraft_api::RaceSelection` and `UnitListingRequest::searching_within` from the domain change.
- Produces: `UnitFilterQuery::new` takes a sixth argument `search_race_scope: RaceSelection`, and the conversion to `UnitListingRequest` narrows the search to it.

- [ ] **Step 1: Extend `UnitFilterQuery` and its conversion**

In `crates/hotkey-editor/src/services/unit_catalog/queries/unit_filter_query.rs`, add the import `use warcraft_api::RaceSelection;` with the other `warcraft_api` imports.

Add the field to the struct after `visibility: CatalogVisibility,`:

```rust
    search_race_scope: RaceSelection,
```

Add the parameter to `new` (last, after `visibility`) and set the field:

```rust
    pub fn new(
        race: Race,
        modes: UnitModeSelection,
        search_query: String,
        search_field: SearchField,
        visibility: CatalogVisibility,
        search_race_scope: RaceSelection,
    ) -> Self {
        Self {
            race,
            modes,
            search_query,
            search_field,
            visibility,
            search_race_scope,
        }
    }
```

Change the conversion to pass the scope through the builder:

```rust
impl From<&UnitFilterQuery> for UnitListingRequest {
    fn from(filter: &UnitFilterQuery) -> Self {
        let search_query = filter.search_query.clone();
        let request = Self::new(
            filter.race,
            filter.modes,
            search_query,
            filter.search_field,
            filter.visibility,
        );
        request.searching_within(filter.search_race_scope.clone())
    }
}
```

- [ ] **Step 2: Update the two test call sites in the same file**

In the `#[cfg(test)] mod tests` of the same file, both `UnitFilterQuery::new(...)` helpers (`human_melee` and the both-modes test) build with five arguments. Add `RaceSelection::All` as the sixth argument to each, and add `use warcraft_api::RaceSelection;` to the test module's imports.

For `human_melee`, the call becomes:

```rust
        UnitFilterQuery::new(
            Race::Human,
            modes,
            owned_query,
            SearchField::UnitName,
            CatalogVisibility::default(),
            RaceSelection::All,
        )
```

For the both-modes test's second construction, add `RaceSelection::All` as the final argument in the same way.

- [ ] **Step 3: Pass the signal in the provider**

In `crates/hotkey-editor/src/services/unit_catalog/context.rs`, inside `use_unit_catalog_provider`, read the new signal and pass it to `UnitFilterQuery::new`. Add near the other signal bindings:

```rust
    let search_race_scope = editor.search_race_scope();
```

Inside the `use_memo` closure, read it before building the query, and add it as the sixth argument:

```rust
        let scope = search_race_scope.read().clone();
        UnitFilterQuery::new(race, modes, query, field, visibility, scope)
```

- [ ] **Step 4: Verify it compiles**

Run: `moon run :check`
Expected: PASS. The scope is `All` everywhere, so behaviour is unchanged, the search still spans every race.

- [ ] **Step 5: Commit**

```bash
cd /home/clemens/.local/src/warcraft-hotkey-editor
git add crates/hotkey-editor/src/services/unit_catalog/queries/unit_filter_query.rs crates/hotkey-editor/src/services/unit_catalog/context.rs
git commit -m "Thread the search race scope into the unit listing request

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

## After this phase

The renderer carries a search race scope end to end, defaulting to every race, so nothing changes for the user yet. The UI phase (2b) replaces the search dialog's race navigation chips with a multi-select control that writes this signal, folds scope, mode and the display toggles into one configuration dropdown, and puts the search field first. The full `moon run :ci` gate runs at the end of the UI phase, once there is real behaviour and UI to exercise in the browser.
