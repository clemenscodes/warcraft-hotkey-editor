# Race Scoped Search, Domain Change Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Teach the unit listing in `warcraft-api` to respect a chosen set of races during a search, instead of dropping the race and searching across every race.

**Architecture:** Replace the single `Option<Race>` on the listing query with a `RaceSelection` value object that is either every race or a named set. A browse builds the selection from the one active race, a search builds it from the scope set. The one line that discarded the race during a search now passes the scope through.

**Tech Stack:** Rust, the `warcraft-api` crate inside the `warcraft-data` repository, plain `cargo test`.

## Global Constraints

- This plan changes the external crate at `/home/clemens/.local/src/warcraft-data`, not the `hotkey-editor` repository. Every path below is relative to `/home/clemens/.local/src/warcraft-data`.
- `hotkey-editor`'s workspace `Cargo.toml` already carries a `[patch]` pointing `warcraft-api` and `warcraft-keybinds` at this local working copy, so the change is picked up without a tag round trip. The change ships later by tagging `warcraft-data` and bumping the tags in `hotkey-editor`'s `Cargo.toml`. Do not commit that patch into `hotkey-editor`, it breaks CI.
- Do not tag or bump versions in this plan. The renderer plan needs the new types too, so the tag bump happens once after both plans land.
- Rust style, from `docs/RUST_STYLE.md` in the hotkey-editor repo. Full semantic names, no abbreviations. No tuples in any form, enum variants carry named fields. Use `Self` inside `impl` blocks. Derive every standard trait the type qualifies for. Private struct fields with accessors. No `as` casts outside `From`/`TryFrom` bodies. Implement the idiomatic standard trait when one fits, `Default`, `From`, `Display`.
- The default of `RaceSelection` is every race, so an untouched search behaves exactly as it does today, across every race.
- `Race` derives `Ord`, so an ordered `BTreeSet<Race>` is the set carrier. `Race` does not derive `Hash`, so a `HashSet` would not compile.
- Run tests from the `warcraft-data` dev shell. If `cargo` is not on the path, enter it first with `nix develop /home/clemens/.local/src/warcraft-data`.
- End every commit message with the trailer `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`. Commit in the `warcraft-data` repository.

---

### Task 1: The `RaceSelection` value object

**Files:**
- Create: `crates/warcraft-api/src/application/unit/listing/race_selection/mod.rs`
- Modify: `crates/warcraft-api/src/application/unit/listing/mod.rs` (add the module declaration)

**Interfaces:**
- Produces: `RaceSelection`, an enum with variants `All` and `Only { races: BTreeSet<Race> }`. Constructors `RaceSelection::only(race: Race) -> Self` and `RaceSelection::of(races: impl IntoIterator<Item = Race>) -> Self`. Predicate `RaceSelection::admits(&self, race: Option<Race>) -> bool`. Query `RaceSelection::is_empty(&self) -> bool`. `Default` yields `All`.

- [ ] **Step 1: Declare the module**

In `crates/warcraft-api/src/application/unit/listing/mod.rs`, add the module next to the existing `pub(crate) mod mode_selection;` line so the block reads:

```rust
pub(crate) mod browse;
pub(crate) mod index;
pub(crate) mod mode_selection;
pub(crate) mod placeholder;
pub(crate) mod query;
pub(crate) mod race_selection;
pub(crate) mod search;
pub(crate) mod search_field;
pub(crate) mod sort;
pub(crate) mod suppress;
pub(crate) mod visibility;
```

- [ ] **Step 2: Write the value object with its failing tests**

Create `crates/warcraft-api/src/application/unit/listing/race_selection/mod.rs`:

