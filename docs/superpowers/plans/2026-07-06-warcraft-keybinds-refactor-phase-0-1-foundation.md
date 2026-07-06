# warcraft-keybinds Refactor — Foundation (Phase 0 + Phase 1) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Lay the guardrails (DDD marker-assertion harness + docs) and complete the crate-wide RUST_STYLE sweep, so every later phase of the `warcraft-keybinds` DDD refactor lands against a conformant, self-checking baseline.

**Architecture:** Phase 0 adds a compile-time DDD conformance harness (generic `assert_*` helpers mirroring the existing `assert_domain_aggregate` pattern) and authors/updates the governing docs — no production logic changes. Phase 1 is a mechanical, behavior-preserving style sweep (derives, `Self`, `verb_noun`→methods, `pub`→accessors, single-letter closures, tuples, stray `as`) driven off the audit's file:line inventory, gated by clippy + the existing test suite.

**Tech Stack:** Rust (native, no wasm), the `ddd` vocabulary crate, `moon` task runner, `cargo` inside the Nix dev shell (loaded via direnv / `nix develop`).

## Global Constraints

- **No behavior change.** Collision/cascade/normalization outputs must be byte-identical before and after every task. The existing test suite is the regression guard.
- **`warcraft-keybinds` stays pure.** Zero `wasm-bindgen` / `web-sys` / `dioxus` / `gloo` deps (R8). Allowed deps unchanged: `warcraft-api`, `warcraft-database`, `serde`, `ddd`.
- **RUST_STYLE is law** (`docs/RUST_STYLE.md`): full semantic names, no tuples, no `as` outside `From`/`TryFrom` bodies, private fields + accessors, `Self` inside impls, derive every qualifying trait, idiomatic std traits, no `verb_noun` free functions, no section-header comments.
- **Every phase ends `moon run :ci` green**, including the Playwright e2e gate (R9). Per-task checks use `cargo` for speed; the phase gate is `moon run :ci`.
- **Commit frequently** — one commit per task minimum. Branch is `develop` (not the default `main`), so commit directly on it.
- Commit trailer on every commit: `Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>`.

---

## File Structure (this plan)

- Create: `crates/warcraft-keybinds/src/ddd_conformance.rs` — `#[cfg(test)]` module of generic DDD marker-assertion helpers (`assert_value_object`, `assert_entity`, `assert_identifier`, `assert_domain_service`, `assert_factory`, `assert_specification`, `assert_read_model`). One responsibility: prove a type carries a given `ddd` role at compile time.
- Modify: `crates/warcraft-keybinds/src/lib.rs` — declare the `#[cfg(test)] mod ddd_conformance;`.
- Create: `docs/DOMAIN.md` — the domain-crate structural + DDD conventions doc (analogue of `COMPONENTS.md`).
- Modify: `docs/ARCHITECTURE.md:241-365` — rewrite the stale §5 (violations) and §6 (refactor plan) to reflect current + target state.
- Modify (Phase 1): the style-violation sites enumerated per task below, across `crates/warcraft-keybinds/src/**`.

---

## Reference: `ddd` trait bounds (verified against `crates/ddd/src/`)

Copy these exactly when writing the harness — the supertrait bounds are what make each assertion meaningful:

```
ValueObject:            Clone + Eq + Layered<Layer = DomainLayer>
Identifier:             ValueObject                       (i.e. Clone + Eq + Layered<DomainLayer>)
Entity:                 Layered<Layer = DomainLayer>      (+ type Identity: Eq; fn identity(&self) -> &Self::Identity)
DomainService:          Layered<Layer = DomainLayer>
Factory<Product>:       Layered<Layer = DomainLayer>      (+ type Blueprint; fn create(&self, Blueprint) -> Product)
Specification<Candidate>: Layered<Layer = DomainLayer>    (+ fn is_satisfied_by(&self, &Candidate) -> bool)
ReadModel:              (no supertrait, no layer)
AggregateRoot:          Clone + Layered<Layer = DomainLayer>
```

The existing worked pattern (`crates/warcraft-keybinds/src/custom_keys.rs:116-132`):

