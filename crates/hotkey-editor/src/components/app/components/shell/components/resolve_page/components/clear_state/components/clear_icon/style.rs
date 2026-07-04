use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "inline-flex",
    "w-[3.5rem]",
    "h-[3.5rem]",
    "text-warcraft-gold",
    "[&_svg]:w-full",
    "[&_svg]:h-full",
    "[filter:drop-shadow(0_0_10px_rgba(255,206,99,0.45))]",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
