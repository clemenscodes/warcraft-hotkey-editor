use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "gap-4",
    "flex-nowrap",
    "w-full",
    "min-w-0",
    "grow",
    "self-stretch",
];
// Phone/tablet: the five banners share one full-width row (no clumping, no
// horizontal scroll), with a tighter gap and a little breathing room below.
const MOBILE: &[&str] = &[
    "mobile:gap-[0.4rem]",
    "mobile:overflow-visible",
    "mobile:p-[0.15rem_0_0.4rem]",
];
const TABLET: &[&str] = &[
    "tablet:gap-[0.4rem]",
    "tablet:overflow-visible",
    "tablet:p-[0.15rem_0_0.4rem]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
