# HANDOFF — cure the stringly-id virus (every id is a domain type, no exceptions)

An id is a **domain type**, everywhere, in every signature, in both repos. A string id
(`String`, `&str`, `&String`, `Cow<str>`, a `ref`, a `Vec<String>`/`HashMap<String,…>`/
`HashSet<String>` of ids) is a **bug** unless a string is *physically being written to or
read from* a serialized medium *at that exact line*. This is a virus: it spreads through
every function signature it touches. Cure it at the root (the domain functions), then let
the compiler force every caller to type up.

> **READ THIS TWICE BEFORE EDITING.** The previous attempt failed because the driver
> invented a "sanctioned `&str` boundary" for domain lookups and String props. **There is
> no such boundary.** Passing `id.value()` into *any* function means that function's
> signature is the bug. See "The two mistakes that spread the virus" below and do not
> repeat them.

---

## The law (absolute, both directions)

**Signatures are ALWAYS the domain id type.** A function, method, prop field, struct
field, enum variant, closure param, return type, tuple element, map key, or set element
that holds an id is typed as the id type (`WarcraftObjectId`, `AbilityId`, `HotkeyToken`,
`KeyCode`, `GridSlotId`, `HotkeyTarget`). No exception for "utility" fns, "lookup" fns,
"just a helper", "it's one line", or "it's internal".

A string touches an id in exactly two directions, each at exactly one kind of place:

1. **id → string (rendering out): only inside the RSX that writes to the DOM, or inside
   a serializer writing the URL / the persisted `CustomKeys.txt` text.** The `.value()`
   lives *in* the DOM attribute/text expression or *in* the serializer — never a line
   earlier, never to feed another function.
   - Good: `span { "{unit_id.value()}" }`, `"data-unit-id": unit_id.value()`.
   - Good: the router `Route` field builder and `CustomKeys` serialize (owned by the
     domain crate) — the URL and the on-disk text are string media by nature.
   - **Bad:** `let s = unit_id.value(); some_fn(s)` — `some_fn` is mis-typed. Fix `some_fn`.

2. **string → id (reading in): only at the DOM/URL input edge, resolved immediately.** A
   raw string arriving from a DOM event, a URL query param, or parsed persisted text is
   turned into an id *on the spot* by the single resolution entry (`WarcraftObjectId::try_from(&str)`
   / a `resolve_raw(&str) -> Option<WarcraftObjectId>`), then only the id flows onward.
   Never thread the raw string past the edge. `DecodedEditorNav::decode` already does this
   for `?unit=`; that is the pattern.

Everywhere between those two edges: **only the typed id exists.** No conversions, because
every id type is `Copy` — there is never a reason to `.clone()`, `.to_owned()`,
`.to_string()`, `.as_str()`, deref, or `.value()` an id except at the two edges above.

## The two mistakes that spread the virus (do NOT repeat)

1. **`.value()` (or `.value().to_string()`) to satisfy a wrong-typed signature.** e.g.
   `UnitDetailHeaderProps { unit_id: unit_id.value().to_string() }` because
   `UnitDetailHeaderProps.unit_id` is `String`; or `open_unit(unit_id.value())` because
   `open_unit` takes `&str`; or `map.get(id.value())` because the map is `HashMap<String,…>`.
   **The prop/fn/map is the bug — retype it to the id type; the `.value()` disappears.**
2. **Treating a domain lookup fn as a "resolution boundary."** `ObjectLookup::by_id(&str)`,
   `has_icon(&str)`, `for_unit(&str)`, `UnitSlotContainers::resolve(&str)` are NOT
   boundaries — they take an *already-known* id, so they take `WarcraftObjectId`. Only the
   ONE `&str`→id resolver (that inspects a genuinely external string) keeps a `&str`
   input, and it returns an id; nothing downstream is stringly.

## The id vocabulary (each `Copy`, in `warcraft-api` / `warcraft-keybinds`)

