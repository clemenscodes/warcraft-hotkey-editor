use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "text-[#d6dcec]",
    "font-medium",
    "[font-variant-numeric:tabular-nums]",
    "flex-[0_0_auto]",
    "group-data-[matchup=strong]:text-[#4ade80]",
    "group-data-[matchup=weak]:text-[#f87171]",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
