# Mobile UnitCard pager — design

**Date:** 2026-07-15
**Status:** Design approved (owner-confirmed). Ready for implementation plan.

Companion: `docs/superpowers/specs/2026-07-15-mobile-editor-redesign.md` (the locked
mobile IA). This spec implements a first slice of that plan's §3 gesture model — the
**whole-card vertical pager over units** — without any of the navigation, search, or
override/stats-dialog machinery. Those remain future batches.

---

## 0. Goal, in one sentence

Replace the primitive mobile screen (which renders a single selected unit's stacked
command grids) with a **vertically scroll-snapped pager of `UnitCard`s — one card per
unit, every race — that only builds the cards in or adjacent to the viewport.**

## 1. What this batch is (and is not)

**In scope**

- A new **`UnitCard`** component: a blue card (mirroring the desktop unit-detail card
  look) whose header is the unit **icon + name + rawcode id**, and whose body is the
  unit's command grids (the existing `UnitCommandGrids`, which the mobile editor already
  reuses).
- **`MobileEditor`** becomes a **windowed vertical scroll-snap pager**: it lists every
  unit (all races), renders one `UnitCard` per unit, and mounts only a small window of
  cards around the current scroll position.

**Explicitly out of scope (future batches, per the locked IA)**

- All navigation/selection wiring — the pager does **not** read or write the
  selected-unit signal, the active-race signal, mode, or search. It is nav-free.
- The search dialog (⌕ unit switcher), the stats dialog (ⓘ), and the per-tile override
  dialog. Tiles keep whatever editing behavior `UnitCommandGrids` already provides.
- Any filtering / "working set" (§3 of the IA's vertical-swipe-walks-the-filtered-set):
  this batch walks **all units**, unfiltered.
- The header-quality `cqi` fluid-scaling pass. Sizing here is ordinary tokens/bands, per
  the IA note that `cqi` is a later stage.
- Promoting `UnitCommandGrids` (and other reused unit-detail leaves) into a `shared/`
  grouping directory. See §7.

## 2. Component structure

New/changed tree under
`…/editor_page/components/mobile_editor/`:

```
mobile_editor/                     the vertical scroll-snap pager (viewport owner)
  presentation/  use_mobile_editor() -> Rc<[WarcraftObjectId]>   (all units, race-ordered)
                 + the viewport-height / active-index UI signals (see §4)
  state/         MobileEditor UI enums if needed (window math helpers)
  style/         h-full, overflow-y-auto, snap-y, snap-mandatory, mobile band
  components/
    unit_card/                     UnitCard { unit_id: WarcraftObjectId }
      presentation/ use_unit_card(unit_id) -> { name, icon_url, unit_id, + 4 slot groups }
      style/       the blue card + h-full + snap-start + shrink-0
      components/
        unit_card_header/          UnitCardHeader { icon_url, name, unit_id }
          components/
            unit_card_portrait/    UnitCardPortrait — the icon img
            unit_card_name/        UnitCardName — the display name text
            unit_card_id/          UnitCardId — the rawcode text
        (UnitCommandGrids is rendered here too — reused cross-tree, see §7)
    unit_card_spacer/              UnitCardSpacer { height_px } — the top/bottom
                                   scroll-height fillers (a single leaf, rendered twice)
```

Each directory obeys `directory == component == class` and the one-class-per-component
rule. The header leaves are **built fresh, mobile-local** — the pager does not reach
cross-tree into the desktop `unit_detail` header components. They share design-token
*values* with their desktop counterparts (never a shared style item).

## 3. The blue card

Mirror the desktop unit-detail card's *values* (the card root at
`…/shared/unit_detail/style/mod.rs`), not the component:
`border`, `border-warcraft-blue-deep`, `rounded-card`, `bg-panel-dark`, `shadow-bevel`,
plus its padding. `UnitCard` writes these into its own `style/mod.rs`; nothing is shared
or extended across the two components (COMPONENTS.md — share values, not looks).

## 4. The windowing mechanism (the core of this batch)

The pager is a **virtual scroll-snap pager**. Its correctness rests on one invariant:

