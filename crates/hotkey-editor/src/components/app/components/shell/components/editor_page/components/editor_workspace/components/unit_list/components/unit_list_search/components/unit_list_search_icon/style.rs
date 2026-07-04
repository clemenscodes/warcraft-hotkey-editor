use crate::{classes, styling::TailwindClass, tw};

// The magnifier glyph inside the mobile search pill. Absent on the sidebar (the
// search box there needs no icon), shown as a gold leading icon on small screens.
const BASE: &[TailwindClass] = tw![
    "hidden",
    "text-warcraft-gold",
    "pointer-events-none",
    "[&_svg]:block",
    "[&_svg]:w-full",
    "[&_svg]:h-full",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:block",
    "mobile:absolute",
    "mobile:left-3",
    "mobile:top-1/2",
    "mobile:-translate-y-1/2",
    "mobile:w-[18px]",
    "mobile:h-[18px]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:block",
    "tablet:absolute",
    "tablet:left-3",
    "tablet:top-1/2",
    "tablet:-translate-y-1/2",
    "tablet:w-[18px]",
    "tablet:h-[18px]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