```rust
//! [`RaceSelection`]: which races a listing admits. A browse narrows to the one
//! active race; a search scopes across a chosen set, defaulting to every race.
//! A query input, so it lives in the application layer with named fields, never
//! a positional flag.

use crate::domain::race::Race;
use std::collections::BTreeSet;

/// The races a listing admits. `All` admits every object, including the rare
/// object that has no race. `Only` admits an object only when its race is one of
/// the named set, so a raceless object is excluded, exactly as a single-race
/// browse excluded it before.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RaceSelection {
    /// Every race, and every raceless object. The default and the widest scope.
    All,
    /// Only the named races.
    Only { races: BTreeSet<Race> },
}

impl RaceSelection {
    /// Only the one race. The scope a browse uses.
    pub fn only(race: Race) -> Self {
        let mut races = BTreeSet::new();
        races.insert(race);
        Self::Only { races }
    }

    /// Only the named races, gathered from any iterator. An empty iterator
    /// yields an `Only` that admits nothing.
    pub fn of(races: impl IntoIterator<Item = Race>) -> Self {
        let races: BTreeSet<Race> = races.into_iter().collect();
        Self::Only { races }
    }

    /// Whether the selection admits an object with this race. A raceless object
    /// (`None`) is admitted only by `All`.
    pub fn admits(&self, race: Option<Race>) -> bool {
        match self {
            Self::All => true,
            Self::Only { races } => match race {
                Some(race) => races.contains(&race),
                None => false,
            },
        }
    }

    /// Whether this selection names no race at all, and so admits nothing.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::All => false,
            Self::Only { races } => races.is_empty(),
        }
    }
}

impl Default for RaceSelection {
    /// Every race, the scope a fresh search opens on.
    fn default() -> Self {
        Self::All
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_is_all_races() {
        let selection = RaceSelection::default();
        assert_eq!(selection, RaceSelection::All);
    }

    #[test]
    fn all_admits_every_race_and_the_raceless() {
        let selection = RaceSelection::All;
        assert!(selection.admits(Some(Race::Human)));
        assert!(selection.admits(Some(Race::Neutral)));
        assert!(
            selection.admits(None),
            "an object with no race is part of a cross-race search"
        );
    }

    #[test]
    fn only_admits_the_named_race_and_rejects_the_others() {
        let selection = RaceSelection::only(Race::Nightelf);
        assert!(selection.admits(Some(Race::Nightelf)));
        assert!(!selection.admits(Some(Race::Human)));
    }

    #[test]
    fn only_rejects_a_raceless_object() {
        let selection = RaceSelection::only(Race::Nightelf);
        assert!(
            !selection.admits(None),
            "a named race scope excludes objects that have no race, as a single-race browse did"
        );
    }

    #[test]
    fn of_gathers_several_races() {
        let selection = RaceSelection::of([Race::Human, Race::Undead]);
        assert!(selection.admits(Some(Race::Human)));
        assert!(selection.admits(Some(Race::Undead)));
        assert!(!selection.admits(Some(Race::Orc)));
    }

    #[test]
    fn an_empty_only_admits_nothing() {
        let selection = RaceSelection::of([]);
        assert!(selection.is_empty());
        assert!(!selection.admits(Some(Race::Human)));
    }

    #[test]
    fn all_is_never_empty() {
        let selection = RaceSelection::All;
        assert!(!selection.is_empty());
    }
}
```

- [ ] **Step 3: Run the tests to verify they pass**

Run: `cargo test -p warcraft-api race_selection`
Expected: PASS, six tests in the `race_selection` module.

- [ ] **Step 4: Commit**

