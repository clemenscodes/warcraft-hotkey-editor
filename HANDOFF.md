# Handoff — Header refactor: endgame reached (2026-07-04, pass 2)

The shell header (`crates/hotkey-editor/src/components/shell/header/`) now scales
coherently on **every** viewport — mobile, tablet, laptop, desktop, qhd, uhd — with no
overlap. This pass finished the three remaining TODOs (A/B/C) from pass 1 and fixed a
pre-existing layout bug that pass 1 had not caught.

Read first: `docs/COMPONENTS.md`, `docs/ARCHITECTURE.md`, `docs/RUST_STYLE.md`.

---

## State of the code

- `cargo clippy -p hotkey-editor -p gallery --target wasm32-unknown-unknown` — **green**
- `cargo fmt --check` — **clean**
- `cargo test -p warcraft-keybinds` — **363 pass** (domain untouched this pass)
- **NOT run this pass:** `moon run :ci` / Playwright e2e. Run before merging. Only
  `style.rs` class arrays and one `whitespace-nowrap` changed — no `aria-label` /
  `data-action` selectors were touched, so the four smoke tests should hold.

All changes are in the working tree (10 `style.rs` + the regenerated
`assets/tailwind.css`). Verified in-browser via Playwright at 375, 768, 1000, 1280, 1440,
1920, 2560, 3840.

---

## What got done this pass (A, B, C + the overlap fix)

### A. Brand width — `flex-auto` replaced with definite, viewport-proportional widths
`brand_host/style.rs`: `mobile:w-[65vw]`, `tablet:w-[40vw]`, and `laptop`+ `w-full`
(fills its grid track) with a per-band `max-w`. `flex-auto` is gone — it grabbed all free
space and blew the wordmark up to a 43px title on tablet, and on laptop it overflowed its
grid track and collided with the centered button. Now the brand fills its 1fr track and
scales as one drawing via its interior `cqi`.

### B. `GridLayoutButton` → cqi
`grid_layout_button_host` is the container (`[container-type:inline-size]` + a
`w-[clamp(10rem,14vw,40rem)]` box); `grid_layout_button` fills it (`w-full`), locks its
shape with `aspect-[39/10]`, and expresses **every** interior length in `cqi` (padding,
gap, border, radius, font, glow, focus ring). `grid_layout_button_icon` is `w/h-[9.4cqi]`.
The label got `whitespace-nowrap` so "GRID LAYOUT" stays one line at every size.

### C. Whole header → viewport-proportional
`header/style.rs`: the laptop+ grid column gap and bottom padding are `vw`
(`gap-[1.2vw]`, `pb-[1.2vw]`), so the bar's own layout grows with the viewport. The
toolbar row scales too: `collisions_button_host` and `inline_actions` size their boxes
`clamp(2rem,2.6vw,7rem)` on laptop+, and the toolbar/inline gaps are `vw` clamps — so the
collisions button and the nine file buttons grow in step with the centered button while
staying square (36px on phone/tablet, unchanged).

### The bug pass 1 missed — toolbar overlapped the centered button
The header centers the layout button with `1fr auto 1fr`, which caps the toolbar at one
side track. The ten-button toolbar at the **old 80px button size** needed ~810px and
overflowed, rendering the icon buttons *on top of* the GRID LAYOUT text at every laptop+
width (the old design shipped this way). Root cause: everything was oversized. **User
decision (asked explicitly):** keep the button dead-centered and shrink the toolbar icons.

### Sizings matched to production (the authoritative reference)
The deployed prod build (`https://clemenscodes.github.io/warcraft-hotkey-editor/`) has the
good, small sizings. It uses a **deprecated** mechanism (a `--hdr-scale` CSS var +
`max-[…]px` breakpoints — do NOT reintroduce either), but its *rendered pixel sizes* are
the target. Measured on prod at 1920 and matched here (dev now within ~1px):

| @1920 | prod | dev now |
|-------|------|---------|
| grid button | 139×35, font 11.8 | 138×35, font 11.8 |
| toolbar/collisions button | 29.6 | 29.8 |
| gap between toolbar buttons | 3.8 | 3.8 |
| brand title | 18.9px | 19.2px |
| header height | ~47px | ~49px |

Coefficients (laptop+ bands): grid button host `w-[clamp(7rem,7.2vw,22rem)]`; collisions
host + inline_actions box `clamp(1.75rem,1.55vw,4rem)`; toolbar + inline gap
`clamp(0.2rem,0.2vw,0.5rem)`; brand host `w-[clamp(12rem,20vw,56rem)]` (capped, left in its
track — title is `5cqi` of that); header `gap-[1vw]` / `pb-[0.65vw]`. Everything scales
with the viewport (grows to 4K) instead of prod's fixed `--hdr-scale`. Verified no overlap
and a one-line "GRID LAYOUT" at 1280/1440/1920/2560/3840; mobile/tablet compact header
unchanged (user confirmed the mobile header was already good).

---

## If you keep going

- The tablet header (flex row, brand + collisions + burger) is fine but static-ish; the
  compact 36px buttons don't scale within the tablet band. If desired, give them the same
  `vw` treatment the laptop+ toolbar got.
- The header's mobile/tablet vertical padding + `min-h-14` are still fixed rem; convert to
  `vw` if you want the compact bar to scale within its band too.
- Run `moon run :ci` (Playwright e2e) before merging — not run this pass.

## Gotchas (still true — see the `tailwind-cqi-gotchas` memory)

1. `text-[Ncqi]` generates nothing — use `text-[length:Ncqi]`.
2. `container-type:inline-size` gives `cqi` to DESCENDANTS; an element's own border/radius
   in `cqi` resolve against its nearest ANCESTOR container. Host = container, leaf fills it.
3. A `cqi` border on a small box drops below 1px; Chromium renders non-zero borders as a
   minimum 1 device pixel, so the collisions/toolbar/grid hairlines still read at 1px on
   laptop and thicken gently toward 4K — but don't rely on that for a border you need thick.
4. Don't size a `cqi` container with `flex-auto` — give it a definite width (a `vw`/clamp
   or a grid track). That was the whole point of task A.
5. **Vertical centering needs SYMMETRIC padding.** The header is `items-center`, but it
   originally had bottom-only padding (`pb-[…]`, no `pt`). At small viewports the gap is
   tiny so it looks fine; at 4K the bottom padding is ~25px, which pins all content to the
   TOP of a tall header with a big empty gap below. Fix = symmetric vertical padding
   (`py-[…]`) so `items-center` actually centers. Verified 0px offset from the padding-box
   center at 375/1000/1280/1920/3840. (Measuring against the *border*-box shows a phantom
   −0.5px — that's just the 1px `border-b`; content is exactly at the padding-box center.)

The dev server is run by the user (see the `user-runs-dev-server` memory) — never start,
kill, or restart it; just edit + rebuild Tailwind and observe via the browser.
