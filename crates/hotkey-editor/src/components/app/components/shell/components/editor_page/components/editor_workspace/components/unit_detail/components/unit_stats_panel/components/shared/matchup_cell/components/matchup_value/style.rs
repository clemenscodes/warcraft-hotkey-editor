use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "text-warcraft-text-secondary",
    "font-medium",
    "[font-variant-numeric:tabular-nums]",
    "flex-[0_0_auto]",
    "group-data-[matchup=strong]:text-warcraft-success",
    "group-data-[matchup=weak]:text-race-orc",
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
