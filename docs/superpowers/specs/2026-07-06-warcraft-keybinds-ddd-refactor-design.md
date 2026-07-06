# warcraft-keybinds DDD + quality refactor — design

Status: approved (brainstorming). Next step: phased implementation plan.

## 1. Goal

Bring the `warcraft-keybinds` domain crate up to the architectural and
code-quality bar the `hotkey-editor` crate reached in its recent refactor:

- **Full CQRS** adoption of the `ddd` vocabulary, realized **cross-crate**
  (domain half in `warcraft-keybinds`, application-layer `Command`/`Query`
  in `hotkey-editor`).
- **Full `docs/RUST_STYLE.md` compliance** across the crate.
- **Semantic directory names and hierarchical file structure** mirroring
  hotkey-editor's discipline: directory == primary type, one type per small
  file, the call/composition tree is the directory tree.

## 2. Starting state (audit findings)

The crate is ~18.8k lines across 51 files. It already has a semantic module
tree (`identity/`, `cascade/`, `collision/`, `command/`, `display/`, `grid/`,
`statistics/`, `system/`, `text/`, `unit/`) and already marks its three
aggregate roots (`CustomKeys`, `GridLayout`, `EditorHistory`) with
`ddd::AggregateRoot`. The gaps are specific:

**Structure — the directory split happened; the file/type split did not.**
Five monoliths versus a role model whose median file is 17 lines and whose
largest is 574:

- `custom_keys.rs` — 3930 lines. Fuses **four DDD roles**: aggregate root +
  parser/serializer (codec) + normalization service + cascade orchestrator.
- `collision/unit_report.rs` — 2241 lines (92% inline tests + codegen dumps).
- `model.rs` — 1964 lines.
- `cascade/queue.rs` — 1434 lines (three separable sub-algorithms).
- `unit/slots.rs` — 1032 lines.
- `identity/keycode.rs` — 631 lines (five parallel value-object enums + umbrella).

Two recurring smells inside the monoliths: (a) 50–92% of each file is inline
`#[cfg(test)]` / codegen-dump blocks; (b) parsing+serialization is one codec
spread across `custom_keys.rs` and `model.rs`. A generic union-find
(`SlotIslandPartition`) sits inlined in `collision/cross_unit.rs`.

**DDD — only `AggregateRoot` is adopted.** The `ddd` crate's
value-object / entity / identifier / domain-service / specification / factory
/ command / event / policy / query / read-model vocabulary compiles and is
used by nothing. The crate is full of unmarked value objects, unmarked domain
services, and predicate free-functions that are textbook specifications.

**Style — smaller than the structural work.** ~29 `Self` fixes, ~55 types
missing derives (the 7 `text/` marker structs are clearest), 9 `verb_noun`
free functions, 8 `pub` fields, ~14 single-letter closures, 2 author tuples,
1 stray `as` cast. Rules 5 (`print*`), 6 (numeric suffixes), 9 (section
comments) are already clean.

## 3. Confirmed decisions

1. **DDD ambition: Full CQRS.** Adopt the whole load-bearing vocabulary plus
   commands, events, policies, queries, read-models, specifications, factories.
2. **Blast radius: cross-crate, end-to-end.** `Command` is bound to
   `ApplicationLayer` in `ddd`, so `Command`/`Query` structs live in
   `hotkey-editor`, not the domain crate. The refactor reaches into the
   frontend's `services/` layer and the Playwright e2e.