> **Every `UnitCard` is exactly one viewport tall** (`h-full` inside the scroll
> viewport, `snap-start`). If a unit's grids exceed one viewport, the card body scrolls
> internally (`overflow-y-auto` on the body region) — the card's own height stays one
> viewport.

Because card height is uniform and equal to the viewport, the visible index is pure
arithmetic — no per-item height measurement, no `IntersectionObserver`:

1. **Capture the viewport.** `MobileEditor`'s root is the scroll container
   (`overflow-y-auto snap-y snap-mandatory`). On `onmounted`, read the element's
   `clientHeight` into a `viewport_height` UI signal (re-read on resize). This mirrors
   the existing `shared/drag_scroll.rs` pattern (capture the element, use `web_sys` for
   metrics) — `web_sys` glue in the renderer is an established, allowed pattern.
2. **Track the active index.** On `onscroll`, compute
   `active_index = round(scroll_top / viewport_height)` and write it to an `active_index`
   UI signal **only when the rounded index changes** — so a full swipe re-renders the
   pager once (at the card boundary), not on every scroll pixel.
3. **Render only the window.** For `unit_ids: Rc<[WarcraftObjectId]>` of length `n` and a
   buffer `B` (default `1` — the card above and below; tunable):
   - `window_start = max(0, active_index - B)`
   - `window_end   = min(n - 1, active_index + B)`
   - Render, in order: a **top spacer** of height `window_start * viewport_height`, the
     `UnitCard`s for indices `window_start..=window_end`, then a **bottom spacer** of
     height `(n - 1 - window_end) * viewport_height`.
   - The spacers preserve total scroll height so the scrollbar and every scroll offset
     stay correct; off-screen cards (and their command-grid editors) are never built in
     the vdom.

At any moment ~`2B + 1` cards exist regardless of `n`. A `±1` buffer means the next card
in each direction is already mounted before it becomes the snap target, so momentum
snapping always lands on a real element.

Note: CSS `content-visibility:auto` is deliberately **not** used — it skips browser
paint but Dioxus would still build every card's vdom and run every grid editor's hooks.
The cost we are avoiding is Rust-side, so true windowing (not rendering off-screen cards)
is required.

## 5. Data sources

- **Unit list:** `WarcraftApi::default().unit().all()` yields every `UnitView` in the
  database. `use_mobile_editor` collects their ids into an `Rc<[WarcraftObjectId]>`,
  **ordered by race** (`UnitView::race()`, in `Race::ALL` order: Human, Orc, NightElf,
  Undead, Neutral; race-less units last) then by name (`UnitView::name()`), so the scroll
  reads as a coherent race-grouped sequence. This is a stateless catalog read, resolved
  in the presentation hook exactly as the existing mobile editor resolves
  `UnitSlotContainers` and as `unit_list` resolves `UnitListing` — no CustomKeys domain
  data is touched, so no `services/customkeys` query is involved.
- **Per-card grids:** `use_unit_card(unit_id)` resolves
  `warcraft_keybinds::UnitSlotContainers::resolve(unit_id)` and maps
  `.command_card() / .build_menu() / .uprooted() / .research()` into the props
  `UnitCommandGrids` already accepts — identical to today's `use_mobile_editor`, just
  keyed by the card's own `unit_id` prop rather than the selected unit.
- **Per-card header:** `use_unit_card(unit_id)` also reads the unit's `UnitView`
  (`name()`, `id()`, `icon()`), resolving the icon path to an asset URL the same way the
  desktop unit portrait does, and hands `UnitCardHeader` the `name`, `unit_id`, and
  `icon_url`.

## 6. State — all UI, no domain (R10)

`viewport_height` and `active_index` are presentation-only signals (window geometry).
They are never persisted, never written to the CustomKeys string, and carry no domain
meaning — R10-clean. The pager mutates no domain state; editing still flows entirely
through `UnitCommandGrids` and its existing domain commands.

## 7. Known debt this batch deliberately does not pay