```bash
cd /home/clemens/.local/src/warcraft-data
git add crates/warcraft-api/src/application/unit/listing/race_selection/mod.rs crates/warcraft-api/src/application/unit/listing/mod.rs
git commit -m "Add RaceSelection value object for the unit listing

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 2: Carry the selection on `UnitQuery` and apply it in the filter

**Files:**
- Modify: `crates/warcraft-api/src/application/unit/listing/query/mod.rs`
- Modify: `crates/warcraft-api/src/application/unit/listing/mod.rs` (the `evaluate` predicate and the in-module test constructors)

**Interfaces:**
- Consumes: `RaceSelection` from Task 1.
- Produces: `UnitQuery.races: RaceSelection` replaces `UnitQuery.race: Option<Race>`. The listing filter admits an object when `query.races.admits(object.race())`.

- [ ] **Step 1: Replace the field on `UnitQuery`**

In `crates/warcraft-api/src/application/unit/listing/query/mod.rs`, change the imports and the field. Remove `use crate::domain::race::Race;` and add the selection import, then swap the field.

Imports become:

```rust
use crate::application::unit::listing::mode_selection::UnitModeSelection;
use crate::application::unit::listing::race_selection::RaceSelection;
use crate::application::unit::listing::search_field::SearchField;
use crate::application::unit::listing::visibility::CatalogVisibility;
use crate::domain::unit::UnitKind;
```

The struct field changes from:

```rust
    /// Restrict to one race (all races when `None`).
    pub race: Option<Race>,
```

to:

```rust
    /// Which races this listing admits.
    pub races: RaceSelection,
```

- [ ] **Step 2: Update the filter predicate in `evaluate`**

In `crates/warcraft-api/src/application/unit/listing/mod.rs`, add the selection import near the other `use` lines at the top of the file:

```rust
use race_selection::RaceSelection;
```

Then, in the `evaluate` function, replace this block:

```rust
    if let Some(race) = query.race
        && object.race() != Some(race)
    {
        return None;
    }
```

with:

```rust
    let object_race = object.race();
    if !query.races.admits(object_race) {
        return None;
    }
```

- [ ] **Step 3: Update the in-module test constructors**

Still in `crates/warcraft-api/src/application/unit/listing/mod.rs`, the `#[cfg(test)]` module builds `UnitQuery` literals with `race: Some(...)`. Add `use race_selection::RaceSelection;` inside the test module's `use` block if it is not already reachable, then change each `race:` field.

At the `browse` test helper (near the top of the test module), change:

```rust
        let query = UnitQuery {
            race: Some(race),
            ...
        };
```

to:

```rust
        let query = UnitQuery {
            races: RaceSelection::only(race),
            ...
        };
```

At the position-cascade test that lists a specific race, change:

```rust
            race: Some(Race::Human),
```

to:

```rust
            races: RaceSelection::only(Race::Human),
```

At the chains test that loops over `(race, chain)`, change:

```rust
                race: Some(race),
```

to:

```rust
                races: RaceSelection::only(race),
```

Search the test module for any remaining `race: Some` and `race: None` on a `UnitQuery` literal. Replace `race: None` with `races: RaceSelection::All`, and `race: Some(x)` with `races: RaceSelection::only(x)`.

- [ ] **Step 4: Run the tests to verify the migration compiles and passes**

Run: `cargo test -p warcraft-api listing`
Expected: PASS. The listing and browse tests still pass, because a browse narrows to `RaceSelection::only(active_race)`, which admits exactly what `Some(race)` admitted.

- [ ] **Step 5: Commit**

```bash
cd /home/clemens/.local/src/warcraft-data
git add crates/warcraft-api/src/application/unit/listing/query/mod.rs crates/warcraft-api/src/application/unit/listing/mod.rs
git commit -m "Filter the unit listing by a RaceSelection instead of one Option race

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 3: Respect the scope in the browse requests

**Files:**
- Modify: `crates/warcraft-api/src/application/unit/listing/browse/mod.rs`

**Interfaces:**
- Consumes: `RaceSelection` from Task 1, `UnitQuery.races` from Task 2.
- Produces: `UnitListingRequest` and `UnitCategoryRequest` each gain a private `search_race_scope: RaceSelection`, default `All` from `new`, set by `searching_within(self, scope: RaceSelection) -> Self`. A search now maps the scope through instead of forcing every race.

- [ ] **Step 1: Write the failing tests that prove a search respects the scope**

In `crates/warcraft-api/src/application/unit/listing/browse/mod.rs`, add these tests to the existing `#[cfg(test)] mod tests` block. They use the module's existing `search_request` and `full_visibility` helpers and the `RaceSelection` in scope.

