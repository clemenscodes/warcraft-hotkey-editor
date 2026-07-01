use crate::classes;

// The magnifier glyph inside the mobile search pill. Absent on the sidebar (the
// search box there needs no icon), shown as a gold leading icon on small screens.
const BASE: &[&str] = &[
    "hidden",
    "text-warcraft-gold",
    "pointer-events-none",
    "[&_svg]:block",
    "[&_svg]:w-full",
    "[&_svg]:h-full",
];

const MOBILE: &[&str] = &[
    "mobile:block",
    "mobile:absolute",
    "mobile:left-3",
    "mobile:top-1/2",
    "mobile:-translate-y-1/2",
    "mobile:w-[18px]",
    "mobile:h-[18px]",
];

const TABLET: &[&str] = &[
    "tablet:block",
    "tablet:absolute",
    "tablet:left-3",
    "tablet:top-1/2",
    "tablet:-translate-y-1/2",
    "tablet:w-[18px]",
    "tablet:h-[18px]",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
