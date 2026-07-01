use crate::classes;

// The "GRID LAYOUT" caption in the friz-quadrata display face.
const BASE: &[&str] = &[
    "font-friz-quadrata",
    "font-normal",
    "uppercase",
    "tracking-[0.12em]",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.6)]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
