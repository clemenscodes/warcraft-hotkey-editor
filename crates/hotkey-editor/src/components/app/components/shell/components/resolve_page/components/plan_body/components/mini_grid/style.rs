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
];
const MOBILE: &[TailwindClass] = tw!["mobile:max-w-[max(130px,32vw)]"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
