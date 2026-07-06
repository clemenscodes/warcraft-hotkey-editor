use crate::{classes, styling::TailwindClass, tw};

// The mini grid frame: a small query container the reused `Grid` fills. Its width
// sets the whole grid's scale (the tiles size in `cqi` off it), its height comes
// from the three rows of square tiles, and it is display-only, so pointer events
// pass through.
const BASE: &[TailwindClass] = tw![
    "shrink-0",
    "w-[calc(80px/3*4)]",
    "[container-type:inline-size]",
    "pointer-events-none",
    "p-[3px]",
    "bg-warcraft-bg-panel/70",
    "border",
    "border-warcraft-blue",
    "rounded-[3px]",
    // The reused tile painter sizes its border/corner for a full editor tile
    // (one tile-face). A mini grid has no tile-face, so the painter would resolve
    // those against this whole 4-column frame and render huge. Scale them back to
    // the frame here, the way they read before the editor was recalibrated.
    "[&_.empty-tile]:border-[0.35cqi]!",
    "[&_.filled-tile]:border-[0.35cqi]!",
    "[&_.empty-tile]:rounded-[1.04cqi]!",
    "[&_.filled-tile]:rounded-[1.04cqi]!",
];
const MOBILE: &[TailwindClass] = tw!["mobile:w-[calc(66px/3*4)]"];
const TABLET: &[TailwindClass] = tw!["tablet:w-[calc(92px/3*4)]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
