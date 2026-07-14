# cqi/fill cascade sweep — handoff

> ## ⛔ STOP — READ ALL FOUR SPEC DOCS IN FULL FIRST. THIS IS NON-NEGOTIABLE. ⛔
>
> You **cannot** do a single correct thing in this codebase until you have read, IN
> FULL and IN DEEPEST DETAIL (Read tool, no offset, no limit, top to bottom, every
> line — do NOT skim, do NOT skip, do NOT rely on this handoff as a substitute):
>
> 1. **`docs/ARCHITECTURE.md`** — the wall, localStorage source of truth, R1–R10.
> 2. **`docs/COMPONENTS.md`** — render-tree==dir-tree, the Host/`@container` model,
>    fill-the-container, no-clamp, one-class-per-component, the header/footer capstones.
> 3. **`docs/RUST_STYLE.md`** — every naming/trait/no-tuple/`Self` rule.
> 4. **`docs/AGENTS.md`** — commands, the dev/e2e loop, the hard-won gotchas.
>
> Then read the **reference component trees IN FULL**: `shell/header`, `shell/footer`,
> and `grid_editors/*`. They are the *only* correct implementations of everything below.
>
> **Why this banner exists:** the entire `cqi` model — that a `cqi` value is broken
> without a definite-width `@container`/`_Host` ancestor — is spelled out in
> `COMPONENTS.md` ("The Host doubles as the leaf's container", "Fill the container").
> A prior agent skimmed the docs, mass-converted ~150 files of spacing→`cqi` **without**
> the host layer, shipped unusable broken layout, got everything reverted, and was
> fired. **Skimming these docs IS the failure.** A repo hook (`spec-gate.sh`) will even
> block your first edit until it sees full-file Reads of the three specs — that gate is
> not a suggestion, it is the floor. Reading them deeply is the cheapest hour you will
> ever spend here; skipping it guarantees you break the app.

Goal: make **every** component fill the box its parent gives it and scale via `cqi`,
cascading from a per-band-sized **top container** down through every layer to the
leaves — exactly like `shell/header` and `shell/footer`.

**Read this whole file too, after the four specs.** The §0 post-mortem is the most
important part here — an app-wide attempt failed hard by ignoring it and was reverted.

---

## 0. POST-MORTEM — the mistake that got a whole session reverted. DO NOT REPEAT.

**A `cqi` value is meaningless without a definite-width `@container` ANCESTOR.** `cqi`
resolves against the *nearest ancestor query container*; if there is none it falls back
to the **viewport**, and if the nearest one is the wrong size (e.g. a dialog mounted
in-tree under a 202px header button host) the magnitude is wildly wrong. So:

> **Converting fixed spacing/size to `cqi` WITHOUT first establishing the right
> `@container` host is BROKEN, not progress.** A prior session mass-converted ~150
> files of spacing→`cqi` (fanned out across ~20 agents) *without* adding the
> `@container`/`_Host` layer. Result: "literal unusable broken junk," fully reverted.

**This is already visible on `HEAD`:** the **stats panel** (`unit_stats_panel` /
"CqiStats" subsystem) is broken *right now* for exactly this reason — its interior was
put on `cqi` without a correct definite-width `@container` host, so it renders wrong.
Fixing the stats panel is the smallest concrete example of the correct method below and
a good first target.

### The correct method — container layer FIRST, per component, measured, verified
1. **Establish the host.** For each region that must scale, make sure a `_Host` (or an
   existing definite-width box-owner) carries `@container` at the width the interior
   should scale off. This is the `shell/header` / `grid_editors` role-model pattern
   (COMPONENTS.md → "The Host doubles as the leaf's container").
   - `@container` (`contain: inline-size`) **collapses** a shrink-to-fit element to 0.
     Only put it on something with a **definite/fill** width: `w-full`, a grid item in a
     `1fr`/fixed track, `flex-1`, `size-full`, or an explicit per-band width. A
     `self-start` / `w-auto` / content-width element must get a definite width FIRST (or
     a `_Host` wrapper that has one).
   - Remember `cqi` on an element resolves against its **ancestor** container, never
     itself. A leaf that scales its own size needs `@container` on its **parent/Host**.
