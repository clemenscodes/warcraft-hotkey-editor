use crate::{classes, styling::TailwindClass, tw};

// One fixed inter-control gap on every band: the icon buttons scale, but a few px of gap
// between them reads the same at any size, so this needs neither a per-band value nor a
// clamp — a single `gap-1` in BASE covers phone through 4K.
const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-row",
    "items-center",
    "justify-end",
    "gap-1",
    "min-w-0",
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