```rust
#[cfg(test)]
mod ddd_marker_tests {
    use super::CustomKeys;
    use ddd::AggregateRoot;
    use ddd::DomainLayer;
    use ddd::Layered;

    fn assert_domain_aggregate<Aggregate>()
    where
        Aggregate: AggregateRoot + Layered<Layer = DomainLayer>,
    {
    }

    #[test]
    fn custom_keys_is_a_domain_aggregate_root() {
        assert_domain_aggregate::<CustomKeys>();
    }
}
```

---

# PHASE 0 — Guardrails & docs

### Task 0.1: DDD conformance harness — value object, identifier, read model

**Files:**
- Create: `crates/warcraft-keybinds/src/ddd_conformance.rs`
- Modify: `crates/warcraft-keybinds/src/lib.rs` (add module declaration)

**Interfaces:**
- Produces (all `pub(crate)`, callable from any in-crate `#[cfg(test)]` module):
  - `fn assert_value_object<T>() where T: ddd::ValueObject`
  - `fn assert_identifier<T>() where T: ddd::Identifier`
  - `fn assert_read_model<T>() where T: ddd::ReadModel`

- [ ] **Step 1: Write the module with the first three assertion helpers plus a self-test**

Create `crates/warcraft-keybinds/src/ddd_conformance.rs`:

```rust
//! Compile-time conformance assertions for the `ddd` role vocabulary.
//!
//! Each helper is a generic function whose `where` bound is the `ddd`
//! trait's own contract. Calling `assert_value_object::<Hotkey>()` from a
//! test fails to compile unless `Hotkey` genuinely implements
//! `ddd::ValueObject`. These are the domain-crate analogue of the existing
//! `assert_domain_aggregate` pattern, generalized to every role this crate
//! adopts.

#![cfg(test)]

pub(crate) fn assert_value_object<Type>()
where
    Type: ddd::ValueObject,
{
}

pub(crate) fn assert_identifier<Type>()
where
    Type: ddd::Identifier,
{
}

pub(crate) fn assert_read_model<Type>()
where
    Type: ddd::ReadModel,
{
}

#[cfg(test)]
mod tests {
    use super::assert_read_model;
    use super::assert_value_object;
    use crate::Hotkey;

    #[test]
    fn hotkey_is_not_yet_marked() {
        // Placeholder self-test proving the harness compiles and links.
        // Real role assertions are added in later phases as types are marked.
        let _ = assert_value_object::<Hotkey> as fn();
        let _ = assert_read_model::<crate::CollisionSummary> as fn();
    }
}
```

Note: the self-test references the assertion functions **as function pointers** (`as fn()`) rather than *calling* them, so this task compiles before any type is actually marked `ValueObject`/`ReadModel`. Later phases replace these with real calls once the markers exist.

- [ ] **Step 2: Declare the module in `lib.rs`**

In `crates/warcraft-keybinds/src/lib.rs`, add alongside the other `mod` declarations:

```rust
#[cfg(test)]
mod ddd_conformance;
```

- [ ] **Step 3: Verify it compiles under the test profile**

Run: `cargo test -p warcraft-keybinds ddd_conformance -- --list`
Expected: lists `ddd_conformance::tests::hotkey_is_not_yet_marked` with no compile error.

- [ ] **Step 4: Commit**

```bash
git add crates/warcraft-keybinds/src/ddd_conformance.rs crates/warcraft-keybinds/src/lib.rs
git commit -m "test(keybinds): add DDD marker-assertion conformance harness

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

### Task 0.2: DDD conformance harness — entity, domain service, factory, specification

**Files:**
- Modify: `crates/warcraft-keybinds/src/ddd_conformance.rs`

**Interfaces:**
- Consumes: the module from Task 0.1.
- Produces (all `pub(crate)`):
  - `fn assert_entity<Type>() where Type: ddd::Entity`
  - `fn assert_domain_service<Type>() where Type: ddd::DomainService`
  - `fn assert_factory<Product, TheFactory>() where TheFactory: ddd::Factory<Product>`
  - `fn assert_specification<Candidate, TheSpecification>() where TheSpecification: ddd::Specification<Candidate>`

- [ ] **Step 1: Add the four remaining helpers**

Append to `crates/warcraft-keybinds/src/ddd_conformance.rs` (before the `#[cfg(test)] mod tests`):

