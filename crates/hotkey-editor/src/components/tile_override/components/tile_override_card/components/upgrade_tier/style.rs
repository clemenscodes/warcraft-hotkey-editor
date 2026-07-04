use crate::{classes, styling::TailwindClass, tw};

// The tier-cycling footer: prev button, level caption, next button, centered and
// pushed to the bottom of the override card.
const BASE: &[TailwindClass] = tw![
    "mt-auto",
    "flex",
    "items-center",
    "justify-center",
    "gap-[0.85rem]",
    "pt-4",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
