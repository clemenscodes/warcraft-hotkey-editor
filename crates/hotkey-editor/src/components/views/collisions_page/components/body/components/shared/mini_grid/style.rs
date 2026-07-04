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
    "bg-[rgba(20,35,60,0.7)]",
    "border",
    "border-warcraft-blue",
    "rounded-[3px]",
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