```rust
pub(crate) fn assert_entity<Type>()
where
    Type: ddd::Entity,
{
}

pub(crate) fn assert_domain_service<Type>()
where
    Type: ddd::DomainService,
{
}

pub(crate) fn assert_factory<Product, TheFactory>()
where
    TheFactory: ddd::Factory<Product>,
{
}

pub(crate) fn assert_specification<Candidate, TheSpecification>()
where
    TheSpecification: ddd::Specification<Candidate>,
{
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cargo test -p warcraft-keybinds ddd_conformance -- --list`
Expected: still lists the harness test, no compile error.

- [ ] **Step 3: Commit**

```bash
git add crates/warcraft-keybinds/src/ddd_conformance.rs
git commit -m "test(keybinds): complete DDD conformance harness (entity/service/factory/spec)

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

### Task 0.3: Author `docs/DOMAIN.md`

**Files:**
- Create: `docs/DOMAIN.md`

**Interfaces:** none (documentation).

- [ ] **Step 1: Write `docs/DOMAIN.md`**

Create the file with these sections (write full prose, not headers-only):

1. **Purpose** — this doc is to `warcraft-keybinds` what `COMPONENTS.md` is to `hotkey-editor`: the structural + DDD contract for the pure-domain crate.
2. **The DDD role map** — reproduce the table from the design spec (§4): which crate types are AggregateRoot / Entity / Identifier / ValueObject / DomainService / Specification / Factory / DomainEvent / Policy / ReadModel. State that every marked type ships a `ddd_conformance` assertion.
3. **Structural rules** (generalized from `COMPONENTS.md`):
   - directory == primary type; one type per small file (~target ≤500 lines, most far smaller).
   - the call/composition tree is the directory tree — a helper used by exactly one parent is a submodule of it.
   - `shared/` module for a leaf used by 2+ siblings.
   - intermediate `mod.rs` files carry only their own sibling surface — no descendant re-export flattening; only `lib.rs` curates the public facade.
4. **The marker-assertion convention** — every role marker is proven by a `crate::ddd_conformance::assert_*::<T>()` call in a `#[cfg(test)]` module; link to the existing `assert_domain_aggregate` example.
5. **The transient-events rule** — DomainEvents are raised and consumed in-tick by Policies; never persisted. localStorage-materialized text stays the source of truth (cross-reference ARCHITECTURE.md R1/R2/R5).
6. **Non-goals** — no event store, no persistence-model change, no game-data logic migrating out to `warcraft-database` in this effort.

- [ ] **Step 2: Cross-check against the spec**

Run: `git diff --stat` and confirm `docs/DOMAIN.md` exists; re-read it against `docs/superpowers/specs/2026-07-06-warcraft-keybinds-ddd-refactor-design.md` §4–§8 to confirm the role map matches exactly.

- [ ] **Step 3: Commit**

```bash
git add docs/DOMAIN.md
git commit -m "docs: add DOMAIN.md — domain-crate structural + DDD conventions

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

### Task 0.4: Update stale `ARCHITECTURE.md` §5–§6

**Files:**
- Modify: `docs/ARCHITECTURE.md:241-365`

**Interfaces:** none (documentation).

- [ ] **Step 1: Rewrite §5 "Where today's code violates these rules"**

The §5 bullets describe a `lib.rs` at 1779 lines holding parser+model+serializer and renderer-time cascade calls — that state no longer exists (the crate has a module tree; cascade is baked at write time). Replace §5 with a short note that the original violations are resolved, and point to `docs/DOMAIN.md` + the design spec for the current refactor's targets. Keep the historical intent as one sentence for context; do not delete the rule numbering elsewhere in the doc.

- [ ] **Step 2: Rewrite §6 "Refactor plan"**

The §6 seven-phase plan (facade → localStorage-truth → bake cascade → strip renderer → clean internals → tests) is complete. Replace it with a pointer to the current phased plan: reference `docs/superpowers/specs/2026-07-06-warcraft-keybinds-ddd-refactor-design.md` §9 and note the per-phase plans live under `docs/superpowers/plans/`.

- [ ] **Step 3: Verify no rule references broke**

Run: `grep -n "R[0-9]" docs/ARCHITECTURE.md | head -40`
Expected: the R1–R10 hard-rule definitions in §4 are untouched; only §5/§6 prose changed.

- [ ] **Step 4: Commit**

```bash
git add docs/ARCHITECTURE.md
git commit -m "docs: refresh ARCHITECTURE.md §5-§6 (stale refactor plan is complete)

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

