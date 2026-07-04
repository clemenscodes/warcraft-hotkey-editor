use crate::{classes, styling::TailwindClass, tw};

// A horizontal strip holding the mode toggle and race tabs, with a clamped height so
// the banners keep a consistent size. On phones it stacks into a column and drops the
// min-height.
const BASE: &[TailwindClass] = tw![
    "flex",
    "items-stretch",
    "flex-none",
    "gap-6",
    "min-h-[clamp(9rem,13vh,18rem)]",
];
const MOBILE: &[TailwindClass] = tw!["mobile:flex-col", "mobile:min-h-0", "mobile:gap-[0.85rem]"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
