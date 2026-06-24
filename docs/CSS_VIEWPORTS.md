# CSS viewport map

This is the map for the editor's responsive component CSS. It exists because
the unit-detail area supports many viewports and the rules used to be ordered
by cascade rather than by viewport, which made every edit risky. Read this
before touching any of:

- `crates/hotkey-editor/src/components/unit_detail/unit_detail.css`
- `crates/hotkey-editor/src/components/command_grid/command_grid.css`
- `crates/hotkey-editor/src/components/tile_override/tile_override.css`

## The rule

Every one of those files follows the same structure: one cosmetic base block
followed by one block per viewport band. **Layout lives in exactly one place.**
If you are changing how something looks at a given viewport, you edit that
viewport's block and nothing else.

```
@layer components {
  /* COSMETIC BASE - viewport independent */
  /* LAYOUT: phone        (max-width: 480px) */
  /* LAYOUT: large-phone  (481px - 700px) */
  /* LAYOUT: tablet       (701px - 1099px) */
  /* LAYOUT: desktop base (min-width: 1100px) */
  /* LAYOUT: desktop - tall-narrow (1100px - 1500px, min-height 1024px) */
  /* LAYOUT: desktop - wide-tall / 4K (min-width: 2000px) */
  /* LAYOUT: desktop - tall description (min-width: 1100px, min-height 1300px) */
}
```

## Cosmetic base vs layout

**Cosmetic base** holds everything that does not change across viewports:
colour, border, background, font family, text shadow, border radius, and all
interaction states (hover, focus, drag, selected, drop-target). Nothing
overrides these for layout, so they are safe to read once and ignore.

**Layout blocks** hold only the properties that drive geometry: `display`,
grid and flex templates, `--tile-size`, width and height, `gap`, padding that
affects spacing, `align-self`, `order`, and the responsive font sizes that
scale with the viewport.

## The viewport ladder (disjoint on width)

| Band         | Range                  | Role                                    |
|--------------|------------------------|-----------------------------------------|
| phone        | `max-width: 480px`     | everything stacks, finger-sized tiles   |
| large-phone  | `481px - 700px`        | stacked, sticky override panel          |
| tablet       | `701px - 1099px`       | grids 2-up, stats 2x2                    |
| desktop      | `min-width: 1100px`    | the main editor layout                  |

These four width bands never overlap, so editing one cannot affect another.

## Desktop sub-regions (min-width: 1100px)

Desktop is two-dimensional: width and height both drive it. It is modelled as
one self-contained **desktop base** (correct for normal landscape, e.g.
1920x937) plus a few **refinement blocks**. Each refinement is contiguous,
labelled with the region it targets, and overrides only a small known set of
properties. They are additive layers on the base, not disjoint regions, so
when editing the base remember a refinement may override a few props for its
region:

- **tall-narrow** - `1100px - 1500px` and `min-height: 1024px`. Portrait-ish
  desktop windows. Stats restack into one column; grids take the freed height.
- **wide-tall / 4K** - `min-width: 2000px`. Larger root font; grids scale up.
- **tall description** - `min-width: 1100px` and `min-height: 1300px`. Allows
  the unit description two lines instead of one.

## Why the bands are what they are

The boundaries (480 / 700 / 1099 / 1100 / 1500 / 2000) come from the union of
the breakpoints the three files already used, snapped to one shared ladder.
The old files mixed `max-width: 700` rules with `481px - 1099px` rules that
overlapped in the 481-700 range, and used `700` against `701` seams. The
ladder removes those overlaps so each band owns its range outright.

## Editing checklist

- Changing one viewport? Edit only that band's block.
- A property that should be identical everywhere (a colour, a border)? It
  belongs in the cosmetic base, not repeated per band.
- Adding a new breakpoint? Update this file first, then add the matching block
  in all three CSS files so they stay parallel.
- After any change, re-run `moon run hotkey-editor:tailwind/build` (dx serve
  does not compile CSS) and hard-refresh.