### Task 0.5: Phase 0 CI gate

- [ ] **Step 1: Run the full CI gate**

Run: `moon run :ci`
Expected: fmt + clippy + tests + wasm build + e2e all green. (Docs + a `#[cfg(test)]` harness change nothing at runtime, so e2e must stay green.)

- [ ] **Step 2: If green, Phase 0 is complete.** No new commit needed unless the gate surfaced a formatting fix.

---

# PHASE 1 — Crate-wide RUST_STYLE sweep

Behavior-preserving. Each task targets one rule category, edits the enumerated
sites (verify each against the live file before editing — line numbers may
have shifted since the audit), and gates on clippy + tests. Where the audit
gave a representative sample rather than a full list, the step says so and the
executor greps for the rest with the provided command.

### Task 1.1: Add missing derives

**Files (audit inventory — verify each before editing):**
- Modify: `crates/warcraft-keybinds/src/text/{tip,description,command_label,substitution_placeholders,color_codes,level_markers,inner_spaces}.rs` — the 7 zero-derive marker structs
- Modify: `crates/warcraft-keybinds/src/cascade/queue.rs` (`PositionAssignmentGroup`, `AssignmentQueue`, `SpillDecision`, `GapPullCandidate`), `crates/warcraft-keybinds/src/cascade/planner.rs` (`CascadePlan`, `PlannedMove`, `UnresolvedMover`), `crates/warcraft-keybinds/src/cascade/conflict_graph.rs` (`ConflictNode`, `CollidingPair`, `ConflictGraph`)
- Modify: `crates/warcraft-keybinds/src/collision/cross_unit.rs` (`CrossUnitCollisionReport`, `CrossUnitPositionGroup`, `SharedAbilityEntry`, `AffectedUnitEntry`)
- Modify: `crates/warcraft-keybinds/src/unit/keyed.rs` (`UnitKeyedCustomKeys`, `UnitAbilityGroup`, `UnitAbilitySlot`), `crates/warcraft-keybinds/src/identity/slot.rs` (`CommandCard` — add `Hash`), `crates/warcraft-keybinds/src/model.rs` (`SystemBinding` — add `PartialEq`), `crates/warcraft-keybinds/src/display/{templates,ability_cell,inspector_detail}.rs`, `crates/warcraft-keybinds/src/system/binding_map.rs`

**Interfaces:** none — additive derives only.

- [ ] **Step 1: Find every derive gap**

Run: `cargo clippy -p warcraft-keybinds --all-targets 2>&1 | rg "derivable|missing_derive" ` (informational), then manually scan for structs/enums with no `#[derive(...)]` line:

Run: `rg -n --multiline '(\n\s*)(pub )?(struct|enum) ' crates/warcraft-keybinds/src | rg -v derive` (informational — surfaces types whose preceding line is not a derive).

- [ ] **Step 2: For each type, add the maximal derive set it mechanically supports**

