# Tablet layout design

**Goal:** Give the `tablet` band (768 to 1279px) a proper touch layout instead of the
cramped desktop layout it currently inherits. The tablet experience is the mobile pager
scaled up ("big mobile"), with each unit's multiple grids laid out side by side instead
of the mobile swipe carousel.

**Status:** design approved (strategy, width usage, and large width behaviour all
confirmed with the user against the mockup at
`scratchpad/tablet-layout-mockup.html`).

---

## 1. Current state

- The band boundaries live in `crates/hotkey-editor/tailwind.css`: `mobile` is
  `width < 768px`, `tablet` is `768px <= width < 1280px`, `laptop` is
  `1280px <= width < 1920px`, and so on. The bands are disjoint, nothing cascades
  across them.
- `editor_page/viewport.rs::use_is_mobile_viewport` matches `(max-width: 767.98px)`,
  the `mobile` band exactly.
- `editor_page/mod.rs` branches on it: `if is_mobile { MobileEditor }` else the desktop
  `Page` frame (`EditorTabsBar` header plus `EditorWorkspace` body).
- Consequence: the whole tablet band (768 to 1279px) renders the **desktop** two column
  layout, which does not fit. At 810px portrait it collapses into a cramped, ungoverned
  stack (verified in the browser).
- The mobile pager (`MobileEditor`) already carries the permanent race navigation
  (`MobileRaceNav`, sticky under the header) built in the prior task.

## 2. The core insight: the side by side grid already exists and is already tablet aware

`editor_page/components/shared/unit_command_grids/` is a shared building block used by
both the desktop unit detail row and the mobile pager card. Its `style/mod.rs` already
encodes the responsive switch:

- `base`: `@container grid grid-cols-2 gap-x-7 gap-y-5 items-start flex-none isolate`
  (desktop: grids side by side).
- `mobile:`: overrides into a horizontal snap carousel
  (`mobile:flex mobile:flex-row mobile:overflow-x-auto mobile:snap-x ...`).
- `tablet:`: `tablet:grid-cols-[repeat(2,1fr)] tablet:gap-x-10 tablet:gap-y-7`
  (keeps the two column grid, wider gaps).

Because the bands are disjoint, **rendering the pager on the tablet band makes the grids
lay out side by side automatically** — the `mobile:` carousel classes simply do not match
at `>= 768px`, so `UnitCommandGrids` falls through to `base`/`tablet:` (the grid). The
same disjoint bands make two more things free:

- `grid_carousel_dots/style` is `base: hidden` with only a `mobile:flex` — so the dots
  **auto hide** on tablet.
- `hotkey_override_section/style` already has `tablet:w-full tablet:self-stretch` — so the
  override row **auto spans full width** under the grids on tablet.

No new grid, dots, or override code is needed.

## 3. What the tablet layout is

Identical component tree to the mobile pager, one unit per vertical snap screen:

```
MobileEditor (vertical snap pager, now mounted for mobile AND tablet)
  MobileRaceNav                     (sticky race nav under the header, reused unchanged)
  PagerSpacer (top)
  for unit: PagerCardHost           (one snap screen; caps + centres the card on tablet)
    PagerCard
      PagerCardHeader               (icon + name + rawcode, reused)
      UnitCommandGrids              (mobile: carousel  |  tablet: 2-col grid, reused)
      GridCarouselDots              (auto hidden on tablet)
      HotkeyOverrideSection         (auto full width on tablet, reused)
  PagerSpacer (bottom)
```

Everything except the two changes in section 4 is reused verbatim, including the three
grid editor variants (`command_grid_editor`, `research_grid_editor`,
`alternate_form_grid_editor`) and their touch drag wiring.

## 4. The changes (this is the whole delta)

### 4a. Mount the pager for the touch bands (mobile + tablet)

`editor_page/viewport.rs`: widen the query from the `mobile` band to the touch bands
(mobile + tablet), i.e. `(max-width: 1279.98px)`, and rename the hook to
`use_is_touch_viewport` to reflect that it now covers both bands. The comment updates to
name the `mobile` and `tablet` bands.

`editor_page/mod.rs`: the branch becomes `if is_touch { MobileEditor } else { desktop
Page }`. So `< 1280px` (mobile + tablet) gets the pager, `>= 1280px` (laptop and up) keeps
the desktop layout. `MobileEditor` keeps its name; it is "big mobile" and serves both
touch bands (a full rename of the subtree is out of scope and buys nothing).

### 4b. Extend the snap pager container to the tablet band

`mobile_editor/style/mod.rs` currently gates the whole container on `mobile:`
(`base: hidden`, then `mobile:flex mobile:flex-col mobile:flex-1 mobile:min-h-0
mobile:min-w-0 mobile:overflow-y-auto mobile:overscroll-contain mobile:snap-y
mobile:snap-mandatory mobile:px-4`). Since the component is now rendered only when the
touch branch is active, move the container behaviour into `base` (drop the `hidden` base
and the `mobile:` prefixes): `flex flex-col flex-1 min-h-0 min-w-0 overflow-y-auto
overscroll-contain snap-y snap-mandatory`. Keep the horizontal padding per band:
`px-4` in `base` with a `tablet:px-6` step for a touch of extra breathing room on the
wider band.

### 4c. Cap and centre the card on tablet (the large width decision)

`pager_card_host/style/mod.rs` currently is `@container flex flex-col h-full min-w-0
shrink-0 snap-start py-4` (full width; the `@container` is the cqi reference for the whole
card). Add a tablet only cap so the card, and therefore its cqi driven interior, stays
ergonomic at landscape widths up to 1279px: `tablet:w-full tablet:max-w-[900px]
tablet:self-center`. On `mobile` the card stays full width (no cap). `900px` is the
starting cap (two 4 column grids side by side land around 100px touch tiles); the exact
value is tuned live in the browser during the build, since it is a one class change and
the user chose "capped and centred".

## 5. Data flow (unchanged)

The grid list per unit already comes from `warcraft_keybinds::UnitSlotContainers::from(unit_id)`
inside `pager_card/presentation` (command card always, plus optional build menu /
uprooted / research). `UnitCommandGrids` renders each optional grid as empty when `None`.
The tablet path reuses this untouched — no domain or presentation change.

## 6. Testing

- Browser verification at three widths: `810x1080` (tablet portrait, just inside the
  band), `~1024` and `1279` (tablet landscape, band ceiling). Confirm at each: the race
  nav is present and sticky; `UnitCommandGrids` renders the two column grid (not the
  carousel); the carousel dots are absent; the hotkey override spans full width under the
  grids; the card is capped and centred at the large widths; a button drag on a grid tile
  works by touch.
- Confirm the desktop layout is unchanged at `>= 1280px` (laptop and up) — the branch and
  all desktop `tablet:`-unprefixed styles are untouched.
- e2e is explicitly out of scope for this task per the user.

## 7. Risks and mitigations

- **The `mobile:` carousel leaking onto tablet.** Mitigated by the disjoint band
  definitions in `tailwind.css` (`mobile` is strictly `< 768px`). Verify in the browser
  that at 768px the grids are the grid, not the carousel.
- **Snap fit.** With grids side by side the card is no taller than the mobile carousel
  card (the carousel already shows one grid row at a time), so one unit still fits one
  vertical snap screen. Verify the card `h-full` does not overflow at portrait.
- **Naming smell.** `MobileEditor` now serves tablet too. Accepted deliberately (big
  mobile); revisit only if a real second consumer or confusion appears (rule of three).