- **`WarcraftObjectId`** (units, abilities, objects) — inner `&'static str`. `.value() ->
  &'static str` (DOM/URL render ONLY). `WarcraftObjectId::new(&'static str)` for a static
  literal. For a runtime string → id, use the single resolver
  (`ObjectLookup::resolve_raw(&str) -> Option<WarcraftObjectId>` / `WarcraftObject::id()`
  after the one external lookup). Because the inner is `&'static str`, an id is only ever a
  catalog value — a runtime string that resolves to nothing is genuinely `None`, not a
  stringly fallback.
- **`AbilityId`**, **`HotkeyToken`**, **`KeyCode`**, **`GridSlotId`**, **`HotkeyTarget`** —
  same rule; grep `warcraft-keybinds/src/identity/` for each type's constructor / boundary
  accessor / `TryFrom`.

## Already done (the baseline this completes)

- **warcraft-data v0.3.0**: the static catalog stopped emitting `String` unit ids —
  `CatalogEntry::unit_id`, `UnitKindHelpers::default_unit_id_for`, `UnitListingEntry`/
  `UnitCategoryEntry::unit_id` return `WarcraftObjectId`. App pins v0.3.0.
- **App, partially typed (and partly leaking via `.value()` bridges that this handoff
  removes):** `ViewNavigationContext::selected_unit_id: Signal<Option<WarcraftObjectId>>`;
  `select_race`/`select_mode` cascade; the editor tab bar restructure; `editor_workspace`,
  `collisions_page`, `resolve_page` unit-id props typed — **but** with `.value()`/`.value().to_string()`
  bridges into still-stringly signatures (`open_unit(&str)`, `UnitDetailHeaderProps.unit_id:
  String`, `UnitSlotContainers::resolve(&str)`, `InspectorPanelInputs.host_unit_id: &str`,
  `HashMap<String,…>` tier-override keys, `grid_editor host_unit_id: String`, …). Those
  bridges are the remaining virus; they vanish when the signatures below are typed.

## The inventory below is a STARTING POINT, not the boundary

Do not trust it as complete. It was built by grep, and grep misses the tail — and a
mis-typed `pub fn foo(unit_id: &str)` still **compiles green**, so a passing `cargo test`
does NOT prove the domain is leak-free. The cure is worthless if it is 90%: one surviving
`&str` id signature keeps the virus alive and hides the UI leaks downstream of it.

**The domain pass is an EXHAUSTIVE AUDIT of the entire public string surface, not this
list.** For each of the three crates (`warcraft-api`, `warcraft-database`,
`warcraft-keybinds`):

1. Enumerate **every** public signature mentioning a string type — `pub fn` params AND
   returns, `pub struct` fields, `pub enum` variant payloads, associated fns, and every
   collection element/key — matching any of: `&str`, `String`, `&String`, `&'static str`,
   `Cow`, `Vec<String>`, `Vec<&'static str>`, `HashMap<String,…>`, `HashSet<String>`,
   `[&'static str]`, `-> String`, `-> &str`, `-> &'static str`.
2. Classify **each**: is the value an object/ability/unit id (or a collection of them), or a
   genuine non-id string (display NAME, label, title, icon path/URL, search query, hotkey
   LETTER, serialized text, island-grouping label)? The tell: does it flow into/out of
   `WARCRAFT_DATABASE.by_id`/the catalog/an id comparison?
3. **Type every id — OUTBOUND as much as INBOUND.** An outbound leak (a `pub fn` that
   *returns* `&'static str`/`String`/`Vec<&'static str>` which is really an id, or a
   `pub struct` field exposing a string id) is exactly as harmful as a `&str` param and is
   easier to miss — hunt returns and fields specifically.
4. The domain is done only when grepping each crate's public API for those string patterns
   yields **only** genuine non-id strings, each justified.

### A. Domain crate `warcraft-data` (the ROOT — audit exhaustively, fix first). Known offenders (non-exhaustive):

