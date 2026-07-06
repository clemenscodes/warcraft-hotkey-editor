# The domain crate

This document is to `warcraft-keybinds` what `docs/COMPONENTS.md` is to
`hotkey-editor`: the **structural and DDD contract** for the pure-domain
crate. `ARCHITECTURE.md` says *where* code lives (the wall between renderer
and domain crate); `RUST_STYLE.md` says *how* Rust is written; this document
says *how the domain crate is modelled and laid out*.

All three are mandatory reading before a non-trivial change to
`warcraft-keybinds`.

---

## 1. The crate is the model

`warcraft-keybinds` is a `ddd::BoundedContext`: one model, one ubiquitous
language, no browser dependencies (R8). Every type in it earns a role from the
`ddd` vocabulary crate, and that role is proven at compile time. A type with
no role is a smell — either it belongs to a role and should say so, or it
belongs in `warcraft-database`/`warcraft-api`.

## 2. The DDD role map

Every marked domain type carries exactly one primary `ddd` role, proven by a
`ddd_conformance` assertion. This table reflects the **adopted** state (~70
conformance assertions across the crate):

| `ddd` role | Layer | Types |
|---|---|---|
| **AggregateRoot** | Domain | `CustomKeys`, `GridLayout`, `EditorHistory` |
| **Identifier** | Domain | `AbilityId`, `GridSlotId` |
| **ValueObject** | Domain | `Hotkey`, `AbilityModifier`, `HotkeyToken`, `HotkeyTarget`, `CommandCard`; the keycode enums (`Letter`, `Digit`, `FunctionKey`, `NumpadKey`, `Punctuation`, `MouseButton`, `KeyCode`); the cascade value objects (`PositionAssignmentGroup`, `GroupKind`, `AssignmentScope`, `PlannedMove`, `UnresolvedMover`, `MoveReason`, `CascadePlan`, `ConflictNode`, `CollidingPair`, `ConflictGraph`) |
| **DomainService** | Domain | `AssignmentQueue` (the cascade solver) |
| **Specification** | Domain | the `unit/ability_rules` predicates: `HiddenAbility`, `RootedOnlyAbility`, `RevertsToHost`, `FormUpgradeSwap` (composable via `.and()/.or()/.not()`) |
| **ReadModel** | Domain | the collision reports (`UnitCollisionReport`, `CrossUnitCollisionReport`, `CollisionSummary`), `UnitGrids`, `UnitListing`, the `display/` view builders |

The application-layer `Command`/`Query` counterparts live across the wall in
`hotkey-editor` (a `Command` is bound to `ApplicationLayer` and cannot live
here without claiming the wrong layer).

### Domain-service *logic* without a marked service type

The normalization pipeline (`custom_keys/normalize.rs`), the parse/serialize
codec (`custom_keys/parser.rs`, `custom_keys/overlay.rs`), and the collision
scans (`collision/`) are all **extracted into dedicated modules**, but they
operate on the aggregate as `impl CustomKeys` continuations / free functions
rather than standalone marked `DomainService` types. That is deliberate: the
logic belongs to the aggregate's invariant, and inventing an empty service
struct to carry a marker would be ceremony. Only `AssignmentQueue`, which is a
genuinely free-standing algorithm object, carries the `DomainService` marker.

### Deliberately deferred roles (with rationale)

These roles from the design spec are **not** adopted, each for a concrete
reason — consistent with the `ddd` crate's "adopt a role when the concept
genuinely appears" philosophy and the repo's "resist premature
framework-ization" principle:

- **Entity** (bindings `AbilityBinding`/`CommandBinding`/`SystemBinding`): a
  binding does **not** store its own identity — the `WarcraftObjectId` is the
  aggregate's `BTreeMap` key, external to the binding. `ddd::Entity` requires
  `fn identity(&self) -> &Self::Identity` returning a stored reference. Marking
  them Entity needs a structural change (embedding the id in each binding),
  deferred to a focused later pass.
- **`statistics/` value types** (`Armor`, `AttackSpeed`, `Matchup`, …): they
  hold `f32` and so cannot be `Eq`, which `ddd::ValueObject` requires. They
  remain plain, correct value types without the marker.
- **Factory** (`CustomKeysFactory`, `UnitGridsFactory`): `CustomKeys::from_text`
  already cannot produce an invalid aggregate (it normalizes), so a factory
  would add indirection with no invariant to enforce.