Order per RUST_STYLE: `Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default`. For the 7 `text/` marker structs (all zero-field), the full set applies:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Tip;
```

For types holding `f32` (`statistics/values.rs` family), **do not** add `Eq`/`Hash`/`Ord` — they cannot derive them; leave those off and add only the ones that compile. For each type, add the derive, then let the compiler reject any trait the fields don't support and remove exactly that one.

- [ ] **Step 3: Verify compile + tests**

Run: `cargo test -p warcraft-keybinds`
Expected: PASS (derives are additive; no test should change).

Run: `cargo clippy -p warcraft-keybinds --all-targets -- -D warnings`
Expected: no warnings.

- [ ] **Step 4: Commit**

```bash
git add crates/warcraft-keybinds/src
git commit -m "style(keybinds): derive every trait each type qualifies for

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

### Task 1.2: Use `Self` inside impl blocks

**Files (audit sample — grep for the full set):**
- Modify: `crates/warcraft-keybinds/src/cascade/planner.rs:176,211`, `crates/warcraft-keybinds/src/cascade/queue.rs:646,665,715,793,800`, `crates/warcraft-keybinds/src/model.rs:1216,1285`, `crates/warcraft-keybinds/src/unit/grids.rs` (`GridRole` arms), `crates/warcraft-keybinds/src/display/templates.rs:120,121,125,167`, `crates/warcraft-keybinds/src/identity/keycode.rs:52,159,221,285`, `crates/warcraft-keybinds/src/identity/hotkey_token.rs:110`, `crates/warcraft-keybinds/src/custom_keys.rs:477`, `crates/warcraft-keybinds/src/unit/keyed.rs:136`

**Interfaces:** none — `Self` is identical to the concrete name.

- [ ] **Step 1: Find every occurrence of a struct/enum name repeated inside its own impl**

Run: `rg -n 'impl(<[^>]*>)? +(\w+)' crates/warcraft-keybinds/src` to list impl blocks, then for each, grep the block body for the concrete type name. Representative fix (`cascade/planner.rs:176`):

```rust
// before
fn from(queue: &AssignmentQueue) -> CascadePlan {
    ...
    CascadePlan { moves, unresolved }
}
// after
fn from(queue: &AssignmentQueue) -> Self {
    ...
    Self { moves, unresolved }
}
```

For match arms / `matches!` inside `impl GridRole` (`unit/grids.rs`), the variants use `Self::MainCommand` etc. For `const ALL: [Letter; 26]` inside `impl Letter` (`identity/keycode.rs:52`), write `const ALL: [Self; 26]`.

Leave the borderline primitive cases (`u32::from(...)` inside `impl From<KeyCode> for u32`) as-is — `u32` is the foreign type, not `Self`.

- [ ] **Step 2: Verify compile + tests**

Run: `cargo test -p warcraft-keybinds`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add crates/warcraft-keybinds/src
git commit -m "style(keybinds): use Self inside impl blocks

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

### Task 1.3: Private fields + accessors (kill `pub`/`pub(crate)` fields)

**Files:**
- Modify: `crates/warcraft-keybinds/src/display/rendered_grid.rs:76-81` (`CommandGridRenderInput` — 6 `pub` fields)
- Modify: `crates/warcraft-keybinds/src/model.rs:694-695` (`SectionResolution` — 2 `pub(crate)` fields)

**Interfaces:**
- Produces: getter methods replacing field access. For `CommandGridRenderInput`: `slots()`, `layout()`, `selected()`, `selected_is_research()`, `tier_overrides()`, `restrict_draggable_to()`. For `SectionResolution`: `canonical_id()`, `kind()`. Match each getter's return type to the field type (return `Copy` values by value, others by `&`).

- [ ] **Step 1: Make the fields private and add getters**

For `CommandGridRenderInput` (`display/rendered_grid.rs`): drop `pub` from each field; add an `impl` block with one getter per field. For a `Copy` field return by value, else return `&T`.

- [ ] **Step 2: Update every read site**

