use crate::{classes, styling::TailwindClass, tw};

// The top row of the alt-state block: label on the left, the position button and key
// cell on the right.
const BASE: &[TailwindClass] = tw![
    "grid",
    "grid-cols-[minmax(0,1fr)_auto_auto]",
    "items-center",
    "gap-x-[0.85rem]",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
