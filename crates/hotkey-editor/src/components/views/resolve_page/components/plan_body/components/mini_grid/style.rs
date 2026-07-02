use crate::classes;

// The move's mini grid frame: a query container the reused `Grid` fills. Its width
// sets the whole grid's scale (the tiles size in `cqi` off it), its height comes
// from the three rows of square tiles, and it is display-only, so pointer events
// pass through.
const BASE: &[&str] = &[
    "flex-[1_1_auto]",
    "w-full",
    "max-w-[260px]",
    "min-w-0",
    "[container-type:inline-size]",
    "pointer-events-none",
    "p-[4px]",
    "bg-[rgba(20,35,60,0.7)]",
    "border",
    "border-[#2a5085]",
    "rounded-[4px]",
];
const MOBILE: &[&str] = &["mobile:max-w-[max(130px,32vw)]"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