3. **DomainEvents are transient, in-process.** Raised during a mutation,
   consumed by Policies in the same commit tick, then discarded. **No event
   store.** localStorage-materialized text stays the single source of truth —
   hard rules **R1/R2/R5/R6/R7 are unchanged**. (`Projection`, `EventStore`,
   `Saga`, `UnitOfWork` stay deliberately unused, consistent with the `ddd`
   crate's own design.)
4. **Deliverable: committed spec → phased implementation plan.** No production
   code until the plan is approved.

## 4. DDD role map (`warcraft-keybinds`, DomainLayer)

Every type gets a compiler-checked role from the `ddd` vocabulary:

| `ddd` role | Types |
|---|---|
| **AggregateRoot** | `CustomKeys`, `GridLayout`, `EditorHistory` (already marked — keep) |
| **Entity** (`identity()`) | `AbilityBinding`, `CommandBinding`, `SystemBinding` — identity = `WarcraftObjectId`; a binding stays "the same" as its hotkey/position change |
| **Identifier** | `AbilityId`, `GridSlotId`, `CommandCard`, `HotkeyTarget` |
| **ValueObject** | `Hotkey`, `AbilityModifier`, `HotkeyToken`; keycode enums (`Letter`, `Digit`, `FunctionKey`, `NumpadKey`, `Punctuation`, `MouseButton`, `KeyCode`); `NamedCommandGrid`, `GridRole`; cascade VOs (`PositionAssignmentGroup`, `GroupKind`, `AssignmentScope`, `PlannedMove`, `UnresolvedMover`, `MoveReason`, `CascadePlan`); collision cards (`CollisionSlots`, `PositionCollisionCard`, `HotkeyCollisionCard`, `HotkeyCollisionAtCell`, `SharedAbilityEntry`, `AffectedUnitEntry`, `UnitCollisionEntry`); the `statistics/` value types |
| **DomainService** | `UnitCollisionScan`, `CrossUnitCollisionScan` (collision detection); the cascade solver + `CascadeResolver` driver (extracted from `custom_keys.rs`); `Normalization` (materialize/mirror/prune pipeline); `SectionResolution`; `CustomKeysCodec` (parse/serialize) |
| **Specification** | `HiddenAbility`, `RootedOnlyAbility`, `RevertsToHost`, `FormUpgradeSwap` (from the `unit/slots.rs` predicate free-functions) — composable via `.and()/.or()/.not()` |
| **Factory** | `CustomKeysFactory` (fresh aggregate from default text), `UnitGridsFactory` (per-unit grids from the game database) |
| **DomainEvent** (transient) | `SlotMoved`, `HotkeyRebound`, `PositionAssigned`, `GridLayoutApplied`, `CollisionIntroduced`, `CollisionResolved` |
| **Policy** (in-tick) | `RewriteHotkeyOnMove`, `MirrorOffState`, `TriggerCascadeOnConflict` — formalizing side-effects currently inlined in `normalize`/mutations |
| **ReadModel** | `UnitCollisionReport`, `CrossUnitCollisionReport`, `CollisionSummary`, `UnitGrids`, `UnitListing`, the `display/` view builders — computed on demand from the aggregate (not projected, since events are transient) |

External types (`WarcraftObjectId`, `GridCoordinate`, `RowIndex`,
`ColumnIndex` from `warcraft_api`; `CommandCatalog`, `BuildingTraits` from
`warcraft_database`) cannot carry our markers; they are consumed as-is.

## 5. Structural decomposition

Apply the hotkey-editor discipline (from `docs/COMPONENTS.md`, generalized to
a pure-domain crate):

- **directory == primary type**; **one type per small file**;
- **the call tree is the directory tree** — a helper type used by exactly one
  parent is a submodule of that parent, not a flat sibling;
- **`shared/` module** for a leaf used by 2+ siblings (decided by a count of
  use-sites);
- **no descendant re-export flattening** in intermediate `mod.rs` files; only
  the crate root `lib.rs` may curate a public facade with full-path `pub use`s;
- target: **no file over ~500 lines**, most far smaller.

Monolith breakdowns:

- **`custom_keys.rs`** → `custom_keys/`:
  - `aggregate.rs` — `CustomKeys` root + `BTreeMap` accessors
  - `codec/` — `parser.rs` (`CustomKeysParser`, `from_text`, `parse_raw`),
    `serializer.rs` (`Display`, `write_section` orchestration),
    `section.rs` (`SectionKind`, `SectionResolution`, `BindingFieldKey`,
    `SectionAccumulator` — merged from `model.rs`)
  - `normalize/` — `normalization.rs` (`Normalization` service driver) plus
    one file per rule: `materialize_defaults.rs`, `mirror_build_commands.rs`
    (`BuildCommandMirror` + table), `mirror_morph_abilities.rs`
    (`MorphAbilityMirror` + table), `prune.rs`, `upgrade_tiers.rs`
  - `mutate/` — `assign_position.rs`, `move_slot.rs` (the dense co-move/swap
    block), `set_hotkey.rs`, `apply_grid.rs`, `system.rs`
  - `import/` — `import_outcome.rs`, overlay `Extend`/`IntoIterator`
  - `events/`, `policies/`
  - `hotkey_conflict.rs`
- **`cascade/queue.rs`** → `cascade/queue/`: `assignment_queue.rs`,
  `group.rs` (`PositionAssignmentGroup`, `GroupKind`, `AssignmentScope`),
  `raster_sweep.rs` (phase-1 `QueueBuildState`), `spill_solver.rs` (phase-2),
  `fight_decomposition.rs` (connected-component/anchor), `debug_view.rs`
  (`Display`).
- **`cascade/planner.rs`** → `cascade/planner/`: `cascade_plan.rs`,
  `planned_move.rs`, `unresolved_mover.rs`, `move_reason.rs`,
  `reason_inference.rs` (`move_reason_for_node`/`from_group`).
- **`collision/unit_report.rs`** → thin `report.rs` read model; codegen dumps
  → `examples/`, cross-cutting tests → `tests/`.
- **`collision/cross_unit.rs`** → `cross_unit/`: `cross_unit_scan.rs`
  (`compute`), `report.rs` (`CrossUnitCollisionReport` + entries),
  `island_partition.rs` (`SlotIslandPartition` union-find, extracted).
- **`model.rs`** → `model/`: `hotkey/` (`Hotkey`, `AbilityModifier`, parse
  errors — sited under `identity/` if that reads better),
  `ability_binding.rs`, `command_binding.rs`, `system_binding.rs`,
  `warcraft_keybinding.rs`, `entries.rs` (`BindingEntry`, `CommandEntry`),
  `builders/`. Parse/serialize guts move to `custom_keys/codec/`.
- **`unit/slots.rs`** → `unit/slots/`: `command_slots.rs` (thin card
  assembly) + `rules/` (predicates as Specifications).
- **`unit/grids.rs`** → `unit/grids/`: `unit_grids.rs` + collision-card value
  objects/iterators split out (`collision_cards.rs` or per-card files).
- **`identity/keycode.rs`** → `identity/keycode/`: `key_code.rs` (umbrella +
  conversions), `letter.rs`, `digit.rs`, `function_key.rs`, `numpad_key.rs`,
  `punctuation.rs`, `mouse_button.rs`, `out_of_range.rs`, `not_a_letter.rs`.

`statistics/`, `text/`, and the already-small parts of `identity/` are the
model to follow — they only need derives/marker impls, not restructuring.

## 6. CQRS application half (`hotkey-editor`)

The domain crate raises events and exposes mutation methods; the *named
intentions* live application-side, where `Command` legally claims
`ApplicationLayer`.

- One `Command<CustomKeys>` struct per mutation in
  `hotkey-editor/src/services/customkeys/commands/`: `SetHotkey`, `MoveSlot`,
  `AssignPosition`, `ApplyGridLayout`, `ApplyTemplate`, `ReplaceWithUploaded`,
  `ClearOverride`, `SetSystemHotkey`, `SwapSystemBindings`. Each
  `execute(self, &mut CustomKeys)` calls the corresponding domain method and
  returns the raised events as its `Outcome`.
- `CustomKeysService` exposes a single mutation path: `dispatch(command)`. The
  write-through `commit` (snapshot → mutate → **normalize** → save → replace)
  is unchanged; `dispatch` routes a `Command` through it. This *strengthens*
  the wall: the renderer can no longer call a bare `keys.set_hotkey(...)`; it
  constructs a named `Command` and dispatches it.
- Domain mutation methods stay `pub` (a cross-crate `Command` calls them). The
  "only Commands mutate" rule is enforced by the service funnel, exactly as
  the layer discipline intends.
- Read side: report requests become `Query` types (`UnitCollisionsQuery`,
  `CrossUnitCollisionsQuery`) resolved against the aggregate's
  `ReadModel`-returning domain services.

Frontend blast radius: `services/customkeys/`, the mutation call sites, and
the Playwright e2e (behavior is identical, so e2e stays green as a regression
guard).

## 7. Testing strategy

- **Unit tests that touch private internals travel with their code.** When a
  monolith splits into a directory, each small file carries its own focused
  `#[cfg(test)] mod` — preserving `pub(crate)` visibility.
- **Cross-cutting tests using only the public API** move to
  `crates/warcraft-keybinds/tests/`, grouped by subsystem (`cascade.rs`,
  `collision.rs`, `normalization.rs`).
- **Codegen "dump" tests** (template regenerator, collision-report
  builder-code dumper) move to `examples/` (or an ignored dev tool) — they are
  generators, not tests.
- **Every new role marker ships with a compile-time assertion** mirroring the
  crate's existing `assert_domain_aggregate::<T>()` pattern:
  `assert_value_object::<Hotkey>()`, `assert_domain_service::<CascadeResolver>()`,
  layer-exclusivity `compile_fail` guards.
- **R9 holds throughout:** each phase ends `moon run :ci` green, including e2e.

## 8. Docs

- **Update `docs/ARCHITECTURE.md` §5–§6** — its refactor plan describes a
  `lib.rs` split that already happened and is now misleading. Rewrite to
  reflect the current + target domain-crate shape.
- **Author `docs/DOMAIN.md`** — the domain-crate analogue of `COMPONENTS.md`:
  codifies the DDD role map, directory==type / one-type-per-file, "call tree
  is the directory tree", and the marker-assertion convention. This is the
  deliverable matching the repo's meta-goal of migrating prose rules into
  compile-enforced conventions.

## 9. Phasing (each phase ends `moon run :ci` green)

| Phase | Content | Risk |
|---|---|---|
| **0. Guardrails & docs** | Update `ARCHITECTURE.md`; draft `DOMAIN.md`; add the marker-assertion test harness | none |
| **1. Style sweep** | Crate-wide mechanical fixes: derives, `Self`, `verb_noun`→methods, `pub`→accessors, single-letter closures, tuples, `as` | low |
| **2. Test/codegen extraction** | Move giant inline blocks to `tests/`; codegen dumps to `examples/` — shrinks monoliths before splitting | low |
| **3. Value objects & identity** | Split `keycode/`, `model/hotkey/`; mark `ValueObject`/`Identifier`/`Entity` | low–med |
| **4. Domain services & codec** | Extract `codec/`, `normalize/`, cascade solver/`resolver`, collision scans, union-find; mark `DomainService`. Sub-sliced per subsystem (codec → normalize → cascade → collision), each green independently | **high** |
| **5. Specifications & Factories** | `unit/slots` predicates → Specifications; `CustomKeysFactory`, `UnitGridsFactory` | med |
| **6. CQRS domain half** | DomainEvents + Policies + `ReadModel` markers | med |
| **7. CQRS application half** | `Command`/`Query` structs in `hotkey-editor`; `dispatch` wiring; e2e | **high** (cross-wall) |
| **8. Facade & final pass** | Curate `lib.rs`; de-flatten intermediate `mod.rs`; final RUST_STYLE + `DOMAIN.md` conformance | low |

## 10. Non-goals / guardrails

- **No persistence-model change.** Text-as-truth stays; no event store, no
  event-sourcing, no export re-derivation (R1/R2/R5/R6/R7 unchanged).
- **The wall stays intact.** No domain logic moves into `hotkey-editor`; only
  the application-layer `Command`/`Query` shells live there. `warcraft-keybinds`
  keeps zero browser/wasm/dioxus deps (R8) and its allowed dep set.
- **No behavior change** the renderer or e2e can observe. This is a structural
  + type-role refactor; the collision/cascade/normalization outputs are
  byte-identical before and after each phase.
- **No scope creep into `warcraft-database`/`warcraft-api`.** Trimming game-data
  concerns into those crates is explicitly out of scope for this effort.