- **DomainEvent / Policy**: the intended reactions (mirror off-state, materialize
  tiers, mirror build commands) turned out to be **deterministic steps of the
  normalization pipeline**, not decoupled event handlers. Introducing transient
  events + policies here would be indirection without decoupling. They are
  adopted the day a genuinely decoupled reaction appears.
- **Command / Query** (application layer): these belong in `hotkey-editor`, and
  that crate is out of scope for this domain-crate refactor.

`Projection`, `EventStore`, `Saga`, and `UnitOfWork` likewise stay unused — the
`ddd` vocabulary is pre-built and roles are adopted as the concepts arise.

## 3. The marker-assertion convention

Every marked type ships a compile-time proof of its role in a `#[cfg(test)]`
module, using the generic helpers in `src/ddd_conformance.rs`:

```rust
#[cfg(test)]
mod ddd_marker_tests {
    use crate::ddd_conformance::assert_value_object;
    use super::Hotkey;

    #[test]
    fn hotkey_is_a_value_object() {
        assert_value_object::<Hotkey>();
    }
}
```

The helpers (`assert_value_object`, `assert_identifier`, `assert_entity`,
`assert_domain_service`, `assert_factory`, `assert_specification`,
`assert_read_model`) each bound their type parameter by the corresponding
`ddd` trait, so the call compiles only if the role genuinely holds. This
mirrors the pre-existing `assert_domain_aggregate` proof on the three
aggregate roots. Adding a role marker without its assertion is incomplete.

Layer exclusivity is enforced by `ddd` itself: `Layered` is single-valued, so
a type cannot claim two layers, and a `compile_fail` doctest in the `ddd`
crate guards it. A domain type therefore cannot accidentally become an
application service.

## 4. Structural rules

Generalized from `COMPONENTS.md`, applied to a pure-domain crate:

- **Directory equals primary type.** A `snake_case` directory holds one
  `PascalCase` type as its reason to exist (`custom_keys/` → `CustomKeys`).
- **One type per small file.** Two distinct concepts in one file is the signal
  to split. Target: no file over ~500 lines; most far smaller (the role model,
  `hotkey-editor`, runs a median of 17 lines per file). `statistics/`, `text/`,
  and the small `identity/` files are the shape to match.
- **The call/composition tree is the directory tree.** A helper type used by
  exactly one parent is a submodule of that parent, not a flat sibling. Reaching
  a collaborator via `use super::...` that you then *use* is the tell of a
  misplacement — descend into children via `use crate::...`.
- **`shared/` for multi-consumer leaves.** A leaf used by two or more siblings
  moves to a `shared/` grouping module at the nearest common parent, decided by
  a count of use-sites, not by who "owns" it. `shared/mod.rs` carries only
  `pub mod` re-exports, no type of its own.
- **No descendant re-export flattening.** An intermediate `mod.rs` exposes only
  its own sibling surface (`pub use group::PositionAssignmentGroup;`), never its
  grandchildren. Only the crate root `lib.rs` may curate a public facade with
  full-path `pub use`s for external consumers (`hotkey-editor`, `gallery`).
- **Base + variants are flat siblings** under one plural group directory, since
  variants depend on the base, not the reverse.

## 5. Transient events, text-as-truth

DomainEvents in this crate are **transient and in-process**: an aggregate
mutation raises them, in-tick `Policy` reactions consume them in the same
commit, and they are then discarded. There is **no event store**. The
localStorage-materialized `CustomKeys.txt` text remains the single source of
truth (`ARCHITECTURE.md` R1/R2/R5); events model *side-effects within a
mutation*, never the persisted state. A `Policy` that needs to run after a
mutation (rewrite a hotkey on move, mirror an off-state, trigger a cascade on
a new conflict) reads the event and issues follow-up changes before the
`commit` normalizes and writes.

## 6. Non-goals

- **No persistence-model change.** No event sourcing, no export re-derivation.
  Export is `localStorage.getItem(KEY)` verbatim (R5).
- **No domain logic across the wall.** Only application-layer `Command`/`Query`
  shells live in `hotkey-editor`; every rule, cascade, and collision decision
  stays here.
- **No game-data logic migrating out** to `warcraft-database`/`warcraft-api`
  during the DDD refactor. That is a separate, later effort.