2. **Then** convert that region's interior lengths to `cqi`, **measured live** against
   the host's real rendered width: `cqi = round(100 * px / hostWidth, 2)`. Do NOT guess
   a "tier" divisor across hundreds of components — that was part of the failure.
3. **Verify each region in the browser** (Playwright MCP, multiple bands) before moving
   on. `width:0` = collapse (bad `@container`); oversized = missing/wrong host or
   same-element-cqi. Judge by **rendered layout**, never by an audit count.

### Process rules (also violated last session)
- **NEVER run `moon run :ci` yourself** — it's a ~7-min blocking gate that hangs the
  session; the USER runs it. Use `moon run :check` (fast, ~7s) for compile feedback, or
  run long jobs with a background flag. Never end a turn on a blocking command.
- Only three commands exist: `moon run :ci` (gate, user-run), `moon run :check` (fast
  compile), `moon run :dev` (dev server — the **USER** starts it; a hook blocks agents).
- The dev app is at `http://localhost:8123/warcraft-hotkey-editor/` (trailing slash) —
  bare `/` is 404. Refresh to clear the "being rebuilt" overlay; don't wait on it.
- **Do NOT declare the sweep "mostly done" from a source audit.** It is a 500+
  component app and is NOT nearly done. An audit that says "already cqi" is exactly the
  trap — much of what "looks converted" is broken because it lacks the host layer.
- Fan-out is wanted, but each agent must own the **container layer + values + a real
  host**, not value-only. Agents can't drive the single shared browser in parallel, so
  the coordinator measures host widths and hands agents numbers — or agents add the
  `_Host`/`@container` structurally and flag the exact `cqi` value for the coordinator
  to measure. Value-only fan-out is what broke everything.

---

## 1. The mental model — the header IS the answer

Read these in full before writing a line:
`shell/header/style` → `toolbar/style` → `toolbar_actions/style` → `inline_actions/style`
→ `shared/toolbar_button/style` → `toolbar_button_surface/style` → `toolbar_button_icon/style`.

- **The TOP container owns the ONE absolute size, per band** (`@container` + per-band
  `min-h-14 … min-h-34`). That is the only absolute length in the subtree.
- **Every layer below FILLS and passes size down** (`h-full`/`w-full`/`size-full`,
  `min-w-0`, `contents`). They carry no size of their own.
- **`items-stretch`** on a row hands each child a definite cross-axis size to fill.
- **The leaf is a drawing that scales off its box**: `ToolbarButton` is `@container
  h-full aspect-square`, and its icon is `w-[44cqi]` — a `cqi` fraction of the button,
  never `w-full`.

So the cascade is: **one per-band knob at the top → fill all the way down → `cqi`-scaled
drawing at the leaf, each leaf sitting under a definite-width `@container` host.**
`grid_editors/*` is the same pattern in the editor — study it too.

## 2. The four killer gotchas

1. **`@container` collapses a shrink-to-fit parent to 0.** Only add it where width is
   already definite (`w-full`/`flex-1`/grid track/explicit width). Give an
   `w-auto`/`self-start` box a definite width (or a `_Host`) FIRST, or skip.
2. **An element's own `cqi` resolves to its nearest ANCESTOR container, never itself.**
   `@container` + `text-[50cqi]` on the SAME element → resolves against the viewport =
   giant. `cqi` goes on the child; `@container` goes on the parent/Host.
3. **A raw `<svg>` sized `w-full`/`h-full` in an INDEFINITE box falls back to its
   intrinsic ~300px and blows up.** Give it a definite box-owner (Host) or size the icon
   in `cqi`.
4. **`items-center` leaves a filling child's height indefinite; `items-stretch` makes it
   definite.** Rows whose children fill by height MUST be `items-stretch`.

## 3. The Host pattern (spec-compliant)

