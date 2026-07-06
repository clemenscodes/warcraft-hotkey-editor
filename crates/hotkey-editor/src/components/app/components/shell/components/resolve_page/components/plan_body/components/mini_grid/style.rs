use crate::{classes, styling::TailwindClass, tw};

// The move's mini grid frame: a query container the reused `Grid` fills. Its width
// sets the whole grid's scale (the tiles size in `cqi` off it), its height comes
// from the three rows of square tiles, and it is display-only, so pointer events
// pass through.
const BASE: &[TailwindClass] = tw![
    "flex-[1_1_auto]",
    "w-full",
    "max-w-[260px]",
    "min-w-0",
    "[container-type:inline-size]",
    "pointer-events-none",
    "p-[4px]",
    "bg-warcraft-bg-panel/70",
    "border",
    "border-warcraft-blue",
    "rounded-[4px]",
    // The reused tile painter sizes its border/corner for a full editor tile
    // (one tile-face). A mini grid has no tile-face, so the painter would resolve
    // those against this whole 4-column frame and render huge. Scale them back to
    // the frame here, the way they read before the editor was recalibrated.
    "[&_.empty-tile]:border-[0.35cqi]!",
    "[&_.filled-tile]:border-[0.35cqi]!",
    "[&_.empty-tile]:rounded-[1.04cqi]!",
    "[&_.filled-tile]:rounded-[1.04cqi]!",
];
const MOBILE: &[TailwindClass] = tw!["mobile:max-w-[max(130px,32vw)]"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