**`warcraft-database` utility fns taking `&str` for an id → take the id type:**
`ObjectLookup::{by_id, has_icon, is_passive_ability, morph_target_unit, ability_code,
off_icon}`; `catalog.rs::{can_attack, can_uproot, ability_is_root,
unit_starts_in_toggle_alt_state, ability_is_on_alt_state_unit, is_burrowed_form,
ability_has_alt_state, primary_commands_for_unit}`; `variant_groups.rs::{is_mergeable_variant_unit,
unit_ability_descriptors, group_for, canonical_for, is_hidden_variant, fanout_siblings}`;
`WarcraftDatabase::by_id`. (Note `canonical_for`/`morph_target_unit`/`ability_code`/`off_icon`
also *return* `&'static str` ids → return `WarcraftObjectId`.)

**`warcraft-keybinds` fns/structs taking `&str` for an id → take the id type:**
`ObjectDisplay::resolve(&str)`; `UnitCollisionReport::for_unit(&str)`;
`UnitSlotContainers::resolve(&str)`; `UnitKeyedCustomKeys::for_unit(&str)`;
`statistics::values::unit_evasion(&str)`; `display/inspector_detail.rs InspectorDetail::build
host_unit_id: &str`; `unit/slots/mod.rs object_id: &str` params.

**Domain `HashMap<String,…>`/`HashSet<String>` keyed by an id → key by the id type:**
`collision/cross_unit.rs units_by_island`; `collision/island_partition.rs parent:
HashMap<String,String>` (island grouping id — introduce a real id type if none exists;
do NOT leave it `String`); `display/rendered_grid.rs tier_overrides: HashMap<String,usize>`
+ `ids_by_token: HashMap<HotkeyToken, HashSet<String>>`; `custom_keys/normalize.rs
abilities_with_independent_off_slot: HashSet<String>`.

**The ONE resolver stays `&str`-in / id-out:** the single genuinely-external-string entry
(`ObjectLookup`'s raw lookup) — repurpose it to return `Option<WarcraftObjectId>` (or add
`WarcraftObjectId::try_from(&str)`), and give known-id callers a typed lookup
(`ObjectLookup::object(WarcraftObjectId) -> Option<&'static WarcraftObject>`). Inside a
domain fn, indexing string-keyed *static SLK data* via `id.value()` is the domain's own
seam and never appears in a signature — but prefer keying that static data by the id type.

Native `cargo test` green; commit as `Clemens <clemenscodes@gmail.com>`; tag **v0.4.0**;
push; bump the app's three `warcraft-*` deps `v0.3.0` → `v0.4.0`.

### B. App `hotkey-editor` — every `.value()` bridge + stringly id signature

After the domain bump, every stringly-id signature the bridges fed is a compile error.
Type them and delete the bridge:

- **`ViewNavigationContext::open_unit(&str)` → `open_unit(WarcraftObjectId)`.** Callers pass
  the id (they all have it); the `.value()` at each `open_unit(id.value())` disappears.
- **String id props → `WarcraftObjectId`:** `UnitDetailHeaderProps.unit_id`,
  `UnitCommandGridsProps.unit_id`, `UnitCardSurfaceProps.unit_id`/`UnitCardInfoProps`,
  `grid_editor host_unit_id`, `tile_override` object-id props, the resolve/collision
  `ObjectId`-badge leaf (`text: String` fed an id — the id→string moves *into that leaf's
  RSX*), and every `*Props`/`*Data`/`*View`/`*Model` id field still stringly.
- **App `HashMap<String,…>`/`HashSet<String>` id keys → id-typed:** `tile_override`
  tier-override maps, `upgrade_tier` `id_key`, collisions `AbilityPairKey`-adjacent maps.
