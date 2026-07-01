use crate::classes;

// The "Level N of M" caption between the tier arrows.
const BASE: &[&str] = &[
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.06em]",
    "text-[1.3rem]",
    "text-[#c0c8da]",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