```rust
    #[test]
    fn a_search_scoped_to_one_race_drops_the_other_races_hits() {
        let across_all = search_request("a");
        let human_only = search_request("a").searching_within(RaceSelection::only(Race::Human));
        let all_count: usize = UnitCatalogListing::from(&across_all)
            .groups()
            .iter()
            .map(|group| group.entries().len())
            .sum();
        let human_count: usize = UnitCatalogListing::from(&human_only)
            .groups()
            .iter()
            .map(|group| group.entries().len())
            .sum();
        assert!(
            human_count > 0,
            "a human-scoped search for a common letter must still find human units"
        );
        assert!(
            human_count < all_count,
            "scoping a cross-race search to one race must drop the other races' hits; \
             equal counts would mean the scope was ignored, the bug this fixes"
        );
    }

    #[test]
    fn a_default_scope_search_still_spans_more_than_one_race() {
        let across_all = search_request("a");
        let human_only = search_request("a").searching_within(RaceSelection::only(Race::Human));
        let all_count: usize = UnitCatalogListing::from(&across_all)
            .groups()
            .iter()
            .map(|group| group.entries().len())
            .sum();
        let human_count: usize = UnitCatalogListing::from(&human_only)
            .groups()
            .iter()
            .map(|group| group.entries().len())
            .sum();
        assert!(
            all_count > human_count,
            "the default scope is every race, so it must find more than one race can alone"
        );
    }
```

- [ ] **Step 2: Run the tests to verify they fail to compile**

Run: `cargo test -p warcraft-api browse`
Expected: FAIL to compile, `no method named searching_within`.

- [ ] **Step 3: Add the scope field and setter to `UnitListingRequest`**

In `crates/warcraft-api/src/application/unit/listing/browse/mod.rs`, add the import at the top with the other `use crate::{...}` lines:

```rust
use crate::application::unit::listing::race_selection::RaceSelection;
```

Add the field to the struct:

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitListingRequest {
    race: Race,
    modes: UnitModeSelection,
    search_query: String,
    search_field: SearchField,
    visibility: CatalogVisibility,
    search_race_scope: RaceSelection,
}
```

Set the default in `new`, leaving the public signature untouched so today's callers keep working:

```rust
    pub fn new(
        race: Race,
        modes: UnitModeSelection,
        search_query: String,
        search_field: SearchField,
        visibility: CatalogVisibility,
    ) -> Self {
        Self {
            race,
            modes,
            search_query,
            search_field,
            visibility,
            search_race_scope: RaceSelection::All,
        }
    }

    /// Narrow the search to a set of races. A browse ignores this, it always
    /// lists the one active race. The default, when this is not called, is every
    /// race, so a search behaves as it did before.
    pub fn searching_within(mut self, search_race_scope: RaceSelection) -> Self {
        self.search_race_scope = search_race_scope;
        self
    }
```

- [ ] **Step 4: Map the scope through `catalog_query`**

Replace the body of `catalog_query`:

```rust
    fn catalog_query(&self) -> UnitQuery<'_> {
        let searching = self.is_searching();
        let races = if searching {
            self.search_race_scope.clone()
        } else {
            RaceSelection::only(self.race)
        };
        let scope = if searching {
            Scope::Search {
                field: self.search_field,
                query: self.search_query.as_str(),
            }
        } else {
            Scope::Browse { modes: self.modes }
        };
        UnitQuery {
            races,
            kind: None,
            visibility: self.visibility,
            scope,
        }
    }
```

- [ ] **Step 5: Add the same scope field to `UnitCategoryRequest`**

Add the field to the `UnitCategoryRequest` struct:

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitCategoryRequest {
    race: Race,
    modes: UnitModeSelection,
    category_kind: UnitKind,
    search_query: String,
    search_field: SearchField,
    visibility: CatalogVisibility,
    search_race_scope: RaceSelection,
}
```

Default it in `UnitCategoryRequest::new`, again leaving the signature untouched:

