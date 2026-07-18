# Search dialog filter consolidation

Date 2026-07-18. Branch feature/mobile-redesign.

## Problem

On mobile the search dialog opens with two full rows of chunky bordered
buttons before the search field appears. The first row holds the five race
filters, the second row holds four more toggles. Together they eat roughly the
top half of the dialog, so the main feature of the dialog, the search, sits
below a wall of controls.

The race colours are not the problem. Every race carries its own accent colour
consistently across the whole application, and that stays. The problem is
purely the vertical cost before the search, and the fact that a heavy filter
block dominates a surface whose purpose is searching.

## Current structure

The dialog body stacks three children in a column.

- `SearchDialogFilters`, which renders `RaceChipRow` (five race `ToggleButton`s)
  and `ModeChipRow` (four `ToggleButton`s).
- `UnitListSearch`, the search row, which already contains a `SearchScopePicker`
  (the `UNIT` picker that opens a floating menu with a backdrop).
- `CategoryScroll`, the results.

`ModeChipRow` mixes two natures. Melee and Campaign are real modes that live in
the URL. Plain units and All tiers are display toggles that live in editor
state. A player sees four identical buttons regardless.

## Decision

Collapse the whole filter apparatus into one configuration dropdown that the
search component owns itself. The dialog header becomes a single row, just the
search field plus one configuration trigger. The trigger opens a full width
panel below the row, floating over the results with a backdrop, using the same
mechanism the scope picker already uses today. Scope, race, mode and display
toggles all live inside that panel.

This was chosen over two lighter alternatives. One alternative kept the race
row visible and folded only the rest away. Another kept scope in the bar and
added a separate filter dropdown. The chosen shape merges scope and every
filter into one trigger so the search field stands alone at the top.

### Accepted tradeoff

Race is the most used navigation filter on mobile, and folding it into the
dropdown makes a race switch two taps instead of one. This tradeoff was chosen
deliberately. It is mitigated by a count badge on the trigger, so active
filters stay glanceable without opening the panel. The filters are tucked away,
not made invisible.

## The shape

The dialog header, one row.

```
+---------------------------------------------+
|  [gear 3 v]  (search)  Search every race... |
+---------------------------------------------+
|  <results, full height>                     |
```

The configuration panel, opened from the trigger.

```
+---------------------------------------------+
|  Search in   [ Unit ] Ability               |
|  Race        Human Orc Elf Undead Neutral   |
|  Mode        [ Melee ] Campaign             |
|  Show        Plain units ( )   All tiers ( )|
+---------------------------------------------+
```

## Three design points

### Control form follows nature

Each row gets the control that matches what it is, instead of nine identical
buttons.

- Search in, a two option segment. This is today's scope picker moved inside.
- Race, the five accent coloured chips, unchanged in colour and behaviour.
- Mode, a two option segment for Melee and Campaign.
- Show, real on and off switches for Plain units and All tiers, because they
  are settings rather than navigation.

Labelled rows turn the button pile into a readable configuration sheet.

### The trigger stays informative

Because everything is folded in, the trigger shows a count badge of the active
filters. That is the mitigation of the accepted tradeoff above.

### The search component owns its filters

The search component of the dialog encapsulates its own filters. The
configuration dropdown is an internal part of that component, not a sibling
block assembled by a parent. This is the better architecture in general, a
component that searches owns the controls that shape its search.

## Architecture and the desktop boundary

`UnitListSearch`, together with its `SearchScopePicker`, currently lives under
`shell/components/shared/` and is rendered in two places, the desktop unit list
and the mobile search dialog. On the desktop, race and mode are handled by the
permanent tabs, so those filters must never enter the shared search row.

It follows that the merged configuration dropdown belongs to the search
dialog, not to the shared `UnitListSearch`. The desktop shared search row stays
exactly as it is, scope only, filters via tabs. The dialog composes its own
self contained search component from the shared leaves it needs, the search
input, the search icon, the scope option logic, the race chips and the mode
row. This matches the components rule that genuinely different variants are
separate components composing shared leaves, never a shared shell that receives
a body.

The old `SearchDialogFilters` block above the search is removed. Its children,
the race chips and the mode toggles, move into the new configuration menu of
the search component.

## Out of scope

This change does not turn race and mode into permanent mobile navigation
outside the dialog. That larger parity goal from PRODUCT.md section 7.3 is a
separate effort. This change only fixes the search dialog itself.

## Verification

`moon run :ci` green, and the dialog opened in a real mobile viewport in the
browser, confirming the search field is the first thing reachable, the
configuration panel opens and dismisses, and every filter still narrows the
catalog as before.
