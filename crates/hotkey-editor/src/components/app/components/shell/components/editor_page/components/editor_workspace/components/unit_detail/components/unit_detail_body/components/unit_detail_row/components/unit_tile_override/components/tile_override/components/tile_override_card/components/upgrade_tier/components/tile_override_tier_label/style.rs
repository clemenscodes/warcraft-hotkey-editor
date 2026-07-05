use crate::{classes, styling::TailwindClass, tw};

// The "Level N of M" caption between the tier arrows.
const BASE: &[TailwindClass] = tw![
    "uppercase",
    "tracking-[0.06em]",
    "text-[1.3rem]",
    "text-warcraft-text-secondary",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