```rust
    pub fn new(
        race: Race,
        modes: UnitModeSelection,
        category_kind: UnitKind,
        search_query: String,
        search_field: SearchField,
        visibility: CatalogVisibility,
    ) -> Self {
        Self {
            race,
            modes,
            category_kind,
            search_query,
            search_field,
            visibility,
            search_race_scope: RaceSelection::All,
        }
    }

    /// Narrow the search to a set of races, as [`UnitListingRequest::searching_within`].
    pub fn searching_within(mut self, search_race_scope: RaceSelection) -> Self {
        self.search_race_scope = search_race_scope;
        self
    }
```

Update the inline query build in `impl From<&UnitCategoryRequest> for UnitCategoryListing`, replacing:

```rust
        let query = UnitQuery {
            race: if searching { None } else { Some(request.race) },
            kind: Some(request.category_kind),
            ...
        };
```

with:

```rust
        let races = if searching {
            request.search_race_scope.clone()
        } else {
            RaceSelection::only(request.race)
        };
        let query = UnitQuery {
            races,
            kind: Some(request.category_kind),
            visibility: request.visibility,
            scope: if searching {
                Scope::Search {
                    field: request.search_field,
                    query: request.search_query.as_str(),
                }
            } else {
                Scope::Browse {
                    modes: request.modes,
                }
            },
        };
```

- [ ] **Step 6: Run the tests to verify they pass**

Run: `cargo test -p warcraft-api browse`
Expected: PASS, including the two new scope tests and every existing browse test.

- [ ] **Step 7: Commit**

```bash
cd /home/clemens/.local/src/warcraft-data
git add crates/warcraft-api/src/application/unit/listing/browse/mod.rs
git commit -m "Respect the search race scope in the listing requests

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 4: Publish `RaceSelection` from the crate root

**Files:**
- Modify: `crates/warcraft-api/src/lib.rs`

**Interfaces:**
- Produces: `warcraft_api::RaceSelection`, so the renderer plan can name it in `UnitFilterQuery`.

- [ ] **Step 1: Add the re-export beside its sibling**

In `crates/warcraft-api/src/lib.rs`, line 9 reads `pub use application::unit::listing::mode_selection::UnitModeSelection;`. Directly under it, add the matching line for the selection:

```rust
pub use application::unit::listing::race_selection::RaceSelection;
```

- [ ] **Step 2: Verify the whole crate builds and every test passes**

Run: `cargo test -p warcraft-api`
Expected: PASS, the full suite, with no unused-import or dead-code warnings from the migration. If clippy is part of the repo's gate, also run `cargo clippy -p warcraft-api --all-targets` and expect no warnings.

- [ ] **Step 3: Commit**

```bash
cd /home/clemens/.local/src/warcraft-data
git add crates/warcraft-api/src/lib.rs
git commit -m "Export RaceSelection from the warcraft-api crate root

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

## After this plan

The domain now respects a race scope during a search, and `warcraft_api::RaceSelection` is public. The `hotkey-editor` build will not compile against this until the renderer plan lands, because `UnitFilterQuery` does not yet pass a scope, but that is expected between the two plans, and the default scope of `All` means the moment the renderer passes nothing, the search still spans every race.

Do not tag `warcraft-data` yet. The renderer plan, authored just-in-time against these real types, consumes `RaceSelection` and adds the multi-select control. The single `warcraft-data` tag and the `Cargo.toml` bump in `hotkey-editor` happen once, after both plans are green and the local `[patch]` is removed.

## Self-review notes

- Spec coverage. This plan covers the spec's domain-core section, the `RaceSelection` value object, the `catalog_query` line that dropped the race, the inverted tests, and the public export. The renderer section, the `UnitFilterQuery` scope field, the configuration dropdown, the control forms, and the desktop boundary are the separate renderer plan.
- The two race concepts stay separate here, a browse maps `RaceSelection::only(active_race)`, a search maps the scope, so navigation race and search scope never share a value.
