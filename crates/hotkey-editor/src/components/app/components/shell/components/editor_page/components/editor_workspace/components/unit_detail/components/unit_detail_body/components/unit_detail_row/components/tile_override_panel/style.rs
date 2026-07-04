use crate::{classes, styling::TailwindClass, tw};

// The right column of the unit-detail row: the "Hotkey override" heading and the
// override card (or its empty placeholder). On phones it becomes a sticky bottom
// sheet so the override stays reachable while the grid scrolls.
const BASE: &[TailwindClass] = tw!["flex", "flex-col", "min-w-0", "self-start"];
const MOBILE: &[TailwindClass] = tw![
    "mobile:w-full",
    "mobile:self-stretch",
    "mobile:items-stretch",
    "mobile:sticky",
    "mobile:bottom-0",
    "mobile:z-[100]",
    "mobile:m-[0_-0.85rem]",
    "mobile:pt-0",
];
const TABLET: &[TailwindClass] = tw!["tablet:w-full", "tablet:self-stretch", "tablet:pt-0"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