- **`collisions_page` `selected_unit`/`selected_island` + resolve `selected_move_category`
  etc.** — these are unit/island/category *selection*; the unit one is `WarcraftObjectId`.
  The URL `?entry=` token is produced by `.value()` *inside the route serializer only*.
- Delete every id `.clone()/.to_owned()/.to_string()/.as_str()/.value()` that is not
  physically inside an RSX DOM write or the route/persist serializer.

### C. Gallery `crates/gallery/src` (its native build is a CI gate — `:dev` skips it)

`fixtures.rs sample_hero_id()/sample_unit_id()` → return `WarcraftObjectId`
(`WarcraftObjectId::new("Hamg"/"hfoo")`). Every consumer passes the id by value; a story
that needs the `WarcraftObject` uses the typed `ObjectLookup::object(id)`; the nav-story
`selected_unit_id` signals become `Signal<Option<WarcraftObjectId>>`. Genuinely-String
collision selection tokens follow whatever their (now-typed) consumer expects.

## Running the cure (agents)

1. **Domain agent (blocking, `warcraft-data`).** Retype ALL of §A — every utility/lookup
   fn, struct field, return, and map key. Keep only the single external-string resolver as
   `&str`-in/id-out. `cargo test` green, tag v0.4.0, push, bump app deps. Report every
   changed public signature.
2. **App agents (parallel, worktree-isolated), after the bump.** One per subtree
   (`editor_page`/workspace, `collisions_page`, `resolve_page`, `shared`, `services`,
   `header`). Each types every id in its subtree and **deletes every id `.value()`/`.clone()`
   that is not inside a DOM RSX write or the route serializer.** Coordinate prop types across
   subtree seams.
3. **Gallery agent.** §C.
4. **Gate:** `moon run :ci` GREEN (kill any `dx serve` first; capture moon's own exit code,
   not a piped tail's). Then drive the app: race/mode switch lands on the right unit;
   collisions/resolve open the right units; deep-link `?unit=` selects.

**Agent instructions MUST quote "The law" and "The two mistakes" verbatim.** The last
failure came from an agent prompt that offered a `.value()`-into-a-function escape hatch.
There is no escape hatch.

## Detection (grep the smell — every hit is guilty until proven a DOM/serializer line)

- Signatures: `: &str`, `: String`, `: &String`, `Option<String>`, `Vec<String>`,
  `HashMap<String`, `HashSet<String>`, `-> String`, `-> &str`, `-> &'static str` on
  anything named `*_id`/holding a unit/ability/object/slot/hotkey/island id.
- Conversions: any `.value()`, `.to_owned()`, `.to_string()`, `.clone()`, `.as_str()`,
  `.as_deref()` on an id, or an id passed to a fn/`format!`/`push_str` — unless the line is
  literally an RSX DOM attribute/text or the route/persist serializer.
- Leave genuine strings alone: search queries, display NAMES, labels, titles, class lists,
  icon paths/URLs, toast/error messages, the serialized `CustomKeys` text.

## Gotchas (already paid for)

- **Two repos.** Domain change = warcraft-data tag bump (v0.4.0) + app `Cargo.toml` bump of
  ALL THREE `warcraft-*` deps. Never a local `[patch]`.
- **Only `moon run :ci` (gate) and `moon run :dev` (dev).** `:dev` builds only `hotkey-editor`
  (wasm) — it does NOT build the gallery, so a green `:dev` can still fail `:ci` on gallery.
  Only `:ci` proves it. Kill `dx serve` before the gate (stale server on 8123 = false green).
- **`WarcraftObjectId` is `Copy` — no `.clone()`, ever, in the app.** If you typed a `.clone()`
  on an id you were wrong.
- Commit as `Clemens <clemenscodes@gmail.com>`. warcraft-data is a real dep repo (tag it);
  the app `develop` branch is throwaway (no commit-hygiene work).
- Composition/style law in `docs/COMPONENTS.md` still holds for any component you touch; do
  not disturb the landed `race_tabs`/`reason_badges` composition.
