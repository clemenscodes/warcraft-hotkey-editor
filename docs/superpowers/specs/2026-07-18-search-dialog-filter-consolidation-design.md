# Search dialog filter consolidation and race scoped search

Date 2026-07-18. Branch feature/mobile-redesign.

## Two problems, one of them the real one

The visible problem is layout. On mobile the search dialog opens with two full
rows of chunky bordered buttons before the search field appears. The first row
holds the five race filters, the second row holds four more toggles. Together
they eat roughly the top half of the dialog, so the main feature of the dialog,
the search, sits below a wall of controls.

The deeper problem is behaviour, and it is the core of this work. The search
does not respect the race at all. Today a race button is navigation, not a
filter. Clicking a race jumps to that race's first unit and sets the active
race that drives the theme. The moment a search runs, the domain drops the race
entirely and searches across every race. There is no way to say "search only
inside the Nightelf units". Two different ideas sit on one control, and they
must be separated.

The race colours are not a problem. Every race carries its own accent colour
consistently across the whole application, and that stays.

## The two race concepts, named

- **Navigation race**. One active race at a time. It drives the theme and the
  browse. Selecting it means "go to this race and edit there". This already
  exists and does not change.
- **Search scope race**. A set of races the search is allowed to cover. Multi
  select, default every race. Selecting Nightelf and Undead means "search only
  inside those two". This does not exist today and is the new capability.

## Where the rule actually lives

The behaviour lives in `warcraft-api`, which is the git pinned external crate
`warcraft-data`, not in the renderer. In `UnitListingRequest::catalog_query`
(browse module) one line forces the race away while searching.

```rust
let race = if searching { None } else { Some(self.race) };
```

The same shortcut sits in `UnitCategoryRequest`. The filter predicate itself
already supports restricting by race. In the listing pipeline an object is
dropped when its race does not match `query.race`. So the filter can already do
the work. Only this browse to query mapping throws the race away during a
search.

There is already a local working copy of `warcraft-data` under
`/home/clemens/.local/src/warcraft-data/` with uncommitted work on this same
listing pipeline, and the `[patch]` in the workspace `Cargo.toml` points at it.
The domain change is iterated there and ships by tagging `warcraft-data` and
bumping the tags in `Cargo.toml`. A committed patch breaks CI, so the patch is
removed before this work lands.

## The domain change, the core

In `warcraft-api`.

- Replace `UnitQuery.race`, today an `Option<Race>` meaning one race or all,
  with a `RaceSelection` value object. It is either `All` or `Only` a set of
  races, and it answers `contains(race)`. The listing filter predicate becomes a
  single membership check against it. Race derives `Eq`, `Hash` and `Ord`, so an
  ordered set is the natural carrier.
- A browse builds `Only` the one active race, unchanged in behaviour. A search
  builds the selection from the scope set, defaulting to `All`.
- Rewrite the mapping so a search no longer discards the race. It passes the
  scope selection through instead of forcing it to all races. `UnitListingRequest`
  and `UnitCategoryRequest` carry the scope selection alongside the navigation
  race, and choose which to apply from whether a search is running.
- The tests that today assert a search drops the race are inverted. They assert
  a search respects the scope selection, an empty or `All` scope still spans
  every race, and a narrowed scope excludes the other races. Every cascade of the
  grouped listing already under test stays green.

This is a real change across the wall, with tests on the domain side, delivered
by a `warcraft-data` tag bump.

## The renderer change

In `hotkey-editor`.

- `UnitFilterQuery` gains the search scope selection as its own field, carried
  separately from the navigation race. It passes both across the wall to
  `UnitListingRequest`. The navigation race still feeds the browse, the scope
  selection feeds the search.
- The scope selection is UI state that the search component owns. Whether it
  lives in navigation state, so it can ride in the URL as a deep link, or in
  editor state like the other display toggles, is a plan level decision. The
  default is every race, so an untouched dialog searches everything exactly as it
  does today.
- The race row in the dialog becomes the multi select control that drives the
  scope selection. It no longer calls `select_race` navigation.

## The user facing shape

The dialog header collapses from two rows of roughly nine chunky buttons to one
row, the search field plus a single configuration trigger.

```
+---------------------------------------------+
|  [gear 3 v]  (search)  Search every race... |
+---------------------------------------------+
|  <results, full height>                     |
```

The trigger opens a full width configuration panel below the row, floating over
the results with a backdrop, using the same mechanism the scope picker uses
today. Scope, race, mode and display toggles all live inside that panel, in
labelled rows.

```
+---------------------------------------------+
|  Search in   [ Unit ] Ability               |
|  Race        Human Orc Elf Undead Neutral   |
|  Mode        [ Melee ] Campaign             |
|  Show        Plain units ( )   All tiers ( )|
+---------------------------------------------+
```

### Control form follows nature

Each row gets the control that matches what it is, instead of nine identical
buttons.

- Search in, a two option segment. This is today's scope picker moved inside.
- Race, the five accent coloured chips as a multi select scope filter. Colour
  and behaviour of the colour stay, the meaning changes from navigation to
  scope.
- Mode, a two option segment for Melee and Campaign.
- Show, real on and off switches for Plain units and All tiers, because they are
  settings rather than navigation.

Labelled rows turn the button pile into a readable configuration sheet.

### The trigger stays informative

Because everything is folded in, the trigger shows a count badge of the active
filters. That keeps active filters glanceable without opening the panel. This is
the mitigation of the accepted tradeoff below.

### Accepted tradeoff

Folding the controls into the dropdown makes a filter change two taps instead of
one. This was chosen deliberately, in exchange for putting the search field
first. The count badge keeps the state visible. The filters are tucked away, not
made invisible.

## Architecture and the desktop boundary

`UnitListSearch`, together with its `SearchScopePicker`, lives under
`shell/components/shared/` and is rendered in two places, the desktop unit list
and the mobile search dialog. On the desktop, race and mode are handled by the
permanent tabs, so those filters must never enter the shared search row.

It follows that the merged configuration dropdown belongs to the search dialog,
not to the shared `UnitListSearch`. The desktop shared search row stays exactly
as it is, scope only, filters via tabs. The dialog composes its own self
contained search component from the shared leaves it needs, the search input,
the search icon, the scope option logic, the race chips and the mode row. The
search component encapsulates its own filters. A component that searches owns the
controls that shape its search, which is the better architecture in general. This
matches the components rule that genuinely different variants are separate
components composing shared leaves, never a shared shell that receives a body.

The old `SearchDialogFilters` block above the search is removed. Its children,
the race chips and the mode toggles, move into the configuration menu of the
search component.

## Out of scope

Navigating to a race in the dialog now happens by tapping a result, which sets
that unit's race active and themes the window as before. Turning race and mode
into permanent mobile navigation outside the dialog stays the separate parity
goal from PRODUCT.md section 7.3. This change fixes the search dialog and the
race scoped search, not the mobile navigation chrome.

## Verification

The domain side, the `warcraft-data` test suite green, including the inverted
race scope tests. The renderer side, `moon run :ci` green with the local patch
active and `--force` where the out of workspace patch defeats the moon cache.
The feature, the dialog opened in a real mobile viewport in the browser,
confirming the search field is the first thing reachable, the configuration panel
opens and dismisses, a narrowed race scope restricts the search to those races,
and an untouched scope still searches every race. Before landing, the local patch
is removed, `warcraft-data` is tagged, and the tags in `Cargo.toml` are bumped.
