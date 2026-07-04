use crate::{classes, styling::TailwindClass, tw};

// The name-and-id column of the override header; centered with a touch floor on the
// mobile panel so the row height stays stable.
const BASE: &[TailwindClass] = tw!["flex", "flex-col", "items-start", "gap-[0.4rem]", "min-w-0"];

const MOBILE: &[TailwindClass] = tw![
    "mobile:gap-[2px]",
    "mobile:min-h-[44px]",
    "mobile:justify-center",
    "mobile:text-left",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:gap-[2px]",
    "tablet:min-h-[44px]",
    "tablet:justify-center",
    "tablet:text-left",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
