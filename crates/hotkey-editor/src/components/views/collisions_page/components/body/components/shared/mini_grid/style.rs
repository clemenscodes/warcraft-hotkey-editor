use crate::classes;

// The mini grid frame: a small query container the reused `Grid` fills. Its width
// sets the whole grid's scale (the tiles size in `cqi` off it), its height comes
// from the three rows of square tiles, and it is display-only, so pointer events
// pass through.
const BASE: &[&str] = &[
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
const MOBILE: &[&str] = &["mobile:w-[calc(66px/3*4)]"];
const TABLET: &[&str] = &["tablet:w-[calc(92px/3*4)]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