`UnitCommandGrids` lives in the desktop `unit_detail` subtree and is rendered by both
`unit_detail_row` (desktop) and `UnitCard` (mobile). By COMPONENTS.md's render-tree ==
directory-tree rule, a leaf with two sibling renderers should be promoted to a `shared/`
grouping at the nearest common parent, reached by full path. `MobileEditor` **already**
reuses it cross-tree today via a full-path import; this batch keeps that existing
precedent (`UnitCard` renders the same reused component) and leaves the `shared/`
promotion as a **noted follow-up**, so this batch stays focused on the pager and the
card. The mobile-local header leaves (§2) are built fresh precisely to avoid *adding* new
cross-tree reuse.

## 8. Risks

- **Nested scroll + snap.** The card body scrolls internally when grids overflow, inside
  a `snap-mandatory` outer pager. This must feel right on a real touch device — verify
  that an internal grid scroll doesn't fight the outer page snap. If it does, fall back
  to letting tall cards be taller than one viewport (which then requires measured offsets
  instead of the uniform-height index math — a larger change, avoided unless forced).
- **Scroll re-render cost.** Guarded by "update `active_index` only when the rounded
  index changes." Confirm in-browser that a fast flick doesn't thrash re-renders.
- **First paint before `viewport_height` is known.** Until `onmounted` fires,
  `viewport_height` is unset; render a safe initial window (e.g. indices `0..=2B`) so the
  first cards show, then correct on mount.

## 9. Verification

- `moon run :ci` green.
- In a real mobile-band browser (Playwright MCP at
  `http://localhost:8123/warcraft-hotkey-editor/`, mobile viewport):
  - the screen shows a blue `UnitCard` with icon + name + id header and the unit's
    command grids;
  - vertical swipe/scroll snaps card-to-card through units of every race in order;
  - the DOM contains only ~`2B + 1` cards at any scroll position (inspect: off-screen
    cards are absent, spacers hold the height);
  - scrollbar length/position is correct for the full unit count.

## 10. As-built notes (deviations discovered during implementation)

Three adjustments were made against the plan while building and verifying in-browser;
they refine, not contradict, the design above.

- **Naming: `UnitCard` → `PagerCard` (and family).** A pre-existing desktop catalog
  component already owns the `unit-card` / `unit-card-id` / `unit-card-name` identity
  classes (the unit-list cards). To keep `directory == component == class` unique, the
  new pager components are named `PagerCard`, `PagerCardHeader`, `PagerCardPortrait`,
  `PagerCardName`, `PagerCardId`, `PagerSpacer` (dirs `pager_card*` / `pager_spacer`
  under `mobile_editor/`). Function and behavior are exactly as specified in §2.
- **The pager needs a fixed-height ancestor: shell is `mobile:h-dvh`.** `MobileEditor`
  is `flex-1` inside `.shell`, which was `mobile:h-auto` (page-scrolls model) — so the
  pager had no bounded height and `h-full` cards collapsed to 0 with a runaway spacer.
  Fixed by making the shell a viewport-height column on mobile (`mobile:h-auto` →
  `mobile:h-dvh` in `shell/style`), which matches the locked IA's "fixed screen, zero
  layout shift" intent. This is the "fix the ancestor, not the card" resolution the §8
  risk anticipated. (Follow-up: the collisions/resolve mobile views will each need
  internal scroll under a fixed shell — noted, out of scope here.)
- **`viewport_px` is kept live via `onresize` (ResizeObserver), not just `onmounted`.**
  A single mount-time read captured a transient (pre-CSS-settle) height and never
  corrected, producing a giant bottom spacer. An `onresize` handler re-reads the
  settled `client_height` whenever the pager's box changes, so `viewport_px` converges
  to the true one-card height.

**Verified in-browser (390×844 mobile viewport):** blue `PagerCard` fills the screen
with portrait + gold name ("Altar of Kings") + rawcode (`halt`) header over the command
grid; ~850 units queried and race-ordered; the DOM holds only 3 cards at any scroll
position (spacers hold the rest of the height); scrolling advances card-by-card through
consecutive units. The one-card-per-jump behavior is `snap-mandatory` snapping to the
nearest windowed card — consistent with the IA's "vertical swipe = prev/next unit"
(far units are reached via the future search, not by scrolling all 850).
```
