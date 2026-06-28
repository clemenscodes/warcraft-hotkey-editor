# CSS viewports

This project supports seven viewport bands. Every gallery preset
(`crates/gallery/src/viewport.rs`) sits inside exactly one band, so selecting a
preset triggers exactly that band's rules inside the preview iframe.

| Band        | Width range      | Gallery preset width |
| ----------- | ---------------- | -------------------- |
| Phone       | up to 480px      | 390                  |
| Large phone | 481px to 767px   | 600                  |
| Tablet      | 768px to 1099px  | 900                  |
| Desktop     | 1100px to 1679px | 1440                 |
| Full HD     | 1680px to 2199px | 1920                 |
| Wide        | 2200px to 3199px | 2200                 |
| 4K          | 3200px and wider | 3840                 |

## How a component styles itself

Every component owns its CSS. The CSS lives in a `styles/` directory next to the
component's `mod.rs`, and the component loads it with `document::Stylesheet`. No
component reaches into a parent for sizing, and no global stylesheet styles a
component on its behalf.

Inside `styles/` the split is one file per viewport, plus a base:

- `base.css` holds structure, color, and state rules that apply at every
  viewport. It also carries the fallback sizing as the default value of a
  self-owned custom property.
- `phone.css`, `large_phone.css`, `tablet.css`, `desktop.css`, `full_hd.css`,
  `wide.css`, `four_k.css` each wrap a single `@media` query for one band and set
  only what changes in that band.

The component lists all eight files in one `Asset` array and loads them together.

## Why a self-owned custom property

Sizes that change per band travel through a custom property the component
declares in its own `base.css` (for example `--command-tile-cap`). Each
per-viewport file overrides only that property. This keeps two things true at
once: a value can be set in one place and consumed by several rules, and the
component never depends on a parent setting the property, because its own
`base.css` provides the fallback. A component rendered on its own in the gallery
sizes correctly without any wrapper.
