use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
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
const MOBILE: &[TailwindClass] = tw![
    "mobile:gap-[0.4rem]",
    "mobile:overflow-visible",
    "mobile:p-[0.15rem_0_0.4rem]",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:gap-[0.4rem]",
    "tablet:overflow-visible",
    "tablet:p-[0.15rem_0_0.4rem]",
];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