To add `FooHost` around leaf `Foo` (leaf nests UNDER the host — render tree == dir tree):
```
foo_host/
  mod.rs            renders <Foo .../> inside div class:CLASS; assert_component!(FooHost)
  model/ view/ style/   ddd trio; style = THE BOX (see below)
  components/foo/   the moved leaf; leaf style → size-full
```
- Host `style`: to **fill a definite row** → `@container h-full w-auto aspect-square
  shrink-0 flex items-center justify-center` (+ make the row `items-stretch`). To carry
  its **own** per-band size → `size-20` + `mobile:/tablet:size-[…]` (definite, never
  collapses).
- Rewire every renderer's `use …::foo::Foo` → `…::foo_host::FooHost` and `Foo {` →
  `FooHost {`. Then `moon run :check`.

## 4. Recipes (pick by situation)
- **A. Fills a row a sibling/parent sizes** → make the row `items-stretch` + per-band
  height; leaf → `h-full aspect-square`.
- **B. Parent has a definite width** → `@container` on the parent; leaf →
  `size-[Vcqi]`, `V = round(100 × leaf_px / parent_px, 2)` (Tailwind px = `n × 4`;
  `Nrem` = `N × 16`).
- **C. No definite box owner** → add a Host (§3); leaf → `size-full`; icon in `cqi`.
- **D. Inline glyph in text** → `em` (footer pattern).

KEEP (never dissolve): `min-w-0`/`min-h-0`/`minmax(0,…)` plumbing; `*-full`/`auto`/`%`/
fractions; intrinsic shape (tap-target `min-h-[44px]`, prose `max-w-[90rem]`, hairline
`h-px`, key-chip `min-w-[18rem]`); viewport `vw/vh` on dialog SHELLS; anything already
`cqi`/`cqh`/`em`; per-band SIZE scaffolds (`w-136`, `flex-[0_0_34rem]`,
`laptop:grid-cols-[34rem_…]`, per-band header heights).

## 5. Subsystem state (as of this reverted session's investigation)

- **`shell/header`, `shell/footer`** — DONE, role models. Do not edit.
- **`grid_editors/*`** — role model, has its `@container` hosts. Reference it.
- **`editor_page` / `editor_workspace` / `mode_tabs`** — use per-band widths/grid-cols +
  per-band gaps. That is CORRECT (bands are absolute-size-at-the-top). Do NOT `cqi`-ify
  these scaffolds.
- **`unit_stats_panel` ("CqiStats")** — **BROKEN on HEAD** (cqi without a correct
  `@container` host). Needs the container layer done properly. Good first target.
- **`unit_detail` spine, collisions detail panels** — have `@container` ancestors in the
  chain (`unit_detail_body`, `panel_card`, conflict panels). Verify each host's width is
  the one you want the interior to scale off before trusting existing `cqi`.
- **`resolve_page`** — has **ZERO `@container` anywhere**; anything `cqi` here resolves
  against the viewport. Needs `@container` box-owners (`plan_body`, the per-move card,
  the unresolved card) before its interiors can be `cqi`.
- **Dialogs** — mount in-tree under a ~202px header button host, so interior `cqi`
  resolves against 202px unless the dialog PANEL is made an `@container`. The
  `WarcraftDialog` panel is `w-[80vw]` (definite) → putting `@container` there is safe
  and makes interiors resolve against ~the panel. Dialogs currently render fine with
  FIXED px and are viewport-capped; only convert them after the panel `@container` exists
  and you verify magnitudes.

## 6. The process that avoids the thrash
1. Pick ONE subsystem. Study the header/grid_editor host for the analogous shape.
2. Establish the `@container` host layer for it (definite width; `_Host` if none).
3. Measure the host width live; convert that region's interior to `cqi` off it.
4. `moon run :check`; verify in the browser at multiple bands (`width:0`=collapse,
   oversized=missing host). Fix before moving on.
5. Only then move to the next subsystem. Never mass-convert values without hosts, never
   run the blocking gate yourself, never trust an audit over the rendered layout.
