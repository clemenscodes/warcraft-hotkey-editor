use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "text-[12px]",
    "tracking-[0.08em]",
    "uppercase",
    "text-warcraft-text-faint",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