Run: `rg -n '\.(slots|layout|selected|selected_is_research|tier_overrides|restrict_draggable_to)\b' crates/warcraft-keybinds crates/hotkey-editor` — replace direct field reads on these types with the getter call. (`CommandGridRenderInput` is consumed by `hotkey-editor`'s grid renderer — update those call sites too; this is a cross-crate read, allowed.)

- [ ] **Step 3: Verify compile + tests (both crates)**

Run: `cargo test -p warcraft-keybinds` then `cargo build -p hotkey-editor`
Expected: PASS / clean build.

- [ ] **Step 4: Commit**

```bash
git add crates/warcraft-keybinds/src crates/hotkey-editor/src
git commit -m "style(keybinds): private fields with accessors on render input + section resolution

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

### Task 1.4: Convert `verb_noun` free functions to methods

**Files:**
- Modify: `crates/warcraft-keybinds/src/unit/slots.rs:46,61,68,88,105,113` (`ability_reverts_to_host`, `ability_is_hidden_for_unit`, `ability_requires_rooted_form`, `units_form_upgrade_swap`, `slot_position_from_database`, `research_slot_position_from_database`)
- Modify: `crates/warcraft-keybinds/src/cascade/conflict_graph.rs:122` (`ability_list_priority`)
- Modify: `crates/warcraft-keybinds/src/cascade/planner.rs:223,233` (`move_reason_for_node`, `move_reason_from_group`)

**Interfaces:**
- Produces: each free function becomes a method on the noun it operates on. Note for the executor: several of these (`ability_is_hidden_for_unit(unit_id, ability_id)`) take `&str` ids, not domain types. Where a natural receiver struct exists, make it a method; where the "noun" is a plain `&str` id with no home type yet, this is a **Phase 5 (Specifications) concern** — leave those specific ones and only convert the ones with an existing receiver type now. Convert `move_reason_for_node`/`move_reason_from_group` to methods on the group/node type they read; convert `ability_list_priority` to a method on the ability-list type.

- [ ] **Step 1: Convert the functions that have an existing receiver type**

For `move_reason_from_group` / `move_reason_for_node`, move them into the relevant `impl` (on `PositionAssignmentGroup` / `ConflictNode`) as methods, updating call sites. Show the moved signature and one updated call site.

- [ ] **Step 2: Defer the `&str`-id predicates**

Add a one-line `// TODO(phase-5): becomes a Specification` is **NOT** allowed (no placeholder comments). Instead, leave those functions untouched and record in the commit message that `ability_is_hidden_for_unit` et al. are deferred to Phase 5 where they become `Specification`s. Do not convert them to methods on `&str`.

- [ ] **Step 3: Verify compile + tests**

Run: `cargo test -p warcraft-keybinds`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add crates/warcraft-keybinds/src
git commit -m "style(keybinds): make move-reason/priority free functions into methods

The unit/slots.rs ability predicates are intentionally left as free functions;
they become ddd::Specification types in Phase 5.

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

### Task 1.5: Remove single-letter closures, author tuples, stray `as`

**Files:**
- Modify: single-letter closures at `crates/warcraft-keybinds/src/custom_keys.rs:967,979,989,1012,1254,1263,1271,1279,3607,3618`, `crates/warcraft-keybinds/src/unit/keyed.rs:248`, `crates/warcraft-keybinds/src/collision/unit_report.rs:382`, `crates/warcraft-keybinds/src/collision/cross_unit.rs:656`, `crates/warcraft-keybinds/src/display/inspector_detail.rs:220`
- Modify: `crates/warcraft-keybinds/src/grid/layout.rs:12` (the `const` `as` cast)
- Modify: `crates/warcraft-keybinds/src/cascade/queue.rs:1021,1027` (test-only author tuple)

**Interfaces:** none.

- [ ] **Step 1: Rename single-letter closure params to full semantic names**

Run: `rg -n '\|[a-z]\|' crates/warcraft-keybinds/src` to find them. Rename e.g. `.is_none_or(|h| h.accepts_grid_letter())` → `.is_none_or(|hotkey| hotkey.accepts_grid_letter())`, `|obj|` → `|object|`, `|e|` → `|entry|`, `|g|` → `|group|`, `|c|` → `|character|` (choose the name matching what the closure body treats it as).

- [ ] **Step 2: Fix the `const` `as` cast in `grid/layout.rs:12`**

The two `as usize` casts sit in a `const` expression, outside any `From`/`TryFrom` body. `COMMAND_GRID_COLUMNS`/`COMMAND_GRID_ROWS` are small constants. Replace with `usize::from(...)` if the source types implement `Into<usize>`; if they are already `usize`-typed consts, drop the cast entirely. Verify the resulting `COMMAND_GRID_TILE_COUNT` value is unchanged with a test:

```rust
#[test]
fn command_grid_tile_count_is_twelve() {
    assert_eq!(COMMAND_GRID_TILE_COUNT, 12);
}
```

- [ ] **Step 3: Replace the test-only author tuple in `cascade/queue.rs`**

The `Vec<(u8, u8)>` at line 1021 (a test) builds `(row_value, column_value)`. Define a named test struct with `row`/`column` fields, or restructure the assertion to compare named-field values. Since it is test code, a local `#[derive(PartialEq, Debug)] struct RowColumn { row: u8, column: u8 }` in the test module is the minimal fix.

- [ ] **Step 4: Verify compile + tests + clippy**

Run: `cargo test -p warcraft-keybinds`
Expected: PASS.
Run: `cargo clippy -p warcraft-keybinds --all-targets -- -D warnings`
Expected: no warnings.

- [ ] **Step 5: Commit**

```bash
git add crates/warcraft-keybinds/src
git commit -m "style(keybinds): full semantic closure names, drop stray as-cast + author tuple

Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"
```

### Task 1.6: Phase 1 CI gate + fmt

- [ ] **Step 1: Format**

Run: `cargo fmt -p warcraft-keybinds`
Then: `git add -u && git commit -m "style(keybinds): cargo fmt" -m "Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>"` (only if fmt changed anything).

- [ ] **Step 2: Full CI gate**

Run: `moon run :ci`
Expected: fmt + clippy + tests + wasm build + e2e all green.

- [ ] **Step 3: Re-audit the touched rules**

Run these greps; each should return no genuine violations (allow the documented exceptions):
- `rg -n '\|[a-z]\| ' crates/warcraft-keybinds/src` — single-letter closures → none
- `rg -n ' as ' crates/warcraft-keybinds/src | rg -v 'impl (From|TryFrom)'` — stray `as` → none outside conversion bodies
- `rg -n 'pub [a-z_]+:' crates/warcraft-keybinds/src` — `pub` fields → none

- [ ] **Step 4: Phase 1 complete** once the gate is green and the re-audit is clean.

---

## Self-Review (against the design spec)

**Spec coverage (this plan covers spec §9 phases 0 and 1 only):**
- Spec §8 (docs: update ARCHITECTURE.md, author DOMAIN.md) → Tasks 0.3, 0.4. ✓
- Spec §7 (marker-assertion harness mirroring `assert_domain_aggregate`) → Tasks 0.1, 0.2. ✓
- Spec §9 Phase 1 (derives, `Self`, `verb_noun`, `pub` fields, single-letter, tuples, `as`) → Tasks 1.1–1.5. ✓
- Deferred correctly: the `unit/slots.rs` `&str`-id predicates are NOT force-converted here; they become Specifications in Phase 5 (Task 1.4 Step 2). ✓
- Out of scope for this plan (later phases, planned just-in-time): value-object/entity/service marking, monolith splits, codec/normalize extraction, CQRS. Correct — those depend on emergent post-split structure.

**Placeholder scan:** No "TBD"/"add error handling"/"similar to Task N". Task 1.4 Step 2 explicitly forbids a placeholder TODO comment and routes the deferral to the commit message instead. ✓

**Type consistency:** The seven `assert_*` helper names in Tasks 0.1/0.2 match their `where` bounds and the `ddd` trait names verified from `crates/ddd/src/`. Getter names in Task 1.3 match the `CommandGridRenderInput`/`SectionResolution` field names from the audit. ✓

---

## Next phases

Phases 2–8 (test/codegen extraction; value-object & identity splits; domain-service & codec extraction; specifications & factories; CQRS domain half; CQRS application half; facade & final pass) get their own `docs/superpowers/plans/` file each, authored just-in-time against the real post-split code — because their bite-sized code steps reference line numbers and structure that this foundation work will move.
