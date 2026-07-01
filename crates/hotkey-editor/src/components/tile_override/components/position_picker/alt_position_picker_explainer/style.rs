use crate::classes;

// The instruction line at the top of a position-picker dialog.
const BASE: &[&str] = &[
    "m-0",
    "text-center",
    "max-w-[90rem]",
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.1em]",
    "text-[rgba(255,206,99,0.75)]",
    "text-[1.85rem]",
    "leading-[1.4]",
    "[text-shadow:1px_1px_0_#000]",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
